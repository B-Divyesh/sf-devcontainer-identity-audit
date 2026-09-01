# Independent verification 8 — Mount Identity Audit

## Verdict: PASS

Candidate commit `680a01b7b9d31610f52d0929e8815613ed9ff1ea` was independently checked from a clean checkout on 1 September 2026 UTC. The deployed product checked was <https://devcontainer-identity-audit.sociobot.in>.

No release-blocking defects were found. The deployed static files match the candidate production build, and the repaired browser-to-CLI mapping, mobile target-size, and invalid-input recovery checks pass.

## First-read check

A cold desktop browser visit returned HTTP 200 with no console or page errors.

- It says what it does: “Check mount permissions before container startup.”
- It says who it is for: developers using Dev Containers or rootless Podman who need a writable workspace on first open.
- It says what to select first: **Try it with sample data**, with the adjacent explanation “Runs a known rootless Podman mismatch.”
- One selection opens the working `/demo/` sample, immediately shows the populated `FAIL` result, and displays the persistent “Demo — sample data, nothing is saved” banner with **Reset demo** and **Start for real**.

This satisfies the plain-language and one-click sample requirements at desktop and 390 px mobile widths.

## Registered claims

`.factory/claims.json` is present and contains 15 entries. After `npm ci`, every exact command listed in the manifest was run independently from the candidate checkout via its documented demo entry point.

| Claim | Result |
| --- | --- |
| `cli-demo` | PASS |
| `browser-demo` | PASS |
| `permission-verdicts` | PASS |
| `read-only-safety` | PASS |
| `config-support` | PASS |
| `compose-user-precedence` | PASS |
| `share-redaction` | PASS |
| `report-contract` | PASS |
| `runtime-mapping` | PASS |
| `conservative-identities` | PASS |
| `browser-private` | PASS |
| `cli-private` | PASS |
| `offline-reload` | PASS |
| `browser-parity` | PASS |
| `mit-license` | PASS |

The independent browser parity case used workspace owner and caller `1000:1000`, remote user `2000:2000`, mode `0755`, rootless Podman, and `keep-id`. The live browser reported `FAIL` with `102000:102000 · keep-id mapping`, agreeing with the packed CLI rule. An invalid `0899` mode produced the announced validation message; **Load safe example** then restored `0755` and produced `PASS`.

## Clean-checkout build, tests, and CLI consumer

- `npm ci`: PASS — 61 packages installed; `npm audit --audit-level=low` reported 0 vulnerabilities.
- `npm test`: PASS — 10 Rust unit tests, 21 Rust integration tests, 23 Vitest checks, and the 62-test Playwright configuration completed successfully (57 applicable checks and 5 documented project-specific skips).
- `npm run lint`: PASS — Rust formatting, strict Clippy, and TypeScript checks.
- Exact `npm run build`: PASS — produced `target/release/mount-identity-audit` and `dist/site/`.
- `cargo package --allow-dirty --no-verify`: PASS — 19 files, 158.5 KiB unpacked and 39.0 KiB compressed. The claim suite also exercises the packed artifact from a separate temporary consumer source tree.
- A new temporary consumer prefix installed the CLI, displayed useful `--help`, reported `mount-identity-audit 0.1.0`, and ran `--demo`. The demo copied its bundled sample to a unique temporary directory, reported the expected `FAIL`, and returned exit code 1.

The checked CLI behavior includes normal ownership checks, group/mode branches, explicit read-only declarations, numeric and unproven identities, JSON/share-safe reports, JSONC and selected Compose metadata, stable exit codes, and separate Docker and rootless Podman mapping rules. Docker and Podman programs are not installed in this worker; their read-only command contracts are covered with deterministic adapters in the integration and claim checks.

## Live deployment, browser quality, and accessibility

- `/`, `/demo/`, `/privacy/`, and `/terms/` return 200. An unknown route returns the designed 404 response with status 404.
- The factory URL verifier passed against the live URL in 756 ms: title, `lang=en`, one H1, main landmark, image text alternatives, and labelled controls are present; browser errors are empty.
- `PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in npx playwright test`: PASS. This checks desktop and 390×844 mobile layouts, keyboard-only primary use, visible skip-link focus, reduced-motion behavior, 200% text reflow, no horizontal overflow, demo reset and exit, route metadata, and offline reload.
- Playwright Axe scans on home, demo, Privacy, Terms, and 404 at both configured viewport projects found zero serious or critical findings.
- At 390 px, every visible link and button on all five public routes measured at least 44 px in both dimensions.
- A fresh service-worker context updated to cache `mia-site-v6`, then reloaded `/demo/` offline with `FAIL` state and a controlling worker. No browser errors occurred.

## Privacy, headers, caching, and build identity

The full live browser sample flow made only same-origin GET requests for the document, local JavaScript, and local stylesheet. Entering the unique value `3141592`, running the preflight, and resetting the demo made no extra request and placed no entered value in a request. Cookies were empty; localStorage, sessionStorage, and IndexedDB each remained empty. No analytics, account system, paid feature, or product server endpoint is present, so request allowance, persistence-concurrency, and sign-in checks do not apply.

Live HTML responses include a self-only CSP (including `frame-ancestors 'none'`), HSTS preload, `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and a restrictive Permissions Policy. HTML and `sw.js` use 30-second revalidation; hashed JavaScript uses `max-age=31536000, immutable`.

All 15 checked deployable files were byte-identical between `dist/site/` and the live site: four HTML routes plus 404, JS, CSS, icons, terminal illustration, hero and social images, robots, sitemap, and service worker.

## Performance

Production build output is 6,085 bytes raw / 2.38 KiB gzip JavaScript and 14,825 bytes raw / 3.92 KiB gzip CSS. The 216,498-byte WebP hero remains below the 300 KB mobile-image budget. The static product has no hosted fonts.

Fresh Lighthouse 12.8.2 mobile JSON for the live home page reported:

| Metric | Result |
| --- | ---: |
| Performance | 98 |
| Accessibility | 100 |
| Best practices | 100 |
| SEO | 100 |
| FCP | 1.2 s |
| LCP | 2.0 s |
| Total blocking time | 120 ms |
| CLS | 0 |
| Total transfer | 224 KiB |

Chromium emitted a tab-close warning after Lighthouse had written its result JSON; the recorded scores and audit data above are present in that report.

## Defects by severity

None found.

