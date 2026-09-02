# Mount Identity Audit — repair 10 handoff

## Status: repaired and deployed

Work order `devcontainer-identity-audit-repair-10` repairs the high-severity
finding in independent verification 15. The product-code repair is commit
`4ff5ab25c6ef0fb7adeca7264e7fa7d584247163`. It was pushed to `main` and deployed
through the factory static-site workflow to the authorized resource
`sf-devcontainer-identity-audit` and
<https://devcontainer-identity-audit.sociobot.in/>.

## Reproduction and root-cause repair

The failing candidate treated every non-kept container ID as though it were
already in the outer rootless Podman namespace. I first added the verifier's
exact packed-consumer case: caller `1000:1000`, workspace owner `1000:1000`,
mode `0755`, remote user `0:0`, `--userns=keep-id`, and live outer maps
`0 1000 1` plus `1 100000 65536`. The installed candidate returned exit `0`;
the regression expected `FAIL` and exit `1`.

The CLI now applies Podman's inner `keep-id` map before the live outer UID and
GID maps. It also checks that outer namespace ID 0 maps to the actual caller.
The browser uses the same piecewise rule. With identical outer ranges, the
packed CLI and browser now agree:

| Container ID | Inner ID | Host ID | Verdict on owner `1000`, mode `0755` |
| ---: | ---: | ---: | --- |
| `0` | `1` | `100000` | `FAIL` |
| `999` | `1000` | `100999` | `FAIL` |
| `1000` | `0` | `1000` | `PASS` |
| `2000` | `2000` | `101999` | `FAIL` |

The packed regression records exactly three read-only calls per case: `info`,
`unshare cat /proc/self/uid_map`, and `unshare cat /proc/self/gid_map`. Unit
coverage also checks the boundary above a configured kept user. Parser coverage
checks explicit `keep-id:uid=1200,gid=1300` options. The registered
`browser-parity` sandbox, demo notes, README, changelog, and generated copy audit
now describe the two-layer behavior.

## Local verification

- Clean install and suite: `cargo clean && npm ci && npm test` passed. Results:
  13 Rust unit tests, 23 Rust integration tests, 33 Vitest tests, and 80
  Playwright checks; eight desktop/mobile applicability skips.
- Every command in `.factory/claims.json` passed separately: 24/24. The repaired
  `@claim:browser-parity` command installed the packed crate and checked all four
  IDs above against the browser.
- `npm run lint`, `npm run copy:audit:check`, and
  `npm audit --audit-level=low` passed; npm reported zero vulnerabilities.
- `npm run build` produced `target/release/mount-identity-audit` and
  `dist/site/`.
- `cargo package --locked --allow-dirty` passed: 20 files, 174.7 KiB unpacked,
  43.5 KiB compressed. A fresh offline consumer install produced one 1,142,536
  byte executable. Version and help passed; isolated `--demo` returned the
  documented `FAIL` and exit `1`.
- Local factory URL verification passed in 550 ms with no console errors,
  correct title and language, one H1 and main landmark, complete image text,
  and named buttons.
- Local Lighthouse: 99 performance, 100 accessibility, 100 best practices, and
  100 SEO; FCP 1.0 s, LCP 2.3 s, TBT 0 ms, CLS 0.
- Initial assets: 7,266 bytes JavaScript, 17,047 bytes CSS, no fonts, and a
  216,498-byte hero WebP.

## Live verification

- Factory `verify-url.sh`: HTTPS 200 in 773 ms; no console errors; title,
  `lang=en`, one H1, main landmark, image alternatives, and button names pass.
- Production Playwright: 36 applicable checks passed with four intentional
  desktop-only skips. Coverage includes desktop and 390×844 mobile, keyboard,
  focus/history announcements, 200% text reflow, 44 px targets, reduced motion,
  invalid-input recovery, legal and 404 routes, service-worker update, and
  offline reload.
- Axe found zero serious or critical findings on Home, Demo, Privacy, Terms,
  and 404 at both viewports.
- The live privacy, offline, and repaired parity claims passed together. The
  privacy flow made only same-origin requests and left cookies, localStorage,
  sessionStorage, and IndexedDB empty.
- Response policy passed: HTTP redirects to HTTPS; an unknown route returns the
  designed page with status 404; HTML revalidates after 30 seconds and returned
  304 conditionally; hashed assets use one-year immutable caching; CSP, HSTS,
  no-referrer, nosniff, and restrictive Permissions Policy headers are present.
- All 12 unique links resolved successfully. All 17 publicly served build files
  matched local `dist/site` byte-for-byte by SHA-256; deployment-only `_headers`
  and `staticwebapp.config.json` were excluded.
- Live Lighthouse: 99 performance, 100 accessibility, 100 best practices, and
  100 SEO; FCP 0.9 s, LCP 2.0 s, TBT 0 ms, CLS 0.
- The verified deployment showed `v0.1.0 · 4ff5ab25c6ef`. The final handoff
  commit is rebuilt and redeployed after this file is committed so the public
  footer identifies the final repository state.

Evidence is in [`.factory/evidence/repair-10-live/`](evidence/repair-10-live/).

## Known limits and next steps

No release blocker remains. Version 1 still excludes POSIX ACLs, security
labels, remote filesystem policy, and identity changes during container
startup. Every detailed report already states these limits. No registry publish
was attempted; Param Factory can publish with `cargo package --locked`.

No out-of-scope resource, service setting, secret, database, staging slot, or
storage account was read or changed.
