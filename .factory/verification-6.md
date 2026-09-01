# Independent verification 6 — Mount Identity Audit

## Verdict: FAIL

Candidate `eb570fd4b2183cde1c4e5d13432583e9d3f47fd7` was tested from a
clean `main` checkout on 1 September 2026 UTC. The live deployment tested was
`https://devcontainer-identity-audit.sociobot.in`.

The candidate is not releasable. Its main CLI can return `PASS` for a workspace
that the explicit Dev Container `containerUser` cannot write, and every exact
claim command in `.factory/claims.json` fails after a clean install unless the
site is built separately first. Both are release-blocking findings under the
work order. The live first-read experience, full repository suite, build,
package, accessibility, privacy, offline, and performance checks otherwise
passed.

## Release-blocking findings

### P1 — `containerUser` is replaced by Compose `user`, producing a false PASS

The configuration reader initially treats `containerUser` as the intended
identity, but the Compose merge protects only `remoteUser`. A selected Compose
service `user` therefore overwrites an explicit `containerUser`.

This valid representative project was tested with the installed packed CLI:

```jsonc
{
  "dockerComposeFile": "compose.yml",
  "service": "app",
  "workspaceFolder": "/work",
  "containerUser": "424242:424242"
}
```

```yaml
services:
  app:
    image: local/example:latest
    user: "0:0"
    volumes:
      - ../:/work
```

The host workspace was owned by `0:0` with mode `0755`. The expected identity
is the explicit Dev Container `containerUser` (`424242:424242`), which can read
but cannot write. The candidate instead reported:

```text
exit: 0
verdict: pass
identity: 0:0
source: Compose service app user
```

The reverse case also failed: `containerUser: "0:0"` with Compose
`user: "424242:424242"` returned exit `1`/`fail` instead of `pass`.

This defeats the core job-to-be-done because the preflight can approve a mount
for the wrong process identity. Preserve both explicit `remoteUser` and
explicit `containerUser` over the Compose service fallback, and add both
conflict directions as packed-CLI regressions and a registered claim case.

### P1 — every registered claim command fails from the clean installed clone

`.factory/claims.json` exists and contains 14 claims. As required, every exact
`test` command was run individually. After `npm ci`, all 14 commands failed
before their assertion because Playwright's configured preview server timed
out after 30 seconds:

```text
Error: Timed out waiting 30000ms from config.webServer.
```

The shared cause is deterministic. Each entry calls:

```text
npm run test:claims -- --grep @claim:<id>
```

`test:claims` builds only the Rust CLI and then starts Playwright. The configured
server runs `vite preview`, which serves only an existing `dist/site`, but a
clean installed clone has not run `npm run build:site`. The mandatory
pre-install attempt also could not load the absent local Playwright package;
`npm ci` resolved that expected setup condition but exposed the independent
missing-site-build failure above.

Claim outcomes after `npm ci`, before any site build:

| Claim ID | Exact registered command |
| --- | --- |
| `cli-demo` | FAIL — preview readiness timeout |
| `browser-demo` | FAIL — preview readiness timeout |
| `permission-verdicts` | FAIL — preview readiness timeout |
| `read-only-safety` | FAIL — preview readiness timeout |
| `config-support` | FAIL — preview readiness timeout |
| `share-redaction` | FAIL — preview readiness timeout |
| `report-contract` | FAIL — preview readiness timeout |
| `runtime-mapping` | FAIL — preview readiness timeout |
| `conservative-identities` | FAIL — preview readiness timeout |
| `browser-private` | FAIL — preview readiness timeout |
| `cli-private` | FAIL — preview readiness timeout |
| `offline-reload` | FAIL — preview readiness timeout |
| `browser-parity` | FAIL — preview readiness timeout |
| `mit-license` | FAIL — preview readiness timeout |

