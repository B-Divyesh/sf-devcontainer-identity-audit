# Adversarial first-read review 3 — Mount Identity Audit

**Verdict: FAIL**

Reviewed commit `41a0c9e93921b7a51032654dc80bf151d4e546d7` and the live site at
<https://devcontainer-identity-audit.sociobot.in/> on 2 September 2026 UTC.
The product flow, live deployment, and all registered claim tests pass. One
earlier finding is nevertheless still present in the generated copy-audit
evidence. The work-order rule makes this a blocking repeat finding.

## Findings

### F-1-25 — BLOCKING (repeated) — the generated copy-audit table corrupts visible README sentences and their counts

- **Exact location:** `.factory/copy-audit.md`, README table.
  The visible README sentence is
  `The CLI discovers .devcontainer/devcontainer.json, .devcontainer.json, or devcontainer.json.`
  (with each path rendered as inline code). It has **7** whitespace-delimited
  words. The audit instead records the altered text
  `The CLI discovers.devcontainer/devcontainer.json,.devcontainer.json, or devcontainer.json.`
  and count **5**.
- **Second confirmation:** the visible README link text is
  `Every public promise is listed in .factory/claims.json.` and has **7**
  words. The audit records `Every public promise is listed in.factory/claims.json.`
  and count **6**. It also changes the visible Compose placeholder
  `services.<name>.user` to `services..user`.
- **Code cause:** `scripts/generate-copy-audit.mjs` removes HTML-looking text
  before it has protected Markdown inline-code content, and then treats a
  leading filename dot as punctuation in `cleanInline`. The committed table is
  regenerated from that lossy string, so `npm run copy:audit:check` passes
  without validating the reader-facing sentence.
- **Why this blocks acceptance:** review 1 required reproducible, correct
  sentence counts, and review 2 recorded F-1-25 as fixed. The audit is again
  neither an exact inventory nor a correct word-count record. A reviewer cannot
  use it to check the 22-word contract or terminology in the affected rows.
- **Concrete fix:** parse/protect Markdown inline code and links before any
  HTML-tag stripping; only remove whitespace before sentence punctuation when
  that punctuation is not the first character of a filename or code token.
  Regenerate the table so the two rows above read 7, and add regressions for
  `services.<name>.user`, the three discovery paths, and `.factory/claims.json`.

## Cold first-screen check

Fresh Chromium contexts were opened without scrolling at 390×844 and 1440×900.

| Question | Mobile | Desktop | Exact evidence |
| --- | --- | --- | --- |
| What does this do? | Confirmed | Confirmed | `Check mount permissions before container startup` |
| For whom? | Confirmed | Confirmed | `For developers using Dev Containers or rootless Podman who need a writable workspace on the first open.` |
| What should I select first? | Confirmed | Confirmed | `Try it with sample data` and `Runs a known rootless Podman mismatch.` |

In plain terms: this predicts whether the remote user of a Dev Container or
rootless Podman workspace can write to the host bind mount before startup. It is
for developers using those runtimes. The first action is the sample-data path.
No first-screen clarity finding was recorded.

## Copy audit

The audited landing inventory is the 49 reader-facing sentence rows in
`.factory/copy-audit.md` under **Landing sentences**, including the dynamic
offline, validation, copy-feedback, PASS, FAIL, read-only, and recovery
messages. The README inventory is the 59 rows under **README sentences**.
Those tables are the complete sentence inventory used for this review; code
blocks are excluded. All other listed rows are at most 22 words and no banned
marketing term was found. The rows quoted in F-1-25 are exceptions because the
table is not an exact transcription of the page and README.

Headings name their sections (`How it works`, `Runtime differences`, `Limits
and privacy`, and `CLI demo`) rather than using mood copy. The landing's
ordinary actions are result-naming verbs: `Try it with sample data`, `Copy
install command`, `Check mount permissions`, `Load safe example`, `Reset demo`,
and `Open blank browser check`. No new jargon, inconsistent identity term, or
marketing-adjective finding was found.

## Demo, privacy, and CLI sandbox

- One click from the cold landing page opens `/?demo=1#demo` and immediately
  computes a realistic rootless Podman failure. At 390×844 the FAIL title,
  `100999:100999` mapped host identity, and `read · no write · traverse`
  access branch are all visible before scrolling.
