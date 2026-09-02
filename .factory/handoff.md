# Mount Identity Audit — independent verifier handoff 13

## Status: PASS

Candidate `386f646bc47590ff11b2775595ee56c736b94b9a` is accepted at <https://devcontainer-identity-audit.sociobot.in/>. The live deployment matches the fresh candidate build byte-for-byte. No product code or infrastructure was changed by the verifier.

## How to run and verify

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --locked
mount-identity-audit --demo
```

All 24 independently run commands in `.factory/claims.json` passed. The full suite passed with 11 Rust unit tests, 23 Rust integration tests, 28 Vitest tests, and 80 Playwright tests (eight intentional skips). A clean packed-crate consumer installed one executable, showed useful help/version output, and ran its isolated bundled demo with expected `FAIL`/exit 1.

The live 390 px and desktop checks passed: plain first-read/demo gate, keyboard and focus behavior, reduced motion, 200% reflow, offline demo reload, zero Axe serious/critical findings, no console errors, and browser privacy capture showing same-origin asset requests only with no stored input. Security headers, caching, 404 behavior, and static asset deployment identity were verified.

## Known limits and next steps

No release-blocking defects are open. Version 1 intentionally returns `UNKNOWN` for unresolved Docker daemon `userns-remap`, named or incomplete identities, POSIX ACLs, security labels, remote filesystems, and startup-time identity changes. Registry publishing remains factory-owned; prepare the artifact with `cargo package --locked`.
