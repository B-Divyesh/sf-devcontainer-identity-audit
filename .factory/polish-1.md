# Polish 1 repair map

Candidate repaired from `190a4a1fe158c9728d9c38c7eeb466898cac8886` in commit
`8334cf22b59a886a3d3a6aceade109daeaadf716`.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Renamed the hero to `mount-ledger-6b7fee8c.webp`; the service worker precaches it and SWA sends immutable one-year caching. | `deployment.test.ts` immutable-hero assertion; live `https://devcontainer-identity-audit.sociobot.in/mount-ledger-6b7fee8c.webp` returned `Cache-Control: public, max-age=31536000, immutable`. |
| F-1-2 | Added focusable route headings and focus handling for demo, fragment, Back, and Forward navigation. | Playwright `forward, back, and fragment navigation focus the destination heading`. |
| F-1-3 | Renamed the install control and its success text to name the copied result. | Landing copy audit; Playwright page structure check. |
| F-1-4 | Replaced every public `preflight` label with literal check/report language. | `copy-audit.md`; Playwright primary-flow check. |
| F-1-5 | Renamed the demo exit to `Open blank browser check`. | Playwright `query demo is isolated, labelled, and moves focus to the sample heading`. |
| F-1-6 | Uses `browser sample` consistently in first-screen privacy facts. | `copy-audit.md`; `@claim:browser-private`. |
| F-1-7 | Replaced the browser-tool heading with `Check numeric workspace access`. | Landing copy audit and query-demo test. |
| F-1-8 | Replaced `Read the map` with `Read the runtime identity map`. | Landing copy audit. |
| F-1-9 | Replaced `Read the inode` with `Read workspace ownership and mode`. | Landing copy audit. |
| F-1-10 | Renamed README heading to `Supported configuration and permission inputs`. | README copy-audit table. |
| F-1-11 | Rewrote the README audience sentence with the numeric report it receives. | README copy-audit table. |
| F-1-12 | Replaced slogan copy with direct `--json` and `--share` instructions. | README copy-audit table; `@claim:report-contract`. |
| F-1-13 | States the bundled sample's exit-code meaning in plain language. | README copy-audit table; `@claim:cli-demo` and `@claim:permission-verdicts`. |
| F-1-14 | Changed terminal caption to `Example output` so it makes no unproved transcript-provenance promise. | Landing copy audit. |
| F-1-15 | Added `browser-report-details` claim and an observable mapping/permission-branch browser assertion. | `@claim:browser-report-details`. |
| F-1-16 | Added configuration-discovery claim covering all three paths and precedence. | `@claim:config-discovery`. |
| F-1-17 | Added numeric/no-runtime behavior as a registered claim. | `@claim:runtime-optional`. |
| F-1-18 | Registered detailed report limits and checks pass, fail, and unknown reports. | `@claim:report-limits`. |
| F-1-19 | Registered Compose build-plus-image conservative handling with a stale-tag fixture. | `@claim:compose-build-image`. |
| F-1-20 | Expanded runtime-mapping claim to promise visible separate runtime cases and assert named reports. | `@claim:runtime-mapping`. |
| F-1-21 | Registered packed installation behavior and checks its sole binary plus help. | `@claim:install-binary`. |
| F-1-22 | Replaced the unsupported test-suite assertion with the command label `Run all repository checks`. | README copy-audit table. |
| F-1-23 | Registered the release executable and `dist/site` build output claim. | `@claim:build-artifacts`. |
| F-1-24 | Completed the claim registry and kept the README registry statement. | `claims.test.ts` one-tag-per-claim validation; all registered commands in fresh clone. |
| F-1-25 | Rebuilt landing and README sentence tables with documented tokenization and corrected counts. | `.factory/copy-audit.md`. |

## Demo and screen evidence

- Query demo: `/?demo=1#demo`, isolated DOM-only sample state, banner, reset, and blank-check exit.
- Desktop screenshot: `/tmp/mia-polish-1-live/screenshot-desktop.png`.
- Mobile screenshot: `/tmp/mia-polish-1-live/screenshot-mobile.png`.

## Verification

`npm test` passed: 10 Rust unit tests, 21 CLI integration tests, 23 Vitest
tests, and 74 Playwright tests (six intentional cross-project skips). The
fresh clone `/tmp/mia-clean-f3Zfgg` completed every exact `claims.json` command
individually; `/tmp/mia-clean-claims.pass` is its completion marker. Cold live
checks passed on `https://devcontainer-identity-audit.sociobot.in/`: factory
URL verifier, 14 desktop browser checks (four mobile-only skips), and 18 mobile
browser checks, including the query demo, focus restoration, 200% text reflow,
privacy, offline reload, 404, and Axe integration.
