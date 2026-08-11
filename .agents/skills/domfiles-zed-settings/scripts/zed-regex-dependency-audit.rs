use regex::Regex;
use serde::Deserialize;
use std::{
    ffi::{OsStr, OsString},
    fs,
    io::Write,
    path::{Path, PathBuf},
};

const HELP: &str = concat!(
    "Usage: zed-regex-dependency-audit --local-manifest <path> --upstream-lock <path> --upstream-revision <commit>\n",
    "\n",
    "Audit the direct Zed-compatible `regex` dependency version\n",
    "\n",
    "Options:\n",
    "  --help                         Print help\n",
    "  --local-manifest <path>        Read the local `regex` pin and adjacent `Cargo.lock`\n",
    "  --upstream-lock <path>         Read the upstream locked `regex` version\n",
    "  --upstream-revision <commit>   Identify the upstream Zed commit\n",
    "\n",
    "Exit statuses:\n",
    "  0  Versions matched or help displayed\n",
    "  1  Versions differed\n",
    "  2  Invalid arguments or data, or an I/O failure\n",
);

const STATUS_ERROR: u8 = 2;
const STATUS_MATCH: u8 = 0;
const STATUS_MISMATCH: u8 = 1;

struct VersionArguments {
    local_manifest: PathBuf,
    upstream_lock: PathBuf,
    upstream_revision: String,
}

struct ManifestPackage {
    name: String,
    version: String,
}

struct PackageIdentity {
    name: String,
    version: String,
    source: Option<String>,
}

struct DependencyReference {
    name: String,
    version: Option<String>,
    source: Option<String>,
}

struct LockedPackage {
    identity: PackageIdentity,
    dependencies: Vec<DependencyReference>,
}

#[derive(Deserialize)]
struct LockDocument {
    #[serde(rename = "version")]
    _version: u64,
    package: Vec<LockPackage>,
}

#[derive(Deserialize)]
struct LockPackage {
    name: String,
    version: String,
    source: Option<String>,
    #[serde(default)]
    dependencies: Vec<String>,
}

struct VersionComparison {
    local_version: String,
    upstream_version: String,
}

enum ParsedArguments {
    Help,
    Run(VersionArguments),
}

fn parse_arguments<I>(arguments: I) -> Result<ParsedArguments, String>
where
    I: IntoIterator<Item = OsString>,
{
    let arguments: Vec<OsString> = arguments.into_iter().collect();

    if arguments.len() == 1 && arguments[0].as_os_str() == OsStr::new("--help") {
        return Ok(ParsedArguments::Help);
    }

    let mut arguments = arguments.into_iter();
    let mut local_manifest = None;
    let mut upstream_lock = None;
    let mut upstream_revision = None;

    while let Some(argument) = arguments.next() {
        let Some(option) = argument.to_str() else {
            return Err("Option names must be valid UTF-8".to_owned());
        };

        match option {
            "--help" => {
                return Err("Option `--help` must be used alone".to_owned());
            }
            "--local-manifest" => {
                if local_manifest.is_some() {
                    return Err("Option `--local-manifest` may be specified only once".to_owned());
                }

                let Some(path) = arguments.next() else {
                    return Err("Option `--local-manifest` requires a path".to_owned());
                };
                local_manifest = Some(PathBuf::from(path));
            }

            "--upstream-lock" => {
                if upstream_lock.is_some() {
                    return Err("Option `--upstream-lock` may be specified only once".to_owned());
                }

                let Some(path) = arguments.next() else {
                    return Err("Option `--upstream-lock` requires a path".to_owned());
                };
                upstream_lock = Some(PathBuf::from(path));
            }
            "--upstream-revision" => {
                if upstream_revision.is_some() {
                    return Err(
                        "Option `--upstream-revision` may be specified only once".to_owned()
                    );
                }

                let Some(revision) = arguments.next() else {
                    return Err("Option `--upstream-revision` requires a commit".to_owned());
                };
                let Some(revision) = revision.to_str() else {
                    return Err("The upstream revision must be valid UTF-8".to_owned());
                };
                upstream_revision = Some(revision.to_owned());
            }
            _ => {
                return Err(format!(
                    "Unknown option `{option}`. Run `zed-regex-dependency-audit --help` for usage"
                ));
            }
        }
    }

    let local_manifest = local_manifest
        .ok_or_else(|| "Missing required option `--local-manifest <path>`".to_owned())?;
    let upstream_lock = upstream_lock
        .ok_or_else(|| "Missing required option `--upstream-lock <path>`".to_owned())?;
    let upstream_revision = upstream_revision
        .ok_or_else(|| "Missing required option `--upstream-revision <commit>`".to_owned())?;

    let revision_pattern = Regex::new(r"^[0-9a-f]{7,40}$").expect("Revision pattern must compile");
    if !revision_pattern.is_match(&upstream_revision) {
        return Err(
            "The upstream revision must be a 7- to 40-character lowercase hexadecimal commit"
                .to_owned(),
        );
    }

    Ok(ParsedArguments::Run(VersionArguments {
        local_manifest,
        upstream_lock,
        upstream_revision,
    }))
}

