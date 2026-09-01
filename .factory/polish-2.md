# Polish 2 repair map

Candidate `5ccbfa2eb78b647d3dc12c3c92cccda7abd29771` was repaired from review
commit `6cffe11c53d7c54ed8541f7ac40057223931eb10`. Production was checked cold at
<https://devcontainer-identity-audit.sociobot.in/> on 1 September 2026 UTC.

Visual evidence:

- [live 390×844 sample](evidence/polish-2-live-mobile-demo.png)
- [local 390×844 sample](evidence/polish-2-mobile-demo.png)
- [desktop first screen](evidence/polish-2-desktop-home.png)

## Review 2 findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-2-1 | At mobile widths, demo results precede the form, use a compact ledger, and scroll below a two-row sticky banner. | `@claim:browser-demo`; `fits the viewport and keeps controls reachable`; live screenshot above. Live positions: title 378–430, mapped ID 545–579, access 610–644 in 390×844. |
| F-1-25 | Replaced hand counts with `scripts/generate-copy-audit.mjs`; the documented whitespace tokenizer counts `identities—the` once. | `generated copy audit matches current landing and README copy`; `counts the em-dash tokenizer regression as nine words`; `npm run copy:audit:check`. |
| F-2-2 | Added one polite route-status region per page and focusable destination headings. Demo, Privacy, Terms, Back, and Forward move focus and announce the heading. | `legal and demo routes focus and announce their headings across history`; `forward, back, and fragment navigation focus the destination heading`; both passed live. |
| F-2-3 | Rewrote the caption to “A bind mount maps the container identity to a host identity.” | Generated copy audit; desktop screenshot; exact live DOM check in the production suite. |
| F-2-4 | Renamed step A to “Read the configuration.” | Generated copy audit; live route-shell check. |
| F-2-5 | Standardized the form, report, summaries, and remedies on “remote user”; “host identity” is reserved for the mapped value. | `@claim:browser-report-details`; generated terminology table; live sample screenshot. |
| F-2-6 | Replaced the footer slogan with “Check whether a remote user can write to a mounted workspace.” | Generated copy audit; live route-shell test on all five pages. |
| F-2-7 | Split the README explanation into two direct sentences about `keep-id` and subordinate ranges. | `npm run copy:audit:check`; 7-word and 9-word generated rows. |
| F-2-8 | Rewrote the README limit as “Version 1 does not check…” | `npm run copy:audit:check`; `@claim:report-limits`. |
| F-2-9 | Rewrote publishing guidance as two direct actions. | `npm run copy:audit:check`; clean-clone `cargo package` PASS. |
| F-2-10 | The button remains “Copy install command”; a polite status explains manual recovery when Clipboard fails. | `copy failure keeps the action label and explains manual recovery`; passed locally and live. |

## Review 1 findings rechecked

| Finding | Change retained or reinforced | Evidence |
| --- | --- | --- |
| F-1-1 | Hashed hero retains immutable one-year caching. | `static deployment response policy`; live hero `Cache-Control: public, max-age=31536000, immutable`. |
| F-1-2 | Demo, fragment, Back, and Forward focus destination headings. | Both route-focus Playwright tests passed live. |
| F-1-3 | Install action remains “Copy install command.” | Clipboard failure test and generated label audit. |
| F-1-4 | Check/report terminology remains free of “preflight.” | `npm run copy:audit:check`; live copy scan. |
| F-1-5 | Demo exit remains “Open blank browser check.” | `query demo is isolated, labelled, and moves focus to the sample heading`. |
| F-1-6 | “Browser sample” remains the single browser try-out term. | Generated terminology table; `@claim:browser-private`. |
| F-1-7 | Browser heading remains “Check numeric workspace access.” | Mobile sample test and live screenshot. |
| F-1-8 | Step B remains “Read the runtime identity map.” | Generated label audit and live DOM. |
| F-1-9 | Step C remains “Read workspace ownership and mode.” | Generated label audit and live DOM. |
| F-1-10 | README heading remains “Supported configuration and permission inputs.” | Clean-clone copy audit check. |
| F-1-11 | README names the numeric ownership report without “precise.” | Generated README table. |
| F-1-12 | README directly explains `--json` and `--share`. | `@claim:report-contract`; generated README table. |
| F-1-13 | Bundled sample states the meaning of exit code 1. | `@claim:cli-demo`; `@claim:permission-verdicts`. |
| F-1-14 | Terminal caption remains explicitly “Example output.” | Generated landing table and live DOM. |
| F-1-15 | Browser detail promise remains registered and observable. | `@claim:browser-report-details`. |
| F-1-16 | All three discovery locations and precedence remain registered. | `@claim:config-discovery`. |
| F-1-17 | Numeric identities without a runtime remain registered. | `@claim:runtime-optional`. |
| F-1-18 | Pass, fail, and unknown reports retain limit text. | `@claim:report-limits`. |
| F-1-19 | Compose `build` plus `image` remains conservative. | `@claim:compose-build-image`. |
| F-1-20 | Docker and rootless Podman outputs name distinct mappings. | `@claim:runtime-mapping`. |
| F-1-21 | The packed crate installs one documented executable. | `@claim:install-binary`; clean-clone package/install sandbox. |
| F-1-22 | README labels `npm test` without an unsupported coverage slogan. | Generated README table; clean-clone `npm test` PASS. |
| F-1-23 | Build outputs remain registered. | `@claim:build-artifacts`; clean-clone `npm run build` PASS. |
| F-1-24 | The 22-entry registry has one unique tagged test per claim. | Claim-registry Vitest; 22/22 commands passed separately. |
| F-1-25 | Counts are generated and checked, including the original 9-word regression. | Two copy-audit Vitest regressions and clean-clone `npm run copy:audit:check` PASS. |

## Additional evidence-review repair

The prior decorative `M↔I` text was replaced with a hand-drawn CSS symbol so
the visible and accessible brand name agree for voice control. Axe reports zero
serious or critical findings on Home, Demo, Privacy, Terms, and 404 at desktop
and mobile widths. Live Lighthouse accessibility is 100.
