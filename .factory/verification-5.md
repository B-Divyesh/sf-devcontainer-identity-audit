# Independent verification 5 — FAIL

**Work order:** `devcontainer-identity-audit-verify-5`

**Requested candidate:** `d82877c57de48b2f68055e551f2b5e9ea1b2badf`

**Reachable repository revision tested:**
`d82877ddec3ba114f820fee6870cbc5d96587be1`

**Live URL:** https://devcontainer-identity-audit.sociobot.in/

**Verified:** 2026-08-30 UTC

## Decision

**FAIL.** This is not a deployment outage. The live site is healthy and is
byte-for-byte identical to the production site built from reachable revision
`d82877d…`. The requested candidate `d82877c…`, however, is not present in the
provided clone or any fetched remote ref and the server rejects a direct fetch
with `upload-pack: not our ref`. It therefore cannot be checked out, tested, or
matched to production.

The acceptance contract also fails independently on the reachable/live product:

- `.factory/claims.json` is missing, so the required claim-test gate cannot run;
- the first screen does not state the audience plainly and has no one-click
  “Try it with sample data” action;
- the CLI has no `--demo` or `demo` mode and ships no sample project;
- the live `/demo` URL is only the home-page fallback and has no demo banner,
  reset action, or “Start for real” action; and
- direct Docker mode reports a false `PASS` for the reserved Linux UID/GID
  value `4294967295`.

Passing repository tests, deployment health, accessibility, privacy, and
performance checks do not override these mandatory release blockers.

## Findings

### P0 — the requested candidate is unpublished and cannot be verified

The initial clean clone was on `main` at the work-order base
`d82877ddec3ba114f820fee6870cbc5d96587be1`. A direct candidate fetch failed:

```text
fatal: remote error: upload-pack: not our ref d82877c57de48b2f68055e551f2b5e9ea1b2badf
```

`git fetch --all --prune --tags`, `git branch -a --contains`, and
`git ls-remote origin` confirmed that the candidate is absent. The only remote
branch is `main` at `d82877d…`. No dangling local object contains the candidate.
Consequently, candidate-local tests and a candidate-to-live comparison are
impossible. Publishing the exact candidate is required before another release
review.

### P1 — the mandatory claims registry is missing

The claims gate was the first repository check. `.factory/claims.json` does not
exist on the reachable revision, so there were zero claim test commands to run.
This is release-blocking under the supplied claims contract.

This is not a product with no claims. Representative unregistered claims include:

- “No telemetry,” “0 files changed,” and “3 read-only runtime calls, at most”
  on the landing page;
- “Nothing entered here leaves the page” in the browser audit;
- “No daemon, account, network request, or telemetry is involved” in README;
- `--share` path redaction, supported configuration formats, and stable exit
  codes in README; and
- offline demo operation in the live offline banner.

Each observable promise needs one tagged sandbox test in the required registry.

### P1 — first-read and sample-demo contracts fail

Cold first screen at 1440×900:

- **What it does:** I could infer that it compares workspace ownership, the
  intended container user, and Docker/Podman ID mapping before startup.
- **For whom:** the visible copy does not plainly say it is for developers using
  Dev Containers or rootless Podman. The audience must be inferred from jargon.
- **What to click first:** there is no primary sample-data action. Visible
  actions are “Try the audit,” “How it works,” “Source,” and “Copy.” “Try the
  audit” only scrolls to the form; a second click is required to produce a
  result.

The headline, “Know who owns the mount before it owns your afternoon,” is a
10-word metaphor rather than the required job statement of at most nine words.
The 23-word supporting sentence exceeds the 22-word limit and still does not
name its audience.

The required CLI demo is also absent:

- `mount-identity-audit --demo` exits 2 as an unknown option;
- `mount-identity-audit demo` treats `demo` as a project path and exits 2;
- there is no `examples/` directory or bundled sample;
- the landing page has a static terminal excerpt, not a recording of the real
  binary running against sample input;
- `.factory/demo.md` is missing; and
- `/demo` serves the same landing page with no persistent “Demo — sample data,
  nothing is saved” banner, “Reset demo,” or “Start for real.”

This independently triggers the explicit first-read failure rule.

