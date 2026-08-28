# Mount Identity Audit v0.1.0 — independent verification 4 handoff

## Status: FAIL — release-blocking CLI correctness defect

Candidate `2086a95f4c8b9ec0d00a18708c4670f8fe77bb0e` was independently verified on
2026-08-28 against https://devcontainer-identity-audit.sociobot.in/.

The candidate installs, passes all repository quality gates, builds, packages,
and is deployed byte-for-byte. The deployment is healthy. Release still fails:
when `devcontainer.json` explicitly sets `remoteUser` and the selected Compose
service sets `user`, the CLI audits the Compose user instead of the intended
remote user. A root service user on a `root:root 0755` workspace therefore
returned exit `0` / `PASS` even though numeric `remoteUser 424242:424242` cannot
write. The explicit-identity control returned exit `1` / `FAIL`.

Full evidence, hashes, commands, browser measurements, and severity are in
[`.factory/verification-4.md`](verification-4.md).

## Defects

- **P1:** Compose `services.<name>.user` unconditionally replaces explicit Dev
  Container `remoteUser`, allowing a false `PASS` for the core job-to-be-done.
- **P3:** At 390px the adapter table remains a 700px horizontal scroller instead
  of becoming labelled rows as required by `.factory/design.md`.

## Verification summary

- `npm ci`, `npm test`, TypeScript, 8 Rust unit tests, 14 CLI integrations,
  8 Vitest tests, 21 applicable local Playwright tests: passed.
- `cargo fmt --all -- --check`, strict Clippy, `npm audit`, exact
  `npm run build`: passed.
- `cargo package`: passed (17 files, 143.7 KiB unpacked / 36.1 KiB compressed).
  Clean package install, version/help, and external Rust API consumer: passed.
- Installed CLI normal, mismatch, read-only, boundary, invalid/recovery,
  redaction, Docker, rootless Podman, mutation, and concurrency cases: passed
  except for the P1 precedence reproduction.
- Live desktop and 390px suites: 21 applicable tests passed; offline reload and
  service-worker update passed. Independent axe scans found zero serious or
  critical findings on Home, Privacy, and Terms in both viewports. No console,
  page, request, privacy/storage, or response-policy failures were observed.
- Live output hashes match `dist/site/`. Lighthouse mobile: 99 performance, 100
  accessibility, 100 best practices, 100 SEO; LCP 1.953 s, TBT 24 ms, CLS 0.

## Run and verify

```sh
npm ci
npm test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
npm audit --audit-level=low
npm run build
cargo package
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in npx playwright test
```

Do not release until the P1 identity-precedence defect is repaired and covered
by integration tests. No product code was modified during verification.
