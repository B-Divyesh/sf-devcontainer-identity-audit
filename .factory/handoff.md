# Mount Identity Audit — verification 9 handoff

## Status: PASS

Candidate `2d9bcf5f1ba0b524f67eb4d4a287d4ba0eb320c6` was independently
verified on 1 September 2026 UTC at
<https://devcontainer-identity-audit.sociobot.in/>. No product code was changed.
Full evidence is in [`.factory/verification-9.md`](verification-9.md).

## What was verified

- Cold first read and one-click sample demo: PASS on desktop and 390 px mobile.
- Registered claims: 22/22 exact manifest commands PASS after `npm ci`.
- `npm test`, `npm run lint`, and exact `npm run build`: PASS.
- Packed-crate install and public CLI (`--help`, `--demo`, exit codes, JSON,
  boundaries, invalid inputs, and recovery): PASS.
- Live desktop/mobile flows, keyboard use, 200% text, reduced motion, route
  focus, offline reload, privacy request log, and legal/404 routes: PASS.
- Axe: zero violations on five public routes at both viewports.
- Live deployment identity: 17 public files byte-identical to `dist/site`.
- Lighthouse mobile: performance 95, accessibility 100, best practices 100,
  SEO 100; LCP 2.0 s and CLS 0.
- Privacy: only same-origin static GETs; form submission adds no request; no
  cookies or browser database/storage values.
- Headers and caching: restrictive CSP/security headers present; immutable
  hashed assets; short-revalidated HTML and service worker.

The product has no server API, unlock call, account, or sign-in, so rate-limit,
backend concurrency/persistence, and Entra checks do not apply.

## Run again

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --allow-dirty
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in \
  npx playwright test site/e2e/site.spec.ts
VERIFY_NODE_MODULES=/work/repo/node_modules \
  /opt/fleet/lib/verify-url.sh \
  https://devcontainer-identity-audit.sociobot.in/ /tmp/mia-verify-9
```

## Known gaps

No release-blocking gaps. Two low-priority advisories remain:

- Add a smaller responsive hero source; Lighthouse estimates about 151 KiB
  mobile transfer savings, although all current budgets pass.
- Consider aligning the decorative `M↔I` mark with the home link's accessible
  label. Standard Axe passes; Lighthouse's experimental label-in-name audit
  reports the decorative text mismatch.
