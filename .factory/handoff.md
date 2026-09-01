# Mount Identity Audit — polish 1 handoff

## Status: deployed and verified

Repair commit: `8334cf22b59a886a3d3a6aceade109daeaadf716`.
The static site was deployed directly to the scoped production Static Web App
and checked cold at `https://devcontainer-identity-audit.sociobot.in/`.

## What changed

- Repaired all 25 adversarial-review findings; the detailed finding-to-change
  map is in `.factory/polish-1.md`.
- The first-screen action opens `/?demo=1#demo`; it starts an isolated DOM-only
  sample, shows the persistent demo banner, resets safely, and exits to a blank
  browser check.
- Hashed the hero as `mount-ledger-6b7fee8c.webp`, precached it, and configured
  one-year immutable caching for its exact URL.
- Added route focus restoration, literal labels/headings, corrected README
  language, a reproducible copy audit, and eight missing claim tests.

## Verification

Executed in `/work/repo`:

```sh
npm run lint
npm test
npm run build
VERIFY_NODE_MODULES=/work/repo/node_modules /opt/fleet/lib/verify-url.sh http://127.0.0.1:4174/ /tmp/mia-polish-1-local
```

Results:

- `npm test`: 10 Rust unit + 21 CLI integration tests, 23 Vitest tests, and
  74 Playwright tests passed; 6 tests were intentional cross-project skips.
- `npm run lint` passed (`cargo fmt`, clippy `-D warnings`, TypeScript).
- `npm run build` passed; static JS is 6.81 kB raw / 2.68 kB gzip and CSS is
  14.86 kB raw / 3.93 kB gzip. The hero is 216,498 bytes.
- Local URL verifier passed: title, `lang=en`, one `h1`, `<main>`, image alt
  text, labelled buttons, and zero browser console errors. Screenshots:
  `/tmp/mia-polish-1-local/screenshot-desktop.png` and
  `/tmp/mia-polish-1-local/screenshot-mobile.png`.
- The bundled `@axe-core/playwright` checks passed on home, query demo, legal,
  and 404 routes on desktop and mobile with no serious or critical violations.
  The standalone Axe CLI could not launch because its Selenium Chrome binary is
  unavailable in this container; the repository's Playwright Axe integration is
  the authoritative accessibility run.
- Fresh clone: `/tmp/mia-clean-f3Zfgg` was created from the repair commit,
  `npm ci` passed with zero vulnerabilities, and every exact command listed in
  `.factory/claims.json` was run independently. Completion marker:
  `/tmp/mia-clean-claims.pass`.

## Run and deploy

```sh
npm ci
npm test
npm run lint
npm run build
```

The factory deploys the static `dist/site` output from `main`. No secrets,
infrastructure, DNS, storage, or unrelated services were accessed.

## Live verification

- `verify-url.sh` passed at the production URL: HTTP 200, one title, `lang=en`,
  one `h1`, `<main>`, image alt text, labelled buttons, and zero console errors.
  Evidence: `/tmp/mia-polish-1-live/verify.json`,
  `/tmp/mia-polish-1-live/screenshot-desktop.png`, and
  `/tmp/mia-polish-1-live/screenshot-mobile.png`.
- Production hero response: `Cache-Control: public, max-age=31536000, immutable`.
- Production desktop browser suite: 14 passed, 4 mobile-only skips. Production
  mobile suite: 18 passed, including 390 px layout and 200% text reflow.
- An unknown production path returns the designed 404 with HTTP 404.

## Known gaps

None.
