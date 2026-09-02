# Mount Identity Audit — repair 8 handoff

## Status: PASS

Release-blocking findings V12-1 and V12-2 are repaired. Product commits
`b1c0c7c` and `84be969` are pushed to `main`. Static Web Apps deployment
`e60a16b8-5121-4544-bef3-dc7c259a36ff` is live at
<https://devcontainer-identity-audit.sociobot.in/> as of 2 September 2026 UTC.

## What changed

- Docker inspection now recognizes `SecurityOptions: ["name=userns"]` as
  daemon `userns-remap`. The audit stops before permission evaluation and
  returns `UNKNOWN`/exit 2 with null host IDs until that map is resolved. The
  runtime check names `userns-remap` and Docker-specific advice never assumes
  direct host IDs.
- Declared read-only workspaces now receive one relevant next step: review and,
  only when edits are intended, remove `readonly`, `read_only`, or `ro`.
  Identity, user-namespace, and host-mode changes are suppressed for this
  branch.
- Two public claims and packed-consumer regressions cover those behaviors. The
  landing runtime table and README now qualify Docker's direct-ID support.
- The service-worker cache advanced from `mia-site-v9` to `mia-site-v10` so
  returning browsers install the repaired shell and retire the prior cache.

## Reproduction and regression evidence

Before edits, a freshly packaged and installed `0.1.0` consumer was run against
the verifier's mode-0755, host-`1000:1000` fixture and a Docker adapter returning
`{"ServerVersion":"27.3.1","SecurityOptions":["name=userns"]}`. It reproduced
the unsafe `PASS`/exit 0 and reported host `1000:1000`. The packed read-only
fixture reproduced `FAIL`/exit 1 with only identity and host-mode suggestions.

After repair:

- `npm run test:claims -- --grep @claim:docker-userns-remap` passes against a
  newly packed and installed binary. It asserts `UNKNOWN`/2, null host UID/GID,
  a `userns-remap` runtime warning, and no direct-ID safety statement.
- `npm run test:claims -- --grep @claim:read-only-remediation` passes against a
  newly packed and installed binary for `readonly`, `read_only`, and `ro`. Each
  report contains one mount-setting remedy and no UID:GID, group/mode, or
  user-namespace suggestion.
- Rust integration tests repeat both cases directly. A runtime unit test locks
  parsing of Docker's `name=userns` security option.
- Every one of the 24 exact commands in `.factory/claims.json` passed
  independently. The offline claim was rerun after the `v10` cache update.

## Complete verification

- Clean dependency install: `npm ci` installed 61 packages; `npm audit
  --audit-level=low` found zero vulnerabilities.
- `npm test`: 11 Rust unit tests, 23 Rust CLI integration tests, 28 Vitest
  tests, and 80 Playwright passes; eight expected project/viewport skips.
- `npm run lint`, `npm run copy:audit:check`, and `npm run build` pass. The copy
  audit reports zero sentences over 22 words and zero banned terms.
- `cargo package --locked --allow-dirty` passes package verification: 20 files,
  172.0 KiB unpacked and 42.8 KiB compressed.
- A fresh consumer unpacked the crate, installed exactly one executable, and
  verified help, version `0.1.0`, bundled demo `FAIL`/1, Docker-remap
  `UNKNOWN`/2, and read-only `FAIL`/1.
- The production build contains the release CLI and `dist/site`. Initial JS is
  7,246 bytes raw across two files and CSS is 17,008 bytes raw. There are no
  font downloads; the 216,498-byte hero remains below budget.

## Live browser, accessibility, privacy, and update evidence

- Factory URL verification returned HTTP 200 in 736 ms with no console errors,
  the expected title, `lang=en`, one H1, a main landmark, complete image text,
  and labelled buttons. Evidence: [live report](evidence/repair-8-live/verify.json),
  [desktop](evidence/repair-8-live/screenshot-desktop.png), and
  [390 px demo](evidence/repair-8-live/demo-mobile.png).
- The live route suite passed 36 checks with four intentional desktop-only
  skips. It covers desktop and 390 px, keyboard operation, focus/history
  announcements, 200% reflow, 44 px targets, reduced motion, privacy, offline
  reload, and Home, Demo, Privacy, Terms, and 404 routes.
- Playwright Axe found zero serious or critical issues across all five routes.
  The privacy flow made only same-origin requests and left cookies,
  localStorage, sessionStorage, and IndexedDB empty.
- A forced live service-worker update activated only `mia-site-v10`; `/demo/`
  then reloaded offline under its controller and retained the sample `FAIL`.
- All 17 browser-served build files are byte-identical to production. The
  deployment config itself returns 404. Unknown routes return HTTP 404.
- Production sends the self-only CSP, restrictive Permissions Policy,
  `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and
  preload-eligible one-year HSTS. Hashed JS/CSS and the hero use one-year
  immutable caching; HTML and `sw.js` revalidate after 30 seconds.
- Live mobile Lighthouse 12.8.2: Performance 99, Accessibility 100, Best
  Practices 100, SEO 100; FCP 0.9 s, LCP 2.0 s, TBT 30 ms, CLS 0, total transfer
  226 KiB.

## Known limits and next steps

No release blocker remains. Version 1 deliberately does not resolve Docker
daemon subordinate-ID allocations; it now returns `UNKNOWN` rather than making
an unsafe claim. POSIX ACLs, security labels, remote filesystem policy, and
startup-time identity changes remain documented limits.

Registry publishing remains factory-owned. The ready artifact is
`target/package/mount-identity-audit-0.1.0.crate`; prepare it with
`cargo package --locked`.
