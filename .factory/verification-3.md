# Independent verification 3 — FAIL

**Work order:** `devcontainer-identity-audit-verify-3`  
**Candidate commit:** `8e0296b7b32d57aba2441d1fdc8c80bee7913b53`  
**Live URL:** https://devcontainer-identity-audit.sociobot.in/  
**Verified:** 2026-08-28 UTC

## Decision

**FAIL.** The candidate installs, tests, builds, packages, and is deployed
byte-for-byte at the live URL. The previous verifier's Dockerfile-only,
share-redaction, mobile reflow, target-size, and deployment-policy findings are
repaired. Fresh packaged-CLI testing nevertheless found two new ways to report
a false `PASS`: a UID-only user is assigned an invented same-number GID, and a
Compose service with both `build` and `image` trusts a stale local image instead
of the effective build user. Both can miss the first-run write failure this
product exists to predict.

## Defects

### P1 — UID-only users are assigned an unproven GID and can false-PASS

The CLI and `--help` accept either a UID or `UID:GID`. For a UID-only value,
`parse_numeric_user` silently uses the UID as the GID. A UID does not establish
the image user's primary group.

Fresh reproduction with the packaged, clean-installed binary:

- config: `remoteUser: "1000"`;
- workspace: owner `0:1000`, mode `0770`;
- command: `--runtime docker --no-runtime --json`;
- actual: exit `0`, `pass`; reported container/host identity `1000:1000` and
  writable group access;
- control: `--remote-user 1000:2000` against the same workspace returned exit
  `1`, `fail`, unreadable and unwritable.

An image may validly resolve UID 1000 to primary GID 2000 (or another group).
Without GID evidence, the safe result is `UNKNOWN` with a request for
`UID:GID`, not an assumed same-number group and `PASS`.

### P1 — Compose `build` plus `image` can trust a stale image and false-PASS

Compose validly permits a service to declare both `build` and the `image` tag
assigned to the build result. The candidate records that the service is
build-backed, but if `image` is also present it still inspects the current local
tag and ignores the Dockerfile identity. That tag can predate a Dockerfile
change—the exact pre-start state in which a preflight is useful.

Fresh reproduction:

```yaml
services:
  app:
    build: .
    image: local/audit-stale:latest
    volumes:
      - ../:/work
```

The build-context Dockerfile contained `USER 424242:424242`. The workspace was
`root:root`, mode `0755`. A strict Docker 27.3.1 metadata fixture allowed only
`info` and `image inspect`; the existing local image tag reported Docker's
empty/root user.

- actual: exit `0`, `pass`; identity source `image default`, mapped `0:0`,
  writable `true`;
- control with `--remote-user 424242:424242`: exit `1`, `fail`, writable
  `false`.

After Compose builds the declared context, the effective user is 424242 and
cannot write this mount. A build-backed identity must be tied to the current
build inputs or return `UNKNOWN`; an arbitrary local tag is not sufficient
evidence.

### P2 — the live demo accepts a Linux ID boundary then emits an impossible ID

At the accepted maximum `u32` UID/GID (`4294967295`), with the default rootless
Podman mapping and mode `0777`, the live demo reports `PASS` and maps the user to
`4295067294:4295067294`. Those values exceed the Linux ID range that the same
form validates. The demo should reject a mapped overflow or represent an
unmappable/unknown result instead of showing a valid access prediction.

### P3 — reduced-motion mode permanently exposes the skip link over the header

The reduced-motion rule applies `transform: none` to `.skip-link`, which is also
the mechanism that hides the link until focus. Before any keyboard input at
390×844 with `prefers-reduced-motion: reduce`, the fixed skip link occupied
`x=8..232`, `y=8..56.8` and overlapped the brand (`x=16..70.2`, `y=18..62`).
Keyboard focus itself still receives the correct 3 px green ring, but the
reduced-motion treatment should keep the link visually hidden until focus
without using motion.

## Clean checkout and quality gates

- Fetched `origin/main`, confirmed it and the detached clean checkout were
  exactly `8e0296b7b32d57aba2441d1fdc8c80bee7913b53`.
- Toolchain: Rust/Cargo 1.98.0, Node 22.23.2, npm 10.9.8, Playwright 1.58.2.
- `npm ci`: passed; 59 packages installed, 0 vulnerabilities.
- `npm test`: passed: 8 Rust unit tests, 9 CLI integration tests, TypeScript,
  7 Vitest tests, and 17 applicable Playwright tests across desktop Chromium
  and 390×844 mobile Chromium (3 intended desktop skips).
- `cargo fmt --all -- --check`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `npm audit --audit-level=low`: passed with 0 vulnerabilities.
- Exact `npm run build`: passed and produced
  `target/release/mount-identity-audit` and `dist/site/`.

## Package, API, CLI, and privacy evidence

- `cargo package` passed verification: 17 files, 135.8 KiB unpacked / 34.9 KiB
  compressed. The packaged source was installed into a clean Cargo root with
  `--locked`; the binary returned version `0.1.0` and complete help.
