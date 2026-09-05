# Mount Identity Audit review 6

## Verdict: PASS

Reviewed 5 September 2026 UTC. The implementation reviewed is `f0338c40b7ad66de276fd87da66db0afd11a8bf9`; the repository head is the documentation-only commit `dbcaa39dc9351eec19fc7e33551431b33e5348ae`. `git diff f0338c40..dbcaa39` contains only `.factory/verification-16.md` and `.factory/handoff.md`. A rebuild with `FACTORY_BUILD_ID=f0338c40…` matched all 17 publicly served files at <https://devcontainer-identity-audit.sociobot.in> by SHA-256, and the live footer says `v0.1.0 · f0338c40b7ad`.

There are zero findings at every severity and zero untested public claims.

## Job, audience, and first action

This is a local CLI that predicts whether the intended Dev Container or rootless Podman user can write to a workspace bind mount before startup. It is for developers using Dev Containers or rootless Podman who need a writable workspace on first open. The first action is **Try it with sample data**; it runs a known rootless Podman mismatch.

I opened fresh desktop (1440×900) and phone (390×844) browser contexts before scrolling. Both showed the same job, audience, and first action at scroll position zero with no console errors. One selection opened the populated demo: `FAIL`, mapped host identity `100999:100999 · rootless subuid map`, and `read · no write · traverse`. On phone the access result ended at 694 px, inside the 844 px viewport. The persistent label was **Demo — sample data, nothing is saved**. **Reset demo** restored the original failing sample; fresh contexts had zero localStorage and sessionStorage entries. The full production browser suite additionally verified no cookies or IndexedDB storage and no entered project data request.

## Claims and local quality gates

After `npm ci` installed the documented lockfile (61 packages, zero audit vulnerabilities), I invoked every `.factory/claims.json` command separately, in manifest order and exactly as written. All 24 passed:

| Claim groups | Result |
| --- | --- |
| CLI demo, browser demo, verdicts, read-only safety | 4/4 PASS |
| Config, Compose precedence, redaction, report contract | 4/4 PASS |
| Runtime mapping, Docker userns-remap, read-only recovery, conservative identities | 4/4 PASS |
| Browser/CLI privacy, offline reload, browser parity, MIT license | 4/4 PASS |
| Browser report, discovery, optional runtime, report limits | 4/4 PASS |
| Compose build/image, packed install, build artifacts | 3/3 PASS |

Other fresh gates:

- `npm test`: PASS — 13 Rust unit, 23 Rust integration, 33 Vitest, and 80 applicable Playwright checks; eight viewport-specific skips are intentional.
- `npm run lint`, `npm run copy:audit:check`, `npm audit --audit-level=low`, and exact `npm run build`: PASS.
- `cargo package --locked --allow-dirty`: PASS — 20 files, 174.7 KiB unpacked, 43.5 KiB compressed.
- Fresh packed-crate consumer install: PASS. It installed one `mount-identity-audit` executable; `--version`, useful `--help`, `--demo`, and JSON share output worked. The isolated demo copied sample data to a unique temporary directory, returned `FAIL`, and exited 1 without changing the shipped sample.

The installed artifact exercised normal failure/recovery evidence: the bundled mismatch returned `FAIL`/1, and a shared JSON report redacted both configuration and workspace paths. The claim and integration suites cover normal pass, invalid/reserved identity, boundary identity, read-only, named/UID-only unknown, Compose precedence, and repaired rootless `keep-id` cases.

## Live site, accessibility, privacy, and routes

`FACTORY_BUILD_ID=f0338c40… PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in npm run test:e2e` passed. It covers home, demo, Privacy, Terms, and the designed 404 on desktop and phone: keyboard-only flow, skip link, focus/history announcements, 200% reflow, target size, reduced motion, invalid mode and safe recovery, service-worker update, offline reload, and legal routes. Its integrated Axe scans found zero serious or critical violations.

The live response has `lang=en`, one H1, one main landmark, route-specific titles and metadata, no console errors, a self-only CSP with `frame-ancestors 'none'`, HSTS preload, no-referrer policy, nosniff, and restrictive Permissions Policy. The hashed assets and content-addressed hero return one-year immutable caching. HTTP redirects to HTTPS. An unknown path returns the designed page with deliberate HTTP 404; this is expected, not a defect.

The product is static plus a local CLI: it has no account, payment, server-side product endpoint, database, health endpoint, tenant, or rate-limited API. Backend persistence, tenant isolation, and 429/Retry-After checks do not apply.

Fresh Lighthouse on production: performance 99, accessibility 100, best practices 100, SEO 100; LCP 1.95 s, TBT 0 ms, CLS 0. Initial JavaScript is 7,266 bytes raw (3,022 bytes gzip), CSS is 17,047 bytes raw (4,327 bytes gzip), no fonts load, and the hero is 216,498 bytes.

## Earlier finding disposition

I read every earlier review and verification report. Their findings are currently disposed as follows:

| Earlier reports/findings | Current disposition and evidence |
| --- | --- |
| Verification 1: split `--userns`, missing headers/caching/HSTS | Fixed: runtime claims pass; live CSP, HSTS and immutable hero headers verified. |
| Verifications 2–4: Dockerfile/build, redaction, UID-only, Compose precedence, reserved ID, mobile reflow/targets, reduced motion, table layout | Fixed: the corresponding 24 claims and the live responsive/accessibility suite pass. |
| Verification 5: unpublished candidate, absent registry/demo/metadata/copy audit | Fixed: live candidate is byte-matched; registry has 24 tested claims; demo, 404, metadata and copy-audit checks pass. |
| Verifications 6–7: clean claim setup, first-screen facts, browser `keep-id`, touch targets, invalid-input recovery | Fixed: all exact claim commands pass after documented `npm ci`; fresh phone result fits the first viewport; live recovery and target tests pass. |
| Review 1 F-1-1…F-1-25; reviews 2–5 minor findings | Fixed: current copy audit passes; the named copy/focus/route/demo/footer/build-label checks pass. Live labels and build identity are current. |
| Verification 12: Docker userns-remap and read-only remediation | Fixed: both registered claims pass. |
| Verification 14: rootless image-user runtime-call cap | Fixed: `read-only-safety` passes its four-read-only-call assertion. |
| Verification 15: unsafe rootless Podman `keep-id` false PASS | Fixed: `browser-parity` and CLI tests cover IDs 0, 999, 1000, and 2000; the packed CLI maps them to 100000, 100999, 1000, and 101999. |
| Verification 9/10 advisory responsive-hero observation | Not a defect: current 216,498-byte hero, 1.95 s LCP, and all current performance budgets pass. The old decorative-text accessibility advisory is also absent: the present CSS mark is aria-hidden and Axe/Lighthouse accessibility are clean. |

## Defects

- Critical: none.
- High: none.
- Medium: none.
- Low: none.

