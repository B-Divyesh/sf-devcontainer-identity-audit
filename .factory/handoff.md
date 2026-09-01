# Mount Identity Audit — review 2 handoff

## Status: FAIL

Adversarial first-read review 2 was completed against repository commit
`5ccbfa2eb78b647d3dc12c3c92cccda7abd29771` and the live site on 1 September
2026 UTC. No product code was changed. The complete evidence and fixes are in
`.factory/review-2.md`.

## What was done

- Captured cold 390×844 and 1440×900 first screens.
- Exercised the one-click browser sample, Reset, blank exit, storage isolation,
  same-origin requests, offline behavior, and the CLI `--demo` in a temp folder.
- Ran all 22 claim commands separately from a clean local clone.
- Rechecked every review-1 finding against live behavior and repository code.
- Audited landing and README sentences, headings, actions, terminology, and
  word counts.
- Checked titles, metadata, 404 behavior, deep links, focus, links, shell,
  accessibility, performance budgets, and visual identity.

## Verification

```sh
npm ci
npm test
npm run lint
npm run build
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in \
  npx playwright test site/e2e/site.spec.ts --project=chromium
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in \
  npx playwright test site/e2e/site.spec.ts --project=mobile
```

Results: all 22 registered claim commands passed; `npm test` passed 74 tests
with 6 intentional project skips; lint and build passed; both live structure
suites passed; link crawl found no dead product links; all demo requests were
same-origin; Web Storage, IndexedDB, and cookies remained empty.

## What remains

Two blockers prevent acceptance:

1. The one-click mobile demo hides its already-computed result below a long
   form; the initial demo viewport does not show the product result.
2. Earlier finding F-1-25 regressed: `.factory/copy-audit.md` records a 10-word
   count for a 9-word sentence under its declared tokenizer.

Nine minor copy and route-announcement findings also remain. Apply the exact
fixes in `.factory/review-2.md`, add the specified mobile viewport regression
test, and run a fresh zero-finding review.
