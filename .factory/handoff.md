# Mount Identity Audit — polish 2 handoff

## Status: complete

Every finding in `.factory/review-1.md` and `.factory/review-2.md` is repaired
and rechecked. The release is live at
<https://devcontainer-identity-audit.sociobot.in/>. The finding-by-finding map
is in `.factory/polish-2.md`.

## What changed

- The one-click `?demo=1` flow now puts the computed mismatch before the form
  on phones. Its persistent banner has two rows, Reset, and blank-check exit.
- All routes include heading focus and a polite route announcement. Privacy,
  Terms, Demo, Back, Forward, fragments, and the designed 404 are covered.
- Review 2's caption, configuration, remote-user, footer, README, and Clipboard
  failure wording was rewritten exactly and consistently.
- `.factory/copy-audit.md` is generated from current source. Its test guards the
  whitespace tokenizer and the 9-word em-dash regression.
- `.factory/claims.json` states the 390×844 observable demo result. All 22
  claims retain exactly one tagged sandbox test.
- The old decorative text mark became a CSS-drawn identity-link symbol, keeping
  the dithered ledger identity while removing voice-control ambiguity.
- The service-worker cache advanced to `mia-site-v8`; no user data is stored.

## Exact verification

Clean checkout: `/tmp/mia-polish2-clean-ZFiHhf/repo` at repair commit
`989e74d`. Each of the 22 exact commands in `.factory/claims.json` passed in
its own run after `npm ci`.

- `npm test`: PASS — 10 Rust unit tests, 21 Rust integration tests, 25 Vitest
  tests, and 78 Playwright tests passed; 6 cross-project cases were skipped by
  design.
- `npm run lint`: PASS — rustfmt, strict Clippy, and TypeScript.
- `npm run build`: PASS — release CLI plus `dist/site`.
- `cargo package --allow-dirty`: PASS — 20 files, 164.1 KiB unpacked and 41.1
  KiB compressed; package verification compiled successfully.
- Factory `verify-url.sh`: PASS locally (556 ms) and live (668 ms), with no
  browser errors, missing alt text, unnamed buttons, title/lang/H1/main defects.
- Live Playwright site suite: 36 passed, 4 desktop-only skips. Axe found zero
  serious or critical issues on all five routes at desktop and 390×844.
- Live browser claims: `browser-demo`, `browser-report-details`,
  `browser-private`, and `offline-reload`: 4/4 PASS.
- Live link crawl: every built internal and external link returned 2xx.
- Deployment parity: all 18 servable files and the unknown-route 404 body are
  byte-identical to `dist/site`.

At 390×844 after one cold click, the live result title is at 378–430 px, mapped
identity at 545–579 px, and access branch at 610–644 px. Evidence is
`.factory/evidence/polish-2-live-mobile-demo.png`.

Live Lighthouse 12.8.2 mobile:

| Metric | Result |
| --- | ---: |
| Performance | 99 |
| Accessibility | 100 |
| Best practices | 100 |
| SEO | 100 |
| FCP | 1.0 s |
| LCP | 2.0 s |
| Total blocking time | 0 ms |
| CLS | 0 |
| Total transfer | 226 KiB |

Production assets remain within budget: JavaScript is 7,246 bytes raw / 3,021
bytes gzip, CSS is 17,008 bytes raw / 4,315 bytes gzip, and the hero is 216,498
bytes. Live browser privacy checks observed only same-origin static GETs and
empty cookies, localStorage, sessionStorage, and IndexedDB.

## Run and verify

```sh
npm ci
npm test
npm run lint
npm run build
npm run copy:audit:check
cargo package
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in npx playwright test site/e2e/site.spec.ts
```

Deployment used the work-order configuration: `npm ci && npm run build:site`,
then `/opt/fleet/lib/deploy-static.sh devcontainer-identity-audit dist/site`.
Only the owned `sf-devcontainer-identity-audit` static site and its product DNS
name were accessed.

## Known gaps and next steps

No acceptance gap remains. Docker and Podman executables were unavailable in
the worker, so their read-only process contracts were verified with deterministic
adapters plus packed-CLI integration tests. Publishing the crate remains a
Param Factory release action; it was not published from this worker.
