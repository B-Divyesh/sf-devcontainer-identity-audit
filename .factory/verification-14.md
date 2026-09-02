# Independent verification 14 — Mount Identity Audit

## Verdict: FAIL

Candidate commit `9ff3773b87a1be0d05a34ab9d50306971f8d935c` was independently verified on 2 September 2026 UTC against <https://devcontainer-identity-audit.sociobot.in/>. The live deployment is byte-identical to the candidate's public build output and identifies itself as build `9ff3773b87a1`.

The candidate fails the acceptance contract because a first-screen, README, and registered quantitative claim is false on a supported rootless Podman path. The CLI can make four runtime calls even though it promises at most three. No product code or infrastructure was changed during verification.

## Release-blocking defect

### High — rootless image-user audits exceed the promised runtime-call cap

The home-page proof strip says **“3 read-only runtime calls, at most.”** README lines 111–113 say an audit makes at most three calls, and `.factory/claims.json` registers the same promise as `read-only-safety`.

I installed the packed crate into a clean Cargo prefix, then audited this supported configuration:

```json
{"image":"local/example:latest"}
```

The recording rootless Podman adapter returned numeric image user `1000:1000` and valid UID/GID maps. The installed release CLI returned a normal `FAIL` verdict, but the adapter log contained four calls:

```text
1  info --format json
2  image inspect local/example:latest --format {{json .Config.User}}
3  unshare cat /proc/self/uid_map
4  unshare cat /proc/self/gid_map
```

This follows the production code path: runtime inspection, image-user inspection, UID-map read, then GID-map read. The `@claim:read-only-safety` test passes only because its fixture supplies `remoteUser: "1000:1000"`, which removes the `image inspect` call. It therefore proves one three-call branch, not the public “at most” guarantee.

The calls remain read-only and the verdict is otherwise correct. The release still fails because the factory claim contract requires quantitative promises to be true and measured across the claimed behavior.

Required fix: either cap the supported path at three subprocess calls, or change the first-screen/README/registry claim to the truthful maximum. Extend `@claim:read-only-safety` with the image-only rootless fixture above.

## Mandatory first-read and claim gates

The cold 390 × 844 live first screen passes its mandatory comprehension gate:

- What it does: **“Check mount permissions before container startup.”**
- Who it serves: developers using Dev Containers or rootless Podman who need a writable workspace.
- First action: **“Try it with sample data,”** followed by “Runs a known rootless Podman mismatch.”

The action opens the computed sample in one click. It shows the mismatch, mapped host identity `100999:100999`, and permission branch in the first post-click viewport. The persistent demo banner exposes Reset and Open blank browser check.

`.factory/claims.json` exists with 24 entries. I ran every listed `test` command separately and exactly as written from the clean candidate. All 24 commands passed:

| Claims | Result |
| --- | --- |
| `cli-demo`, `browser-demo`, `permission-verdicts`, `read-only-safety` | PASS |
| `config-support`, `compose-user-precedence`, `share-redaction`, `report-contract` | PASS |
| `runtime-mapping`, `docker-userns-remap`, `read-only-remediation`, `conservative-identities` | PASS |
| `browser-private`, `cli-private`, `offline-reload`, `browser-parity` | PASS |
| `mit-license`, `browser-report-details`, `config-discovery`, `runtime-optional` | PASS |
| `report-limits`, `compose-build-image`, `install-binary`, `build-artifacts` | PASS |

The independent four-call counterexample shows why passing registered commands is necessary but not sufficient when a test underspecifies its claim.

## Clean quality gates and packaged CLI