### P2 — reserved Linux identity produces a false `PASS`

Both the packed CLI and production browser calculator accept
`4294967295:4294967295` in direct Docker mode. This value is `(uid_t)-1`, which
Linux reserves as the invalid/no-change sentinel rather than a usable process
identity.

CLI reproduction used a `0777` directory and a Dev Container `remoteUser` of
`4294967295:4294967295`:

```text
mount-identity-audit <fixture> --runtime docker --no-runtime --json
verdict: pass
container_uid: 4294967295
host_uid: 4294967295
exit: 0
```

The live browser demo with all four IDs set to `4294967295`, mode `0777`, and
Docker likewise displayed `PASS / Workspace is writable`. Values above `u32`
are rejected, and rootless mapping overflow is tested, but the maximum value
itself must also be rejected for every runtime path. A configuration a runtime
cannot launch must not receive a safe preflight verdict.

### P2 — required routing and metadata are incomplete

- `GET /does-not-exist` returns HTTP 200 and the home page. There is no designed
  404 document or 404 response override.
- No page has a canonical link, Open Graph image, Twitter card, or Apple touch
  icon. The home page has only partial Open Graph text metadata.
- Privacy and Terms omit the standard header, navigation, and footer.
- The footer has no version/build ID, which also makes deployment identity less
  observable.

All of these are explicit requirements in the supplied site-structure contract.

### P3 — required copy audit is absent

`.factory/copy-audit.md` is missing. The first-screen violations above show why
the required sentence extraction, word counts, banned-word scan, and terminology
table have not been satisfied.

## Clean repository checks

Because the requested candidate was unavailable, these checks were run against
the nearest reachable revision, `d82877d…`, after a clean `npm ci`.

- Toolchain: Rust/Cargo 1.98.0, Node 22.23.2, npm 10.9.8, Playwright 1.58.2.
- `npm ci`: passed; 59 packages installed, 0 vulnerabilities.
- `npm test`: passed 8 Rust unit tests, 16 CLI integration tests, TypeScript
  checking, 8 Vitest tests, and 22 applicable Playwright tests across desktop
  and 390×844 mobile Chromium. Four desktop executions of mobile-only cases
  were intentionally skipped.
- `cargo fmt --check`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `npm audit --audit-level=low`: passed with 0 vulnerabilities.
- No separate lint command or lint configuration exists.
- Exact `npm run build`: passed and produced the release binary plus `dist/site`.

Production payloads are within budget:

| Asset | Raw size | Budget |
| --- | ---: | ---: |
| JavaScript | 4,898 B | 200 KB |
| CSS | 12,739 B | 50 KB |
| Fonts | 0 B | 120 KB |
| Hero WebP | 216,498 B | 300 KB |

The fixed-name hero is nevertheless served with a one-year `immutable` cache;
future replacements at the same URL can remain stale in existing browsers.

## Package, public API, and CLI behavior

- `cargo package --locked` passed: 17 files, 146.6 KiB unpacked / 36.5 KiB
  compressed (`37,370` bytes).
- The packed source installed into a clean Cargo root. The binary reported
  version `0.1.0` and complete non-interactive help.
- A separate consumer compiled against the unpacked crate, called `audit`, and
  asserted schema 1, `Verdict::Pass`, exit code 0, and share-redacted config and
  workspace paths; it printed `public API PASS`.
- Explicit Docker/no-runtime owner access returned `PASS`/0.
- A declared read-only bind returned `FAIL`/1.
- Malformed JSONC and an invalid named override returned `UNKNOWN`/2 with a
  recovery instruction.
- `--share` replaced the config and workspace paths with neutral labels.
- Twenty-four concurrent audits returned success. A hash of fixture paths,
  sizes, modes, owners, and mtimes was identical before and after, supporting
  the read-only behavior.
- Docker and Podman executables were unavailable in the verifier container.
  Runtime command paths were therefore exercised by the repository's adapter
  fixtures rather than a live daemon.

The normal Rust dependency graph contains no HTTP client or telemetry package.
Source inspection found no analytics, beacon, WebSocket, or application fetch
path.

## Live functional, accessibility, privacy, and PWA evidence

The full Playwright suite against production passed the same 22 applicable
cases. Independent checks at 1440×900 and 390×844 confirmed:

