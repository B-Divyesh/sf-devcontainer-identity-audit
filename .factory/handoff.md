# Mount Identity Audit — verification 12 handoff

## Status: FAIL

Candidate `90190fb5224abf8f0a30c097ad8570a36ec83e61` was independently
verified on 2 September 2026 UTC against
<https://devcontainer-identity-audit.sociobot.in/>. No product code or
infrastructure was changed.

The release is blocked because Docker daemon `userns-remap` is treated as a
direct UID/GID map. A fresh packed-binary fixture whose Docker `info` response
contained `SecurityOptions: ["name=userns"]` returned `PASS`/exit 0 for
container `1000:1000` on a host `1000:1000` workspace. Under user namespace
remapping those identities are not direct. The CLI must resolve the remap or
return `UNKNOWN`; it must not report a safe mount.

A second, medium-severity defect affects recovery: an explicitly read-only
workspace correctly returns `FAIL`, but both suggested fixes change identity or
host mode. Neither addresses the read-only declaration. The CLI should identify
the relevant `readonly`, `read_only`, or `ro` configuration instead.

## Verification completed

- All 22 exact `.factory/claims.json` commands passed separately after
  `npm ci`.
- `npm test` passed: 10 Rust unit, 21 Rust integration, 27 Vitest, and 78
  Playwright tests, with six intentional skips.
- `npm run lint`, `npm run copy:audit:check`, `npm run build`,
  `cargo package --locked --allow-dirty`, and `npm audit --audit-level=low`
  passed.
- The packaged crate installed one CLI in a clean prefix. Demo, pass, fail,
  world-writable, invalid JSONC, reserved ID, read-only, and share-redaction
  cases were exercised with stable exits.
- The cold first-read and one-click sample gate passed on desktop and 390 px
  mobile.
- The live browser suite passed 36 checks with four intentional skips. Axe had
  zero serious/critical findings; keyboard, focus, reduced motion, 200% reflow,
  44 px targets, route navigation, validation recovery, and offline reload
  passed.
- Live demo traffic was same-origin only; no entered value was sent or stored.
  Cookies and Web Storage/IndexedDB remained empty.
- Security headers and cache behavior passed. All 18 browser-served build files
  matched production byte-for-byte. A real unknown URL returned HTTP 404.
- Lighthouse: Performance 99, Accessibility 100, Best Practices 100, SEO 100;
  LCP 2.0 s, TBT 0 ms, CLS 0.

Full commands, measurements, reproduction steps, and severity details are in
[`.factory/verification-12.md`](verification-12.md).

## Required next steps

1. Detect Docker `name=userns` and return `UNKNOWN` unless its host mapping is
   proven. Add a registered regression claim for this exact case.
2. Give read-only failures a remediation that addresses the mount declaration,
   with CLI regression coverage.
3. Rerun every claim command and the complete verification matrix, then deploy
   the repaired site if its browser artifact changes. Registry publishing
   remains a factory-owned action.
