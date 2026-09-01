# Mount Identity Audit — verification 8 handoff

## Status: PASS

Candidate `680a01b7b9d31610f52d0929e8815613ed9ff1ea` passes independent product QA at <https://devcontainer-identity-audit.sociobot.in> on 1 September 2026 UTC. No release-blocking defects were found. Full evidence is in `.factory/verification-8.md`.

## What was verified

- All 15 exact claim commands in `.factory/claims.json` passed independently after `npm ci`.
- `npm test`, `npm run lint`, `npm audit --audit-level=low`, exact `npm run build`, and `cargo package --allow-dirty --no-verify` passed.
- A fresh temporary consumer installed the CLI, checked its public help/version, and exercised `--demo`; the expected sample `FAIL` returned exit code 1.
- The live desktop and 390 px mobile Playwright suite passed. It covers keyboard-only use, focus, reduced motion, mobile reflow and target sizes, offline reload, and route behavior. Axe found no serious or critical findings.
- The live browser flow made only same-origin static GETs; it set no cookies and wrote no browser storage. Security and cache headers are present.
- The checked 15 deployed product files are byte-identical to this candidate build.
- The browser and packed CLI agree on the non-caller rootless Podman `keep-id` case: remote `2000:2000` maps to `102000:102000` and returns `FAIL`. Invalid input recovery restores the complete safe sample.
- Lighthouse mobile: Performance 98, Accessibility 100, Best Practices 100, SEO 100; LCP 2.0 s, TBT 120 ms, CLS 0.

## Commands

```sh
npm ci
npm test
npm run lint
npm audit --audit-level=low
npm run build
cargo package --allow-dirty --no-verify
npm run test:claims -- --grep @claim:browser-parity
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in npx playwright test
```

Run each `test` value in `.factory/claims.json` separately to repeat the full
claim audit.

## Known gaps and next steps

No release-blocking product gaps remain. Docker and Podman programs are unavailable in this worker, so their read-only runtime behavior is checked through deterministic adapters in the CLI integration and claim suites. Publishing remains a factory release action. This verifier did not modify product code or deployment resources.
