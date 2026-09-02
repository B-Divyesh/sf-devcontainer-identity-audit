# Polish 3 repair map

Candidate `989e74d76a5c1b709f705b6538d712b8084d41ea` and review commit
`f3ae0df9d8927e99360bf51367ee9c02e00c29c2` were repaired in
`dac69bc`. Production was deployed and checked cold at
<https://devcontainer-identity-audit.sociobot.in/> on 2 September 2026 UTC.

Visual evidence:

- [live 390×844 query demo](evidence/polish-3-live-mobile-demo.png)
- [live 1440×900 first screen](evidence/polish-3-live-desktop-home.png)
- [URL verifier report](evidence/polish-3-live/verify.json)

## Review 3 repeated finding

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-25 | Protected Markdown inline code and visible link labels before HTML stripping. Sentence punctuation cleanup now preserves leading dots in filename/code tokens. Regenerated the audit with exact 7-word rows for all three reported cases. | `preserves reader-visible Markdown code, links, and angle placeholders`; `matches current landing and README copy with reproducible counts`; `npm run copy:audit:check`; exact rows in `.factory/copy-audit.md`. |

The regression test checks `services.<name>.user`, all three configuration
discovery paths, and `.factory/claims.json` as exact reader-visible strings.

## Review 1 findings rechecked

| Finding | Retained change | Evidence |
| --- | --- | --- |
| F-1-1 | The content-hashed hero remains precached and receives immutable one-year caching. | `static deployment response policy`; live `/mount-ledger-6b7fee8c.webp` returned `Cache-Control: public, max-age=31536000, immutable`. |
| F-1-2 | Demo, fragment, Back, and Forward navigation focus the destination heading. | Live `forward, back, and fragment navigation focus the destination heading`. |
| F-1-3 | The install action says `Copy install command`, with an explicit copied state. | Live structure and clipboard recovery checks. |
| F-1-4 | Public actions use check/report language instead of `preflight`. | Generated copy audit; source scan; live primary flow. |
| F-1-5 | Demo exit says `Open blank browser check`. | Live `query demo is isolated, labelled, and moves focus to the sample heading`. |
| F-1-6 | `browser sample` remains the one browser try-out term. | Generated terminology table; `@claim:browser-private`. |
| F-1-7 | The browser tool heading remains `Check numeric workspace access`. | Live query-demo focus test and mobile screenshot. |
| F-1-8 | Step B remains `Read the runtime identity map`. | Generated copy audit and live page. |
| F-1-9 | Step C remains `Read workspace ownership and mode`. | Generated copy audit and live page. |
| F-1-10 | README uses `Supported configuration and permission inputs`. | Clean-clone copy-audit check. |
| F-1-11 | README names the numeric ownership report without `precise`. | Generated README sentence table. |
| F-1-12 | README directly explains `--json` and `--share`. | `@claim:report-contract`; generated README table. |
| F-1-13 | The bundled sample states what exit code 1 means. | `@claim:cli-demo`; `@claim:permission-verdicts`. |
| F-1-14 | Terminal caption remains `Example output`, without an unproved recording claim. | Generated landing table and live DOM. |
| F-1-15 | Browser output exposes mapped host identity and permission branch. | `@claim:browser-report-details`. |
| F-1-16 | All three configuration paths and their precedence are covered. | `@claim:config-discovery`; repaired exact 7-word audit row. |
| F-1-17 | Numeric identities work without Docker or Podman installed. | `@claim:runtime-optional`. |
| F-1-18 | Detailed pass, fail, and unknown reports list model limits. | `@claim:report-limits`. |
| F-1-19 | Compose `build` plus `image` remains conservative. | `@claim:compose-build-image`. |
| F-1-20 | Docker and rootless Podman reports name separate mappings. | `@claim:runtime-mapping`. |
| F-1-21 | A packed crate installs one documented executable. | `@claim:install-binary`; clean-clone locked `cargo package`. |
| F-1-22 | README uses the non-claiming label `Run all repository checks`. | Generated README table; clean-clone `npm test`. |
| F-1-23 | The build produces the release CLI and `dist/site`. | `@claim:build-artifacts`; clean-clone `npm run build`. |
| F-1-24 | All public promises remain registered with one tagged test each. | `claims.test.ts`; all 22 exact commands passed separately in the clean clone. |
| F-1-25 | Copy counts are generated, reproducible, and now preserve Markdown exactly. | Three copy-audit regressions plus clean-clone `npm run copy:audit:check`. |

## Review 2 findings rechecked

| Finding | Retained change | Evidence |
| --- | --- | --- |
| F-2-1 | The mobile query demo puts its computed FAIL result before the form beneath a two-row banner. | `@claim:browser-demo`; live `fits the viewport and keeps controls reachable`; [390×844 screenshot](evidence/polish-3-live-mobile-demo.png). |
| F-2-2 | Privacy, Terms, Demo, Back, and Forward focus and announce route headings. | Live `legal and demo routes focus and announce their headings across history`. |
| F-2-3 | Hero caption directly explains bind-mount identity mapping. | Generated landing table and live first-screen screenshot. |
| F-2-4 | Step A says `Read the configuration`. | Generated copy audit and live DOM. |
| F-2-5 | Browser form and reports consistently use `remote user`; mapped output uses `host identity`. | `@claim:browser-report-details`; generated terminology table. |
| F-2-6 | Footer explains that the tool checks remote-user write access. | Live route-shell test on all public routes. |
| F-2-7 | README explains `keep-id` and subordinate ranges in two short sentences. | Clean-clone copy-audit check. |
| F-2-8 | README directly states what version 1 does not check. | `@claim:report-limits`; generated README table. |
| F-2-9 | README gives direct crate preparation and publishing instructions. | Clean-clone locked `cargo package`; generated README table. |
| F-2-10 | Clipboard failure keeps the action label and gives a manual recovery message. | Live `copy failure keeps the action label and explains manual recovery`. |

## Complete verification

- Fresh clone: `/tmp/mia-polish3-clean-JVJg3W/repo`.
- Claims: all 22 exact `.factory/claims.json` commands passed separately;
  combined output is `/tmp/mia-polish3-claims.log` in the worker.
- Clean-clone gates: `npm test`, `npm run lint`, `npm run build`,
  `cargo package --locked --allow-dirty`, and `npm run copy:audit:check` passed.
- Browser suite: 78 passed locally across desktop and mobile; six intentional
  project skips. Production site suite: 36 passed; four desktop-only skips.
- Accessibility: Playwright Axe found zero serious or critical violations on
  Home, Demo, Privacy, Terms, and 404 at both viewports. The fleet URL verifier
  found no console errors, missing alternatives, or unnamed buttons.
- Privacy/offline: browser storage remained empty, requests stayed same-origin,
  and a dedicated context reloaded the browser sample offline.
- Production integrity: all 18 served files matched `dist/site` byte for byte;
  every crawled link returned 200 and an unknown route returned HTTP 404.
- Lighthouse: performance 99, accessibility 100, best practices 100, SEO 100;
  FCP 0.9 s, LCP 2.0 s, total blocking time 0 ms, CLS 0, total 226 KiB.

No finding from reviews 1–3 remains open.
