# Polish 4 repair map

Candidate `c81448fc95703e85f4eada28d35152373b412fc2` and cumulative review
commit `1b1cdfbb399d52f434158bfea4eb49ade48bb6bc` were repaired by
`b01b201`. It was deployed as Static Web Apps deployment
`1120a0a0-d53e-4f3d-95b1-0b974f5ff462` and cold-checked at
<https://devcontainer-identity-audit.sociobot.in/> on 2 September 2026 UTC.

Live visual evidence:

- [Desktop first screen](evidence/polish-4-live-desktop-home.png)
- [390×844 query demo](evidence/polish-4-live-mobile-demo.png)
- [Cold URL verifier](evidence/polish-4-live/verify.json)

All rows below include the retained repair, named automated evidence, and the
fresh live check. The browser evidence is the live Playwright route suite (36
passed, four intentional project skips), URL verifier, and five-route Axe run.

## Review 1 findings

| Finding | Change retained | Evidence |
| --- | --- | --- |
| F-1-1 | Hashed hero stays precached with one-year immutable caching. | `static deployment response policy`; live hero header; desktop screenshot. |
| F-1-2 | Demo, fragment, Back, and Forward focus and announce the destination heading. | `forward, back, and fragment navigation focus the destination heading`; live route suite. |
| F-1-3 | Install action says `Copy install command` and names the successful state. | `copy failure keeps the action label and explains manual recovery`; live Home. |
| F-1-4 | Public language consistently says check/report, not preflight. | Generated copy audit; live Home. |
| F-1-5 | Demo exit says `Open blank browser check`. | `query demo is isolated, labelled, and moves focus to the sample heading`; mobile screenshot. |
| F-1-6 | `browser sample` is used consistently. | `@claim:browser-private`; live Home. |
| F-1-7 | Browser heading is `Check numeric workspace access`. | `@claim:browser-demo`; mobile screenshot. |
| F-1-8 | Step B says `Read the runtime identity map`. | Generated copy audit; live Home. |
| F-1-9 | Step C says `Read workspace ownership and mode`. | Generated copy audit; live Home. |
| F-1-10 | README heading names configuration and permission inputs. | `npm run copy:audit:check` in clean clone. |
| F-1-11 | README describes a numeric ownership report without unsupported praise. | Clean-clone copy audit. |
| F-1-12 | README explains `--json` and `--share` directly. | `@claim:report-contract`; clean-clone README audit. |
| F-1-13 | Bundled-sample text explains exit code 1. | `@claim:cli-demo`, `@claim:permission-verdicts`. |
| F-1-14 | Terminal caption is explicitly example output, not an unproved recording claim. | Generated landing audit; live Home. |
| F-1-15 | Browser report shows mapped host identity and selected access branch. | `@claim:browser-report-details`; mobile screenshot. |
| F-1-16 | All three discovery paths and precedence are covered. | `@claim:config-discovery`. |
| F-1-17 | Numeric identities work without Docker or Podman. | `@claim:runtime-optional`. |
| F-1-18 | Detailed pass, fail, and unknown reports state the model limits. | `@claim:report-limits`. |
| F-1-19 | Compose `build` plus `image` stays conservative. | `@claim:compose-build-image`. |
| F-1-20 | Docker and rootless Podman reports name their different mappings. | `@claim:runtime-mapping`; live Home. |
| F-1-21 | Packed crate installs exactly the documented executable. | `@claim:install-binary`; clean-clone package. |
| F-1-22 | README labels the repository check command without a coverage promise. | Clean-clone `npm test`; README audit. |
| F-1-23 | Build outputs are registered and tested. | `@claim:build-artifacts`; clean-clone `npm run build`. |
| F-1-24 | Every public behavioural promise has one registered claim test. | `claims.test.ts`; 22 independent clean-clone claim commands. |
| F-1-25 | Markdown paths, links, placeholders, and exact counts are preserved. | `preserves reader-visible Markdown code, links, and angle placeholders`; clean-clone copy-audit check. |

## Review 2 findings

| Finding | Change retained | Evidence |
| --- | --- | --- |
| F-2-1 | The one-click mobile sample puts its computed result before the form and keeps the banner to two rows. | `@claim:browser-demo`, `fits the viewport and keeps controls reachable`; [live mobile screenshot](evidence/polish-4-live-mobile-demo.png). |
| F-2-2 | Privacy, Terms, Demo, Back, and Forward focus and announce their headings. | `legal and demo routes focus and announce their headings across history`; live route suite. |
| F-2-3 | Hero caption names the bind-mount identity mapping literally. | Generated landing audit; desktop screenshot. |
| F-2-4 | Step A says `Read the configuration`. | Generated landing audit; live Home. |
| F-2-5 | Forms use `remote user`; post-map output uses `host identity`. | `@claim:browser-report-details`; mobile screenshot. |
| F-2-6 | Footer says what can be checked. | `every route exposes complete metadata and the standard shell`; live routes. |
| F-2-7 | README separates keep-id from subordinate-range explanation. | Clean-clone copy audit. |
| F-2-8 | README states the version-1 limits directly. | `@claim:report-limits`; clean-clone README audit. |
| F-2-9 | README gives direct crate preparation and factory publishing steps. | Clean-clone `cargo package --locked --allow-dirty`. |
| F-2-10 | Clipboard failure preserves the action name and gives manual recovery. | `copy failure keeps the action label and explains manual recovery`; live Home. |

## Review 4 finding

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-4-1 | `htmlSentences()` now uses `splitSentences()` for quoted UI source strings. The audit has six separate rows: 5/4 for the demo error, 2/7 for copy recovery, and 5/9 for the ownership remedy. | `lists each sentence from multi-sentence browser messages separately`; clean-clone `npm run copy:audit:check`; live production recheck at <https://devcontainer-identity-audit.sociobot.in/>. |

## Completion evidence

- Every command listed in `.factory/claims.json` passed separately in fresh
  clone `/tmp/mia-polish4-clean-u5zYbI/repo`.
- That clone passed `npm test`, `npm run lint`, `npm run build`,
  `npm run copy:audit:check`, and `cargo package --locked --allow-dirty`.
- Live URL verification reported no console errors, title/lang/main/alt issues,
  or unnamed buttons. Axe passed on Home, Demo, Privacy, Terms, and 404.
- Live mobile Lighthouse scored 99 performance, 100 accessibility, 100 best
  practices, and 100 SEO (FCP 1.0 s, LCP 2.0 s, TBT 20 ms, CLS 0).
- The catalog description now reads: `Check workspace mount access before Dev
  Container or rootless Podman startup.`
