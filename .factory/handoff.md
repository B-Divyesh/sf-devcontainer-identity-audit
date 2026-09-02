# Mount Identity Audit — polish 3 handoff

## Status: PASS

Review 3's repeated copy-audit blocker is repaired in `dac69bc`. The deployed
site is the matching round-3 build at
<https://devcontainer-identity-audit.sociobot.in/>. No known acceptance gap
remains.

## What changed

- Markdown inline code and visible link labels are protected before HTML
  stripping in the generated copy audit.
- Filename-leading dots and `services.<name>.user` survive normalization.
- Exact-string regressions cover the three corrupt rows reported in review 3.
- `.factory/copy-audit.md` now records correct 7-word rows for configuration
  discovery, the Compose placeholder, and `.factory/claims.json`.
- The README links directly to the isolated `?demo=1#demo` sample.
- The public crate description and bundled example use the established
  `permission check` and `remote user` terms.
- The catalog description is a 68-character, verb-first sentence.
- The visible build identity and service-worker cache are advanced to
  `polish-3` and `mia-site-v9`.

The full finding map is in [`.factory/polish-3.md`](polish-3.md).

## Exact verification

From clean clone `/tmp/mia-polish3-clean-JVJg3W/repo`:

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --locked --allow-dirty
npm run copy:audit:check
```

Results:

- `npm ci`: 61 packages, zero audit vulnerabilities.
- `npm test`: 10 Rust unit, 21 CLI integration, 26 Vitest, and 78 Playwright
  checks passed; six viewport-specific checks were intentionally skipped.
- All 22 exact commands in `.factory/claims.json` passed separately from the
  same clean clone. Output: `/tmp/mia-polish3-claims.log` in this worker.
- `npm run lint`, `npm run build`, and locked `cargo package` passed.
- Package: 20 files, 165.1 KiB unpacked and 41.5 KiB compressed.
- Site payload: 7,246 bytes JavaScript raw, 17,008 bytes CSS raw, no fonts, and
  a 216,498-byte hero image.

## Production verification

Deployment used:

```sh
/opt/fleet/lib/deploy-static.sh devcontainer-identity-audit /work/repo/dist/site
```

The fleet deployment completed successfully for the existing scoped resource
`sf-devcontainer-identity-audit`. Cold production checks then confirmed:

- `/`, `/demo/`, `/privacy/`, `/terms/`, `/404.html`, `robots.txt`, and
  `sitemap.xml` return the intended responses; an unknown route returns 404.
- All 18 served files match `dist/site` byte for byte.
- All crawled site, source, license, and Param Factory links return 200.
- The hashed hero returns one-year immutable caching.
- The live desktop/mobile site suite passed 36 checks with four intentional
  desktop viewport skips.
- Axe found zero serious or critical issues on every public route in both
  viewports. The fleet URL verifier found zero console errors.
- Demo values stay out of cookies, Web Storage, IndexedDB, and requests. The
  service worker reloads the sample offline in its own browser context.
- Lighthouse: performance 99, accessibility 100, best practices 100, SEO 100;
  FCP 0.9 s, LCP 2.0 s, total blocking time 0 ms, CLS 0, total 226 KiB.

Screenshots and verifier output:

- [390×844 live query demo](evidence/polish-3-live-mobile-demo.png)
- [1440×900 live first screen](evidence/polish-3-live-desktop-home.png)
- [Fleet verifier JSON](evidence/polish-3-live/verify.json)

## Known gaps and next steps

None. Publishing the crate remains a Param Factory release action; the worker
did not publish it or modify infrastructure outside this product's scoped
static app and DNS record.