fn read_utf8_file(path: &Path, description: &str) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| {
        format!(
            "Failed to read {description} file `{}`:\n\n{error}",
            path.display()
        )
    })?;

    String::from_utf8(bytes).map_err(|error| {
        format!(
            "Invalid UTF-8 in {description} file `{}`:\n\n{error}",
            path.display()
        )
    })
}

pub(crate) fn local_regex_version(manifest: &str) -> Result<String, String> {
    let document: toml::Value = toml::from_str(manifest)
        .map_err(|error| format!("Local manifest is invalid TOML: {error}"))?;
    let dependency = document
        .get("dependencies")
        .and_then(toml::Value::as_table)
        .and_then(|dependencies| dependencies.get("regex"))
        .ok_or_else(|| {
            "Local manifest must contain an exact `[dependencies].regex` requirement".to_owned()
        })?;
    let requirement = match dependency {
        toml::Value::String(requirement) => Some(requirement.as_str()),
        toml::Value::Table(dependency) => dependency.get("version").and_then(toml::Value::as_str),
        _ => None,
    }
    .ok_or_else(|| {
        "Local manifest `[dependencies].regex` must contain an exact `=VERSION` requirement"
            .to_owned()
    })?;
    let version = requirement
        .strip_prefix('=')
        .filter(|version| !version.is_empty());

    version.map(str::to_owned).ok_or_else(|| {
        "Local manifest `[dependencies].regex` must contain an exact `=VERSION` requirement"
            .to_owned()
    })
}

fn local_manifest_package(manifest: &str) -> Result<ManifestPackage, String> {
    let mut in_package = false;
    let mut name = None;
    let mut package_sections = 0;
    let mut version = None;

    for line in manifest.lines() {
        let line = line.trim_end_matches('\r');
        if line == "[package]" {
            in_package = true;
            package_sections += 1;
            continue;
        }
        if line.starts_with('[') {
            in_package = false;
            continue;
        }
        if !in_package {
            continue;
        }

        if let Some(value) = line
            .strip_prefix("name = \"")
            .and_then(|value| value.strip_suffix('"'))
            && name.replace(value.to_owned()).is_some()
        {
            return Err("Local manifest package contains more than one `name` field".to_owned());
        }
        if let Some(value) = line
            .strip_prefix("version = \"")
            .and_then(|value| value.strip_suffix('"'))
            && version.replace(value.to_owned()).is_some()
        {
            return Err("Local manifest package contains more than one `version` field".to_owned());
        }
    }

    if package_sections != 1 {
        return Err("Local manifest must contain exactly one `[package]` section".to_owned());
    }

    Ok(ManifestPackage {
        name: name.ok_or_else(|| "Local manifest package has no quoted `name`".to_owned())?,
        version: version
            .ok_or_else(|| "Local manifest package has no quoted `version`".to_owned())?,
    })
}

