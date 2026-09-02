# Adversarial first-read review 4 — Mount Identity Audit

**Verdict: FAIL**

Reviewed commit `c81448fc95703e85f4eada28d35152373b412fc2` and the production
site at <https://devcontainer-identity-audit.sociobot.in/> on 2 September 2026
UTC. The cold product explanation, real sample flow, privacy behaviour, live
routes, and all 22 registered claims pass. One documentation-audit defect
remains, so the required zero-finding threshold is not met.

## Findings

### F-4-1 — Minor — the committed copy audit combines separate landing sentences

- **Exact locations:** `.factory/copy-audit.md` records these as single rows:
  `The demo could not complete. Reload and try again.` (**9**),
  `Couldn’t copy. Select the command and copy it manually.` (**9**), and
  `No ownership change is indicated. Confirm with the CLI against the real
  runtime map.` (**14**). The displayed source strings are at
  `site/src/main.ts:55` and `site/src/audit.ts:90`.
- **What the audit must list:** `The demo could not complete.` (**5**) and
  `Reload and try again.` (**4**); `Couldn’t copy.` (**2**) and `Select the
  command and copy it manually.` (**7**); `No ownership change is indicated.`
  (**5**) and `Confirm with the CLI against the real runtime map.` (**9**).
- **Why this matters:** the work order requires every landing and README
  sentence with its own word count. A combined row can conceal a later
  sentence that breaks the 22-word rule, and it prevents an exact copy review.
  This is evidence/documentation QA, not a visitor-facing product failure.
- **Concrete fix:** in `htmlSentences()`, pass quoted strings from
  `site/src/main.ts` and `site/src/audit.ts` through `splitSentences()` instead
  of adding each cleaned string as one row. Regenerate the table, then add
  regression assertions for all three quoted source strings and their six
  individual rows.

## Cold first-screen check

Fresh Chromium contexts were opened without scrolling at 390×844 and 1440×900.

| Question | Mobile | Desktop | Exact evidence |
| --- | --- | --- | --- |
| What does this do? | Confirmed | Confirmed | `Check mount permissions before container startup` |
| For whom? | Confirmed | Confirmed | `For developers using Dev Containers or rootless Podman who need a writable workspace on the first open.` |
| What should I click first? | Confirmed | Confirmed | `Try it with sample data` followed by `Runs a known rootless Podman mismatch.` |

In plain words: it predicts whether a Dev Container or rootless Podman remote
user can write to the mounted workspace before startup. It is for developers
using those tools. The first action is the sample-data link. No first-screen
blocking finding was recorded.

## Copy audit

The complete landing and README inventories, including word counts, are in
the committed [`.factory/copy-audit.md`](copy-audit.md), generated from the
landing, reachable dynamic strings, and README. `npm run copy:audit:check`
passes from the clean clone. The one exception is F-4-1: its three combined
landing rows must be read as these six individual rows:

| Words | Landing sentence |
| ---: | --- |
| 5 | The demo could not complete. |
| 4 | Reload and try again. |
| 2 | Couldn’t copy. |
| 7 | Select the command and copy it manually. |
| 5 | No ownership change is indicated. |
| 9 | Confirm with the CLI against the real runtime map. |

All other listed landing rows and all 59 README rows are at most 22 words. No
banned marketing term appeared. Headings name their sections, and the ordinary
actions name their results: `Try it with sample data`, `Copy install command`,
`Check mount permissions`, `Load safe example`, `Reset demo`, and `Open blank
browser check`. The terminology table consistently uses `remote user`,
`workspace`, `identity map`, `browser sample`, and `bundled sample`.

The landing and README were also cross-checked against `.factory/claims.json`.
Every relied-on behaviour has a registered claim. No unlisted product or
privacy claim was found.

## Demo, sandbox, and privacy verification

- One click from the cold home page opened `/?demo=1#demo`. At 390×844, the
  initial post-click view showed `Mount mismatch predicted`, mapped identity
  `100999:100999 · rootless subuid map`, and `read · no write · traverse`.
- The persistent banner read `Demo — sample data, nothing is saved` and exposed
  **Reset demo** plus **Open blank browser check**. After the safe example,
  Reset restored the original `fail` result; exiting returned to the blank
  `Ready` calculator.
- A fresh-context request log recorded only
  `https://devcontainer-identity-audit.sociobot.in` requests. Cookies, local
  storage, session storage, and IndexedDB were all empty during the demo.
- The offline browser test passed in its own context after service-worker
  activation. The sample reloads offline after the first visit.
- The clean-clone CLI demo test ran the shipped `--demo` command in an isolated
  temporary directory. It copied the bundled sample there, returned its known
  `FAIL`/exit 1, and did not modify the invoking directory.

## Claims and quality gates

Every exact command listed in `.factory/claims.json` was run separately in a
fresh clone at `/tmp/mia-review4-clean-tk3Aof/repo` after `npm ci`.

| Claims | Result |
| --- | --- |
| `cli-demo`, `browser-demo`, `permission-verdicts`, `read-only-safety`, `config-support`, `compose-user-precedence` | PASS |
| `share-redaction`, `report-contract`, `runtime-mapping`, `conservative-identities`, `browser-private`, `cli-private` | PASS |
| `offline-reload`, `browser-parity`, `mit-license`, `browser-report-details`, `config-discovery`, `runtime-optional` | PASS |
| `report-limits`, `compose-build-image`, `install-binary`, `build-artifacts` | PASS |

