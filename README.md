# Mount Identity Audit

Mount Identity Audit is a read-only preflight for Dev Containers and rootless
Podman workspaces. Before an editor opens or a container starts, it compares the
workspace’s host ownership and mode bits with the numeric identity that the
configured remote user will have at the bind mount.

It is for developers and CI maintainers who would rather see a precise ownership
report than discover a `Permission denied` after startup.

Live documentation and local demo:
[devcontainer-identity-audit.sociobot.in](https://devcontainer-identity-audit.sociobot.in)

## Install

Build the single binary with a stable Rust toolchain:

```sh
cargo install --path .
mount-identity-audit --help
```

No daemon, account, network request, or telemetry is involved. Docker or Podman
is optional when every identity is supplied numerically.

## Usage

Run it from a repository. The CLI discovers `.devcontainer/devcontainer.json`,
`.devcontainer.json`, or `devcontainer.json`, then follows Compose metadata when
the configuration names a Compose service.

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
container, or starts one. Runtime calls are limited to `info`, `image inspect`,
and, for rootless Podman, `unshare … /proc/self/{uid,gid}_map`.

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

Named image users and UID-only values do not prove a primary GID without
running a container. In either case the audit returns `UNKNOWN` and tells you
to provide `--remote-user UID:GID`; it never invents a same-number group. POSIX
ACLs, SELinux/AppArmor labels, remote filesystems, and runtime mutations
performed during container creation remain outside the v1 permission model and
are called out in every detailed report. Build-backed configurations without an
explicit numeric user also return `UNKNOWN`, even when Compose also declares an
`image` tag: the CLI never builds an image, assumes its Dockerfile user is root,
or trusts a possibly stale tag as evidence of current build inputs.

## Develop, test, and build

Requirements: a Linux host (or WSL2), stable Rust, Node 20+, and npm. Docker
Desktop filesystem translation on native macOS/Windows is outside the v1 model.

```sh
npm install
npm test
npm run build
```

`npm test` runs Rust unit/integration tests and site tests. `npm run build`
produces the release binary and the deployable documentation site at
`dist/site/index.html`. Run the site locally with `npm run dev`.

To build only one artifact:

```sh
cargo build --release
npm run build:site
```

Prepare the unpublished registry artifact with `cargo package`; publishing is
intentionally left to the Param Factory.

## Privacy and security

All analysis runs locally. The CLI has no network client and no telemetry. The
website’s demo runs entirely in the browser, stores nothing, and sends no
project contents. Use `--share` before attaching reports to public issues.

## License

[MIT](LICENSE) © 2026 Sociobot (Param Factory).
