# Independent verification 4 — FAIL

**Work order:** `devcontainer-identity-audit-verify-4`

**Candidate commit:** `2086a95f4c8b9ec0d00a18708c4670f8fe77bb0e`

**Live URL:** https://devcontainer-identity-audit.sociobot.in/

**Verified:** 2026-08-28 UTC

## Decision

**FAIL.** The candidate checks out cleanly, installs, passes every repository
gate, builds and packages successfully, and is deployed byte-for-byte at the
live URL. The deployment is healthy; this is not a deployment-only failure.

Fresh testing of the clean-installed package found a release-blocking false
`PASS`: a Dev Container `remoteUser` is silently replaced by the selected
Compose service's `user`. In a representative Compose project, the CLI audited
root and exited `0` even though the configured remote user cannot write the
workspace. This misses the first-run mount permission failure the product exists
to predict.

## Defects

### P1 — Compose `user` overrides Dev Container `remoteUser`, causing false `PASS`

The Dev Container specification defines `remoteUser` as the user used to spawn
lifecycle scripts and remote editor/IDE processes; its default is the container
user. It is therefore the intended remote identity when explicitly configured.
The candidate initially reads `remoteUser`, but `merge_compose` then
unconditionally replaces it with `services.<name>.user`.

Fresh reproduction used the packaged, clean-installed v0.1.0 binary:

```json
{
  "dockerComposeFile": "compose.yml",
  "service": "app",
  "remoteUser": "424242:424242",
  "workspaceFolder": "/work"
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

The workspace was `root:root`, mode `0755`.

- Command: `mount-identity-audit <project> --runtime docker --no-runtime --json`
- Actual: exit `0`, verdict `pass`, identity `0:0`, source
  `Compose service app user`, writable `true`, and summary “The intended remote
  user can read and write this bind mount.”
- Expected: audit configured `remoteUser` `424242:424242`, which can read but
  cannot write this workspace, and exit `1`.
- Control: adding `--remote-user 424242:424242` returned exit `1`, verdict
  `fail`, writable `false`, and “can read but cannot write”.

This is not an ambiguous named-user case: both identities are explicit numeric
pairs. A common Compose service can run as root while VS Code/devcontainer
processes run as a non-root `remoteUser`; reporting the service user as the
“intended remote user” defeats the brief's core preflight guarantee.

Specification evidence checked during verification: the
[Dev Container base schema](https://github.com/devcontainers/spec/blob/main/schemas/devContainer.base.schema.json)
describes `remoteUser` as
“The username to use for spawning processes in the container including
lifecycle scripts and any remote editor/IDE server process. The default is the
same user as the container.”

### P3 — the 390px adapter table does not implement its recorded mobile design

`.factory/design.md` says the audit table becomes labelled rows at 390px and no
essential content is removed. At 390×844, the deployed `.comparison` viewport
is 354 px wide while its table remains 700 px wide; the third header starts at
x=491 and is off-screen until horizontal scrolling. The scroll region remains
operable (Chromium exposes it with `tabIndex=0`), so this is a design-contract
and mobile-scannability defect, not a keyboard blocker.

## Clean checkout and quality gates

- Confirmed the clean workspace, local `HEAD`, freshly fetched `origin/main`,
  and remote `refs/heads/main` were all exactly
  `2086a95f4c8b9ec0d00a18708c4670f8fe77bb0e` before testing.
- Toolchain: Rust/Cargo 1.98.0, Node 22.23.2, npm 10.9.8, Playwright 1.58.2.
- `npm ci`: passed; 59 packages installed, 0 vulnerabilities.
- `npm test`: passed 8 Rust unit tests, 14 CLI integration tests, TypeScript
  checking, 8 Vitest tests, and 21 applicable Playwright tests across desktop
  Chromium and 390×844 mobile Chromium. Three desktop runs of mobile-only tests
  were intentionally skipped.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `npm audit --audit-level=low`: passed with 0 vulnerabilities.
- No separate JavaScript lint command or ESLint configuration exists.
- Exact `npm run build`: passed and produced
  `target/release/mount-identity-audit` and `dist/site/`.

## Package, API, CLI, and privacy evidence

- `cargo package` passed verification: 17 files, 143.7 KiB unpacked / 36.1 KiB
  compressed. The packaged source was installed with `--locked` into a clean
  Cargo root; the resulting binary reported `mount-identity-audit 0.1.0` and
  complete, non-interactive help.
- A clean external Rust consumer compiled against the packaged crate, invoked
  `audit`, asserted schema 1, `Verdict::Pass`, and share-redacted workspace
  output, then printed `public API PASS`.
- Normal direct Docker ownership passed. Explicit mismatch and a declared
  read-only Compose workspace failed. Malformed JSONC, missing config,
  file-as-workspace, negative ID, over-`u32` ID, UID-only identity, named user,
  and `--no-runtime` with `auto` returned exit `2`; a valid explicit `0:0`
  recovered to exit `0`.
- Maximum `u32` UID/GID passed against a `0777` workspace under direct Docker
  mapping. Rootless Podman mapped through strict live UID/GID-map fixtures and
  passed. A strict Docker image fixture accepted only `info` and `image inspect`
  and passed for the image-default root identity; unexpected runtime commands
  would have failed the test.
- Twenty-four concurrent package audits all returned `PASS`; file content,
  owner, group, mode, size, and mtime snapshots were identical before and after.
- `--share` reports for malformed configuration and a missing private runtime
  wrapper contained none of the fixture project or wrapper paths.
- The normal Rust dependency tree contains no HTTP/network client or telemetry
  dependency. Source inspection found no analytics or telemetry path.

The candidate repairs verified in the previous cycle do work: UID-only
identities return `UNKNOWN` without inventing a GID, build-backed Compose
services do not trust a stale image tag, mapped browser IDs reject overflow, and
the reduced-motion skip link stays hidden before focus.

## Live deployment, accessibility, privacy, and PWA

- Factory `verify-url.sh`: HTTPS 200, 756 ms observed load, correct title and
  `lang=en`, exactly one `h1`, one `main`, complete image alt/button labels, and
  zero console errors.
- The full repository Playwright suite against production passed all 21
  applicable desktop/mobile cases: normal fail/recovery, invalid input,
  overflow, keyboard-only operation, visible skip link focus, 200% mobile text
  reflow, 44 px targets, reduced motion, local-only behavior, service-worker
  update, offline banner, offline reload, and legal routes.
- Independent axe scans on Home, Privacy, and Terms at both 1440×900 and
  390×844 found zero serious or critical violations. All six page/viewport
  combinations had zero console errors, page errors, failed requests, or page
  overflow.
- First Tab focused “Skip to main content” with a 3 px solid
  `rgb(23, 98, 74)` outline and 3 px offset. Reduced motion yielded a 0.01 ms
  result animation and `scroll-behavior: auto`.
- Every observed browser request was first-party. There were no cookies and
  localStorage, sessionStorage, and IndexedDB were empty. Running the demo
  issued no request.
- Visual inspection of full-page desktop and 390px screenshots found no
  overlap or clipped primary flow. The horizontally scrolling adapter table is
  recorded separately above because it contradicts the mobile design thesis.

## Deployment identity, response policy, caching, and budgets

Fresh local production output and live bytes had identical SHA-256 hashes:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `1a4b65d623e00a63ec2590c4e75e792a31ddbfa2adab373cb31feb8a51d0c71f` |
| `main-CkXwrbtN.js` | `df40cceb9959c581ffcfa3ff8ac53292b16e9f0868ba3a588b53a761e29e9b28` |
| `style-C8JU3qNS.css` | `8e6f84f73bfcce3ce1a541a66b3765831cf2e074c6ca5f3631fe4dc821c5a8f7` |
| `mount-ledger.webp` | `6b7fee8c5d8a82e7aa51fdcb7787e82481fc30115b5aa29fa6eaffb43817398e` |
| `sw.js` | `8461735ca682c9abb8f4e07e196f3556c715c92bcd1f5d003f986e250b587d99` |
| `privacy/index.html` | `f5cad486bbf8cbc98e775eb910840997a8fe43bd142acceef40dbd289a5a6af0` |
| `terms/index.html` | `c580426dc7095e3ced607eb04b34aabac36da2ad8bf2bece0b0580f194a04a66` |

- HTTP redirects to HTTPS. Root, legal pages, hashed assets, hero, and service
  worker return CSP; `Permissions-Policy: camera=(), microphone=(),
  geolocation=()`; `Referrer-Policy: no-referrer`; `nosniff`; and preload-ready
  HSTS (`max-age=31536000; includeSubDomains; preload`).
- Hashed JS/CSS and the hero return `public, max-age=31536000, immutable`.
  HTML and `sw.js` use 30-second revalidation.
- JS is 4,898 B raw / 2,167 B gzip; CSS is 11,949 B raw / 3,381 B gzip; there
  are no font files; the hero is 216,498 B. All supplied static budgets pass.
- Lighthouse 12.8.2 mobile: Performance **99**, Accessibility **100**, Best
  Practices **100**, SEO **100**; FCP 1.145 s, LCP 1.953 s, TBT 24 ms, CLS 0,
  Speed Index 1.145 s, and 226,928 B total transfer. Navigation Lighthouse does
  not provide field INP; the tested demo updates synchronously.

## Required before release

1. Preserve an explicitly configured Dev Container `remoteUser` when merging
   Compose metadata. Use Compose `user` only as the fallback container identity
   when `remoteUser` is absent, then add the exact numeric false-`PASS`
   regression above (plus the inverse mismatch case).
2. Implement the recorded labelled-row mobile adapter comparison, or update the
   design contract and provide an equally scannable 390px treatment.