The same clean clone passed `npm test`: 10 Rust unit tests, 21 Rust integration
tests, 26 Vitest tests, and 78 Playwright tests, with six intentional
project-specific skips. `npm run build`, `npm run copy:audit:check`, and all
claim commands also passed. The output includes the release executable and
`dist/site`.

## Earlier finding verification

I read every earlier `review-*.md`, `polish-*.md`, verification report, and
the prior handoff. Each prior finding was checked against live behaviour and
the current repository rather than accepted from its repair map.

| Earlier finding | Status | Current evidence |
| --- | --- | --- |
| F-1-1 | Fixed | Hashed hero is immutable for one year in the live response. |
| F-1-2 | Fixed | Live demo, fragment, Back, and Forward focus the destination heading. |
| F-1-3 | Fixed | Install action says `Copy install command`. |
| F-1-4 | Fixed | Public copy uses check/report language; no `preflight` remains. |
| F-1-5 | Fixed | Demo exit says `Open blank browser check`. |
| F-1-6 | Fixed | Browser try-out terminology is consistently `browser sample`. |
| F-1-7 | Fixed | Browser section says `Check numeric workspace access`. |
| F-1-8 | Fixed | Step B says `Read the runtime identity map`. |
| F-1-9 | Fixed | Step C says `Read workspace ownership and mode`. |
| F-1-10 | Fixed | README heading names supported configuration and permission inputs. |
| F-1-11 | Fixed | README audience copy names the numeric ownership report without marketing copy. |
| F-1-12 | Fixed | README directly explains `--json` and `--share`. |
| F-1-13 | Fixed | Bundled sample documents the exit-1 meaning. |
| F-1-14 | Fixed | Terminal caption is explicitly an example output. |
| F-1-15 | Fixed | Browser result shows mapped identity and permission branch. |
| F-1-16 | Fixed | All three configuration paths and precedence pass the registered test. |
| F-1-17 | Fixed | Numeric identities work without a runtime. |
| F-1-18 | Fixed | Pass, fail, and unknown reports state the documented limits. |
| F-1-19 | Fixed | Compose `build` plus `image` returns a conservative unknown result. |
| F-1-20 | Fixed | Docker and rootless Podman report distinct named mappings. |
| F-1-21 | Fixed | Packed crate installs one documented executable. |
| F-1-22 | Fixed | README uses the non-claiming repository-check command label. |
| F-1-23 | Fixed | Registered build-artifacts test confirms both declared outputs. |
| F-1-24 | Fixed | Registry validation and all 22 independently run commands pass. |
| F-1-25 | Fixed | Markdown paths, links, placeholders, and their exact word counts are preserved. |
| F-2-1 | Fixed | Mobile demo result, mapping, and access branch are in the first viewport. |
| F-2-2 | Fixed | Privacy, Terms, Demo, Back, and Forward focus and announce their headings. |
| F-2-3 | Fixed | Caption says that the bind mount maps container identity to host identity. |
| F-2-4 | Fixed | Step A says `Read the configuration`. |
| F-2-5 | Fixed | Forms and results use `remote user`; mapped result is host identity. |
| F-2-6 | Fixed | Footer explains the remote-user workspace-write check. |
| F-2-7 | Fixed | README separates the keep-id and subordinate-range explanation. |
| F-2-8 | Fixed | README plainly says what version 1 does not check. |
| F-2-9 | Fixed | README gives direct package and publishing responsibilities. |
| F-2-10 | Fixed | Clipboard failure keeps the action name and gives manual recovery text. |

F-4-1 is a new audit-granularity issue; it does not repeat the now-fixed
Markdown corruption and miscount cases of F-1-25.

## Structure and visual checks

Live desktop and mobile Playwright runs passed: 16 desktop checks plus four
intentional mobile-only skips, and all 20 mobile checks. They cover one h1,
main landmark, keyboard-only flow, focus visibility, reduced motion, 200%
reflow, 44-pixel targets, route focus and live announcements, console errors,
and Axe serious/critical violations on Home, Demo, Privacy, Terms, and 404.

`/`, `/demo/`, `/privacy/`, `/terms/`, `/404.html`, `robots.txt`, and
`sitemap.xml` served correctly. An unknown route returned HTTP 404. All crawled
links returned HTTP 200 (or were same-page fragments). Header/footer links,
canonical metadata, Open Graph/Twitter metadata, favicon, and designed 404 are
present. Live CSP, referrer policy, and nosniff headers are present; the CSP
uses `frame-ancestors 'none'` as a response header. The dithered technical
ledger, warm paper palette, hard rules, and stamped permission report are
distinct from a generic SaaS template and match `.factory/design.md`.

## Missed leverage

No missing AI feature was found. The brief calls for a deterministic local
permission audit, where an AI step would weaken rather than improve a numeric
verdict. The product already supplies the implied export and safe-sharing path
through JSON reports and `--share`, plus browser and CLI sample paths.

## What would make this perfect

Repair F-4-1, regenerate the copy-audit table, and rerun the full review. The
product itself is clear, tryable, privacy-preserving, and well verified.
