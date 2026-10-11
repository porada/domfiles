# Manage Task Artifacts

Create scratch storage only when needed. Follow an applicable project storage policy. Otherwise create a fresh `.agent-<task>` directory under the project root, establish task ownership, and put a single `*` line in its `.gitignore` before other writes. Reuse only after verifying task continuity and that all contents remain ignored. A matching name alone establishes no ownership, and ignore status establishes no privacy.

Preserve other tasks’ contents, keep writes inside the owned directory, and do not activate copied instructions or configuration as governing guidance. Bound validation inputs so scratch copies do not enter unrelated scans. Retain artifacts needed for handoff or expected reuse. Remove only owned disposable content after its consumers no longer need it, keeping the ignore file until the directory is removed. Do not sweep up other task directories or leave durable project deliverables solely in scratch storage.
