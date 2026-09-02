# Mount Identity Audit — independent verification 16 handoff

## Status: PASS

Candidate `f0338c40b7ad66de276fd87da66db0afd11a8bf9` was independently verified on 2 September 2026 UTC at <https://devcontainer-identity-audit.sociobot.in/>. No release-blocking defect was found. Product code and infrastructure were not changed.

The full evidence and defect accounting are in [`.factory/verification-16.md`](verification-16.md).

## What was verified

- Required cold first-read and one-click sample-data flow on desktop and 390×844 mobile: PASS.
- Every `.factory/claims.json` command after clean dependency installation: 24/24 PASS.
- `npm test`: PASS — 13 Rust unit, 23 CLI integration, 33 Vitest, and 80 applicable Playwright checks; eight intentional viewport skips.
- `npm run lint`, `npm run copy:audit:check`, `npm audit --audit-level=low`, and exact `npm run build`: PASS.
- Cargo package verification and a fresh consumer install: PASS; one 1,142,536-byte executable with working help, version, demo, JSON, and exit codes.
- The prior rootless Podman `keep-id` false-PASS case: repaired. IDs below, equal to, and above the kept identity map correctly in the packed CLI and browser.
- Production desktop/mobile, keyboard, focus, reduced motion, 200% reflow, touch targets, error recovery, Axe, offline reload, service-worker update, privacy request/storage probe, headers, caching, and 404: PASS.
- All 17 public deployment files match fresh local build bytes; every route identifies build `f0338c40b7ad`.
- Lighthouse: 97 performance, 100 accessibility, 100 best practices, 100 SEO; LCP 1.97 seconds, TBT 175.5 ms, CLS 0.

## How to reproduce

```sh
npm ci
npm test
npm run lint
npm run copy:audit:check
npm audit --audit-level=low
npm run build
cargo package --locked --allow-dirty
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in npx playwright test
```

Run each command in `.factory/claims.json` separately for the registered claim gate. Run the CLI sample with `target/release/mount-identity-audit --demo`; its intentional mismatch returns exit 1.

## Defects and next steps

Critical: none. High: none. Medium: none. Low: none.

Version 1 deliberately excludes POSIX ACLs, security labels, remote filesystem policy, and identity changes made during container startup. Detailed reports state these limits. No registry publish was attempted; Param Factory can publish the verified crate with `cargo package --locked`.

No out-of-scope resource, service setting, secret, database, staging slot, or storage account was read or changed.
