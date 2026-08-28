# Mount Identity Audit v0.1.0 — independent verification handoff

## Status: **FAIL — candidate deployed, release blocked**

Candidate `059bc082801b0150e93d5e8c4e71e9eb64e689c3` was independently tested on
2026-08-28 against https://devcontainer-identity-audit.sociobot.in/. The live
HTML, JS, CSS, hero, and service worker match the candidate build byte-for-byte,
and the prior rootless-Podman split-argument and production response-policy
defects are fixed.

Release remains blocked by a core false `PASS`: a valid `build`-backed Dev
Container with Dockerfile `USER 424242:424242`, no `remoteUser`, and a
`root:root` mode-`0755` workspace is reported writable as assumed `0:0`.
Supplying the real identity changes the same audit to `FAIL`. The safe unresolved
result must be `UNKNOWN`. Two additional P2 findings cover path leaks in
`--share` error output and clipped content at 200% mobile text size; undersized
link targets are P3.

Full evidence and exact reproductions are in
[`.factory/verification-2.md`](verification-2.md).

## Verification summary

- Clean candidate and `origin/main`: `059bc082801b0150e93d5e8c4e71e9eb64e689c3`.
- `npm ci`, `npm test`, TypeScript, 7 Vitest tests, 7 Rust unit tests, 6 Rust
  CLI integration tests, 11 applicable Playwright tests, `cargo fmt`, strict
  Clippy, `npm audit`, and exact `npm run build`: all passed.
- `cargo package`: passed (17 files, 126.7 KiB / 33.3 KiB compressed); clean
  install, public library API consumer, help/version, exit codes, Docker,
  Compose, rootless Podman, invalid/boundary/recovery, privacy, and no-mutation
  paths were exercised.
- Live 1440×900 and 390×844: no normal-size overflow, console/page/request
  errors, or axe violations; keyboard and reduced-motion behavior pass. Service
  worker update and offline reload pass. Demo interaction caused no request or
  storage writes. Security headers and immutable caching pass.
- Lighthouse mobile: 99 Performance / 100 Accessibility / 100 Best Practices /
  100 SEO; FCP 1.0 s, LCP 2.0 s, TBT 30 ms, CLS 0. JS, CSS, font, and hero
  budgets pass.

## Re-run

```sh
npm ci
npm test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
npm audit --audit-level=low
npm run build
cargo package
```

After the three documented fixes, repeat packaged-consumer CLI testing and the
live deployment equivalence/browser/header/PWA audit before changing this status
to PASS. Do not publish the crate while this handoff is FAIL.
