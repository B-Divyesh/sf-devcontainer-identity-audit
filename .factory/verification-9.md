# Independent verification 9 — Mount Identity Audit

## Verdict: PASS

Candidate commit `2d9bcf5f1ba0b524f67eb4d4a287d4ba0eb320c6` was
independently verified on 1 September 2026 UTC against
<https://devcontainer-identity-audit.sociobot.in/>. The deployed product is
byte-identical to the candidate's public build files. No release-blocking defect
was found.

## First-read acceptance

A cold 1440×900 visit answered all three required questions in the first screen:

- What it does: **“Check mount permissions before container startup.”**
- Who it is for: developers using Dev Containers or rootless Podman who need a
  writable workspace on first open.
- What to do first: **“Try it with sample data”**, followed by “Runs a known
  rootless Podman mismatch.”

The action takes one click. It opens `/?demo=1#demo`, immediately reports the
sample as `FAIL`, and shows the persistent “Demo — sample data, nothing is
saved” banner with **Reset demo** and **Open blank browser check**. The same
first-screen content is present at 390×844.

## Claims

`.factory/claims.json` is present with 22 valid entries. After the required
clean-checkout dependency install, every exact `test` command was run
independently. Result: **22/22 PASS**.

| Claim | Result |
| --- | --- |
| `cli-demo` | PASS |
| `browser-demo` | PASS |
| `permission-verdicts` | PASS |
| `read-only-safety` | PASS |
| `config-support` | PASS |
| `compose-user-precedence` | PASS |
| `share-redaction` | PASS |
| `report-contract` | PASS |
| `runtime-mapping` | PASS |
| `conservative-identities` | PASS |
| `browser-private` | PASS |
| `cli-private` | PASS |
| `offline-reload` | PASS |
| `browser-parity` | PASS |
| `mit-license` | PASS |
| `browser-report-details` | PASS |
| `config-discovery` | PASS |
| `runtime-optional` | PASS |
| `report-limits` | PASS |
| `compose-build-image` | PASS |
| `install-binary` | PASS |
| `build-artifacts` | PASS |

The first attempted command before `npm ci` reached the site build but could not
launch the absent local `vite` dependency (`vite: not found`, exit 127); no claim
assertion ran. `npm ci` then installed the lockfile with zero reported
vulnerabilities, and all 22 exact manifest commands passed. This is a normal
uninstalled-clone prerequisite, not a failed product claim.

Cross-checking the landing page, legal pages, and README found each substantive
promise represented by the claim registry. The browser/CLI parity test uses
real packed-CLI output, not only the presence of controls.

## Repository gates and packaged consumer

- `npm ci`: PASS — 61 packages installed; zero audit findings.
- `npm test`: PASS — 10 Rust unit, 21 Rust integration, 23 Vitest, and 74
  Playwright checks passed; 6 project-specific checks were intentionally skipped
  in the non-applicable viewport project.
- `npm run lint`: PASS — Rust format, strict Clippy, and TypeScript checks.
- `npm run build`: PASS — created `target/release/mount-identity-audit` and
  `dist/site/`.
- `cargo package --allow-dirty`: PASS — 19 files, 158.5 KiB unpacked and 38.9
  KiB compressed; Cargo's package verification build passed.
- Clean consumer install from `target/package/mount-identity-audit-0.1.0`: PASS.
  It installed one 1,145,624-byte executable, displayed useful `--help`, and ran
  the bundled isolated demo with the documented `FAIL`/exit 1 result.

Independent CLI cases confirmed owner access (exit 0), group `0770` access
(exit 0), other `0007` access (exit 0), a readable/non-writable `0755` mismatch
(exit 1), UID-only and reserved-ID inputs (exit 2), missing configuration (exit
2), and successful recovery on the next corrected invocation (exit 0). JSON
reports retained schema version 1 and the expected identity/mode evidence.

## Live browser and accessibility

`PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in npx
playwright test site/e2e/site.spec.ts` passed 32 checks with 4 intentional
desktop skips for mobile-only assertions. Coverage included:

- normal mismatch and safe-result flows;
- invalid octal and reserved-ID errors with recovery;
- desktop and 390×844 responsive layouts, 200% text reflow, and 44 px targets;
- keyboard-only operation, visible skip-link focus, and reduced motion;
- browser history/focus restoration, legal routes, designed 404, and no console
  or page errors;
- service-worker update followed by an offline `/demo/` reload.

Playwright Axe reported zero violations, including zero serious or critical
findings, on home, demo, Privacy, Terms, and 404 at both viewports. The factory
URL verifier returned HTTP 200 in 704 ms with `lang=en`, one H1, a main landmark,
no missing image alternatives, no unnamed buttons, and no console errors.

All discovered internal and external links returned HTTP 200. An unknown path
returned the designed page with HTTP 404.

## Privacy, headers, and endpoint scope

The recorded live demo request log contained only same-origin GETs for the HTML,
hashed JavaScript/CSS, hero image, and terminal illustration. Entering the unique
value `3141592` and running the audit added **zero** requests; the value appeared
in no request. Cookies, localStorage, sessionStorage, and IndexedDB remained
empty. Console and page error lists were empty.

The document response includes a self-only CSP with `frame-ancestors 'none'`,
HSTS preload, `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`,
and a restrictive permissions policy. HTML and `sw.js` use 30-second
revalidation. Hashed JS, CSS, and the hashed hero use one-year immutable caching.

This is a static product with no server-side or unlock endpoint, account system,
or sign-in. API allowance/429, backend concurrency/persistence, and Entra tenant
checks therefore do not apply. The brief does not benefit from a model-backed
step, so the absence of an AI feature is appropriate.

## Deployment identity and performance

Seventeen public deployment files were fetched and compared byte-for-byte with
`dist/site`, including every HTML route, JS, CSS, service worker, images, icons,
robots, sitemap, provenance file, and `_headers`; all matched. The deployment
configuration file correctly is not served.

| Budget or metric | Result |
| --- | ---: |
| JavaScript | 6,805 bytes raw / 2,708 bytes gzip |
| CSS | 14,859 bytes raw / 3,943 bytes gzip |
| Hero WebP | 216,498 bytes |
| Lighthouse performance | 95 |
| Lighthouse accessibility | 100 |
| Lighthouse best practices | 100 |
| Lighthouse SEO | 100 |
| FCP | 1.4 s |
| LCP | 2.0 s |
| Total blocking time | 210 ms |
| CLS | 0 |
| Total transfer | 225 KiB |

The release stays comfortably inside the JavaScript, CSS, hero, LCP, and CLS
budgets. No hosted fonts are used.

## Defects by severity

- Critical: none.
- High: none.
- Medium: none.
- Low/advisory: Lighthouse estimates about 151 KiB mobile transfer savings from
  a smaller responsive hero source. The current 216,498-byte WebP and 2.0 s LCP
  remain within contract budgets.
- Low/advisory: Lighthouse's experimental `label-content-name-mismatch` audit
  flags the decorative `M↔I` text inside the home link because its accessible
  name is “Mount Identity Audit home.” The mark is explicitly `aria-hidden`, the
  destination has a clear accessible name, and standard Axe reports no
  violation, but a future revision could use a non-text decorative mark or align
  the visible and accessible labels for voice-control parity.
