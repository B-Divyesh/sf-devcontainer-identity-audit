# Mount Identity Audit v0.1.0 — independent verification 3 handoff

## Status: FAIL

Candidate `8e0296b7b32d57aba2441d1fdc8c80bee7913b53` was independently
verified on 2026-08-28 UTC against
https://devcontainer-identity-audit.sociobot.in/. The live static artifacts
match the candidate byte-for-byte, and every repository-declared automated
quality command passes. Release is blocked by two fresh core CLI false-PASS
defects.

## Release blockers

- **P1:** `remoteUser: "1000"` is treated as `1000:1000` without GID evidence.
  A `root:1000` mode-`0770` workspace returned exit 0/PASS, while the plausible
  actual identity `1000:2000` returned exit 1/FAIL.
- **P1:** a valid Compose service with both `build` and `image` trusts the
  current local image user even when the Dockerfile changes the effective user.
  A stale root image produced exit 0/PASS for a Dockerfile using
  `424242:424242` on `root:root 0755`; the explicit effective user produced
  exit 1/FAIL.

The live demo also overflows maximum rootless IDs yet reports PASS (P2), and its
reduced-motion rule permanently exposes the skip link over the header (P3).
Exact reproductions and all passing evidence are in
`.factory/verification-3.md`.

## Verification summary

- Clean detached checkout at the exact candidate; Rust/Cargo 1.98.0,
  Node 22.23.2, npm 10.9.8, Playwright 1.58.2.
- `npm ci`, `npm test`, `cargo fmt --all -- --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`,
  `npm audit --audit-level=low`, and exact `npm run build` all passed.
- `cargo package` verified 17 files (135.8 KiB unpacked / 34.9 KiB compressed).
  A clean install and external public-API consumer passed. No package was
  published.
- Live Playwright: 17 applicable tests passed on desktop and 390×844 mobile;
  axe found 0 serious/critical issues across home and legal pages; privacy,
  keyboard, 200% reflow, service-worker update, and offline reload passed.
- Live headers and caching meet policy. Lighthouse mobile scored 99 performance,
  100 accessibility, 100 best practices, and 100 SEO (LCP 2.0 s, CLS 0).
- Live and local hashes match for HTML, JS, CSS, hero, and service worker.

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

Outputs are `target/release/mount-identity-audit`, `dist/site/`, and
`target/package/mount-identity-audit-0.1.0.crate`. Fix both P1 findings and add
the listed regressions before another release decision.
