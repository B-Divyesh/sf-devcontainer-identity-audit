# Mount Identity Audit — repair 5 handoff

## Status: PASS — release blockers repaired

This repair addresses every release-blocking finding in independent verifier
report commit `396cf01c80eb755c3fc354167f59448ee4ba866c` for candidate
`d82877ddec3ba114f820fee6870cbc5d96587be1`.

Repair commits:

- `a3828e2` — product, demo, claim, identity-boundary, metadata, accessibility,
  privacy, offline, package, and regression repairs.
- `87b8f56` — remove the SPA fallback so Azure Static Web Apps can apply the
  real 404 response override.

Both commits are on `origin/main`. The deployed product source is `87b8f56`.

## Findings reproduced and repaired

### Claims gate

Reproduced: `.factory/claims.json` did not exist, while the site and README made
testable claims.

Repaired:

- Added `.factory/claims.json` with 14 claims.
- Added exactly one `@claim:<id>` Playwright test for each claim.
- Added a Vitest contract check that rejects missing, duplicate, or untested
  claim IDs.
- `npm run test:claims` passes all 14 claim cases.

### One-click and CLI demos

Reproduced: the first screen lacked a named audience and one-click sample; the
CLI had no demo; `/demo` was the home fallback; demo documentation was absent.

Repaired:

- The first screen now names Dev Container users and presents “Try it with
  sample data” beside the real CLI install step and three plain facts.
- `/demo/` opens a working rootless-Podman mismatch immediately.
- The demo displays “Demo — sample data, nothing is saved” with **Reset demo**
  and **Start for real** controls. It reads or writes no browser storage.
- `mount-identity-audit --demo` copies the bundled project to a unique OS
  temporary directory, audits it, and prints the path.
- Added `examples/mismatch/` and `.factory/demo.md`.
- Added an accessible, hand-authored terminal recording based on the real CLI
  output. Asset provenance is recorded in `.factory/design.md`.

### Reserved Linux identity

Reproduced: UID/GID `4294967295` produced a false `PASS` in direct Docker mode
in both the CLI and browser calculator.

Repaired at the numeric identity boundary:

- Rust rejects the reserved value while parsing remote and image identities.
- Runtime UID/GID map boundaries and checked additions also reject it.
- The browser parser and mapped-ID calculation reject it with an actionable
  error announced through the form live region.
- Exact CLI regressions cover Docker direct IDs and Podman host-userns IDs.
- Unit and browser regressions cover each direct field and a rootless mapping
  that reaches the sentinel. Unproven identities now return `UNKNOWN`/input
  error, never a safe verdict.

### Routing, metadata, and site shell

Reproduced: unknown routes returned the home page with HTTP 200; canonical,
social-card, Twitter, and Apple-touch metadata were missing; legal pages lacked
the standard shell; the footer lacked a build ID.

Repaired:

- Added a styled `404.html` and Azure `responseOverrides` configuration.
- Removed `navigationFallback`, which had intercepted unknown routes before the
  404 override. The live unknown-route check now returns HTTP 404.
- Added route-specific titles, canonical links, Open Graph/Twitter metadata,
  a 1200×630 social card, and a 180×180 Apple-touch icon to every route.
- `/`, `/demo/`, `/privacy/`, `/terms/`, and `/404.html` now share the
  header, navigation, footer, and `v0.1.0 · repair-5` build label.
- Updated sitemap, Vite inputs, security headers, and cache policy. The fixed
  hero image no longer receives immutable caching.
- The social and touch assets are deterministic crops of the original product
  artwork; provenance is in `.factory/design.md`.

### Copy audit

Reproduced: `.factory/copy-audit.md` was absent.

Repaired: added every landing-page sentence with word counts, checked the
22-word maximum and banned words, and documented the terminology table. There
are no remaining flags.

## Verification evidence

All commands were run in `/work/repo` on 2026-08-30 UTC.

### Clean install, tests, lint, and build

- `npm ci` — passed; 61 packages installed, 0 vulnerabilities.
- `npm test` — passed:
  - 8 Rust unit tests;
  - 19 Rust CLI integration tests;
  - TypeScript typecheck;
  - 21 Vitest tests;
  - production site build;
  - 54 applicable Playwright tests across desktop Chromium and a 390×844
    mobile viewport. Four desktop executions of mobile-only assertions were
    intentionally skipped.
