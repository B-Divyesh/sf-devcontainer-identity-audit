# Independent verification 13 — Mount Identity Audit

## Verdict: PASS

Candidate commit `386f646bc47590ff11b2775595ee56c736b94b9a` was independently verified on 2 September 2026 UTC against <https://devcontainer-identity-audit.sociobot.in/>. The live deployment is byte-identical to the candidate's deployable output. No product code or infrastructure was changed during this verification.

## Mandatory first-read and demo gate

Cold-loading the live desktop page answers all three questions in plain words:

- **What it does:** “Check mount permissions before container startup.”
- **For whom:** developers using Dev Containers or rootless Podman who need a writable workspace on the first open.
- **What to click:** the first-screen “Try it with sample data” action, with the adjacent explanation “Runs a known rootless Podman mismatch.”

That action opens `/?demo=1#demo` in one click. At 390 × 844 the sample shows `Mount mismatch predicted`, mapped identity `100999:100999`, and the `read · no write · traverse` access branch in the first viewport. The persistent banner says “Demo — sample data, nothing is saved” and has Reset demo and Open blank browser check controls. This passes the first-read and sandbox requirements.

## Registered claims

`.factory/claims.json` exists and contains 24 claims. From this clean candidate, I ran **each listed command separately**, using the precise `npm run test:claims -- --grep @claim:<id>` command. Every command passed:

| Claims | Result |
| --- | --- |
| `cli-demo`, `browser-demo`, `permission-verdicts`, `read-only-safety` | PASS |
| `config-support`, `compose-user-precedence`, `share-redaction`, `report-contract` | PASS |
| `runtime-mapping`, `docker-userns-remap`, `read-only-remediation`, `conservative-identities` | PASS |
| `browser-private`, `cli-private`, `offline-reload`, `browser-parity` | PASS |
| `mit-license`, `browser-report-details`, `config-discovery`, `runtime-optional` | PASS |
| `report-limits`, `compose-build-image`, `install-binary`, `build-artifacts` | PASS |

In particular, the formerly unsafe Docker `name=userns` case returns `UNKNOWN`/exit 2 rather than assuming direct host IDs, and all `readonly`, `read_only`, and `ro` cases lead with mount-setting recovery rather than an irrelevant identity fix.

## Local quality gates and CLI consumer

- `npm ci`: PASS — 61 packages installed; `npm audit --audit-level=low` reported zero vulnerabilities.
- `npm test`: PASS — 11 Rust unit tests, 23 Rust integration tests, 28 Vitest tests, and 80 Playwright tests passed; eight intentional project-specific skips.
- `npm run lint`: PASS — rustfmt, warnings-as-errors Clippy, and TypeScript typecheck.
- `npm run copy:audit:check`: PASS.
- Exact `npm run build`: PASS — produced `target/release/mount-identity-audit` and `dist/site`.
- `cargo package --locked --allow-dirty`: PASS — package verification succeeded; 20 files, 172.0 KiB unpacked and 42.8 KiB compressed.
- Fresh consumer check: unpacked `target/package/mount-identity-audit-0.1.0.crate`, installed it with `cargo install --path … --locked --offline` into a new temporary prefix, and found exactly one `mount-identity-audit` executable. `--version` printed `0.1.0`, `--help` described the public CLI, and `--demo` copied the bundled sample to `/tmp`, reported the known `FAIL`, and exited 1.

The automated and consumer cases cover normal readable/writable and denied modes, rootless Podman map boundaries, Docker userns-remap, Compose precedence, JSONC/invalid configuration recovery, reserved/named/UID-only identities, read-only mounts, share redaction, and isolated demo behavior.

## Live deployment, accessibility, privacy, and performance

- Live Playwright route verification at the production URL: **36 passed, 4 intentional desktop-only skips**. It exercised desktop and 390 px mobile, keyboard-only operation, visible skip-link focus, 200% text reflow, 44 px targets, reduced motion, demo reset/exit, history focus announcements, offline reload, and all public routes.
- Axe, through the live Playwright suite, found **zero serious or critical violations** on Home, Demo, Privacy, Terms, and 404. The live test also recorded no console or page errors.
- Independent privacy capture filled Owner UID with the unique value `3141592`, then ran the browser calculation. The only requests were same-origin GETs for the document and self-hosted assets; the value was in neither URL nor body. Cookies were empty; `localStorage`, `sessionStorage`, and IndexedDB each had zero entries.
- Production returns HTTP 200 for the landing page, explicit 404 for an unknown route, a self-only CSP including `frame-ancestors 'none'`, HSTS preload, `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and restrictive Permissions Policy. HTML and `sw.js` revalidate after 30 seconds; a conditional HTML request returned 304. Hashed JS/CSS use `max-age=31536000, immutable`.
- Every one of the 18 deployable files in the freshly built `dist/site` output matched the corresponding live response byte-for-byte. The deployment is the candidate build, not a stale release.
- Initial JS is 7,246 bytes raw / 3,021 bytes gzip across two files; CSS is 17,008 bytes raw / 4,315 bytes gzip; no font files are loaded; the first-screen image is 216,498 bytes. These are within the stated static budgets.

This is a static local-first CLI landing site: it has no server-side product endpoints, accounts, payment calls, sign-in, or product-unlock requests. Rate limiting, concurrency/persistence, and Entra tenant checks are not applicable. The deterministic product does not need an AI feature.

## Defects by severity

- Critical: none.
- High: none.
- Medium: none.
- Low: none.
