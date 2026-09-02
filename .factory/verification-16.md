# Independent verification 16 — Mount Identity Audit

## Verdict: PASS

Candidate commit `f0338c40b7ad66de276fd87da66db0afd11a8bf9` was independently verified on 2 September 2026 UTC against <https://devcontainer-identity-audit.sociobot.in/>. The live deployment is the candidate: every one of the 17 publicly served build files matched a fresh local production build byte-for-byte, and all public route footers identify `v0.1.0 · f0338c40b7ad`.

No product code or infrastructure was changed. No release-blocking defect was found.

## Required first-read check

A cold desktop visit and a fresh 390×844 visit both answer the required three questions in the first screen:

- What it does: **“Check mount permissions before container startup.”**
- Who it serves: developers using Dev Containers or rootless Podman who need a writable workspace on first open.
- What to select first: **“Try it with sample data,”** followed by “Runs a known rootless Podman mismatch.”

In my own words, this predicts whether the intended numeric container user can write to a workspace bind mount before a developer opens the container. The first action is the sample-data link.

The three price, offline, and privacy facts end at 684 CSS pixels in the fresh mobile viewport. The sample takes one selection and immediately shows `MOUNT MISMATCH PREDICTED`, `FAIL`, mapped host identity `100999:100999 · rootless subuid map`, and `read · no write · traverse`. On mobile, the result title, mapped identity, and access result end at 430, 617, and 694 CSS pixels, all within the first post-selection viewport. The persistent banner says “Demo — sample data, nothing is saved” and provides Reset demo and Open blank browser check actions.

## Registered claims

`.factory/claims.json` is present with 24 entries. I invoked every listed command separately and in manifest order. Before dependency installation, the commands reached the site-build step but could not launch the absent local `vite` executable; no claim assertion ran. After the required clean-checkout `npm ci`, which installed the lockfile with zero vulnerabilities, every exact manifest command passed. Result: **24/24 PASS**.

| Claims | Result |
| --- | --- |
| `cli-demo`, `browser-demo`, `permission-verdicts`, `read-only-safety` | PASS |
| `config-support`, `compose-user-precedence`, `share-redaction`, `report-contract` | PASS |
| `runtime-mapping`, `docker-userns-remap`, `read-only-remediation`, `conservative-identities` | PASS |
| `browser-private`, `cli-private`, `offline-reload`, `browser-parity` | PASS |
| `mit-license`, `browser-report-details`, `config-discovery`, `runtime-optional` | PASS |
| `report-limits`, `compose-build-image`, `install-binary`, `build-artifacts` | PASS |

The landing page, legal pages, CLI help, detailed reports, demo documentation, and README were cross-checked against the registry. Functional, privacy, offline, licensing, safety, output-contract, and supported-input promises have registered tests. `npm run copy:audit:check` passed with no sentence above 22 words and no banned term. No unlisted substantive claim was found.

## Clean repository gates

- `cargo clean && npm test`: PASS — 13 Rust unit tests, 23 CLI integration tests, 33 Vitest checks, and 80 applicable Playwright checks passed. Eight cross-project checks were intentionally skipped in the inapplicable viewport.
- `npm run lint`: PASS — rustfmt, warnings-as-errors Clippy, and TypeScript typechecking all passed.
- `npm run copy:audit:check`: PASS.
- `npm audit --audit-level=low`: PASS — zero vulnerabilities.
- Exact `npm run build`: PASS — produced `target/release/mount-identity-audit` and deployable `dist/site/`.

## Packaged CLI and end-to-end behavior

`cargo package --locked --allow-dirty` passed Cargo's verification: 20 files, 174.7 KiB unpacked and 43.5 KiB compressed. A fresh consumer unpacked and installed the crate into an isolated Cargo root. It produced exactly one 1,142,536-byte executable. `--version` returned `0.1.0`, `--help` documented the public options, and `--demo` copied the bundled sample to a unique temporary directory, reported `FAIL`, and exited 1.

Independent installed-binary cases produced:

| Case | Result |
| --- | --- |
| Owner `0:0`, mode `0755`, direct numeric IDs | `PASS`, exit 0 |
| Non-owner `1000:1000`, mode `0755` | `FAIL`, exit 1 |
| Named user without numeric proof | `UNKNOWN`, exit 2 |
| Correct numeric override after that error | `PASS`, exit 0 |
| Declared read-only mount | `FAIL`, exit 1; mount-flag remediation |
| Reserved Linux ID `4294967295` | `UNKNOWN`, exit 2 |
| Highest usable ID `4294967294`, mode `0000` / `0777` | `FAIL`/1 then `PASS`/0 |

Twenty-four concurrent direct-ID audits all returned `PASS` without mutation. The shared JSON reports retained schema version 1, and share-mode redaction is covered by both the claim suite and CLI integration suite.

