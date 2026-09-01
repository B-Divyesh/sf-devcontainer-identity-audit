# Mount Identity Audit — independent verification 6 handoff

## Status: FAIL

Candidate `eb570fd4b2183cde1c4e5d13432583e9d3f47fd7` and
`https://devcontainer-identity-audit.sociobot.in` were independently tested on
1 September 2026 UTC. Do not release this candidate.

Two P1 findings block release:

1. An explicit Dev Container `containerUser` is overwritten by a selected
   Compose service `user`. A `containerUser` of `424242:424242` with Compose
   `user: 0:0` on a root-owned `0755` workspace incorrectly returns exit 0 and
   `PASS`. The reverse conflict also produces the wrong result.
2. After clean `npm ci`, every exact command in `.factory/claims.json` times
   out waiting for the Playwright preview server because `test:claims` does not
   build `dist/site`. The same 14 assertions pass only after a separate site
   build, which does not satisfy the clean-clone claims contract.

One P2 presentation finding remains: the three required plain facts are below
the initial viewport at both 1440×900 and 390×844, although the job, audience,
and one-click sample action are visible and pass the explicit first-read gate.

Full evidence, exact reproductions, claim-by-claim outcomes, and recommended
repairs are in [verification-6.md](verification-6.md).

## Verification summary

- `npm ci`: PASS; 0 vulnerabilities.
- `npm test`: PASS — 8 Rust unit, 19 CLI integration, 21 Vitest, and 54
  applicable Playwright tests; 4 expected mobile-only skips.
- `npm run lint`: PASS.
- `npm run build`: PASS; produced the release binary and `dist/site`.
- `cargo package --allow-dirty`: PASS; clean-prefix install and public CLI
  cases completed.
- Exact claim commands from the clean installed state: **14/14 FAIL** at the
  preview readiness timeout.
- Claim assertions after an explicit site build: 14/14 PASS.
- Full live suite: 54 PASS, 4 expected skips.
- Live files match the candidate build byte-for-byte.
- Axe serious/critical: 0; keyboard, focus, reduced motion, 390 px, 200% text,
  error recovery, and 44 px targets passed.
- Privacy: same-origin public-file GETs only; no request on calculation; no
  cookies or user-value storage.
- Offline reload and service-worker update: PASS.
- Security headers and cache policy: PASS.
- Lighthouse mobile: 98 performance, 100 accessibility, 100 best practices,
  100 SEO; LCP 2.1 s; CLS 0; 224 KiB transfer.
- Static product: no server API, rate-limit requirement, sign-in, or payment
  integration applies.

No product code, deployment, infrastructure, runtime, or user project was
modified during verification. Only this report and handoff were changed.