fn parse_dependency_reference(value: &str) -> Result<DependencyReference, String> {
    let (package_and_version, source) = if let Some((package, source)) = value.rsplit_once(" (") {
        let source = source
            .strip_suffix(')')
            .ok_or_else(|| format!("Invalid lockfile dependency reference `{value}`"))?;
        (package, Some(source.to_owned()))
    } else {
        (value, None)
    };
    let (name, version) = match package_and_version.split_once(' ') {
        Some((name, version))
            if !name.is_empty() && !version.is_empty() && !version.contains(' ') =>
        {
            (name, Some(version.to_owned()))
        }
        None if !package_and_version.is_empty() => (package_and_version, None),
        _ => return Err(format!("Invalid lockfile dependency reference `{value}`")),
    };

    Ok(DependencyReference {
        name: name.to_owned(),
        version,
        source,
    })
}

fn parse_lock_packages(lockfile: &str, description: &str) -> Result<Vec<LockedPackage>, String> {
    let document: LockDocument = toml::from_str(lockfile)
        .map_err(|error| format!("{description} lockfile is invalid TOML: {error}"))?;

    document
        .package
        .into_iter()
        .map(|package| {
            let dependencies = package
                .dependencies
                .iter()
                .map(|dependency| parse_dependency_reference(dependency))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(LockedPackage {
                identity: PackageIdentity {
                    name: package.name,
                    version: package.version,
                    source: package.source,
                },
                dependencies,
            })
        })
        .collect()
}