After an explicit `npm run build`, `npm run test:claims` passed all 14
assertions in 12.5 seconds. This proves the assertions themselves work, but it
does not satisfy the contract that each listed command works from a clean
installed clone. Make `test:claims` build the production site before starting
Playwright, then re-run each manifest command from a clone containing no
`dist/`.

### P2 — the three first-screen facts are below the initial viewport

The job, audience, and sample action are visible without scrolling, so the
explicit first-read acceptance gate passes. However, the supplied plain-words
contract also requires three short privacy/offline/price facts on the first
screen. The facts begin at `y=977` in a 1440×900 viewport and `y=923` in a
390×844 viewport, so none is visible initially. Keep the headline, audience,
sample action, and three facts within the initial desktop and mobile viewport.

## Mandatory first-read test

Cold navigation to the live home page returned HTTP 200 with no console or page
errors in the normal flow.

- What it does: “Check mount permissions before container startup.”
- Who it is for: developers using Dev Containers or rootless Podman who need a
  writable workspace on first open.
- What to select first: “Try it with sample data,” with the adjacent explanation
  “Runs a known rootless Podman mismatch.”
- One-click sample: PASS. One selection opened `/demo/`, immediately showed the
  populated rootless Podman mismatch and `FAIL`, and displayed the persistent
  “Demo — sample data, nothing is saved” banner with reset and exit controls.

## Clean checkout, tests, lint, build, and package

The checkout began clean at the exact candidate commit.

- `npm ci`: PASS — 61 packages installed; 0 audit vulnerabilities.
- `npm test`: PASS.
  - 8 Rust unit tests passed.
  - 19 Rust CLI integration tests passed.
  - TypeScript checking passed.
  - 21 Vitest tests passed.
  - The production site build passed.
  - 54 applicable Playwright tests passed across desktop Chromium and 390×844
    mobile Chromium; four desktop instances of mobile-only tests were skipped.
- `npm run lint`: PASS — Rust formatting, strict Clippy, and TypeScript.
- `npm audit --audit-level=low`: PASS — 0 vulnerabilities.
- Exact `npm run build`: PASS — produced `target/release/mount-identity-audit`
  and `dist/site/`.
- Production bundles: JavaScript 5.26 kB raw / 2.22 kB gzip; CSS 14.43 kB raw /
  3.82 kB gzip. Both are far below the 200 kB JS and 50 kB CSS budgets.
- `cargo package --allow-dirty`: PASS — 19 files, 154.9 KiB unpacked / 38.3 KiB
  compressed; Cargo's verification build passed.
- The packed source was installed into a clean temporary prefix. The installed
  `mount-identity-audit 0.1.0` exposed useful `--help`, and the bundled demo
  returned the expected exit `1` mismatch from a unique `/tmp` copy.

Installed-package functional cases:

| Case | Expected and observed result |
| --- | --- |
| owner `0:0`, Docker/no-runtime | exit 0, versioned JSON `pass` |
| non-owner `424242:424242`, mode `0755` | exit 1, concise `FAIL` |
| reserved `4294967295:4294967295` | exit 2, JSON `unknown` with recovery text |
| invalid identity `not-an-id` | exit 2, JSON `unknown` with recovery text |
| safe recovery using `--remote-user 0:0` | exit 0, JSON `pass` |
| `--share` | paths replaced by neutral labels |
| `containerUser` conflicting with Compose `user` | **incorrect results; P1 above** |

Docker and Podman executables were not installed in the worker. Real-process
runtime calls were therefore covered by the repository's deterministic
recording adapters, including the three-call ceiling and live UID/GID-map
shapes. No external runtime or project was changed.

## Live deployment identity and routing

The deployed site matches the candidate production output. Seventeen servable
files, including every HTML route, hashed JS/CSS, images, icons, sitemap,
robots file, and `sw.js`, matched `dist/site` byte-for-byte. The only built file
not downloadable was `staticwebapp.config.json`, correctly returning 404 as
deployment configuration rather than public content.

