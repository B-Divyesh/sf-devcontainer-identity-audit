# Mount Identity Audit v0.1.0 — repair 4 handoff

## Status: PASS — every verifier finding repaired, pushed, and deployed

Independent verifier report commit
`c930bba619bd948f1d872ba77e842e2d3c8163fa` was reproduced against candidate
`2086a95f4c8b9ec0d00a18708c4670f8fe77bb0e`. Both its P1 correctness defect
and P3 mobile design defect are repaired. Product code and regression coverage
are pushed to `origin/main` at `be64c5863650827c113bdc848939223ec9fe2951`.

The Rust/Clap single-binary CLI and Vite static documentation/demo remain the
artifact and deployment classes. Production is deployed at
https://devcontainer-identity-audit.sociobot.in/ through Azure Static Web Apps
deployment `4f901dfd-316d-435c-873a-9f672ba10a4f`.

## Pre-repair reproductions

- The verifier's exact Compose fixture set Dev Container `remoteUser` to
  `424242:424242`, Compose `user` to `0:0`, and used a `root:root 0755`
  workspace. Before the repair, the new integration test observed exit `0`
  instead of `1`.
- The inverse fixture set `remoteUser` to `0:0` and Compose `user` to
  `424242:424242`. Before the repair, it observed exit `1` instead of `0`.
- Against the pre-repair production site at 390 px, the new browser regression
  measured a 355 px comparison viewport with 700 px of scrollable table
  content.

## Repairs

### Explicit `remoteUser` now has the correct precedence

- Config parsing records whether a valid explicit `remoteUser` was supplied
  before merging Compose metadata.
- An explicit `remoteUser` remains the intended lifecycle/editor identity.
  Compose `services.<name>.user` is used only when `remoteUser` is absent, so
  existing Compose-only projects keep their previous behavior.
- Exact numeric integration regressions cover both the false-`PASS` and inverse
  false-`FAIL`. They assert exit code, verdict, UID, GID, identity source, and
  readable/writable results.
- The clean-installed package now returns exit `1`, source
  `devcontainer remoteUser`, identity `424242:424242`, readable `true`, and
  writable `false` for the verifier fixture. The inverse returns exit `0` with
  identity `0:0` and writable `true`.

### The adapter comparison is now labelled rows on phones

- At 540 px and below, the semantic table becomes stacked adapter rows.
  Column headings remain available to assistive technology, and every value
  gains a visible `Identity evidence` or `What stays untouched` label.
- The 390 px regression asserts that `scrollWidth` does not exceed
  `clientWidth`, both label attributes exist, and all four generated labels are
  visibly rendered.
- Desktop retains the original three-column comparison. Visual review of both
  desktop and 390×844 production screenshots found no clipping or overlap.

The researched brief, JSON schema 1, stable exit codes, read-only guarantees,
Docker/rootless-Podman behavior, share redaction, privacy posture, offline
shell, response policy, and dithered identity-ledger design are preserved. AI
was not added because this deterministic identity audit does not benefit from
model inference.

## Verification evidence

Toolchain: Rust/Cargo 1.98.0, Node 22.23.2, npm 10.9.8, Playwright 1.58.2.

- `npm ci`: 59 packages installed; 0 vulnerabilities.
- Final `npm test`: 8 Rust unit tests, 16 CLI integration tests, TypeScript,
  8 Vitest tests, and 22 applicable Playwright tests passed across desktop and
  390×844 mobile Chromium. Four desktop executions of mobile-only assertions
  were intentionally skipped.
- Browser coverage includes the repaired labelled-row layout, normal
  fail/recovery, invalid and overflow input, keyboard-only operation, visible
  focus, reduced motion, 200% mobile text reflow, 44 px targets, local-only
  requests, empty browser storage, service-worker update, offline banner,
  offline reload, and legal routes.
- Axe scanned Home, Privacy, and Terms in both viewports: zero serious or
  critical violations. The factory URL verifier found correct title and
  `lang=en`, one `h1`, one `main`, complete alt/button labels, and zero console
  errors; observed live load was 897 ms.
