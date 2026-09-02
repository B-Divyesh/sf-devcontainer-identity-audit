# Polish 5 repair map

Candidate `bd6b50d7299628387bdca1ee582829f97a106e58` and review commit
`a0f5d68f3fb9bc4ece47e25ab278bdf8e5d864c3` were repaired in
`b4d532e1ff74886205bfdc835c0f08a8737acbbd`. Static Web Apps deployment
`025aac22-a569-470b-ac06-aa7f5eb850f2` was checked cold at
<https://devcontainer-identity-audit.sociobot.in/> on 2 September 2026 UTC.

Live evidence:

- [Desktop first screen](evidence/polish-5-live-desktop-home.png)
- [390×844 query demo](evidence/polish-5-live-mobile-demo.png)
- [Cold URL verifier](evidence/polish-5-live/verify.json)
- [Mobile Lighthouse report](evidence/polish-5-live/lighthouse.json)

The production footer is `v0.1.0 · devcontainer-identity-audit-polish-5` on
Home, Demo, Privacy, Terms, the direct 404 page, and the unknown-route response.

## Review 1 findings

| Finding | Change retained | Evidence |
| --- | --- | --- |
| F-1-1 | The content-hashed hero is precached and served immutable for one year. | `static deployment response policy`; live `/mount-ledger-6b7fee8c.webp` returned `max-age=31536000, immutable`; desktop screenshot. |
| F-1-2 | Demo, fragment, Back, and Forward navigation focus and announce the destination heading. | `forward, back, and fragment navigation focus the destination heading`; production route suite. |
| F-1-3 | The action says `Copy install command` and its status names success or recovery. | `copy failure keeps the action label and explains manual recovery`; live Home. |
| F-1-4 | Public copy consistently uses check/report language instead of `preflight`. | Generated copy audit; production browser suite. |
| F-1-5 | Demo exit says `Open blank browser check`. | `query demo is isolated, labelled, and moves focus to the sample heading`; mobile screenshot. |
| F-1-6 | `browser sample` remains the single browser try-out term. | `@claim:browser-private`; generated terminology table. |
| F-1-7 | The browser heading says `Check numeric workspace access`. | `@claim:browser-demo`; mobile screenshot. |
| F-1-8 | Step B says `Read the runtime identity map`. | Generated copy audit; live Home. |
| F-1-9 | Step C says `Read workspace ownership and mode`. | Generated copy audit; live Home. |
| F-1-10 | The README heading names supported configuration and permission inputs. | Clean-clone `npm run copy:audit:check`. |
| F-1-11 | The README describes the numeric ownership report without unsupported praise. | Clean-clone copy audit. |
| F-1-12 | The README directly explains `--json` and `--share`. | `@claim:report-contract`; clean-clone README audit. |
| F-1-13 | The bundled sample states that a confirmed mismatch returns exit code 1. | `@claim:cli-demo`; `@claim:permission-verdicts`. |
| F-1-14 | The terminal caption says `Example output` and makes no recording-provenance claim. | Generated landing audit; live Home. |
| F-1-15 | The browser report shows the mapped host identity and selected access branch. | `@claim:browser-report-details`; mobile screenshot. |
| F-1-16 | All three configuration discovery paths and precedence are covered. | `@claim:config-discovery`. |
| F-1-17 | Numeric identities work without Docker or Podman installed. | `@claim:runtime-optional`. |
| F-1-18 | Detailed pass, fail, and unknown reports list model limits. | `@claim:report-limits`. |
| F-1-19 | Compose `build` plus `image` stays conservative and returns UNKNOWN. | `@claim:compose-build-image`. |
| F-1-20 | Docker and rootless Podman reports name their distinct mappings. | `@claim:runtime-mapping`; live runtime comparison. |
| F-1-21 | The packed crate installs one documented executable. | `@claim:install-binary`; clean-clone locked package check. |
| F-1-22 | The README labels the repository check command without an unsupported coverage claim. | Clean-clone `npm test`; README audit. |
| F-1-23 | The build produces the release CLI and `dist/site`. | `@claim:build-artifacts`; clean-clone `npm run build`. |
| F-1-24 | Every public behavioural promise has one registered tagged test. | `claims.test.ts`; all 24 exact claim commands passed independently. |
| F-1-25 | Generated counts preserve Markdown paths, links, placeholders, em-dash tokens, and separate UI sentences. | Four `copy-audit.test.ts` regressions; clean-clone `npm run copy:audit:check`. |

