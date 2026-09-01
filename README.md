# Mount Identity Audit

Mount Identity Audit checks bind-mount ownership before a Dev Container or
rootless Podman workspace starts. It compares host permissions with the mapped
numeric container identity.

It is for developers and CI maintainers who would rather see a precise ownership
report than discover a `Permission denied` after startup.

Live documentation and one-click sample:
[devcontainer-identity-audit.sociobot.in/demo/](https://devcontainer-identity-audit.sociobot.in/demo/)

## Install

Build the single binary with a stable Rust toolchain:

```sh
cargo install --path .
mount-identity-audit --help
```

The CLI needs no account. It includes no HTTP or telemetry client. Docker or
Podman is optional when every identity is supplied numerically.

## Try the bundled sample

Run one command without pointing the audit at your project:

```sh
mount-identity-audit --demo
```

The command copies `examples/mismatch/` into a unique temporary directory. It
prints that path and audits the copy. The known write mismatch returns `FAIL`
with exit code `1`; this preserves the normal scripting contract.

## Usage

Run it from a repository. The CLI discovers `.devcontainer/devcontainer.json`,
`.devcontainer.json`, or `devcontainer.json`. It follows Compose metadata when
the configuration names a service.

```sh
mount-identity-audit .
```

Typical report:

```text
MOUNT IDENTITY AUDIT                       PASS

CHECK          EXPECTED             OBSERVED              STATUS
runtime        podman (rootless)     podman 5.2.2           PASS
remote user    1000:1000             1000:1000 on host      PASS
workspace      read + write          drwxr-xr-x 1000:1000   PASS

The intended remote user can read and write this bind mount.
No files were changed. No container was started.
```

Use an explicit numeric identity when an image stores only a user name or is not
available locally:

```sh
mount-identity-audit . --runtime podman --remote-user 1000:1000
```

Scriptable JSON and share-safe output are first-class:

```sh
mount-identity-audit . --json > audit.json
mount-identity-audit . --json --share > audit-for-issue.json
```

`--share` replaces host paths, repository names, and config paths with neutral
labels. JSON output is versioned with `schema_version: 1`.

```text
Usage: mount-identity-audit [OPTIONS] [PROJECT]

Arguments:
  [PROJECT]  Project directory to inspect [default: .]

Options:
      --demo                   Run the bundled sample in an isolated temporary project
      --config <FILE>          Dev Container configuration to read
      --workspace <PATH>       Host workspace path when it cannot be inferred
      --remote-user <UID:GID>  Numeric intended container identity
      --runtime <RUNTIME>      Runtime adapter [default: auto] [docker|podman|auto]
      --runtime-bin <PATH>     Runtime executable (useful for wrappers and tests)
      --no-runtime             Do not inspect an installed container runtime
      --json                   Emit stable machine-readable JSON
      --share                  Redact local path names in any output
  -q, --quiet                  Print only the verdict and primary explanation
  -h, --help                   Print help
  -V, --version                Print version
```

Exit codes are stable:

- `0`: the mount is predicted readable and writable;
- `1`: a definite permission or read-only failure was found;
- `2`: the audit could not reach a safe conclusion (configuration, runtime, or
  named-user resolution error).

The process never runs `chown`, edits configuration, pulls an image, creates a
container, or starts one. An audit makes at most three read-only runtime calls.
Those calls use `info`, `image inspect`, or rootless Podman identity maps.

## What it understands

- JSON-with-comments Dev Container files;
- `remoteUser`, `containerUser`, `image`, `build`, `workspaceFolder`,
  `workspaceMount`, `runArgs`, `dockerComposeFile`, and `service`;
- Compose `services.<name>.user`, `image`, `build`, `volumes`, and `read_only`;
- Docker’s direct Linux ID mapping;
- rootful Podman and rootless Podman’s live UID/GID maps, including both
  `--userns=keep-id` / `--userns=host` and split `--userns keep-id` /
  `--userns host` intent;
- host owner/group/mode and read-only mount declarations.

An explicit Dev Container `remoteUser` is the intended editor identity. It
stays authoritative when a selected Compose service declares a different
`user`. Otherwise, that Compose service `user` overrides `containerUser`.
The CLI uses `containerUser` only when neither of those values is present.

Named image users and UID-only values do not prove a primary GID without
running a container. The audit returns `UNKNOWN` and requests
`--remote-user UID:GID`; it never invents a same-number group. Linux reserves
ID `4294967295`, so the CLI rejects it as `UNKNOWN`.

POSIX ACLs, security labels, remote filesystems, and startup mutations remain
outside the v1 permission model. Every detailed report states these limits.
Build-backed configurations without an explicit numeric user also return
`UNKNOWN`. The CLI never trusts a possibly stale image tag as current build
evidence.

## Develop, test, and build

Requirements: a Linux host (or WSL2), stable Rust, Node 20+, and npm. Docker
Desktop filesystem translation on native macOS/Windows is outside the v1 model.

```sh
npm install
npm test
npm run lint
npm run build
```

`npm test` runs Rust unit, integration, claim, and browser tests. `npm run build`
produces the release binary and the deployable site at `dist/site`. Run the
site locally with `npm run dev`.

To build only one artifact:

```sh
cargo build --release
npm run build:site
```

Prepare the unpublished registry artifact with `cargo package`; publishing is
intentionally left to the Param Factory.

## Privacy and security

The CLI includes no network or telemetry client. The browser sample stores no
entered values and sends no project data. After the first visit, it reloads
offline. Use `--share` before attaching reports to public issues.

Every public promise is listed in [`.factory/claims.json`](.factory/claims.json).
Run one claim with its listed command or run all coverage with `npm test`.

## License

[MIT](LICENSE) © 2026 Sociobot (Param Factory).