- `npm run test:claims` — passed all 14 registered claims from the documented
  demo entry points.
- `npm run lint` — `cargo fmt --check`, strict Clippy, and TypeScript passed.
- `npm audit --audit-level=low` — 0 vulnerabilities.
- `npm run build` — release CLI and `dist/site/` produced successfully.
- Final site assets: JavaScript 5.26 kB raw / 2.22 kB gzip; CSS 14.43 kB raw /
  3.82 kB gzip.

### Package and clean consumer

- `cargo package --locked` — passed; 19 files, 154.9 KiB unpacked and 38.3 KiB
  compressed (`target/package/mount-identity-audit-0.1.0.crate`).
- Installed the packed crate into a clean `target/qa-install` prefix.
- The installed `--demo --quiet` command audited the bundled sample and
  returned the expected mismatch exit code `1`.
- A separate consumer crate under `target/qa-consumer` used only the public API
  and asserted schema version 1, `Verdict::Pass`, exit code 0, and share-path
  redaction. It printed `public API PASS`.

### Browser, keyboard, accessibility, privacy, and offline

The same 54 applicable cases passed against both the local production build and
`https://devcontainer-identity-audit.sociobot.in`:

- desktop Chromium and 390×844 mobile behavior;
- no horizontal clipping, including at 200% text size;
- 44×44 px mobile link/button targets;
- complete keyboard-only primary flow and visible skip-link focus;
- reduced-motion behavior;
- one `<h1>`, landmarks, labels, live error announcements, and route metadata;
- axe scan of home, demo, privacy, terms, and 404 with 0 serious or critical
  violations;
- no console errors;
- same-origin requests only, no cookies, and empty localStorage,
  sessionStorage, and IndexedDB after entering project values;
- isolated demo reset/exit behavior;
- service-worker update, offline banner, offline reload, cached assets, and
  recovery to the legal routes.

The factory URL verifier recorded `loadMs: 794`, no console errors, title and
`lang=en`, one `<h1>`, a `<main>`, no images missing alt text, and no
unlabeled buttons. Evidence is in `/tmp/mia-final-verify` in the repair worker.

### Performance

Lighthouse 12.8.2 against the final live deployment, using the supplied
Playwright Chromium:

- Performance: 99
- Accessibility: 100
- Best Practices: 100
- SEO: 100
- FCP: 1.0 s
- LCP: 2.0 s
- Total blocking time: 30 ms
- CLS: 0
- Speed Index: 1.0 s
- Total transfer: 224 KiB

### Deployment and response policy

Deployed with:

```sh
/opt/fleet/lib/deploy-static.sh devcontainer-identity-audit dist/site
```

- Azure Static Web Apps deployment ID:
  `d6e115b8-5a4f-45ba-9e01-3a4c5ffaf578`
- Custom domain: `https://devcontainer-identity-audit.sociobot.in`
- `/`, `/demo/`, `/privacy/`, and `/terms/`: HTTP 200.
- `/does-not-exist`: HTTP 404 with the product 404 page.
- HTTP redirects to HTTPS.
- Live response headers include CSP with `frame-ancestors 'none'`, HSTS,
  `Permissions-Policy`, `Referrer-Policy: no-referrer`, and
  `X-Content-Type-Options: nosniff`.
- The static product has no server API, login, paid feature, third-party script,
  analytics, or identity provider. Server authentication, rate limiting, and
  provider identity checks are therefore not applicable.

## Known environment limitation

Docker and Podman executables were unavailable in the worker. Runtime adapter
behavior was exercised with deterministic executable fixtures that record the
exact read-only CLI calls and return representative Docker/Podman identity-map
responses. No product capability was removed or weakened.

## Reverify

```sh
npm ci
npm test
npm run test:claims
npm run lint
npm audit --audit-level=low
npm run build
cargo package --locked
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in npm run test:e2e
curl -i https://devcontainer-identity-audit.sociobot.in/does-not-exist
```