### Prior high-severity `keep-id` defect

The exact unsafe case from independent verification 15 was retested against the freshly installed release executable. The fixture uses caller and workspace owner `1000:1000`, mode `0755`, split `--userns keep-id`, and live outer UID/GID maps `0 1000 1` plus `1 100000 65536`.

| Container ID | Mapped host ID | Verdict / exit |
| ---: | ---: | --- |
| `0` | `100000` | `FAIL` / 1 |
| `999` | `100999` | `FAIL` / 1 |
| `1000` | `1000` | `PASS` / 0 |
| `2000` | `101999` | `FAIL` / 1 |

Each audit made exactly three read-only adapter calls: `info`, UID-map read, and GID-map read. This confirms that the release composes Podman's inner `keep-id` map with the live outer maps and no longer emits the prior unsafe false `PASS`. The browser produced the same four mapped identities and verdicts in the registered parity test.

## Live browser, accessibility, and recovery

Running the complete Playwright configuration against production passed 80 applicable checks with eight intentional viewport skips. Coverage includes the desktop and 390×844 layouts, keyboard-only operation, visible focus, route history and announcements, 200% text reflow, 44-pixel mobile targets, invalid input and recovery, legal pages, designed 404, service-worker update, and offline reload.

- The factory `verify-url.sh` returned HTTPS 200 in 600 ms with `lang=en`, one H1, a main landmark, complete image alternatives, named buttons, and no console errors.
- Axe found zero violations, including zero serious or critical findings, in an independent mobile sample scan. The full suite scans Home, Demo, Privacy, Terms, and 404 in both viewport projects.
- The first Tab focuses the visible Skip to main content link at `y=8` with a 3-pixel green outline. Enter and subsequent Tab actions complete the audit and safe recovery without a pointer.
- With reduced motion requested, the media query matches and remaining animation durations are 0.01 ms.
- Invalid mode `0899` announces “Directory mode must be three or four octal digits, such as 0755.” Load safe example then restores `0755` and reaches `PASS`.
- No horizontal clipping or layout loss was observed in the captured desktop or mobile full-page views. The dithered identity-ledger visual system matches `.factory/design.md` and remains product-specific.

## Privacy, headers, caching, and endpoint scope

A fresh production browser probe recorded the cold page, one-click demo, entered marker value `3141592`, calculation, invalid input, and recovery. All 18 document/resource requests were same-origin GETs. The calculation itself made zero requests. There were no failed requests, console errors, page errors, cookies, localStorage keys, sessionStorage keys, or IndexedDB databases.

The document response sends a self-only CSP with `frame-ancestors 'none'`, HSTS for one year with subdomains and preload, `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and a restrictive Permissions Policy. HTTP redirects to HTTPS. HTML and `sw.js` revalidate after 30 seconds; a conditional HTML request returned 304. Hashed JavaScript/CSS and the content-addressed hero use one-year immutable caching. An unknown route returns the designed page with HTTP 404. All applicable internal links and links into the supplied GitHub repository returned 200; the footer's out-of-scope platform link was not requested.

This product is a static site plus a local CLI. It has no server-side product or unlock endpoint, account, payment, or sign-in flow, so API rate-limit, server-persistence, health, and Entra-authority checks do not apply. Its core job is deterministic POSIX identity and mode calculation; an AI feature would not improve the brief's job-to-be-done.

## Deployment identity and performance

All 17 public build files matched local `dist/site` bytes by SHA-256. This includes all HTML routes, hashed JS/CSS, images, source provenance, service worker, robots file, and sitemap. Deployment-only `_headers` and `staticwebapp.config.json` are not public files.

| Budget or metric | Fresh result |
| --- | ---: |
| JavaScript | 7,266 bytes raw / 3.00 KiB gzip |
| CSS | 17,047 bytes raw / 4.32 KiB gzip |
| Fonts | 0 bytes |
| Hero WebP | 216,498 bytes |
| Lighthouse performance | 97 |
| Lighthouse accessibility | 100 |
| Lighthouse best practices | 100 |
| Lighthouse SEO | 100 |
| FCP / LCP | 1.10 s / 1.97 s |
| Total blocking time / CLS | 175.5 ms / 0 |
| Calculator action, 50-run maximum | 0.70 ms |
| Total transfer | 231,103 bytes |

The static budgets, LCP under 2.5 seconds, TBT under 200 ms, sub-millisecond calculator update, and zero-layout-shift requirements pass.

## Defects by severity

- Critical: none.
- High: none.
- Medium: none.
- Low: none.

Known model limits are deliberate and documented: version 1 does not evaluate POSIX ACLs, security labels, remote filesystem policy, or identity changes made during container startup.
