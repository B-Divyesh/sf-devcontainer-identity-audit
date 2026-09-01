# Copy audit

Audited 2026-09-01 against `site/index.html`, `site/src/main.ts`, and
`README.md`. Counts use this tokenizer: every whitespace-delimited token that
contains a Unicode letter or number counts once; punctuation does not. Headings,
buttons, navigation, field labels, table cells, and code examples are checked
separately because they are not sentences.

## Landing sentences

| Words | Sentence |
| ---: | --- |
| 2 | You’re offline. |
| 9 | The demo still works locally; install links may not. |
| 5 | Checks without starting a container. |
| 6 | Check mount permissions before container startup. |
| 17 | For developers using Dev Containers or rootless Podman who need a writable workspace on the first open. |
| 6 | Runs a known rootless Podman mismatch. |
| 5 | Free under the MIT License. |
| 9 | The browser sample works offline after the first visit. |
| 7 | The browser sample sends no project data. |
| 10 | Host identity and container identity meet at one bind mount. |
| 14 | This browser check uses the same owner, group, and mode rules as the CLI. |
| 7 | It stores and sends nothing you enter. |
| 10 | Three or four octal digits, such as 0755 or 0775. |
| 10 | Use numeric identities—the CLI refuses to guess named users. |
| 11 | Enter the caller identity and range starts from /etc/subuid and /etc/subgid. |
| 9 | Run the sample mismatch or load the safe example. |
| 12 | The report will explain the effective host identity and exact permission branch. |
| 12 | Find the remote user and workspace bind in JSONC or Compose metadata. |
| 12 | Ask Docker or Podman for runtime metadata and rootless user namespace ranges. |
| 12 | Compare the effective host UID/GID with owner, group, and other permission bits. |
| 6 | Docker and Podman are separate cases. |
| 7 | The report names the mapping it used. |
| 12 | It never treats container user 1000 as host user 1000 without evidence. |
| 18 | The audit does not inspect ACLs, security labels, remote filesystem policy, or changes made while a container starts. |
| 6 | It never changes ownership or permissions. |
| 11 | The browser sample stays in memory and sends no project data. |
| 5 | Example output from mount-identity-audit --demo. |
| 6 | Local identity evidence for mounted workspaces. |

## README sentences

| Words | Sentence |
| ---: | --- |
| 15 | Mount Identity Audit checks bind-mount ownership before a Dev Container or rootless Podman workspace starts. |
| 10 | It compares host permissions with the mapped numeric container identity. |
| 15 | It is for developers and CI maintainers who want a numeric ownership report before startup. |
| 5 | The CLI needs no account. |
| 7 | It includes no HTTP or telemetry client. |
| 11 | Docker or Podman is optional when every identity is supplied numerically. |
| 10 | Run one command without pointing the audit at your project. |
| 9 | The command copies examples/mismatch/ into a unique temporary directory. |
| 8 | It prints that path and audits the copy. |
| 19 | The known write mismatch returns FAIL with exit code 1, the same as a project with a confirmed mismatch. |
| 17 | The browser sample also accepts the host caller identity and the allocated /etc/subuid and /etc/subgid range starts. |
| 22 | These values let it distinguish the identity kept by Podman keep-id from other remote users, which still map through the subordinate ranges. |
| 8 | The CLI reads the live runtime map instead. |
| 5 | Run it from a repository. |
| 7 | The CLI discovers .devcontainer/devcontainer.json, .devcontainer.json, or devcontainer.json. |
| 10 | It follows Compose metadata when the configuration names a service. |
| 18 | Use an explicit numeric identity when an image stores only a user name or is not available locally. |
| 4 | Use --json for scripts. |
| 9 | Add --share to replace local paths with neutral labels. |
| 12 | --share replaces host paths, repository names, and config paths with neutral labels. |
| 7 | JSON output is versioned with schema_version: 1. |
| 16 | The process never runs chown, edits configuration, pulls an image, creates a container, or starts one. |
| 9 | An audit makes at most three read-only runtime calls. |
| 11 | Those calls use info, image inspect, or rootless Podman identity maps. |
| 10 | An explicit Dev Container remoteUser is the intended editor identity. |
| 12 | It stays authoritative when a selected Compose service declares a different user. |
| 7 | Otherwise, that Compose service user overrides containerUser. |
| 12 | The CLI uses containerUser only when neither of those values is present. |
| 16 | Named image users and UID-only values do not prove a primary GID without running a container. |
| 14 | The audit returns UNKNOWN and requests --remote-user UID:GID; it never invents a same-number group. |
| 11 | Linux reserves ID 4294967295, so the CLI rejects it as UNKNOWN. |
| 15 | POSIX ACLs, security labels, remote filesystems, and startup mutations remain outside the v1 permission model. |
| 6 | Every detailed report states these limits. |
| 10 | Build-backed configurations without an explicit numeric user also return UNKNOWN. |
| 13 | The CLI never trusts a possibly stale image tag as current build evidence. |
| 12 | Requirements: a Linux host (or WSL2), stable Rust, Node 20+, and npm. |
| 12 | Docker Desktop filesystem translation on native macOS/Windows is outside the v1 model. |
| 4 | Run all repository checks. |
| 13 | npm run build produces the release binary and the deployable site at dist/site. |
| 8 | Run the site locally with npm run dev. |
| 16 | Prepare the unpublished registry artifact with cargo package; publishing is intentionally left to the Param Factory. |
| 8 | The CLI includes no network or telemetry client. |
| 12 | The browser sample stores no entered values and sends no project data. |
| 7 | After the first visit, it reloads offline. |
| 8 | Use --share before attaching reports to public issues. |
| 7 | Every public promise is listed in .factory/claims.json. |
| 14 | Run one claim with its listed command or run all coverage with npm test. |

## Labels, terminology, and results

Literal labels: `Try it with sample data`, `Copy install command`, `Check mount
permissions`, `Permission report`, `Check numeric workspace access`, `Read the
runtime identity map`, `Read workspace ownership and mode`, `Reset demo`, and
`Open blank browser check`.

| Concept | Term |
| --- | --- |
| Host directory mounted into a container | workspace |
| UID/GID transformation | identity map |
| Container configuration | Dev Container |
| Rootless preservation mode | keep-id |
| Browser try-out | browser sample |
| Command-line try-out | bundled sample |
| Access results | PASS, FAIL, UNKNOWN |

## Result

- Sentences over 22 words: **0**.
- Banned terms: **0**.
- The first screen contains the job headline, audience sentence, sample action,
  next-step note, and three plain facts at 390 px and desktop widths.
