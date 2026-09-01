# Landing-page copy audit

Audited 2026-09-01 against `site/index.html`, `site/src/main.ts`, and
`site/src/audit.ts`. Counts use whitespace-separated words. UI labels and table
fragments are listed separately because they are not sentences.

## Visible prose

| Words | Sentence |
| ---: | --- |
| 2 | You’re offline. |
| 7 | The demo still works locally; install links may not. |
| 6 | Check mount permissions before container startup. |
| 17 | For developers using Dev Containers or rootless Podman who need a writable workspace on the first open. |
| 6 | Runs a known rootless Podman mismatch. |
| 5 | Free under the MIT License. |
| 9 | The browser sample works offline after the first visit. |
| 7 | The browser demo sends no project data. |
| 10 | Host identity and container identity meet at one bind mount. |
| 14 | This browser check uses the same owner, group, and mode rules as the CLI. |
| 6 | It stores and sends nothing you enter. |
| 11 | Three or four octal digits, such as 0755 or 0775. |
| 9 | Use numeric identities—the CLI refuses to guess named users. |
| 11 | Enter the caller identity and range starts from `/etc/subuid` and `/etc/subgid`. |
| 8 | Run the sample mismatch or load the safe example. |
| 11 | The report will explain the effective host identity and exact permission branch. |
| 12 | Find the remote user and workspace bind in JSONC or Compose metadata. |
| 12 | Ask Docker or Podman for runtime metadata and rootless user namespace ranges. |
| 12 | Compare the effective host UID/GID with owner, group, and other permission bits. |
| 6 | Docker and Podman are separate cases. |
| 7 | The report names the mapping it used. |
| 12 | It never treats “container user 1000” as “host user 1000” without evidence. |
| 17 | The audit does not inspect ACLs, security labels, remote filesystem policy, or changes made while a container starts. |
| 6 | It never changes ownership or permissions. |
| 9 | The browser sample stays in memory and sends no project data. |
| 7 | Recorded from the real binary with `mount-identity-audit --demo`. |
| 10 | Demo — bundled sample data; your project was not read or changed. |
| 12 | The mapped remote identity can read but cannot write the workspace. |
| 6 | Local identity evidence for mounted workspaces. |

## Dynamic result and error prose

| Maximum words | Copy family |
| ---: | --- |
| 10 | Numeric fields explain that values must be non-negative numbers. |
| 10 | Reserved-ID errors require a value below Linux’s `4294967295` sentinel. |
| 12 | Mode errors require three or four octal digits and give `0755` as an example. |
| 11 | A passing report says the mapped identity can read, write, and traverse the workspace. |
| 12 | A passing remedy says no ownership change is indicated and asks for a real CLI check. |
| 10 | A read-only report states that the mount flag overrides matching ownership. |
| 11 | Its remedy says to remove that flag only when workspace edits are intended. |
| 10 | A read failure states that the mapped identity cannot read and traverse the directory. |
| 13 | A write failure states that the identity can read but cannot create or edit entries. |
| 13 | Remedies name either Podman keep-id, the workspace owner, or deliberate host group access. |
| 9 | The unexpected-error fallback asks the user to reload and try again. |

## UI labels and fragments

Primary terms stay short and literal: `Try it with sample data`, `Copy`,
`Run preflight`, `Load safe example`, `Read config`, `Read the map`,
`Read the inode`, `Reset demo`, `Start for real`, `PASS`, `FAIL`, and `UNKNOWN`.
The numbered decorative labels from the previous page were removed.

## Banned-word and length result

- Sentences over 22 words: **0**.
- Banned terms: **0** (`leverage`, `seamless`, `effortless`, `robust`,
  `powerful`, `intuitive`, `reimagine`, `supercharge`, `unlock`, `delightful`,
  `journey`, `ecosystem`, `AI-powered`).
- First screen: the six-word job headline, 17-word audience sentence, sample
  action, next-step note, and three plain facts all fit in one screen.

## Terminology table

| Concept | One term used |
| --- | --- |
| Host directory mounted into a container | workspace |
| UID/GID transformation | identity map |
| Container development configuration | Dev Container |
| Rootless Podman preservation mode | keep-id |
| Browser try-out | browser sample |
| Shipped command-line try-out | bundled sample |
| Result that proves access | PASS |
| Result that proves an access problem | FAIL |
| Result without enough evidence | UNKNOWN |
