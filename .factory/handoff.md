# Mount Identity Audit — repair 9 handoff

## Status: repaired and deployed

Work order `devcontainer-identity-audit-repair-9` repaired the release-blocking
finding in independent verification 14. Product code and public copy were
committed as `d338a95a8dd8` and deployed to the authorized Static Web App
`sf-devcontainer-identity-audit` on 2 September 2026 UTC.

## Reproduction and repair

The original `@claim:read-only-safety` fixture supplied `remoteUser`, so it
skipped image inspection and recorded only three calls. Before the repair, I
changed the fixture to the verifier's supported image-only rootless Podman case:

```text
info --format json
image inspect local/example:latest --format {{json .Config.User}}
unshare cat /proc/self/uid_map
unshare cat /proc/self/gid_map
```

The unchanged three-call assertion failed exactly as reported:
`Expected <= 3; Received 4`.

The product now truthfully promises at most four read-only runtime calls on the
landing proof strip, in the README, and in `.factory/claims.json`. The single
tagged claim regression uses `{"image":"local/example:latest"}`, records the
four commands above, checks that none can start/create/pull a container, proves
the project tree is unchanged, and asserts the expected `FAIL` mapping from
container `1000:1000` to host `100999:100999`. The generated copy audit and
changelog were updated. Runtime behavior was intentionally preserved.

## Local verification

- `npm ci`: PASS — 61 packages; zero audit vulnerabilities.
- Exact repaired claim before the copy fix: FAIL — expected at most 3, received
  4. Exact repaired claim after the fix: PASS.
- All 24 commands in `.factory/claims.json`, run separately: PASS (24/24).
- `npm test`: PASS — 11 Rust unit tests, 23 Rust integration tests, 30 Vitest
  tests, and 80 Playwright checks; eight intentional cross-project skips.
- `npm run lint`: PASS — rustfmt, warnings-as-errors Clippy, and TypeScript.
- `npm run copy:audit:check`: PASS.
- `npm audit --audit-level=low`: PASS — zero vulnerabilities.
- `npm run build`: PASS — release CLI plus `dist/site/`.
- `cargo package --locked --allow-dirty`: PASS — 20 files, 172.4 KiB unpacked,
  42.9 KiB compressed.
- Clean consumer install: PASS — one 1,141,400-byte executable; version and help
  output passed; isolated `--demo` returned the documented `FAIL`/exit 1.
- Initial site assets: 7,246 bytes JavaScript, 17,047 bytes CSS, no fonts, and a
  216,498-byte hero image. These remain below the product budgets.

The browser suite covers desktop and 390×844 mobile layouts, keyboard-only use,
focus/history announcements, 200% text reflow, 44 px targets, reduced motion,
invalid-input recovery, all five public routes, Axe, privacy storage/request
capture, service-worker update, and a dedicated offline reload context.

## Live verification

- Factory `verify-url.sh`: PASS — HTTPS 200 in 720 ms; correct title and
  `lang=en`; one H1 and one main; no missing alt text, unnamed buttons, or
  console errors.
- Production Playwright: PASS — 36 checks and four intentional viewport skips.
  Axe found zero serious or critical issues across Home, Demo, Privacy, Terms,
  and 404 at desktop and mobile sizes.
- Privacy/offline/update: PASS — the browser flow made only same-origin GETs,
  stored no entered values, reloaded offline after service-worker activation,
  and exercised the update path.
- Deployment identity: PASS — footer `v0.1.0 · d338a95a8dd8`; all 17 served
  files matched local `dist/site` by SHA-256.
- Response policy: PASS — HTTP redirects to HTTPS; the designed unknown route
  returns 404; HTML and `sw.js` revalidate after 30 seconds; conditional HTML
  returned 304; hashed art is immutable for one year; CSP, HSTS, no-referrer,
  nosniff, and restrictive Permissions Policy headers are present.
- Mobile Lighthouse: 99 performance, 100 accessibility, 100 best practices,
  and 100 SEO; FCP 1.0 s, LCP 2.0 s, TBT 0 ms, CLS 0.

## Known gaps and next steps

No release-blocking gap remains. The documented version-1 model still excludes
POSIX ACLs, security labels, remote filesystem policy, and identity changes made
during container startup. These are stated in every detailed report and are not
regressions from this repair.

The factory can publish the crate after running `cargo package --locked`. No
registry publish was attempted by this worker.