- `/`, `/demo/`, `/privacy/`, and `/terms/`: HTTP 200.
- An unknown route: HTTP 404 with the designed product 404 page.
- Every tested route has its own title, `lang=en`, exactly one `<h1>`, one
  `<main>`, a standard header/footer, canonical and social metadata, and alt
  text.
- Factory `verify-url.sh`: PASS — 881 ms observed load, no console errors,
  correct title/language/landmarks, no missing alt text, and no unlabeled
  buttons.
- Full suite against the live URL: 54 passed, 4 expected mobile-only skips.

## Accessibility, keyboard, mobile, and recovery

- Axe on home, demo, privacy, terms, and 404 at desktop and 390 px: 0 serious or
  critical findings.
- Keyboard-only flow: the first Tab focused the skip link; Enter moved to main;
  subsequent Tabs reached “Run preflight”; Enter produced `FAIL`; the safe
  action produced `PASS`.
- Focus treatment: a visible 3 px solid green outline on the paper background.
- Invalid mode `0899` produced a focused/announced plain error. Correcting it
  and loading the safe case recovered to `PASS` and the keep-id mapping.
- Reduced motion produced effectively instant `0.00001 s` transitions and
  automatic scrolling.
- At 390×844 there was no horizontal overflow, including at 200% root text
  size. No visible link or button was smaller than 44×44 CSS px.
- The desktop and mobile visual treatment matches the recorded dithered
  identity-ledger thesis and retains its product-specific hierarchy.

## Privacy, requests, headers, caching, and offline

The complete live sample flow made only same-origin GET requests for public
site files. Running the calculation after entering a unique value made zero
additional requests and did not put the value into a URL or body. Cookies,
localStorage, sessionStorage, and IndexedDB remained empty. The only browser
cache was the documented public service-worker cache.

Source inspection found no application API, unlock call, analytics, telemetry,
account, third-party script, or network client. The site is static, has no
server-side product endpoint, sign-in, or paid feature. API request allowances,
429/`Retry-After`, and Entra authority checks are therefore not applicable.

Live responses include:

- CSP limited to self, with `object-src 'none'` and `frame-ancestors 'none'`;
- HSTS for one year with subdomains and preload;
- `Referrer-Policy: no-referrer`;
- `X-Content-Type-Options: nosniff`;
- camera, microphone, and geolocation disabled by Permissions Policy.

Hashed JS/CSS use `public, max-age=31536000, immutable` and Brotli. HTML,
fixed-name art, and `sw.js` use `must-revalidate, max-age=30`, allowing updates.
The active `/sw.js` registration updated successfully and reloaded `/demo/`
offline with its populated mismatch and controlling service worker intact.

## Performance

Lighthouse 12.8.2 mobile collection wrote a complete report before the
headless browser emitted a post-collection tab-crash warning:

- Performance 98; Accessibility 100; Best Practices 100; SEO 100.
- FCP 1.0 s; LCP 2.1 s; total blocking time 120 ms; CLS 0; Speed Index 1.4 s.
- Total first-load transfer 224 KiB, including the 216.5 kB hero WebP.

The tested LCP and CLS satisfy the supplied budgets. Lighthouse does not report
field INP; the sample's tested state updates were synchronous.

## Claim and copy cross-check

Every material behavior claimed on the site and in the README maps to one of
the 14 registered claims; no separate unlisted functional or privacy promise
was found. The registry-to-test tag contract test passes. The registered
commands nevertheless fail from the required clean state as described above.

## Required next steps

1. Preserve explicit `containerUser` when merging a selected Compose service,
   and add both conflict directions as packed-CLI and claim regressions.
2. Make every exact claim command build or otherwise serve the production site
   from a clean installed clone; verify with `dist/` absent.
3. Bring the three short facts into the initial desktop and 390 px viewport.
4. Repeat all 14 exact claim commands before the normal test/build gates.