- A clean external Rust consumer compiled against the packaged crate, invoked
  the public `audit` API, asserted schema 1, `Verdict::Pass`, and share-redacted
  workspace output, then printed `public API PASS`.
- Normal direct Docker ownership passed. Explicit identity mismatch and an
  explicitly read-only workspace failed. Named user, malformed JSONC, missing
  config, file-as-workspace, negative ID, over-`u32` ID, and `--no-runtime`
  with `auto` returned exit `2`; supplying `0:0` recovered to exit `0`.
- Maximum `u32` UID/GID on a `0777` workspace passed. Rootless Podman with split
  `--userns`, `keep-id:uid=1234,gid=1234`, mapped to the calling host identity
  and passed without an unexpected runtime command.
- Share-safe malformed-config and missing-runtime reports did not expose the
  fixture project or private runtime-wrapper paths.
- Twenty-four concurrent audits all succeeded. File content, mode, owner, size,
  and mtime snapshots were identical before and after the audits.
- Normal dependency inspection contains no HTTP/network client, telemetry, or
  analytics dependency. Runtime fixtures rejected commands outside the
  documented read-only `info`, `image inspect`, and mapping reads.

## Live deployment, browser, accessibility, and PWA

- Factory `verify-url.sh`: HTTPS 200, 915 ms observed load, correct title and
  `lang=en`, exactly one `h1`, a `main`, complete image alt/button labels, and
  zero console errors.
- The repository's full Playwright suite against production passed all 17
  applicable cases on desktop and 390×844 mobile, including keyboard-only
  fail/recovery, 200% text reflow, 44 px targets, local-only behavior, service
  worker update, offline banner, offline reload, and legal routes.
- Independent axe scans of home, Privacy, and Terms on both viewports found
  zero serious or critical violations. There were zero console errors,
  page errors, or failed requests, and zero horizontal overflow.
- First Tab focused “Skip to main content”; outside the reduced-motion defect
  above it became visible with a 3 px solid green focus ring. Primary actions
  worked with Enter and no keyboard trap was found.
- Reduced-motion result transitions used `0.00001s`, `transform: none`, and
  `scroll-behavior: auto`. At mobile 200% text size, overflow was 0 and no
  tested heading, input, select, button, result heading, or result summary was
  clipped.
- All observed browser requests were first-party. After interaction there were
  no cookies and localStorage, sessionStorage, and IndexedDB were empty.
- Empty numeric input, over-`u32` input, and invalid octal mode produced clear
  alerts; valid values recovered. The maximum-ID mapped-overflow defect is
  recorded above.

## Deployment identity, response policy, caching, and budgets

Local production output and live bytes had identical SHA-256 hashes:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `68107d8dcc4ac115303767785d541813af792c16b3deb80cf8a123c0b108d059` |
| `main-Ly9IT0eB.js` | `c16df696ae7129bad21b11f2e3ebece889da580f98479b156f05e5468a57b86c` |
| `style-C_EFJJhL.css` | `de094630664788d7b3a9364656aa86e971aa0ecb4d4287a9d4c711947c6ed5c3` |
| `mount-ledger.webp` | `6b7fee8c5d8a82e7aa51fdcb7787e82481fc30115b5aa29fa6eaffb43817398e` |
| `sw.js` | `8461735ca682c9abb8f4e07e196f3556c715c92bcd1f5d003f986e250b587d99` |

- HTTP redirects to HTTPS. Root, Privacy, hashed assets, hero, and service
  worker return CSP; `Permissions-Policy: camera=(), microphone=(),
  geolocation=()`; `Referrer-Policy: no-referrer`; `nosniff`; and preload-ready
  HSTS (`max-age=31536000; includeSubDomains; preload`).
- Hashed JS/CSS and the hero return `public, max-age=31536000, immutable`.
  HTML and `sw.js` use 30-second revalidation.
- JS is 4,753 B raw / 2,137 B gzip; CSS is 11,960 B raw / 3,385 B gzip; there
  are no font files; the hero is 216,498 B. All supplied static budgets pass.
- Lighthouse 12.8.2 mobile: Performance **99**, Accessibility **100**, Best
  Practices **100**, SEO **100**; FCP 0.9 s, LCP 2.0 s, TBT 0 ms, CLS 0,
  221 KiB transfer. Navigation Lighthouse does not provide field INP; observed
  demo updates were synchronous with no long task.

## Required before release

1. Do not derive a GID from a UID-only user. Resolve the effective group from
   trustworthy image/runtime metadata or return `UNKNOWN` and require
   `UID:GID`; add group-writable false-PASS regressions for config, CLI override,
   Compose user, and image metadata inputs.
2. For services that declare `build`, do not trust an unrelated/stale local
   `image` tag. Prove that inspected metadata represents the current build, or
   return `UNKNOWN`; add the exact Compose `build` + `image` regression.
3. Validate mapped demo IDs after rootless arithmetic and preserve the skip
   link's pre-focus hidden state under reduced motion.