- default rootless Podman sample: `FAIL`, mapped host `100999:100999`;
- “Load safe example”: `PASS`, keep-id mapping `1000:1000`;
- invalid mode `0899`: an announced, actionable validation error;
- zero serious or critical axe findings on Home, Privacy, and Terms;
- no horizontal page overflow and no sub-44 px visible links/buttons at 390 px;
- first Tab reaches the skip link with a 3 px green focus outline and 3 px
  offset; keyboard-only submit and recovery pass;
- reduced motion is detected, smooth scrolling becomes `auto`, and result
  animation is reduced to 0.01 ms;
- no console errors, page errors, or failed requests; and
- every observed request in the complete demo flow is same-origin. The only
  cold-load requests were HTML, hashed JS, hashed CSS, and the hero image.

There were no cookies and localStorage, sessionStorage, and IndexedDB all
remained empty after the demo flow.

The service worker registered, `registration.update()` completed, the offline
banner appeared, and an offline reload restored the page under service-worker
control. Privacy and Terms also load offline. This validates PWA mechanics but
does not satisfy the missing isolated demo contract.

## Headers, caching, links, and performance

- HTTP redirects to HTTPS.
- Root, legal pages, assets, hero, and service worker return a restrictive CSP,
  `Permissions-Policy`, `Referrer-Policy: no-referrer`, `nosniff`, and
  preload-ready HSTS.
- HTML and `sw.js` revalidate after 30 seconds; a conditional root request
  returned 304. Hashed JS/CSS and the hero are cached for one year as immutable.
- Every linked local, GitHub, license, and Param Factory URL returned HTTP 200.
- Lighthouse 12.8.2 mobile: Performance **99**, Accessibility **100**, Best
  Practices **100**, SEO **100**; FCP 0.888 s, LCP 1.980 s, TBT 67 ms, CLS 0,
  Speed Index 0.938 s, and 227,061 B total transfer.

The product is static and has no server-side application or unlock endpoint, so
the API allowance/429/`Retry-After`, backend concurrency/persistence, health
identity, and sign-in checks are not applicable. No AI feature is warranted for
this deterministic local diagnostic.

## Deployment identity

Fresh local production output from reachable `d82877d…` matches live bytes:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `e04b03fb419f268dc1ee7e1898a5d52bcde0ecf56920a02eec203beb1e223b0c` |
| `privacy/index.html` | `f812a851c27b9c39076fe11bccb1ab7c92db53ca283d5f89dbfecd908cfe1552` |
| `terms/index.html` | `d9294a386899905d04b31d8ee84ec16a7fc0aef952435c8356f6c09420340044` |
| `assets/main-CVi0qvRm.js` | `df40cceb9959c581ffcfa3ff8ac53292b16e9f0868ba3a588b53a761e29e9b28` |
| `assets/style-CZ0y_vAu.css` | `f65501029c8ee7c7efc5f3d2a505b6a5a80e5cc0f0c658161c5454cfa3ccbdd6` |
| `mount-ledger.webp` | `6b7fee8c5d8a82e7aa51fdcb7787e82481fc30115b5aa29fa6eaffb43817398e` |
| `sw.js` | `8461735ca682c9abb8f4e07e196f3556c715c92bcd1f5d003f986e250b587d99` |

This proves the live deployment matches the reachable base. It does **not**
prove that live matches the requested, unavailable candidate.

## Required before release

1. Publish the exact candidate SHA and rerun all verification from that clean
   checkout.
2. Add `.factory/claims.json` and one demo-sandbox claim test for every product
   promise; all listed commands must pass.
3. Implement the CLI sample project and `--demo`/`demo` entry point, the live
   one-click sample action and isolated demo state, persistent demo banner and
   controls, and `.factory/demo.md`.
4. Rewrite the first screen to name the job, audience, and first action in the
   required plain-word limits; add `.factory/copy-audit.md`.
5. Reject UID/GID `4294967295` in the CLI and browser calculator, with direct
   Docker and Podman regression coverage.
6. Add a real 404 and the required metadata, consistent legal-page shell, and
   footer build identity. Use a content-hashed hero filename or remove its
   immutable caching policy.
