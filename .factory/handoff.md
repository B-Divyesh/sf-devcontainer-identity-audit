# Mount Identity Audit — review 3 handoff

## Status: FAIL

Reviewer-only documentation was added in commit `HEAD`; product code and live
resources were not changed. The live site was reviewed on 2 September 2026 UTC.

## What was verified

- Cold 390×844 and desktop screens answered the job, audience, and first action.
- The one-click browser sample immediately showed a realistic rootless Podman
  FAIL result, retained its demo banner, reset correctly, used no browser
  storage, and made only same-origin static requests.
- The CLI `--demo` command ran from an empty temporary directory, copied the
  bundled sample to its own temporary path, returned exit 1, and left the
  invoking directory untouched.
- Every one of the 22 exact `.factory/claims.json` commands passed in a fresh
  clone after `npm ci`.
- The live desktop and mobile route/accessibility suite passed. It covered
  metadata, 404, links, keyboard, focus, mobile result visibility, offline
  reload, privacy, console errors, and Axe serious/critical violations.

## Blocking issue

`.factory/copy-audit.md` still corrupts visible Markdown code/link sentences
and undercounts them. This repeats F-1-25 from review 1, which review 2 had
marked fixed. Details and the concrete repair are in
[`.factory/review-3.md`](review-3.md).

## How to verify

```sh
npm ci
npm run copy:audit:check
npm run test:claims
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in \
  npx playwright test site/e2e/site.spec.ts --project=chromium --project=mobile
```

Run the product demo with `target/release/mount-identity-audit --demo` or visit
<https://devcontainer-identity-audit.sociobot.in/?demo=1#demo>.

## Next step

Repair the Markdown-normalization order in `scripts/generate-copy-audit.mjs`,
add exact-string/count regressions for inline code, links, and angle-bracket
placeholders, regenerate `.factory/copy-audit.md`, and rerun the whole review.
