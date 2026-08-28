# Independent verification — FAIL

**Work order:** `devcontainer-identity-audit-verify-1`  
**Candidate commit:** `45ced4e27f5f75a0e3257edf82502bf024d50b49`  
**Live URL:** https://devcontainer-identity-audit.sociobot.in/  
**Verified:** 2026-08-28

## Decision

**FAIL.** The site deployed at the live URL is the candidate's built artifact,
but the CLI gives a false negative for a supported rootless-Podman configuration.
That is a core preflight result, not a presentation-only issue.

## Release and package gates

- Clean checkout confirmed at the candidate SHA; `npm ci` completed with `0`
  npm audit vulnerabilities.
- `npm test` passed: 6 Rust unit tests, 5 Rust CLI integration tests, 5 Vitest
  tests, and Playwright's 12 project runs (11 applicable; the desktop instance
  of the mobile-only assertion is intentionally skipped). The final Playwright
  result is `{"status":"passed","failedTests":[]}`.
- `npm run build` passed and produced `target/release/mount-identity-audit` and
  `dist/site/`.
- `cargo clippy --all-targets --all-features -- -D warnings` passed.
- `cargo package --allow-dirty` passed: 17 files, 122.9 KiB unpacked / 32.5 KiB
  compressed, and Cargo's package verification build passed.
- A fresh consumer unpacked that `.crate`, installed it with `cargo install
  --path ... --root ...`, and ran `mount-identity-audit --help`, `--version`
  (`0.1.0`), and a Docker/no-runtime JSON audit successfully.

## End-to-end CLI evidence

- Normal case: a packaged consumer project with `remoteUser: "0:0"` and
  `--runtime docker --no-runtime --json` returned schema version `1`, `pass`,
  and the no-change/no-container guarantees.
- Invalid/recovery path: named `remoteUser: "vscode"` returned exit `2` and
  `UNKNOWN`; adding `--remote-user 0:0` recovered to exit `0` and `PASS`.
- Permission-failure, malformed-config, JSON redaction, Compose, rootless map,
  and documented no-runtime cases are covered by the passing Rust integration
  suite.

### P1 — rootless Podman `--userns` split syntax is misread

The runtime parser only recognizes `--userns=<value>` (or the nonstandard
`--userns:<value>`). It does not recognize the standard two-argument spelling
that Dev Container `runArgs` may contain: `"--userns", "keep-id"`.

Fresh isolated reproduction, executed as UID/GID `65534` against a rootless
Podman metadata fixture and a workspace owned `65534:65534`, mode `0755`:

| Config `runArgs` | Expected effective identity / verdict | Actual result |
| --- | --- | --- |
| `["--userns", "keep-id"]` | `65534:65534`, PASS | exit `1`, `FAIL`; reports mapped identity `165533:165533` |
| `["--userns=keep-id"]` | `65534:65534`, PASS | exit `0`, `PASS`; reports `65534:65534` |

`keep-id` is central to the product's rootless-Podman remediation and the
brief explicitly requires separate rootless Podman support. This false
permission failure makes the ownership prediction unreliable for a valid
configuration syntax.

## Live deployment equivalence, privacy, PWA, and browser QA

The live deployment **does match the candidate's generated files**:

- `index.html`: SHA-256 `d31261acb3f4ae59ebeb9c9d3ed1729a14b8288868d359c8fa1792f509490118`
  for both live and `dist/site/index.html`.
- Live JS/CSS/hero/service-worker matched the locally built source byte-for-byte:
  `main-DuLBLfrd.js`, `style-CiwEsfZ-.css`, `mount-ledger.webp`, and `sw.js`.
- Live browser request capture observed only
  `https://devcontainer-identity-audit.sociobot.in`; no analytics, font CDN, or
  third-party script/request was made. The CLI has no network client.
- `verify-url.sh` passed: HTTP 200, title, `lang=en`, exactly one `h1`, `main`,
  image alt text, labelled buttons, and no browser errors.
- Independent Playwright checks on desktop and 390×844 mobile passed: no
  horizontal overflow, no console/page errors, initial mismatch, invalid octal
  validation, and safe-example recovery all worked. Keyboard Tab reached the
  skip link with a visible `3px solid` focus outline and Enter operated the
  preflight button.
- Axe on both viewports reported **0 serious/critical** violations. Reduced
  motion changed the result-stamp animation to `0.00001s`.
- The live service worker controls the page; `registration.update()` completed
  with active `/sw.js`; offline banner and offline reload retained the home
  page and demo shell.
- Lighthouse 12.8.2 against the live URL: Performance **99**,
  Accessibility **100**; FCP **1.1 s**, LCP **2.0 s**, TBT **40 ms**, CLS **0**.
- Built payloads meet the stated budgets: JS 4,753 B raw / 2,110 B gzip; CSS
  11,702 B raw / 3,300 B gzip; hero WebP 216,498 B.

### P2 — intended browser security headers are absent in production

The committed `site/public/_headers` specifies a CSP and Permissions Policy,
but fresh HTTPS responses from the live root, JS, privacy, and terms routes do
not contain either `Content-Security-Policy` or `Permissions-Policy`. They do
contain HSTS, Referrer-Policy, and `X-Content-Type-Options`.

### P2 — production caching misses the static-product cache policy

The live hashed JS and CSS, and the immutable hero WebP, all return
`Cache-Control: public, must-revalidate, max-age=30`; they do not receive the
configured `max-age=31536000, immutable`. This violates the supplied static
performance/cache policy and adds unnecessary repeat-transfer/revalidation.

### P3 — HSTS preload directive is not preload-eligible

Production sends `Strict-Transport-Security: max-age=10886400; includeSubDomains;
preload`. The `preload` token is present, but its 126-day max age is below the
one-year preload requirement. This is not the reason for the release failure,
but the header should not claim preload readiness until corrected.

## Required fixes before re-verification

1. Parse `runArgs` option/value pairs as well as equals syntax for
   `--userns keep-id` and `--userns host`; add an integration test using the
   split form and a rootless map.
2. Configure the actual deployment host to emit the committed CSP and
   Permissions Policy, and apply immutable one-year caching to hashed assets
   and the hero asset. Recheck live headers after deployment.
3. Raise HSTS `max-age` to at least 31536000 before retaining `preload`.
