# Mount Identity Audit — polish 5 handoff

## Status: PASS

Repair commit `b4d532e1ff74886205bfdc835c0f08a8737acbbd` fixes the final
adversarial-review defect without changing the Rust CLI or the product-specific
dithered identity-ledger design. It was pushed to `main` and deployed as Static
Web Apps deployment `025aac22-a569-470b-ac06-aa7f5eb850f2`.

The live footer now shows package version `0.1.0` and factory build ID
`devcontainer-identity-audit-polish-5` on every route. Ordinary builds inject
the current 12-character Git commit; factory and CI identifiers take priority.
The source templates contain one checked marker, so stale per-page literals
cannot diverge again. Offline cache v11 refreshes the corrected shell.

All cumulative findings from reviews 1–5 are mapped with evidence in
[`.factory/polish-5.md`](polish-5.md). The one-click `?demo=1` path, isolated
DOM-only state, persistent banner, Reset, blank-check exit, first-mobile-screen
result, real routing, titles, metadata, focus announcements, designed 404,
legal links, copy audit, claims registry, privacy, offline behavior, and CLI
package behavior all passed again.

## Verification

Fresh clone: `/tmp/mia-polish5-clean-ZZADyV/repo` at the repair commit.

```sh
npm ci
npm test
npm run lint
npm run build
npm run copy:audit:check
cargo package --locked --allow-dirty
```

- Every one of the 24 exact claim commands in `.factory/claims.json` passed in
  its own invocation from that clone.
- `npm test`: 11 Rust unit tests, 23 CLI integration tests, 30 Vitest checks,
  and 80 Playwright checks passed; eight cross-project skips were intentional.
- Production repeated the 80 passing Playwright checks, including Axe on all
  five routes, desktop/mobile layout, keyboard, focus/history, privacy, and
  offline reload.
- Cold route checks: Home, Demo, Privacy, Terms, and `/404.html` returned 200.
  `/review-polish-5-missing` returned the designed page with HTTP 404. All six
  responses contained `v0.1.0 · devcontainer-identity-audit-polish-5`.
- The URL verifier reported zero console errors, one H1, one main, `lang=en`,
  complete image alternatives, and labelled buttons.
- Live response headers include the repository CSP, Permissions Policy,
  no-referrer policy, nosniff, and preload-eligible HSTS. The hashed hero has
  one-year immutable caching.
- Mobile Lighthouse: performance 99, accessibility 100, best practices 100,
  SEO 100; FCP 0.9 s, LCP 2.0 s, TBT 30 ms, CLS 0.
- Initial JS: 7,246 bytes raw / 3,022 bytes gzip. CSS: 17,047 bytes raw /
  4,327 bytes gzip. Hero: 216,498 bytes.

Live evidence:

- [Desktop first screen](evidence/polish-5-live-desktop-home.png)
- [390×844 sample result](evidence/polish-5-live-mobile-demo.png)
- [Cold verifier](evidence/polish-5-live/verify.json)
- [Lighthouse JSON](evidence/polish-5-live/lighthouse.json)

## Run and deploy

Run the CLI sample with `mount-identity-audit --demo`. Run the site locally
with `npm run build:site && npm run preview`. A factory deployment uses:

```sh
FACTORY_BUILD_ID=<factory-build-id> npm ci
FACTORY_BUILD_ID=<factory-build-id> npm run build:site
/opt/fleet/lib/deploy-static.sh devcontainer-identity-audit dist/site
```

## Known gaps and next steps

No finding or known gap remains within this work order. Registry publishing is
still owned by Param Factory, as required; no package was published here.
