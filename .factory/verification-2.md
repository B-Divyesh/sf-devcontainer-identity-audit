# Independent verification 2 — FAIL

**Work order:** `devcontainer-identity-audit-verify-2`
**Candidate commit:** `059bc082801b0150e93d5e8c4e71e9eb64e689c3`
**Live URL:** https://devcontainer-identity-audit.sociobot.in/
**Verified:** 2026-08-28 UTC

## Decision

**FAIL.** The candidate installs, builds, packages, deploys, and presents well,
and the previous rootless-Podman and production-header defects are repaired.
However, the CLI returns a false `PASS` for a common valid Dockerfile-backed Dev
Container when `remoteUser` is omitted. This is a core false negative for the
job-to-be-done. Share-safe error output also leaks local paths, and the live page
clips content when text is enlarged to 200% on the required mobile viewport.

## Defects

### P1 — Dockerfile-backed Dev Containers can produce a false `PASS`

The config reader ignores the valid Dev Container `build` object. When a config
has neither `remoteUser` nor `image`, the CLI silently assumes image user `0:0`
instead of returning `UNKNOWN`.

Fresh packaged-artifact reproduction:

```json
{
  "name": "Valid Dockerfile-backed Dev Container",
  "build": {"dockerfile": "Dockerfile"}
}
```

```dockerfile
FROM ubuntu:24.04
USER 424242:424242
```

The workspace was `root:root`, mode `0755`. A strict Docker fixture returned
successful Docker 27.3.1 `info`; it did not need to start a container. Actual:

- exit `0`, verdict `pass`;
- claimed container/host identity `0:0` from `image default`;
- claimed the workspace was writable.

The Dockerfile's effective user is `424242:424242`, which can read but cannot
write that workspace. Supplying the same identity explicitly with
`--remote-user 424242:424242` correctly changes the result to exit `1`, `FAIL`.
Without an explicit identity or inspectable built image, the safe result is
`UNKNOWN`, never an assumed-root `PASS`. This defect can miss exactly the
first-run permission failure the product exists to predict.

### P2 — `--share` leaks local paths on error paths

Successful `--share --json` output correctly uses `<workspace>` and
`<devcontainer-config>`, but error fields are not comprehensively redacted.

- With `--runtime-bin /tmp/private/acme/team/docker-wrapper --share --json`,
  exit `2` included the full path in `checks[0].observed`, even though the
  summary used `<local-path>`.
- With an explicit relative malformed config,
  `--config target/qa/cases/malformed/.devcontainer/devcontainer.json --share
  --json`, the full relative path remained in `summary`.

This contradicts the documented promise that `--share` replaces local,
repository, and config paths before a report is attached publicly.

### P2 — 200% text resize clips mobile content

At 390×844, the normal page has zero horizontal overflow. After text-only resize
to 200% (`html` font size 16 px → 32 px), the demo's content expands beyond the
viewport and is hidden by `main { overflow: hidden }`. The measured `h1` box was
551.98 px wide in a 390 px viewport; inputs, the “Load safe example” action,
result heading, and result copy were visibly cut off with no horizontal recovery
path. This fails the supplied “200% without loss” accessibility requirement.

### P3 — several mobile click targets are below 44×44 px

At 390 px, the home/brand link measured 54×36 px, the Source link 43×44 px,
the Terms link 42×44 px, and the Param Factory footer link 101×15 px. The
checkbox's 22×22 visual control was not counted as a defect because its bound
label provides a 44 px row. Axe does not flag these target-size measurements.

## Clean checkout and quality gates

- Started clean at exactly `059bc082801b0150e93d5e8c4e71e9eb64e689c3`;
  `origin/main` independently resolved to the same SHA.
- Toolchain: Rust/Cargo 1.98.0, Node 22.23.2, npm 10.9.8, Playwright 1.58.2.
- `npm ci`: passed, 59 packages installed, 0 vulnerabilities.
- `npm test`: passed.
  - Rust: 7 unit tests, 6 CLI integration tests, 0 failures.
  - TypeScript: `tsc` passed.
  - Vitest: 7 tests passed.
  - Playwright: 11 applicable tests passed across desktop and 390×844; one
    intentional desktop skip for the mobile-only assertion.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `npm audit --audit-level=low`: 0 vulnerabilities.
- Exact `npm run build`: passed; produced
  `target/release/mount-identity-audit` and `dist/site/`.

## Package and CLI end-to-end evidence

`cargo package` passed its verification build: 17 files, 126.7 KiB unpacked /
33.3 KiB compressed. The `.crate` was installed with `cargo install --path ...
--root ... --locked` into a clean target. From that install:

- `--version` returned `mount-identity-audit 0.1.0`; `--help` documented all
  options and exit-code-oriented noninteractive usage.
- A clean external Rust consumer compiled against the packed crate, invoked the
  public `audit` API, asserted schema `1`, `Verdict::Pass`, and share-redacted
  workspace output, then printed `public API PASS`.
