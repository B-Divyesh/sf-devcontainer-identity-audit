# Independent verification 7 — Mount Identity Audit

## Verdict: FAIL

Candidate `b319eb51e96e2b7b4c72ebd14eebc353cc1a8100` was checked from a
fresh clone on 1 September 2026 UTC. The live product checked was
<https://devcontainer-identity-audit.sociobot.in>.

The candidate is not ready for release. The editable browser sample can report
`PASS` for a rootless Podman `keep-id` case that the packed CLI correctly
reports as `FAIL`. Two legal-page links also miss the required mobile touch
target height. A separate recovery action does not replace invalid sample
values as its label promises.

All 15 registered claim commands, the full repository suite, lint, build,
package, clean-consumer install, live deployment comparison, privacy checks,
offline reload, axe scans, and performance budgets otherwise passed.

## Release-blocking findings

### P1 — browser `keep-id` logic can give a false PASS

The registered `browser-parity` claim says the browser sample follows the CLI's
owner, group, mode, and rootless mapping rules. The registered test passes only
the default case where every identity is `1000:1000`. A representative
non-matching remote identity disproves the broader promise.

Input checked in both products:

| Input | Value |
| --- | --- |
| Workspace owner | `1000:1000` |
| Directory mode | `0755` |
| Remote user | `2000:2000` |
| Runtime | rootless Podman |
| User namespace | `keep-id` |

Live browser result:

```text
verdict: pass
mapped identity: 1000:1000 · keep-id mapping
title: Workspace is writable
```

Packed CLI result using a deterministic live-map adapter:

```text
exit: 1
verdict: fail
mapped identity: 102000:102000
summary: The mapped remote identity can read but cannot write the workspace.
```

The adapter supplied a normal keep-id shape: container `1000` maps to host
`1000`, while container `2000` maps through the subordinate range. The CLI
correctly reads that map because the requested remote identity is not the kept
identity. The browser instead assigns the workspace owner's UID and GID to
every remote identity whenever `keep-id` is selected.

This is a false safe result in the public calculator and contradicts registered
claim `browser-parity`. The browser needs the host caller identity and relevant
map, or it must return an unknown result when the supplied fields cannot prove
the mapping. Add claim coverage where workspace owner, host caller, and remote
identity differ.

### P2 — two mobile links have 19 px touch-target heights

At a 390×844 viewport, the main-content links below were measured from their
rendered bounding boxes:

| Route | Link | Measured box | Required minimum |
| --- | --- | ---: | ---: |
| `/privacy/` | `public repository` | `173.44 × 19 px` | `44 × 44 px` |
| `/terms/` | `MIT License` | `112.22 × 19 px` | `44 × 44 px` |

The links remained unclipped at the repository's 200% root-text check, but
their hit areas remained 19 px high. This misses the attached accessibility
and site-structure requirement that touch targets be at least 44 px. Axe does
not report target size, and the repository's target-size test checks only the
home page. Extend that check to every public route.

The 22 px read-only checkbox is not included in this finding because its bound
label provides a clickable 44 px row.

### P2 — “Load safe example” does not replace invalid form values

On `/demo/`, entering directory mode `0899` and selecting **Run preflight**
correctly announces:

```text
Directory mode must be three or four octal digits, such as 0755.
```

Selecting **Load safe example** then leaves the mode at `0899` and repeats the
same error. The action changes only the runtime namespace and read-only flag;
it does not load a complete safe set of values. **Reset demo** does restore
`0755`, after which **Load safe example** reaches `PASS`, so recovery is
available but the named action is incomplete. Make the action load every
sample field or rename it to describe the narrower change, and add a regression
starting from invalid edited input.

## Required first-read check

A cold desktop visit returned HTTP 200 without console or page errors.

- What it does: “Check mount permissions before container startup.”
- Who it is for: developers using Dev Containers or rootless Podman who need a
  writable workspace on first open.
- What to select first: “Try it with sample data.” The adjacent sentence says
  it runs a known rootless Podman mismatch.
- One-click sample: PASS. One selection opened `/demo/`, immediately displayed
  the populated mismatch and `FAIL`, and showed “Demo — sample data, nothing is
  saved” with reset and exit actions.
- The price, offline, and privacy facts were all visible in the initial
  1440×900 and 390×844 viewports.

## Registered claims

`.factory/claims.json` is present with 15 entries. An initial pre-install
invocation could not locate the repository's local `vite` command. After the
required locked install (`npm ci`), every exact command from the claims file
was run independently from the candidate checkout:

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
| `browser-parity` | PASS for its registered default fixture; the independent counterexample above disproves the full claim |
| `mit-license` | PASS |

The manifest commands now build both release artifacts before Playwright and
work from an installed clean clone. Passing the narrow parity fixture does not
remove the P1 false result.

## Clean clone, tests, lint, build, and package

The independent clone began at exactly
`b319eb51e96e2b7b4c72ebd14eebc353cc1a8100` with no changes.

- `npm ci`: PASS — 61 packages installed; 0 audit vulnerabilities.
- `npm test`: PASS.
  - 10 Rust unit tests passed.
  - 21 Rust CLI integration tests passed.
  - TypeScript checking passed.
  - 21 Vitest tests passed.
  - 57 Playwright tests passed across desktop Chromium and 390×844 mobile;
    5 project-configured duplicate/mobile-only cases were skipped.
- `npm run lint`: PASS — rustfmt, strict Clippy, and TypeScript.
- `npm audit --audit-level=low`: PASS — 0 vulnerabilities.
- Exact `npm run build`: PASS — produced
  `target/release/mount-identity-audit` and `dist/site/`.