## Review 2 findings

| Finding | Change retained | Evidence |
| --- | --- | --- |
| F-2-1 | The 390×844 one-click sample puts its computed result before the form beneath a two-row banner. | `@claim:browser-demo`; `fits the viewport and keeps controls reachable`; mobile screenshot. |
| F-2-2 | Privacy, Terms, Demo, Back, and Forward focus and announce their headings. | `legal and demo routes focus and announce their headings across history`; production suite. |
| F-2-3 | The hero caption literally explains the bind-mount identity mapping. | Generated landing audit; desktop screenshot. |
| F-2-4 | Step A says `Read the configuration`. | Generated copy audit; live Home. |
| F-2-5 | Forms use `remote user`; mapped output uses `host identity`. | `@claim:browser-report-details`; mobile screenshot. |
| F-2-6 | The footer explains that the tool checks remote-user write access. | `every route exposes complete metadata and the standard shell`; live routes. |
| F-2-7 | The README separates `keep-id` and subordinate-range explanations. | Clean-clone copy audit. |
| F-2-8 | The README directly states the version-1 limits. | `@claim:report-limits`; README audit. |
| F-2-9 | The README directly assigns crate preparation and publishing responsibility. | Clean-clone `cargo package --locked --allow-dirty`. |
| F-2-10 | Clipboard failure retains the action name and gives manual recovery. | `copy failure keeps the action label and explains manual recovery`; production suite. |

## Reviews 3 and 4 findings

Review 3 repeated F-1-25; its repair and evidence are in the Review 1 table.

| Finding | Change retained | Evidence |
| --- | --- | --- |
| F-4-1 | The audit generator splits each multi-sentence browser string into its own row. | `lists each sentence from multi-sentence browser messages separately`; clean-clone copy-audit check. |

## Review 5 finding

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-5-1 | Replaced all five hard-coded `polish-3` footers with one Vite HTML transform. It injects the package version plus `FACTORY_BUILD_ID`, a CI commit, or the current short Git commit. Invalid or missing identities fail instead of producing an untraceable release. The service-worker cache advanced to v11. | `uses the current package version and a traceable build identifier`; `prefers a factory build ID and normalizes full commit hashes`; `ships complete metadata and the shared shell`; `every route exposes complete metadata and the standard shell`; production curl checks on all routes and the designed unknown-route response. |

## Completion evidence

- All 24 exact `.factory/claims.json` commands passed independently in fresh
  clone `/tmp/mia-polish5-clean-ZZADyV/repo`.
- That clone passed `npm test` (11 Rust unit, 23 Rust integration, 30 Vitest,
  and 80 Playwright tests; eight intentional project skips), `npm run lint`,
  `npm run build`, `npm run copy:audit:check`, and
  `cargo package --locked --allow-dirty`.
- The same production suite passed 80 tests with eight intentional skips.
  Axe found no serious or critical issue on Home, Demo, Privacy, Terms, or 404
  at desktop and mobile widths.
- The cold URL verifier found no console errors, title/lang/main/alt defect, or
  unnamed button. All public routes returned 200; an unknown path returned the
  designed 404 document with HTTP 404.
- Mobile Lighthouse scored 99 performance, 100 accessibility, 100 best
  practices, and 100 SEO (FCP 0.9 s, LCP 2.0 s, TBT 30 ms, CLS 0).
- Initial JavaScript is 7,246 bytes raw and 3,022 bytes gzip. CSS is 17,047
  bytes raw and 4,327 bytes gzip. The hero is 216,498 bytes.
- The catalog sentence is verb-first and 92 characters:
  `Check whether a workspace mount is writable before Dev Container or
  rootless Podman startup.`

No finding from reviews 1–5 remains open.
