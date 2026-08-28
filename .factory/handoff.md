# Mount Identity Audit v0.1.0 — handoff

## What was built

- A typed Rust/Clap single-binary CLI, `mount-identity-audit`, with stable exit
  codes and a versioned `--json` report.
- JSONC Dev Container discovery plus Compose service, user, image, and bind-volume
  metadata support.
- Separate Docker/rootful and rootless Podman adapters. Runtime access is limited
  to `info`, `image inspect`, and `podman unshare cat /proc/self/{uid,gid}_map`;
  the tool never pulls, creates, starts, edits, chmods, or chowns.
- POSIX owner/group/other read, write, and traverse evaluation, declared
  read-only detection, `--remote-user` overrides, `--no-runtime`, quiet output,
  actionable safe remediations, and `--share` path redaction.
- Unit and integration coverage for JSONC, Compose, permissions, subuid maps,
  Docker offline mode, rootless Podman fixture behavior, redaction, malformed
  configuration, and exit codes.
- A static Vite documentation site at `dist/site` with a local identity-mapping
  demo, invalid/empty/offline states, responsive 390 px layout, keyboard focus,
  privacy and terms pages, CSP/security headers, and a versioned offline cache.
- An original 1200×800 halftone hero at `site/public/mount-ledger.webp` (216,498
  bytes). Its exact prompt and factory deployment provenance are recorded in
  `.factory/design.md` and `site/public/mount-ledger.source.json`.
- MIT license, changelog, complete usage/readme, and ready-to-package crate.

## Run and verify

From a clean clone with stable Rust, Node 20+, npm, and the pinned Playwright
Chromium available:

```sh
npm ci
npm test
npm run build
cargo package
```

Outputs:

- CLI: `target/release/mount-identity-audit`
- Static deploy root: `dist/site/index.html`
- Publish-ready crate: `target/package/mount-identity-audit-0.1.0.crate`

Verification completed on 2026-08-28:

- `npm test`: 11 Rust tests, 5 browser-model tests, and 11 applicable Playwright
  tests passed (the desktop run intentionally skips one mobile-only assertion).
- Playwright ran desktop and 390×844 Chromium, full axe scans, error/empty/offline
  flows, legal routes, interaction flow, overflow checks, and console capture.
  Serious/critical axe violations: 0. Browser console errors: 0.
- `/opt/fleet/lib/verify-url.sh`: HTTP 200, valid title and `lang`, exactly one
  `<h1>`, `<main>` present, all images have alt text, no unlabeled buttons, no
  console errors.
- Lighthouse 12.8.2 mobile simulation: Performance **99**, Accessibility **100**,
  Best Practices **100**, SEO **100**; FCP 1.0 s, LCP 2.1 s, TBT 0 ms, CLS 0.
  INP is not produced for a navigation-only lab trace; demo interactions are
  synchronous and the browser suite confirms immediate state changes.
- Production payloads: initial JS 4.75 KB raw / 2.11 KB gzip; CSS 11.70 KB raw /
  3.30 KB gzip; eager hero WebP 216,498 bytes. All are within budget.
- `npm audit`: 0 vulnerabilities.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo package`: 17 files, 122.8 KiB raw / 32.4 KiB compressed, verification
  build passed. The worker did not publish it.

## Known v1 gaps

- Named image users cannot be safely resolved from OCI metadata alone. The CLI
  returns `UNKNOWN` and requests `--remote-user UID:GID` instead of starting a
  container or guessing. Likewise, Dev Containers' later `updateRemoteUserUID`
  mutation is not assumed during preflight.
- POSIX ACLs, SELinux/AppArmor labels, remote filesystem policy, and mutations
  performed during container creation are outside this ownership/mode model and
  appear in report caveats.
- Native Windows and Docker Desktop macOS filesystem translation are not modeled;
  v1 targets Linux hosts and WSL2.
- Compose merging covers the chosen service fields needed for this audit, not
  every Compose merge/interpolation rule. Unusual generated configurations can
  use explicit `--config`, `--workspace`, and `--remote-user` inputs.

## Suggested next steps

1. Run the stated 20-repository pilot across current Docker and rootless Podman
   versions, recording false positives/negatives without collecting path names.
2. Add opt-in read-only ACL and security-label diagnostics where platform APIs
   make the result trustworthy.
3. Publish the verified crate through factory-owned registry credentials and add
   release binaries for Linux targets.