- `cargo package`: PASS — 19 files, 157.9 KiB unpacked and 38.7 KiB
  compressed; Cargo's package verification build passed.
- Clean-consumer install: PASS. The packaged source installed into a new
  prefix, reported version `0.1.0`, showed useful help, and ran the bundled
  demo from a unique temporary directory with expected exit `1`.

Independent packed-CLI checks confirmed:

- owner, group, and other permission branches;
- largest usable Linux UID/GID `4294967294`;
- reserved ID `4294967295` returns exit `2`/`unknown`;
- missing and invalid configuration return actionable `unknown` reports;
- a directory without traverse permission returns exit `1`/`fail`;
- an explicit read-only bind returns exit `1`/`fail`;
- a named identity recovers to `pass` with a numeric `--remote-user` override;
- `--no-runtime` with automatic runtime selection returns an actionable
  `unknown` result;
- share output removes the temporary project path.

Docker and Podman executables are not installed in the worker. Runtime process
behavior was checked with deterministic read-only adapters, including rootless
UID/GID maps and the three-call ceiling. No product code or project data was
changed.

## Live deployment identity and routing

The deployed product matches the candidate build byte-for-byte for all 17
servable files: every HTML route, the designed 404 response, JS, CSS, art,
icons, sitemap, robots file, `_headers`, and service worker. The deployment
configuration file correctly returns 404 rather than being served.

- `/`, `/demo/`, `/privacy/`, and `/terms/`: HTTP 200.
- Unknown path: HTTP 404 with the candidate's designed 404 page.
- Every internal navigation link returns HTTP 200; same-page skip links work.
- External links point only to the declared source repository, its license,
  and the Param Factory home page.
- Every route has a specific title under 60 characters, `lang=en`, one `<h1>`,
  one main landmark, ordered headings, canonical and social metadata, and
  complete image alt text.
- Factory `verify-url.sh`: PASS locally in 549 ms and live in 586 ms, with no
  console errors, missing alt text, or unlabeled buttons.
- Full Playwright suite against the live URL: 57 passed, 5 configured skips.

## Accessibility, keyboard, mobile, and visual checks

- Axe scans of home, demo, privacy, terms, and 404 in desktop and mobile
  projects: 0 serious or critical findings.
- Keyboard-only flow reaches the skip link, main content, calculator inputs,
  and both result actions without a trap.
- Focus treatment is a 3 px green outline. Its contrast is 6.15:1 against the
  paper background and 7.16:1 against the white sheet.
- Reduced motion limits animations and transitions to `0.01 ms`; no loop or
  flashing behavior was observed.
- The invalid mode error is announced as an alert. **Reset demo** restores
  valid values, and the unchanged default sample can then reach the safe
  keep-id result.
- At 390 px, all routes have zero horizontal overflow. Home, demo, and 404
  controls meet the touch-size rule; the two legal-page exceptions are listed
  above.
- Privacy and Terms remain unclipped under the repository's 200% root-text
  check.
- Desktop and mobile screenshots confirm the product-specific dithered
  identity-ledger treatment, hierarchy, single light theme, and responsive
  table layout described in `.factory/design.md`.

## Privacy, requests, headers, caching, and offline

The complete live browser flow made only same-origin GET requests for product
files. Entering the unique value `3141592`, running the check, and using sample
actions made no additional request and placed no entered value in a URL or
request body. Cookies, localStorage, sessionStorage, and IndexedDB remained
empty. No console or page error occurred.

Source and dependency inspection found no application API, product-unlock
call, analytics, telemetry, account system, third-party script, or CLI network
client. The site is static and has no server-side product endpoint, sign-in, or
paid feature. Request allowances, 429/`Retry-After`, persistence concurrency,
and Entra authority checks are therefore not applicable.

Live responses include the repository CSP, one-year preload HSTS,
`Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and a policy
that disables camera, microphone, and geolocation. Hashed JS/CSS use one-year
immutable caching. HTML, fixed-name assets, and `sw.js` use
`must-revalidate, max-age=30`.

The active service worker updated to cache `mia-site-v5`. A dedicated fresh
context reloaded `/demo/` offline with its populated mismatch, `FAIL` state,
and controlling service worker intact. No errors were logged.

## Performance and budgets

Fresh Lighthouse 12.8.2 mobile results for the live home page:

| Metric | Result |
| --- | ---: |
| Performance | 99 |
| Accessibility | 100 |
| Best Practices | 100 |
| SEO | 100 |
| FCP | 1.1 s |
| LCP | 2.0 s |
| Total blocking time | 110 ms |
| CLS | 0 |
| Total transfer | 224 KiB |

Production assets remain below their supplied budgets:

- JavaScript: 5.26 kB raw / 2.22 kB gzip, under 200 kB.
- CSS: 14.68 kB raw / 3.89 kB gzip, under 50 kB.
- Hero WebP: 216,498 bytes, under 300 kB.
- Fonts: 0 bytes; only local system stacks are used.

Lighthouse does not produce field INP in this lab run. Calculator updates are
synchronous, and no long interaction task was observed.

## Required next steps

1. Make the browser rootless mapping model agree with the CLI when the remote
   identity differs from the kept host identity; return unknown when evidence
   is insufficient.
2. Add the non-matching keep-id case to `browser-parity` claim coverage.
3. Give the Privacy and Terms content links at least 44 px touch-target height,
   and test target sizes on every route.
4. Make **Load safe example** replace all sample values, including invalid
   edits, or rename the action and provide a complete recovery action.
5. Repeat every claim command and the full clean-clone and live checks.