fn resolve_dependency(
    dependency: &DependencyReference,
    packages: &[LockedPackage],
    description: &str,
    parent: &PackageIdentity,
) -> Result<usize, String> {
    let candidates = packages
        .iter()
        .enumerate()
        .filter(|(_, package)| {
            package.identity.name == dependency.name
                && dependency
                    .version
                    .as_ref()
                    .is_none_or(|version| package.identity.version == *version)
                && dependency
                    .source
                    .as_ref()
                    .is_none_or(|source| package.identity.source.as_ref() == Some(source))
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    match candidates.as_slice() {
        [index] => Ok(*index),
        [] => Err(format!(
            "{description} lockfile cannot resolve dependency `{}` from `{} {}`",
            dependency.name, parent.name, parent.version
        )),
        _ => Err(format!(
            "{description} lockfile resolves dependency `{}` from `{} {}` ambiguously",
            dependency.name, parent.name, parent.version
        )),
    }
}

fn locked_regex_version(
    lockfile: &str,
    description: &str,
    local_package: Option<&ManifestPackage>,
) -> Result<String, String> {
    let packages = parse_lock_packages(lockfile, description)?;

    if let Some(local_package) = local_package {
        let root_packages = packages
            .iter()
            .filter(|package| {
                package.identity.name == local_package.name
                    && package.identity.version == local_package.version
                    && package.identity.source.is_none()
            })
            .collect::<Vec<_>>();
        let root_package = match root_packages.as_slice() {
            [package] => *package,
            [] => {
                return Err(format!(
                    "{description} lockfile does not contain source-less root package `{} {}`",
                    local_package.name, local_package.version
                ));
            }
            _ => {
                return Err(format!(
                    "{description} lockfile contains multiple source-less root packages named `{} {}`",
                    local_package.name, local_package.version
                ));
            }
        };
        let regex_dependencies = root_package
            .dependencies
            .iter()
            .filter(|dependency| dependency.name == "regex")
            .collect::<Vec<_>>();
        let dependency = match regex_dependencies.as_slice() {
            [dependency] => *dependency,
            [] => {
                return Err(format!(
                    "{description} lockfile root package `{} {}` does not depend on `regex`",
                    local_package.name, local_package.version
                ));
            }
            _ => {
                return Err(format!(
                    "{description} lockfile root package `{} {}` contains multiple `regex` dependency references",
                    local_package.name, local_package.version
                ));
            }
        };
        let regex_index =
            resolve_dependency(dependency, &packages, description, &root_package.identity)?;

        return Ok(packages[regex_index].identity.version.clone());
    }

    let mut versions = packages
        .iter()
        .filter(|package| package.identity.name == "regex")
        .map(|package| package.identity.version.clone())
        .collect::<Vec<_>>();
    versions.sort();
    versions.dedup();

    match versions.as_slice() {
        [version] => Ok(version.to_owned()),
        [] => Err(format!(
            "{description} lockfile does not contain a `regex` package"
        )),
        _ => {
            let versions = versions
                .iter()
                .map(|version| format!("`{version}`"))
                .collect::<Vec<_>>()
                .join(", ");
            Err(format!(
                "{description} lockfile contains multiple `regex` versions: {versions}"
            ))
        }
    }
}

#[cfg(test)]
pub(crate) fn upstream_regex_version(lockfile: &str) -> Result<String, String> {
    locked_regex_version(lockfile, "Upstream", None)
}

fn evaluate_versions(arguments: &VersionArguments) -> Result<VersionComparison, String> {
    let local_manifest = read_utf8_file(&arguments.local_manifest, "local manifest")?;
    let local_lock_path = arguments.local_manifest.with_file_name("Cargo.lock");
    let local_lock = read_utf8_file(&local_lock_path, "local lockfile")?;
    let upstream_lock = read_utf8_file(&arguments.upstream_lock, "upstream lockfile")?;
    let local_package = local_manifest_package(&local_manifest)?;
    let local_version = local_regex_version(&local_manifest)?;
    let locked_local_version = locked_regex_version(&local_lock, "Local", Some(&local_package))?;
    let upstream_version = locked_regex_version(&upstream_lock, "Upstream", None)?;

    if local_version != locked_local_version {
        return Err(format!(
            "Local manifest pins `regex` `{local_version}`, but adjacent `Cargo.lock` resolves `{locked_local_version}`"
        ));
    }

    Ok(VersionComparison {
        local_version,
        upstream_version,
    })
}

fn report_error(stderr: &mut dyn Write, message: &str) {
    let _ = writeln!(stderr, "zed-regex-dependency-audit: {message}");
}

fn run_version_audit(
    arguments: &VersionArguments,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
) -> u8 {
    let comparison = match evaluate_versions(arguments) {
        Ok(comparison) => comparison,
        Err(error) => {
            report_error(stderr, &error);
            return STATUS_ERROR;
        }
    };

    if comparison.local_version != comparison.upstream_version {
        if writeln!(
            stderr,
            "Zed commit `{}` uses `regex` `{}`, but `Cargo.toml` pins `{}`",
            arguments.upstream_revision, comparison.upstream_version, comparison.local_version
        )
        .is_err()
        {
            return STATUS_ERROR;
        }

        return STATUS_MISMATCH;
    }

    if let Err(error) = writeln!(
        stdout,
        "Zed commit `{}` and `Cargo.toml` use `regex` `{}`",
        arguments.upstream_revision, comparison.local_version
    ) {
        report_error(stderr, &format!("Failed to write result:\n\n{error}"));
        return STATUS_ERROR;
    }

    STATUS_MATCH
}

pub(crate) fn run<I>(arguments: I, stdout: &mut dyn Write, stderr: &mut dyn Write) -> u8
where
    I: IntoIterator<Item = OsString>,
{
    let parsed_arguments = match parse_arguments(arguments) {
        Ok(parsed_arguments) => parsed_arguments,
        Err(error) => {
            report_error(stderr, &error);
            return STATUS_ERROR;
        }
    };

    let ParsedArguments::Run(arguments) = parsed_arguments else {
        if let Err(error) = stdout.write_all(HELP.as_bytes()) {
            report_error(stderr, &format!("Failed to write help:\n\n{error}"));
            return STATUS_ERROR;
        }

        return STATUS_MATCH;
    };

    run_version_audit(&arguments, stdout, stderr)
}

#[cfg(not(test))]
fn main() -> std::process::ExitCode {
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    let mut stdout = stdout.lock();
    let mut stderr = stderr.lock();

    std::process::ExitCode::from(run(std::env::args_os().skip(1), &mut stdout, &mut stderr))
}
