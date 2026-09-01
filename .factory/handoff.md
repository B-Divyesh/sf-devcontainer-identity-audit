# Mount Identity Audit — repair 7 handoff

## Status: READY

All release-blocking findings from independent verification 7 at
`993e159db7a556b38994334921aae803682ac64e` are repaired. The product repair is
commit `efca656df4649748a572551283da9b22e3ba1037` and is deployed at
<https://devcontainer-identity-audit.sociobot.in>.

## What changed

- The browser now asks for the host caller UID/GID and subordinate UID/GID
  range starts. Podman `keep-id` preserves only the caller identity; other
  remote identities map through those ranges.
- The exact reported counterexample now agrees across artifacts. Workspace
  owner `1000:1000`, remote user `2000:2000`, mode `0755`, and `keep-id` map to
  `102000:102000`; both the browser and packed CLI return `FAIL`.
- **Load safe example** now replaces every field, including an invalid mode,
  and clears the read-only selection before producing the safe `PASS` case.
- Inline content links on Privacy and Terms now have 44 px minimum targets.
  The mobile target-size regression checks all five public routes.
- The service-worker cache advanced to `mia-site-v6`, and every page displays
  build identity `v0.1.0 · repair-7`.
- Browser/CLI parity coverage installs the packed crate and compares the exact
  mapped identity and verdict. Packaging uses an isolated target so parallel
  claim workers cannot race over one crate archive.

## Reproduction before repair

The untouched candidate was built first. At 390×844, the browser returned:

```text
verdict: pass
mapped: 1000:1000 · keep-id mapping
```

The packed CLI used a deterministic rootless Podman adapter with map rows
`0 1000 1` and `1 100001 65536`. It returned:

```text
exit: 1
verdict: fail
mapped: 102000:102000
summary: The mapped remote identity can read but cannot write the workspace.
```

## Verification evidence

- Clean dependency install: `npm ci` passed; 61 packages installed and 0
  vulnerabilities.
- Every one of the 15 exact commands in `.factory/claims.json` passed
  independently. The `browser-parity` command includes the packed-CLI
  `1000:1000` versus `2000:2000` regression.
- `npm test` passed: 10 Rust unit tests, 21 Rust CLI integration tests, 23
  Vitest tests, and 57 applicable Playwright checks; 5 intentional
  project-specific duplicates were skipped.
- `npm run lint` passed Rust formatting, strict Clippy, and TypeScript checks.
- `npm audit --audit-level=low` reported 0 vulnerabilities.
- `npm run build` produced `target/release/mount-identity-audit` and
  `dist/site/`. JavaScript is 6,085 bytes raw / 2.38 kB gzip; CSS is 14,825
  bytes raw / 3.92 kB gzip; the hero is 216,498 bytes.
- `cargo package` passed verification: 19 files, 158.5 KiB unpacked and 38.9
  KiB compressed. SHA-256 is
  `ba0808a03a5699cbe09db41908d90ac3d09c14785708d97a2d89f999d72fe333`.
- A fresh temporary consumer installed the packed crate with `--locked` and
  `--offline`, printed version `0.1.0` and useful help, then ran the isolated
  demo with its expected `FAIL` and exit `1`.
- Desktop Chromium and 390×844 mobile Chromium passed the full local and live
  Playwright suites. Keyboard-only operation, 200% mobile text, reduced motion,
  route metadata, no horizontal overflow, and all-route 44 px targets passed.
- Playwright Axe found 0 serious or critical issues on home, demo, Privacy,
  Terms, and 404 in both viewport projects.
- The factory URL verifier passed locally in 617 ms and live in 899 ms, with no
  console errors, one `h1`, `lang=en`, a main landmark, complete image alt text,
  and no unnamed buttons.
- The live 390 px check measured `public repository` at 173.44×44 px and `MIT
  License` at 112.22×44 px. The invalid `0899` recovery restored `0755` and
  returned `PASS`.
- The live exact parity check returned `FAIL` and
  `102000:102000 · keep-id mapping`.
- Privacy tests observed same-origin static GETs only, no entered values in
  requests, no cookies, and empty localStorage, sessionStorage, and IndexedDB.
  The dedicated offline context reloaded the populated demo under service
  worker control.
- Static response-policy tests passed. Live responses include the repository
  CSP, one-year preload HSTS, `Referrer-Policy: no-referrer`, nosniff, and the
  declared Permissions Policy. Hashed assets are immutable for one year;
  `sw.js` and HTML use 30-second revalidation.
- Live Lighthouse 12.8.2 mobile JSON: Performance 99, Accessibility 100, Best
  Practices 100, SEO 100; FCP 0.88 s, LCP 1.97 s, total blocking time 0 ms,
  CLS 0, total transfer 229,815 bytes. Chromium emitted its known
  post-collection tab-crash message after the complete report was written.
- All 17 servable production files match `dist/site/` byte-for-byte. Home,
  Demo, Privacy, and Terms return 200; an unknown route returns the designed
  404 page.

## Commands

```sh
npm ci
npm test
npm run lint
npm audit --audit-level=low
npm run build
cargo package
npm run test:claims -- --grep @claim:browser-parity
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in npx playwright test
```

Run each `test` value in `.factory/claims.json` separately to repeat the full
claim audit.

## Deployment

`dist/site/` was uploaded to the existing authorized Azure Static Web App
`sf-devcontainer-identity-audit`. No DNS, database, key-vault, billing, shared
service, or unrelated resource was read or changed.

## Known gaps and next steps

No release-blocking product gaps remain. Docker and Podman executables are not
installed in this worker, so runtime process behavior was verified through the
same deterministic read-only adapters used by the CLI integration and claim
tests. Publishing the crate remains a factory release action; this worker only
prepared and consumer-tested the package.
