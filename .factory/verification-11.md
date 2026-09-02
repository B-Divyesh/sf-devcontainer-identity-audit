# Independent verification 11 — Mount Identity Audit

## Verdict: PASS

Candidate commit `fdf2863a611e161f06d2dbf260cc9eeb78024aad` was independently
verified on 2 September 2026 UTC against
<https://devcontainer-identity-audit.sociobot.in/>. The candidate is a
documentation-only successor to the deployed product build; a fresh production
comparison found all 18 public deployable files byte-identical to this
candidate's `dist/site` output. No release-blocking defect was found.

## Cold first read and demo

A cold 1440×900 visit plainly says **“Check mount permissions before container
startup”**. It says this is for developers using Dev Containers or rootless
Podman who need a writable workspace on first open, and the first primary
action is **“Try it with sample data”**, with the outcome stated beside it:
“Runs a known rootless Podman mismatch.” This satisfies the what, who, and
first-action acceptance test.

The one-click action loaded `/?demo=1#demo`, immediately showing `FAIL`, mapped
host identity `100999:100999`, and the read/no-write/traverse access branch.
The persistent demo banner says “Demo — sample data, nothing is saved” and
offers Reset demo and Open blank browser check. At 390×844 the result title,
mapped identity, and access branch ended at 430, 617, and 694 CSS px
respectively; horizontal overflow was zero.

## Claims and clean quality gates

`.factory/claims.json` exists with 22 entries. After a clean `npm ci` (61
packages, zero audit vulnerabilities), `npm run test:claims` executed the
declared Playwright demo suite and all **22/22** registered tags passed:

`cli-demo`, `browser-demo`, `permission-verdicts`, `read-only-safety`,
`config-support`, `compose-user-precedence`, `share-redaction`,
`report-contract`, `runtime-mapping`, `conservative-identities`,
`browser-private`, `cli-private`, `offline-reload`, `browser-parity`,
`mit-license`, `browser-report-details`, `config-discovery`,
`runtime-optional`, `report-limits`, `compose-build-image`, `install-binary`,
and `build-artifacts`.

The other clean candidate gates passed:

- `npm test`: 10 Rust unit tests, 21 Rust CLI integration tests, 26 Vitest
  tests, and 78 Playwright tests passed; 6 explicitly non-applicable
  viewport-only tests were skipped.
- `npm run lint`: Rust formatting, warnings-as-errors Clippy, and TypeScript
  type checking passed.
- `npm run build`: produced `target/release/mount-identity-audit` and
  deployable `dist/site/`.
- `npm run copy:audit:check` passed.
- `cargo package --locked --allow-dirty` passed verification: 20 files,
  165.1 KiB unpacked and 41.5 KiB compressed.

## CLI consumer verification

I installed the packaged crate into a fresh temporary Cargo prefix. It created
exactly one `mount-identity-audit` executable with useful help. Its bundled
`--demo` made an isolated temporary sample copy, returned `FAIL` and exit 1.
With a Docker/no-runtime fixture, explicit `0:0` returned `PASS`/exit 0,
`1000:1000` on the root-owned mode-0755 workspace returned `FAIL`/exit 1, and
the invalid identity `invalid` returned `UNKNOWN`/exit 2 with recovery advice.
The JSON shared report retained schema version 1 and neutral path labels.

## Live browser, accessibility, privacy, and deployment

- `/opt/fleet/lib/verify-url.sh` passed: HTTP 200 in 596 ms, title present,
  `lang=en`, one H1, main landmark, no missing image alt text or unnamed
  buttons, and no console errors.
- Independent Playwright Axe scans of `/`, `/demo/`, `/privacy/`, `/terms/`,
  and `/404.html` found zero serious or critical violations.
- Keyboard-only testing reached the skip link, the Check button, the mismatch
  result, and the safe recovery. The 3 px designed focus ring was visible.
  The skip link became visible under reduced motion. Mobile layout, 44 px
  controls, 200% reflow, route focus, validation error recovery, and browser
  demo reset/exit are covered by the clean Playwright suite and passed.
- A fresh dedicated service-worker context reloaded `/demo/` offline after
  activation, with a controlling service worker.
- During the live calculator flow a unique entered value (`3141592`) never
  appeared in requests. Every request was same-origin. Cookies, localStorage,
  sessionStorage, and IndexedDB remained empty.
- Browser console and page errors were empty. The document sends a self-only
  CSP with `frame-ancestors 'none'`, HSTS preload, `Referrer-Policy:
  no-referrer`, `X-Content-Type-Options: nosniff`, and restrictive permissions
  policy. HTML and `sw.js` use 30-second revalidation; hashed JS uses one-year
  immutable caching.
- Every discovered public link returned HTTP 200 after redirects. An unknown
  path returned the designed page with HTTP 404.
- All 18 browser-served product files, including every HTML route, hashed
  JavaScript/CSS, service worker, images, icons, metadata, robots, sitemap,
  and `_headers`, matched the locally built candidate byte-for-byte.

Initial bundle output is 7.25 KB JavaScript raw (3.02 KB gzip), 17.01 KB CSS
raw (4.30 KB gzip), no fonts, and a 216.5 KB hero WebP: all within the stated
budgets. This is a static landing page plus local CLI; it has no product API,
account/sign-in, server state, unlock endpoint, or rate-limit surface, so
backend concurrency, persistence, Entra, and 429 allowance checks are not
applicable. AI is appropriately absent from this deterministic local audit.

## Defects by severity

- Critical: none.
- High: none.
- Medium: none.
- Low: none.

