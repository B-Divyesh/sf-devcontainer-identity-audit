# Independent verification 10 — Mount Identity Audit

## Verdict: PASS

Candidate commit `989e74d76a5c1b709f705b6538d712b8084d41ea` was
independently verified on 2 September 2026 UTC against
<https://devcontainer-identity-audit.sociobot.in/>. The deployed product is
byte-identical to the candidate's served build files. No release-blocking
defect was found.

## First-read acceptance

A cold 1440×900 visit answered the three required questions in the first
screen:

- What it does: **“Check mount permissions before container startup.”**
- Who it is for: developers using Dev Containers or rootless Podman who need a
  writable workspace on first open.
- What to select first: **“Try it with sample data”**, followed by “Runs a known
  rootless Podman mismatch.”

In my own words: this predicts whether the intended remote user can write to a
workspace bind mount before a Dev Container or rootless Podman container starts.
It is for developers who otherwise discover identity mismatches only after
opening a project. The first action is the sample-data link.

That action takes one click to `/?demo=1#demo`. It immediately shows `FAIL`, the
mapped host identity `100999:100999`, the `read · no write · traverse` branch,
and a persistent “Demo — sample data, nothing is saved” banner with **Reset
demo** and **Open blank browser check**. At 390×844, the result title ends at
430 px, mapped identity at 617 px, and access result at 694 px, all inside the
first post-click viewport. First-read screenshots were captured at
`/tmp/mia-verify-10/live-cold-desktop.png`,
`/tmp/mia-verify-10/live-demo-one-click.png`, and
`/tmp/mia-verify-10/live-mobile-demo.png`.

## Registered claims

`.factory/claims.json` is present with 22 entries. From the clean candidate,
`npm ci` installed the lockfile with zero audit findings. Every entry's exact
`test` command was then run separately against the declared demo entry point.
Result: **22/22 PASS**. The combined command output is at
`/tmp/mia-verify-10/claim-tests.log`.

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

The landing page, Privacy, Terms, README, CLI help, and report output were
cross-checked against the registry. Functional, privacy, offline, licensing,
format, and safety promises are represented by claim entries. No unlisted
substantive claim was found. `npm run copy:audit:check` also passed; the
generated audit has no sentence above 22 words and no banned term.

## Clean repository gates

- `npm ci`: PASS — 61 packages installed; zero audit vulnerabilities.
- `npm test`: PASS — 10 Rust unit tests, 21 Rust integration tests, 25 Vitest
  tests, and 78 Playwright tests passed. Six project-specific Playwright checks
  were intentionally skipped in the non-applicable viewport project.
- `npm run lint`: PASS — Rust format, strict Clippy with warnings denied, and
  TypeScript checks all passed.
- `npm run build`: PASS — produced `target/release/mount-identity-audit` and the
  deployable `dist/site/` directory.

Logs are at `/tmp/mia-verify-10/npm-test.log`, `npm-lint.log`, and
`npm-build.log` under the same directory.

## Packaged CLI and end-to-end cases

`cargo package --allow-dirty` passed Cargo's package verification: 20 files,
164.1 KiB unpacked and 41.1 KiB compressed. Installing the packaged crate under
a fresh Cargo root produced exactly one 1,145,624-byte
`mount-identity-audit` executable with useful `--help` output.

The installed executable was exercised independently:

| Case | Observed result |
| --- | --- |
| Bundled isolated `--demo` | `FAIL`, exit 1; printed its unique temporary copy |
| Owner `0:0` on mode `0755` | `PASS`, exit 0 |
| Non-owner `1000:1000` on mode `0755` | `FAIL`, exit 1; read/no write |
| Named/malformed identity `invalid` | `UNKNOWN`, exit 2; requested numeric `UID:GID` |
| Reserved UID `4294967295` | `UNKNOWN`, exit 2; explained the Linux limit |
| Corrected invocation after invalid input | `PASS`, exit 0 |
| `--json --share` | schema version 1; no tested local project path remained |

