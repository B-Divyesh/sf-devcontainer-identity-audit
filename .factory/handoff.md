# Mount Identity Audit — repair 6 handoff

## Status: repaired, pushed, and deployed

The release blockers in independent verification commit
`bca96e39f78a3ae8aabf38a893013b69d3bfae92` were reproduced and repaired.
The product revision is `04a1e63`; it is pushed to `origin/main` and deployed at
<https://devcontainer-identity-audit.sociobot.in>.

Deployment `2f2bf146-f331-45f1-970c-667287455175` targeted only the existing
`sf-devcontainer-identity-audit` Static Web App. The shared factory deployment
wrapper was intentionally not used because it also accesses shared DNS, which
this work order forbids. No database, key vault, shared service, app settings,
DNS, or unrelated Azure resource was read or changed.

## What changed

### Identity precedence and exact regressions

The verifier case was reproduced before edits:

- `containerUser 424242:424242` plus Compose `user 0:0` returned exit `0`,
  `pass`, identity `0:0`, source `Compose service app user`;
- `containerUser 0:0` plus Compose `user 424242:424242` returned exit `1`,
  `fail`, identity `424242:424242`, with the same source.

The controller's newer explicit rule supersedes the verifier report's proposed
ordering: a selected Compose service `user` overrides `containerUser`, while an
explicit `remoteUser` overrides the Compose service user. The configuration
merge now names and documents that ordering directly.

Coverage exists at four levels:

- two Rust configuration unit cases for both `containerUser` conflicts;
- two CLI integration cases for both conflicts, alongside both
  existing `remoteUser` conflict directions;
- registered claim `compose-user-precedence`;
- a consumer regression that creates the `.crate`, extracts it, installs it to
  a fresh prefix, and checks all four root/non-root conflicts using that packed
  binary.

README identity documentation now states the same ordering.

### Claims from a clean clone

The missing build ordering was reproduced after `npm ci` with `dist/site`
absent: the exact `@claim:cli-demo` command timed out after 30 seconds waiting
for preview. `test:claims` now builds both the release CLI and `dist/site`
before Playwright starts.

All 15 exact commands in `.factory/claims.json` were then invoked individually;
15 passed and 0 failed. A separate fresh local clone began with no `dist`,
`node_modules`, or `target`; `npm ci` followed by the exact `@claim:cli-demo`
command built both artifacts and passed without any prior command.

### Cold first viewports

The three required price, offline, and privacy facts now precede the install
strip. The hero type and spacing were tightened without changing the recorded
dithered identity-ledger visual system. A browser regression measures every
fact against the cold viewport in desktop Chromium and at 390×844.

Live geometry after deployment:

| Viewport | Headline | Action | Fact bottoms | Viewport bottom |
| --- | ---: | ---: | ---: | ---: |
| 1440×900 | y 148–430 | y 538–644 | 676, 698, 721 | 900 |
| 390×844 | y 134–344 | y 477–575 | 603, 643, 663 | 844 |

The service-worker cache generation moved to `mia-site-v5`, so installed users
receive the repaired shell and old public caches are removed on activation.
Footer build identity is `v0.1.0 · repair-6` on every route.

## Verification evidence

### Clean install, tests, lint, build, and package

- Fresh clone `npm ci`: pass, 61 packages, 0 vulnerabilities.
- Fresh clone exact claim command with `dist` absent: pass.
- `npm test`: pass — 10 Rust unit tests, 21 Rust CLI integration tests,
  21 Vitest tests, and 57 Playwright tests; 5 intended project-specific skips.
- `npm run lint`: pass — rustfmt, strict Clippy, and TypeScript.
- `npm audit --audit-level=low`: pass, 0 vulnerabilities.
- `npm run build`: pass; produced `target/release/mount-identity-audit` and
  `dist/site`.
- `cargo package --allow-dirty`: pass; Cargo verification build passed;
  19 files, 157.9 KiB unpacked and 38.7 KiB compressed.
- Packed consumer install: pass from an extracted `.crate` into a fresh prefix.
- Production bundles: JavaScript 5.26 kB raw / 2.22 kB gzip; CSS 14.68 kB raw /
  3.89 kB gzip.

The full clean-clone sequence was:

```sh
npm ci
npm run test:claims -- --grep @claim:cli-demo
npm test
npm run lint
npm run build
```

Run every registered claim independently with its exact command from
`.factory/claims.json`, or run their combined sandbox with:

```sh
npm run test:claims
```

### Browser, accessibility, privacy, offline, and response policy

The local and live browser suites both passed in desktop Chromium and 390×844
mobile Chromium. They cover:

- one-click demo, reset, recovery, and browser/CLI permission parity;
- cold first-viewport facts, mobile reflow, 200% text, and no horizontal
  overflow;
- keyboard-only primary flow, skip link, focus order, reduced motion, and
  44×44 px mobile targets;
- axe scans of home, demo, privacy, terms, and 404 with 0 serious or critical
  findings in both projects;
- validation errors and live-region result announcements;
- request capture, cookies, local/session storage, and IndexedDB: user values
  remain local and storage stays empty;
- dedicated-context offline reload, service-worker control, and cache update;
- route metadata, 404 behavior, CSP, HSTS, Permissions Policy,
  Referrer-Policy, `nosniff`, and immutable hashed-asset policy.

Factory `verify-url.sh` passed locally in 631 ms and live in 914 ms. The live
page had no console errors and reported a title, `lang=en`, one h1, a main
landmark, no missing image alt text, and no unlabeled buttons.

Lighthouse 12.8.2 mobile results:

| Target | Performance | Accessibility | Best practices | SEO | FCP | LCP | TBT | CLS | Transfer |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Local | 99 | 100 | 100 | 100 | 1.0 s | 2.1 s | 10 ms | 0 | 224 KiB |
| Live | 99 | 100 | 100 | 100 | 1.0 s | 2.0 s | 0 ms | 0 | 224 KiB |

In both runs Lighthouse wrote a complete report before Chromium emitted the
same post-collection tab-crash warning recorded by the independent verifier.
The figures above come from those complete JSON reports.

### Live deployment identity

- `/`, `/demo/`, `/privacy/`, and `/terms/`: HTTP 200.
- Unknown route: HTTP 404 with the designed page.
- All 17 public files match `dist/site` byte-for-byte.
- `staticwebapp.config.json`: HTTP 404 as deployment configuration, expected.
- Live HTML contains `repair-6` and the offline first-screen fact.
- Live service worker contains `mia-site-v5`.
- Live headers include the repository CSP, one-year preload HSTS,
  `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and the
  camera/microphone/geolocation Permissions Policy.
- Full live Playwright run: 57 passed, 5 intended skips.

## Known limits and next steps

Docker and Podman executables are not installed in this worker. Runtime process
calls remain covered by deterministic recording adapters, including rootless
UID/GID maps, keep-id, host mode, and the three-call ceiling. No registry
package was published; the verified `.crate` is ready for the factory's normal
publishing process. No product release blocker remains.