- Direct Docker owner mapping passed with exit `0`; a UID/GID mismatch failed
  with exit `1`; an explicit read-only workspace and a Compose long-syntax
  read-only bind both failed with exit `1`.
- Named user returned exit `2`/`UNKNOWN`; `--remote-user 0:0` recovered to exit
  `0`. Malformed JSONC, missing config, a file used as workspace, negative ID,
  over-`u32` ID, and `--no-runtime` with `auto` all returned exit `2`.
- Maximum `u32` UID/GID (`4294967295`) on a `0777` workspace passed.
- Image metadata resolution through the read-only Docker fixture correctly
  resolved `424242:424242` and failed the `0755 root:root` workspace.
- The repaired standard split form `runArgs: ["--userns", "keep-id"]` was
  re-run as UID/GID 65534 against a rootless Podman map fixture; it mapped to
  host 65534:65534 and passed.
- Before/after file hashes, modes, owners, sizes, and mtimes were identical for
  an audit. Runtime fixtures rejected every command except documented `info`,
  `image inspect`, and `unshare ... uid_map/gid_map` calls. Normal dependency
  inspection found no network client in the CLI.

## Live deployment, browser, privacy, and PWA evidence

The deployment matches the candidate. Locally built and live bytes had identical
SHA-256 hashes for:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `d31261acb3f4ae59ebeb9c9d3ed1729a14b8288868d359c8fa1792f509490118` |
| `main-DuLBLfrd.js` | `c16df696ae7129bad21b11f2e3ebece889da580f98479b156f05e5468a57b86c` |
| `style-CiwEsfZ-.css` | `c0aa1ffc2294a42051e950d30a2015298355426f7da6fa4ef411df6a01046ed5` |
| `mount-ledger.webp` | `6b7fee8c5d8a82e7aa51fdcb7787e82481fc30115b5aa29fa6eaffb43817398e` |
| `sw.js` | `8461735ca682c9abb8f4e07e196f3556c715c92bcd1f5d003f986e250b587d99` |

- Factory `verify-url.sh`: HTTP 200, title, `lang=en`, one `<h1>`, `<main>`,
  image alt text, labelled buttons, and zero console/page errors.
- Independent desktop 1440×900 and mobile 390×844 Playwright runs: zero
  horizontal overflow at normal text size, zero console/page/request failures,
  and zero axe violations of any impact (therefore 0 serious/critical).
- Keyboard-only: first Tab exposed “Skip to main content” with a visible 3 px
  green outline; Enter skipped header navigation and the next Tab reached the
  first main-content control. Tab reached “Run preflight”; Enter produced the
  expected fail; the next Tab reached “Load safe example”; Enter recovered to
  pass. No trap was found.
- Empty UID and `4294967296` produced clear alerts; maximum `u32` IDs with mode
  `0777` recovered to pass. Privacy and Terms each had one `<h1>`, no browser
  errors, and 0 serious/critical axe findings.
- `prefers-reduced-motion: reduce` produced `0.00001s` stamp animation,
  `transform: none`, and `scroll-behavior: auto`.
- Browser traffic used only the first-party origin. After editing and running
  the demo, there were zero additional requests, no cookies, and empty
  localStorage, sessionStorage, and IndexedDB. The service worker cached only
  public site assets.
- Service worker was activated and controlling from `/sw.js`;
  `registration.update()` completed; offline banner appeared and an offline
  reload retained the complete home/demo shell without errors.

## Response policy, caching, and budgets

- HTTP redirects to HTTPS. Root, legal, JS/CSS, hero, and service-worker
  responses include CSP, `Permissions-Policy: camera=(), microphone=(),
  geolocation=()`, `Referrer-Policy: no-referrer`, `X-Content-Type-Options:
  nosniff`, and preload-eligible HSTS (`max-age=31536000; includeSubDomains;
  preload`).
- Hashed JS/CSS and the hero return `Cache-Control: public,
  max-age=31536000, immutable`; HTML and `sw.js` use 30-second revalidation.
- Built JS: 4,753 B raw / 2,137 B gzip (budget 200 KB). CSS: 11,702 B raw /
  3,315 B gzip (budget 50 KB). No fonts ship. Hero: 216,498 B (budget 300 KB).
- Lighthouse 12.8.2 mobile: Performance **99**, Accessibility **100**, Best
  Practices **100**, SEO **100**; FCP 1.0 s, LCP 2.0 s, TBT 30 ms, CLS 0,
  total transfer 221 KiB. A navigation-only lab run does not emit field INP;
  observed demo updates were immediate and had no long task.

## Required before release

1. Recognize Dev Container/Compose build-backed configurations. Resolve their
   image user safely or return `UNKNOWN`; never default a valid but unresolved
   build to root. Add a packaged CLI regression for this exact false pass.
2. Apply share redaction recursively to summaries, checks, remediations, and
   absolute or relative paths; add error-path JSON tests.
3. Reflow at 200% text size without clipping, and bring interactive hit areas
   to at least 44×44 px. Re-run mobile keyboard, axe, and resize checks.