Repository integration coverage also passed group-write, other-write,
read-only, JSONC, Compose selection and precedence, configuration-discovery,
Docker direct mapping, rootless Podman live mapping, build-backed unknown, and
runtime-call-limit cases. The demo did not alter its shipped input.
Docker and Podman executables were unavailable in the verifier container, so
runtime process behavior was exercised through deterministic recording adapters.

## Live browser, accessibility, and recovery

The candidate's site suite was rerun against the live URL:

- desktop Chromium: 16 passed, 4 intentional mobile-only skips;
- 390×844 mobile Chromium: 20 passed;
- Playwright Axe: zero violations, including zero serious or critical
  findings, on home, demo, Privacy, Terms, and 404 at both viewports;
- `/opt/fleet/lib/verify-url.sh`: HTTP 200 in 759 ms, `lang=en`, one H1, a
  `<main>` landmark, no missing image alternatives, no unnamed buttons, and no
  console errors.

The live exercises covered keyboard-only completion, a designed 3 px focus
ring, skip-link visibility, reduced motion, route focus and polite
announcements, 44 px mobile targets, 200% text reflow, history navigation,
normal mismatch and safe states, reset/exit, invalid octal and reserved-ID
errors, and recovery. An independent reserved-ID attempt focused a `role=alert`
message explaining the limit; **Load safe example** then returned the page to
“Workspace is writable.” No horizontal overflow or keyboard trap appeared.

The service worker was explicitly updated. After the browser context was set
offline, `/demo/` reloaded with a controlling service worker and the sample
remained usable.

Every link discovered across home, demo, Privacy, Terms, and 404 returned HTTP
200 after redirects. An unknown route returned the designed 404 with HTTP 404.

## Privacy, headers, and server-side scope

The full live sample flow recorded only same-origin GET requests for HTML,
hashed JavaScript/CSS, and the two product images. Entering the unique value
`3141592` and calculating produced no additional request containing that value.
Cookies, localStorage, sessionStorage, and IndexedDB all remained empty.
Console and page error lists were empty.

The document response includes a self-only Content Security Policy with
`frame-ancestors 'none'`, HSTS preload, `Referrer-Policy: no-referrer`,
`X-Content-Type-Options: nosniff`, and a restrictive permissions policy. HTML
and `sw.js` use 30-second revalidation. Hashed JavaScript, CSS, and the hashed
hero use one-year immutable caching.

This is a static site plus a local CLI. It has no server endpoint, product
unlock call, account, or sign-in. API rate-limit/429, backend concurrency,
server persistence, health, and Entra authority checks are therefore not
applicable. The deterministic permission calculation does not benefit from a
model-backed step; omitting AI is appropriate.

## Deployment identity and performance

All 18 served deployment files were fetched and matched byte-for-byte against
`dist/site`, including every HTML route, hashed JS/CSS, service worker, images,
icons, metadata files, robots, sitemap, and `_headers`. The deployment-only
`staticwebapp.config.json` correctly returns 404. This freshly proves that the
live deployment matches candidate `989e74d`.

| Budget or metric | Result |
| --- | ---: |
| JavaScript | 7,246 bytes raw / 3,021 bytes gzip |
| CSS | 17,008 bytes raw / 4,315 bytes gzip |
| Fonts | 0 bytes |
| Hero WebP | 216,498 bytes |
| Lighthouse performance | 99 |
| Lighthouse accessibility | 100 |
| Lighthouse best practices | 100 |
| Lighthouse SEO | 100 |
| FCP | 1.1 s |
| LCP | 2.0 s |
| Total blocking time | 40 ms |
| CLS | 0 |
| Total transfer | 226 KiB |

The clean Lighthouse run used mobile throttling and disabled only its optional
full-page screenshot artifact. All scored audits ran. The release stays within
the JavaScript, CSS, font, hero, LCP, and CLS budgets.

## Defects by severity

- Critical: none.
- High: none.
- Medium: none.
- Low/advisory: Lighthouse estimates about 151 KiB mobile transfer savings from
  serving a responsive hero image. The current 216,498-byte WebP and 2.0 s LCP
  remain within the contract budgets.
