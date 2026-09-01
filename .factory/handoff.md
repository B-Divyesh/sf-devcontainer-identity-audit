# Mount Identity Audit — review 1 handoff

## Status: FAIL

Adversarial first-read review 1 was completed against commit
`190a4a1fe158c9728d9c38c7eeb466898cac8886` and the live site on 1 September
2026 UTC. The full report is `.factory/review-1.md`.

## What was done

- Checked the cold first screen in fresh 390×844 and 1440×900 browser contexts.
- Checked the one-click sample, reset, exit, offline behavior, request log, and
  empty cookie/Web Storage/IndexedDB state.
- Listed and counted every landing-page and README sentence or prose line.
- Ran all 15 exact claim commands independently from a fresh clone.
- Ran `npm test`, `npm run lint`, `npm run build`, the full suite against the
  live URL, the factory URL verifier, route metadata checks, and a complete link
  crawl.
- Checked every earlier verification defect against current code and live
  behavior.
- Confirmed nine core live files match the clean-clone build byte for byte.

## Result

The first read, demo, registered tests, privacy behavior, accessibility checks,
build, routing, links, and visual identity all work. The verdict remains FAIL
because the earlier hero-cache correction is incomplete. The hero still sends
`Cache-Control: public, must-revalidate, max-age=30`. The review also records
route-focus, plain-language, terminology, heading, button-label, unlisted-claim,
and copy-audit count findings.

## How to verify

```sh
npm ci
npm test
npm run lint
npm run build
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in npx playwright test
```

Run every `test` value in `.factory/claims.json` separately from a fresh clone.
Then check the live hero response header and confirm route changes focus the new
heading.

## Product changes

No product code, deployment configuration, infrastructure, DNS, storage, or
external resource was modified. Only this review and handoff were written.
