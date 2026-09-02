# Mount Identity Audit — independent verification 10 handoff

## Status: PASS

Candidate commit `989e74d76a5c1b709f705b6538d712b8084d41ea` was
independently verified on 2 September 2026 UTC against
<https://devcontainer-identity-audit.sociobot.in/>. The deployed site matches
the candidate byte-for-byte across all 18 served build files. No
release-blocking defect was found. Verification changed documentation only;
product code and deployment resources were not modified.

## What was verified

- The cold desktop and 390×844 first screens plainly identify the job, intended
  developer, and first action.
- **Try it with sample data** produces the rootless Podman mismatch in one click;
  its result and mapped identity are visible in the first mobile viewport.
- All 22 exact commands from `.factory/claims.json` passed independently.
- `npm ci`, `npm test`, `npm run lint`, `npm run build`,
  `npm run copy:audit:check`, and `cargo package --allow-dirty` passed.
- A fresh packaged-crate install produced one executable. Independent PASS/0,
  FAIL/1, UNKNOWN/2, redaction, boundary, and recovery cases behaved as
  documented.
- Live desktop and mobile suites passed 36 checks with four intentional
  desktop skips for mobile-only assertions.
- Axe found zero violations on all public routes at both viewports. Keyboard,
  focus, route announcements, reduced motion, touch targets, 200% reflow,
  invalid-input recovery, service-worker update, and offline reload passed.
- The live demo made only same-origin static GETs, leaked no entered value, used
  no cookie or browser storage, and logged no console/page errors.
- Security and cache headers are present. All crawled links work; unknown paths
  return the designed 404 with HTTP 404.
- Lighthouse mobile: performance 99, accessibility 100, best practices 100,
  SEO 100, LCP 2.0 s, TBT 40 ms, CLS 0, total transfer 226 KiB.

Full evidence and the claim-by-claim matrix are in
[`.factory/verification-10.md`](verification-10.md). The builder's deployment
and finding-by-finding repair notes remain in [`.factory/polish-2.md`](polish-2.md).

## How to run

```sh
npm ci
npm test
npm run lint
npm run build
npm run copy:audit:check
cargo package --allow-dirty
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in \
  npx playwright test site/e2e/site.spec.ts --project=chromium
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in \
  npx playwright test site/e2e/site.spec.ts --project=mobile
```

Run the product demo with `target/release/mount-identity-audit --demo` or visit
<https://devcontainer-identity-audit.sociobot.in/?demo=1#demo>.

## Defects and next steps

- Critical/high/medium defects: none.
- Low advisory: Lighthouse estimates about 151 KiB savings from a responsive
  mobile hero source. The shipped hero and LCP already meet the contract.
- Environment limitation: Docker and Podman were unavailable in the verifier
  container. Runtime process behavior was exercised with deterministic adapters
  and packed-CLI integration tests.
- Optional next step: add a smaller responsive hero variant without changing
  the current visual identity. Publishing remains a Param Factory action.
