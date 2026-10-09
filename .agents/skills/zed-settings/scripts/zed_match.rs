use regex::RegexBuilder;
use std::{
    env,
    ffi::OsString,
    io::{self, Write},
    process::ExitCode,
};

const HELP: &str = concat!(
    "Usage:\n",
    "  zed-match <case-sensitive> <pattern> <input>\n",
    "  zed-match --help\n",
    "\n",
    "Report whether one Rust regex matches one input string\n",
    "\n",
    "Arguments:\n",
    "  <case-sensitive>  `true` or `false`, applied as the initial case setting. Inline flags can override it\n",
    "  <pattern>         The regex, taken literally, including an empty value or one beginning with `--`\n",
    "  <input>           The string to search, taken literally without anchoring, trimming, or normalization\n",
    "\n",
    "Output:\n",
    "  A match prints `true` and a nonmatch prints `false` to standard output, followed by a newline\n",
    "  Invalid arguments or an invalid regex print a diagnostic to standard error and nothing to standard output\n",
    "\n",
    "Exit statuses:\n",
    "  0  The regex was evaluated, or help was displayed\n",
    "  1  Invalid arguments or an invalid regex\n",
);
const NAME: &str = "zed-match";

enum Route {
    Help,
    Match {
        case_sensitive: bool,
        input: String,
        pattern: String,
    },
}

fn parse_arguments(arguments: &[OsString]) -> Result<Route, String> {
    match arguments {
        [argument] if argument == "--help" => Ok(Route::Help),
        [case_sensitive, pattern, input] => {
            let case_sensitive = match case_sensitive.to_str() {
                Some("false") => false,
                Some("true") => true,
                _ => return Err("`<case-sensitive>` must be `true` or `false`".to_owned()),
            };
            let pattern = text(pattern, "<pattern>")?;
            let input = text(input, "<input>")?;

            Ok(Route::Match {
                case_sensitive,
                input,
                pattern,
            })
        }
        _ => Err("Expected `<case-sensitive> <pattern> <input>` or `--help` alone".to_owned()),
    }
}

fn text(argument: &OsString, name: &str) -> Result<String, String> {
    argument
        .to_str()
        .map(str::to_owned)
        .ok_or_else(|| format!("`{name}` must be valid UTF-8"))
}

fn run(arguments: &[OsString]) -> Result<String, String> {
    match parse_arguments(arguments)? {
        Route::Help => Ok(HELP.to_owned()),
        Route::Match {
            case_sensitive,
            input,
            pattern,
        } => {
            let regex = RegexBuilder::new(&pattern)
                .case_insensitive(!case_sensitive)
                .build()
                .map_err(|error| format!("Invalid regex: {error}"))?;

            Ok(format!("{}\n", regex.is_match(&input)))
        }
    }
}

fn main() -> ExitCode {
    let arguments: Vec<OsString> = env::args_os().skip(1).collect();
    let diagnostic = match run(&arguments) {
        Ok(output) => {
            let mut stdout = io::stdout().lock();
            match stdout
                .write_all(output.as_bytes())
                .and_then(|()| stdout.flush())
            {
                Ok(()) => return ExitCode::SUCCESS,
                Err(error) => format!("Failed to write to standard output: {error}"),
            }
        }
        Err(diagnostic) => diagnostic,
    };

    // The failure status still reports the error when standard error rejects the diagnostic
    let _ = writeln!(io::stderr(), "{NAME}: {diagnostic}");

    ExitCode::from(1)
}
