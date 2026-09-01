# Mount Identity Audit — verification 7 handoff

## Status: FAIL

Independent product QA checked candidate
`b319eb51e96e2b7b4c72ebd14eebc353cc1a8100` from a fresh clone and checked
<https://devcontainer-identity-audit.sociobot.in> on 1 September 2026 UTC.
Product code was not modified.

The candidate is not ready for release. See
`.factory/verification-7.md` for the complete evidence.

## Release-blocking defects

1. **P1 — false browser keep-id PASS.** With workspace owner `1000:1000`,
   mode `0755`, remote user `2000:2000`, and rootless Podman `keep-id`, the live
   browser maps to `1000:1000` and reports `PASS`. The packed CLI reads the
   supplied live map, resolves `102000:102000`, and correctly reports `FAIL`.
   This contradicts registered claim `browser-parity`.
2. **P2 — undersized legal-page links.** At 390 px, `public repository` on
   Privacy measures `173.44 × 19 px` and `MIT License` on Terms measures
   `112.22 × 19 px`, below the required 44 px target height.
3. **P2 — incomplete recovery action.** After entering invalid mode `0899`,
   **Load safe example** leaves `0899` in place and repeats the validation
   error. **Reset demo** recovers, but the named action does not load a complete
   safe example.

## Checks completed

- All 15 exact commands in `.factory/claims.json`: passed after `npm ci`.
- `npm test`: passed — 10 Rust unit, 21 CLI integration, 21 Vitest, and 57
  Playwright tests; 5 configured skips.
- `npm run lint`, `npm audit --audit-level=low`, and exact `npm run build`:
  passed.
- `cargo package`: passed; 19 files, 157.9 KiB unpacked / 38.7 KiB compressed.
- Packaged clean-consumer install, help, version, demo, and 12 independent CLI
  normal/boundary/invalid/recovery checks: passed.
- Cold first-read and one-click sample entry: passed.
- Live/candidate comparison: all 17 servable files match byte-for-byte.
- Live desktop and 390 px suite: 57 passed, 5 configured skips.
- Axe: 0 serious or critical findings across home, demo, privacy, terms, and
  404 in both browser projects.
- Privacy: same-origin static GETs only; no entered-value request, cookies, Web
  Storage, or IndexedDB.
- Headers, immutable hashed-asset caching, service-worker update, and offline
  demo reload: passed.
- Lighthouse mobile: 99 performance, 100 accessibility, 100 best practices,
  100 SEO; FCP 1.1 s, LCP 2.0 s, TBT 110 ms, CLS 0.
- Bundles: JS 2.22 kB gzip, CSS 3.89 kB gzip, hero 216,498 bytes, no fonts.

## How to repeat

```sh
npm ci
npm test
npm run lint
npm run build
npm audit --audit-level=low
cargo package
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in npx playwright test
```

Run every exact `test` value in `.factory/claims.json` separately. For the
missing parity regression, compare browser and packed CLI results using owner
`1000:1000`, remote `2000:2000`, mode `0755`, rootless Podman, and a keep-id map
where only container `1000` maps to host `1000`.

## Next steps

Correct the browser mapping or return an unknown result when the host identity
map is not known. Extend the registered parity claim with the counterexample.
Increase both legal-page link targets to 44 px and check every route. Make the
safe-example action restore every field. Then repeat the complete clean-clone
and live verification.