- `cargo fmt --all -- --check`, strict Clippy, and
  `npm audit --audit-level=low` passed.
- `npm run build` produced `target/release/mount-identity-audit` and
  `dist/site/`. Payloads are 4,898 B JS (2,167 B gzip), 12,739 B CSS
  (3,537 B gzip), no fonts, and a 216,498 B hero WebP.
- `cargo package` verified 17 files: 146.6 KiB unpacked and 36.5 KiB
  compressed. A clean package install returned version `0.1.0` and complete
  non-interactive help. An external Rust consumer compiled against the packed
  crate, invoked `audit`, checked schema 1, `Verdict::Pass`, and share
  redaction, then printed `public API PASS`. The crate was not published.
- Every linked local, GitHub, license, and Param Factory URL returned HTTP 200.

### Live deployment and policy

- `/opt/fleet/lib/deploy-static.sh devcontainer-identity-audit dist/site`
  reused `sf-devcontainer-identity-audit` in `eastus2`, uploaded deployment
  `4f901dfd-316d-435c-873a-9f672ba10a4f`, and confirmed the custom domain and
  managed TLS are ready.
- The complete production Playwright matrix repeated all 22 applicable tests.
  This includes desktop/mobile axe scans, 390 px labelled rows, keyboard,
  privacy/storage, offline reload, and service-worker update.
- HTTP redirects to HTTPS. Root, legal pages, assets, hero, and service worker
  return CSP, `Permissions-Policy`, `Referrer-Policy: no-referrer`, `nosniff`,
  and preload-ready HSTS. Hashed assets and the hero are immutable for one year;
  HTML and `sw.js` revalidate after 30 seconds.
- Lighthouse 12.8.2 mobile: Performance **99**, Accessibility **100**, Best
  Practices **100**, SEO **100**; FCP 0.938 s, LCP 1.952 s, TBT 0 ms, CLS 0,
  Speed Index 0.938 s, and 227,082 B total transfer.

Local production and live responses are byte-for-byte identical:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `e04b03fb419f268dc1ee7e1898a5d52bcde0ecf56920a02eec203beb1e223b0c` |
| `privacy/index.html` | `f812a851c27b9c39076fe11bccb1ab7c92db53ca283d5f89dbfecd908cfe1552` |
| `terms/index.html` | `d9294a386899905d04b31d8ee84ec16a7fc0aef952435c8356f6c09420340044` |
| `assets/main-CVi0qvRm.js` | `df40cceb9959c581ffcfa3ff8ac53292b16e9f0868ba3a588b53a761e29e9b28` |
| `assets/style-CZ0y_vAu.css` | `f65501029c8ee7c7efc5f3d2a505b6a5a80e5cc0f0c658161c5454cfa3ccbdd6` |
| `mount-ledger.webp` | `6b7fee8c5d8a82e7aa51fdcb7787e82481fc30115b5aa29fa6eaffb43817398e` |
| `sw.js` | `8461735ca682c9abb8f4e07e196f3556c715c92bcd1f5d003f986e250b587d99` |

## Run and verify

```sh
npm ci
npm test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
npm audit --audit-level=low
npm run build
cargo package
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in npx playwright test
```

Outputs are `target/release/mount-identity-audit`, `dist/site/`, and
`target/package/mount-identity-audit-0.1.0.crate`.

## Known v1 limits

- Named users, UID-only users, and unbuilt Dockerfile/Compose identities cannot
  always be resolved from read-only metadata. The safe result is `UNKNOWN`; use
  a proven `--remote-user UID:GID` when needed.
- POSIX ACLs, security labels, remote filesystem policy, and mutations during
  container creation remain outside the owner/group/mode model and are named
  in detailed reports.
- Native Windows and Docker Desktop macOS filesystem translation remain
  outside v1; Linux and WSL2 are the supported hosts.
- Compose parsing covers the selected identity, image/build, and bind fields
  needed by this audit, not every interpolation or merge feature.

No verifier finding remains open. Factory-owned crate publication and the
planned multi-repository Docker/Podman pilot remain post-release work.