- The persistent banner says `Demo — sample data, nothing is saved`, exposes
  **Reset demo** and **Open blank browser check**, and Reset restores the
  original mismatch after both edited and safe-example states.
- The demo used no cookies, Local Storage, Session Storage, or IndexedDB. Its
  request log contained only same-origin static requests and no entered value.
  Entered values remained in the DOM only; leaving demo opened the blank
  calculator.
- From an empty temporary directory, the fresh-clone release binary ran
  `mount-identity-audit --demo`, copied the bundled sample into a unique `/tmp`
  directory, returned `FAIL`/exit 1, and left the invoking directory empty.
- The offline claim was re-run in its own browser context after service-worker
  activation. The browser sample reloaded while offline.

## Claims

All 22 exact commands in `.factory/claims.json` passed independently in a
fresh clone at `/tmp/mia-review3-VksuzR` after `npm ci`. Each command used its
registered `@claim:` grep and its own build/test invocation.

| Claim | Result |
| --- | --- |
| cli-demo | PASS |
| browser-demo | PASS |
| permission-verdicts | PASS |
| read-only-safety | PASS |
| config-support | PASS |
| compose-user-precedence | PASS |
| share-redaction | PASS |
| report-contract | PASS |
| runtime-mapping | PASS |
| conservative-identities | PASS |
| browser-private | PASS |
| cli-private | PASS |
| offline-reload | PASS |
| browser-parity | PASS |
| mit-license | PASS |
| browser-report-details | PASS |
| config-discovery | PASS |
| runtime-optional | PASS |
| report-limits | PASS |
| compose-build-image | PASS |
| install-binary | PASS |
| build-artifacts | PASS |

The live landing and README claim-like statements were checked against this
registry. Apart from the copy-audit evidence defect above, each relied-on
product behavior has a corresponding registered test. No claim test failed.

## History check

Read `.factory/review-1.md`, `.factory/review-2.md`, `.factory/polish-1.md`,
`.factory/polish-2.md`, all `verification*.md` files, and the previous handoff.

- F-1-1 is fixed: the live hashed hero response is
  `Cache-Control: public, max-age=31536000, immutable`.
- F-1-2 through F-1-24 are fixed in both live behavior and code: route heading
  focus/announcements, named copy action, direct check language, demo exit,
  browser-sample terminology, clear headings, registered claims, and build and
  install behavior all passed their associated browser or clean-clone tests.
- F-1-25 is not fixed, as detailed above.
- F-2-1 is fixed: the first mobile sample screen contains the computed result.
  F-2-2 through F-2-10 are fixed: legal route focus/live status, literal
  caption and configuration labels, remote-user terminology, footer one-liner,
  README wording, and copy-failure recovery all passed the live checks.

## Structure and visual checks

Live desktop and mobile Playwright checks passed: 36 assertions, with four
desktop-skipped mobile-only assertions. They confirmed one h1 per route,
`main`, titles, descriptions, canonicals, OG/Twitter metadata, favicon and
apple icon, keyboard flow, visible focus, reduced motion, 200% reflow, 44 px
targets, zero serious/critical Axe violations, designed 404, deep-link history
focus, route announcements, and no console errors.

`/`, `/demo/`, `/privacy/`, `/terms/`, `/404.html`, `robots.txt`, and
`sitemap.xml` returned the expected live content. The unknown-path response is
HTTP 404. Every internal and external crawled link returned 200 (or was an
explicit in-page fragment). The header/footer shell is consistent and includes
Privacy and Terms. The printed-ledger palette, halftone illustration, hard
rules, and stamp interaction are visibly product-specific rather than a generic
SaaS template.

## Missed leverage

No additional AI feature is expected: the brief calls for a deterministic,
local CLI audit, and an AI step would not improve the permission verdict. JSON
reports, `--share`, and the browser/CLI sample paths already cover the implied
export, safe-sharing, and try-before-install needs.

## What would make this perfect

Repair and test the copy-audit generator so it preserves visible Markdown code,
links, and placeholders before counting. Then rerun the full review: the live
product itself is otherwise clear, tryable, private by default, and well
verified.
