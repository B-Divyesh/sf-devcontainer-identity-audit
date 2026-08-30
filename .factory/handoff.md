# Mount Identity Audit — independent verification 5 handoff

## Status: FAIL — do not release

Requested candidate `d82877c57de48b2f68055e551f2b5e9ea1b2badf`
cannot be fetched from the provided repository and is absent from all remote
refs. The live deployment is healthy and byte-identical to the site built from
reachable `main` revision `d82877ddec3ba114f820fee6870cbc5d96587be1`, but
that cannot establish a match to the requested candidate.

The full evidence and reproductions are in
[`.factory/verification-5.md`](verification-5.md).

## Release blockers

- `.factory/claims.json` is missing, so the mandatory first claim-test gate
  cannot run despite many user-facing claims.
- The cold first screen does not plainly name its audience and has no one-click
  “Try it with sample data” action.
- The CLI has no demo command or bundled sample; `.factory/demo.md` is missing.
  Live `/demo` is only the home fallback and has none of the required isolated
  demo banner/reset/exit controls.
- CLI and live direct-Docker calculations accept reserved Linux UID/GID
  `4294967295` and report `PASS`.
- Unknown web routes return the home page with HTTP 200; required canonical and
  social metadata, consistent legal-page shell, footer build ID, and
  `.factory/copy-audit.md` are absent.

## Verification summary

Against reachable `d82877d…`:

- `npm ci`: passed, 59 packages, 0 vulnerabilities.
- `npm test`: passed 24 Rust tests, TypeScript, 8 Vitest tests, and 22 applicable
  Playwright cases; 4 desktop executions of mobile-only checks were skipped.
- `cargo fmt --check`, strict Clippy, `npm audit`, and exact `npm run build`:
  passed.
- `cargo package --locked`: passed; the packed crate clean-installed and both
  public API and CLI smoke tests passed.
- Normal, read-only, malformed, invalid-identity, redaction, and 24-process
  concurrency/read-only cases behaved as documented. The max-ID boundary case
  exposed the false `PASS` above.
- Production Playwright passed desktop/mobile functionality, keyboard, 200%
  reflow, touch targets, axe, privacy, service-worker update, and offline reload.
- Browser traffic was same-origin only; cookies and all tested browser storage
  remained empty. Security headers and cache revalidation are present.
- Lighthouse mobile: Performance 99, Accessibility 100, Best Practices 100,
  SEO 100; LCP 1.980 s, TBT 67 ms, CLS 0, 227,061 B transfer.
- Live/local SHA-256 hashes match for HTML, JS, CSS, hero, and service worker.

Docker and Podman daemons were unavailable in the verifier container, so live
runtime adapters were covered by repository fixtures. The site is static with
no server endpoint or sign-in, making rate-limit and identity-provider checks
not applicable.

## Reverify

After publishing the corrected candidate:

```sh
npm ci
npm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
npm audit --audit-level=low
npm run build
cargo package --locked
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in npx playwright test
```

Before rerunning the general suite, execute every command in the newly required
`.factory/claims.json` through the documented CLI demo entry point.