- `npm ci`: PASS — 61 packages installed; zero audit vulnerabilities.
- `npm test`: PASS — 11 Rust unit tests, 23 Rust integration tests, 30 Vitest tests, and 80 Playwright tests; eight intentional cross-project skips.
- `npm run lint`: PASS — rustfmt, warnings-as-errors Clippy, and TypeScript typechecking.
- `npm run copy:audit:check`: PASS.
- `npm audit --audit-level=low`: PASS — zero vulnerabilities.
- Exact `npm run build`: PASS — produced `target/release/mount-identity-audit` and `dist/site/`.
- `cargo package --locked --allow-dirty`: PASS — 20 files, 172.3 KiB unpacked and 42.9 KiB compressed.
- Clean consumer install: PASS — exactly one 1,141,400-byte executable; `--version` returned `0.1.0`; `--help` documented the public CLI; `--demo` copied bundled data to a unique `/tmp` directory, returned the known `FAIL`, and exited 1.

Independent installed-CLI cases produced: ordinary mismatch `FAIL`/1; corrected owner identity `PASS`/0; maximum usable Linux ID `4294967294` on `/tmp` `PASS`/0; reserved `4294967295` `UNKNOWN`/2; named user `UNKNOWN`/2; missing project `UNKNOWN`/2. A share-mode missing-runtime report removed both tested private path prefixes. Twenty-four concurrent read-only audits all returned `PASS`.

## Live browser, privacy, accessibility, and recovery

- Factory `verify-url.sh`: HTTPS 200 in 867 ms; correct title and `lang=en`; one H1 and one main; no missing image alternatives, unnamed buttons, or console errors.
- Production Playwright suite: 36 passed with four intentional desktop-only skips. It covered desktop and 390 px mobile, keyboard-only operation, focus/history announcements, 200% text reflow, 44 px targets, reduced motion, invalid input and recovery, demo reset/exit, legal routes, service-worker update, and offline reload.
- Axe through Playwright found zero serious or critical findings on Home, Demo, Privacy, Terms, and 404 at both viewports.
- The first Tab focuses the skip link. Under reduced motion it moves from above the viewport to `y=8` and receives a 3 px green outline.
- A live privacy probe entered unique value `3141592` and ran the calculation. All six observed requests were same-origin GETs; the value appeared in no URL or body. Cookies, localStorage, sessionStorage, and IndexedDB remained empty. No console, page, or request errors occurred.
- The browser document response includes a self-only CSP with `frame-ancestors 'none'`, preload HSTS, `Referrer-Policy: no-referrer`, `nosniff`, and a restrictive Permissions Policy.
- HTML and `sw.js` use 30-second revalidation; a conditional document request returned 304. Hashed JS/CSS and the hero use one-year immutable caching. HTTP redirects to HTTPS. An unknown route returns the designed page with HTTP 404.
- Every discovered internal and external link returned 200 after redirects.

This is a static local-first site plus CLI. It has no server-side product endpoint, account, product-unlock request, payment, or sign-in. API rate limits, backend health/concurrency/persistence, and Entra authority checks are not applicable. The deterministic permission calculation has no useful model-backed step, so the absence of AI is appropriate.

## Deployment identity and performance

All 17 publicly served build files matched fresh local `dist/site` bytes by SHA-256, including every HTML route, hashed asset, image, service worker, robots file, and sitemap. The two remaining build files are deployment configuration (`_headers` and `staticwebapp.config.json`) rather than public content. The live footer reports `v0.1.0 · 9ff3773b87a1`.

| Budget or metric | Fresh result |
| --- | ---: |
| JavaScript | 7,246 bytes raw / 3,022 bytes gzip |
| CSS | 17,047 bytes raw / 4,327 bytes gzip |
| Fonts | 0 bytes |
| Hero WebP | 216,498 bytes |
| Lighthouse performance | 99 |
| Lighthouse accessibility | 100 |
| Lighthouse best practices | 100 |
| Lighthouse SEO | 100 |
| FCP / LCP | 1.2 s / 2.0 s |
| Total blocking time / CLS | 90 ms / 0 |
| Transfer | 226 KiB |

Evidence is in [`.factory/verification-14-evidence/`](verification-14-evidence/), including the cold mobile capture, factory URL report, desktop/mobile captures, and fresh Lighthouse JSON.

## Defects by severity

- Critical: none.
- High: one — the supported rootless image-user path makes four runtime calls despite a tested public maximum of three.
- Medium: none.
- Low: none.
