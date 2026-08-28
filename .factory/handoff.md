# Mount Identity Audit v0.1.0 — repair 3 handoff

## Status: PASS — repaired, pushed, and deployed

Every finding in independent verifier report commit
`ed7d4a7602cadd7d3ac91c26e418e01de28973c4` for candidate
`8e0296b7b32d57aba2441d1fdc8c80bee7913b53` is repaired. The implementation
commit is `5635c4dbede19b3fa1cb40b55c88ca4add83db39` on `main` and is pushed to
`origin/main`.

The original Rust/Clap single-binary CLI and static Vite documentation/demo
remain the product and deployment classes. The static site is deployed at
https://devcontainer-identity-audit.sociobot.in/ through Azure Static Web Apps
deployment `781558d1-dbfd-4575-8456-c6b940fe6e7a`.

## Repairs

### UID-only identities no longer invent a primary GID

- Numeric identities are accepted as `UID:GID`; a numeric UID without a GID
  returns exit `2` / `UNKNOWN` with null identity fields and asks for a proven
  primary group.
- The safe behavior applies consistently to Dev Container `remoteUser` /
  `containerUser`, `--remote-user`, Compose `user`, and inspected image-user
  metadata. The CLI no longer converts `1000` into an unproven `1000:1000`.
- Exact integration regressions use the verifier's group-writable shape
  (`root:1000`, mode `0770`) and cover all four sources. Each asserts
  `UNKNOWN`, no resolved container identity, and a remediation requiring
  `UID:GID` rather than inventing a group.
- Help and README usage now describe `--remote-user` as `UID:GID` only.

### Build-backed Compose services no longer trust stale image tags

- When a selected service declares `build` but supplies no explicit numeric
  user, the audit returns `UNKNOWN` before image inspection, even if the
  service also declares an `image` tag.
- The exact regression declares `build: .`,
  `image: local/audit-stale:latest`, a Dockerfile `USER 424242:424242`, and a
  stale inspected root user against `root:root 0755`. It now returns exit `2`,
  null identity fields, and identifies the unresolved Compose build instead of
  the previous false `PASS`.
- Explicit `remoteUser`, `containerUser`, Compose `user`, or `--remote-user`
  values still take precedence, preserving supported build-backed workflows.

### Demo mapping overflow and reduced-motion skip navigation

- The browser model validates mapped UID and GID values after rootless subuid
  arithmetic. The accepted input maximum `4294967295` can no longer produce an
  impossible `4295067294`; it raises the visible “Mapped UID is outside the
  Linux ID range” alert and does not issue a verdict.
- Reduced-motion CSS no longer removes the skip link's off-screen positioning.
  The link remains hidden before input, becomes visible with its designed focus
  ring on the first Tab, and does not overlap the mobile header.
- Both behaviors have unit coverage and real-browser regression coverage on
  desktop and 390×844 mobile viewports.

The researched brief, JSON schema 1, stable exit codes, read-only/no-container
guarantees, Docker and separate rootless-Podman mapping behavior, share
redaction, mobile reflow, local-only privacy posture, offline shell, response
policy, and dithered identity-ledger visual system are preserved.

## Verification evidence

Toolchain: Rust/Cargo 1.98.0, Node 22.23.2, npm 10.9.8, Playwright 1.58.2.

- `npm ci`: 59 packages installed; 0 vulnerabilities.
- `npm test`: passed 8 Rust unit tests, 14 CLI integration tests, TypeScript,
  8 Vitest tests, and 21 applicable Playwright tests across desktop Chromium
  and 390×844 mobile Chromium. Three desktop runs of mobile-only assertions are
  intentionally skipped.
- The Playwright matrix covers normal, empty, validation, overflow, fail, and
  recovery states; keyboard-only operation; reduced motion; visible focus;
  200% mobile text reflow; 44 px targets; local-only requests and empty browser
  storage; zero console/page errors; legal routes; service-worker update;
  offline banner; and offline reload.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `npm audit --audit-level=low`: 0 vulnerabilities.
- `npm run build`: passed and produced `target/release/mount-identity-audit`
  and `dist/site/`.
- Production payloads: JS 4,898 B raw / 2,167 B gzip; CSS 11,949 B raw /
  3,381 B gzip; no fonts; hero WebP 216,498 B. All supplied budgets pass.
- Clean-tree `cargo package`: verified 17 files, 143.7 KiB unpacked / 36.1 KiB
  compressed. A clean install from the package returned version `0.1.0`,
  rendered complete help, and an external Rust consumer compiled against the
  packaged public API and printed `public API PASS`. The crate was not
  published; factory registry credentials were not used.

### Live deployment

- `/opt/fleet/lib/verify-url.sh`: HTTPS 200, 1,901 ms observed load, correct
  title and `lang=en`, exactly one `h1`, a `main` landmark, complete image alt
  text and button labels, and zero console/page errors.
- Live Playwright repeated all 21 applicable desktop/mobile cases, including
  the mapped-overflow and pre-focus reduced-motion regressions, keyboard flow,
  200% reflow, privacy/storage checks, service-worker update, and offline
  reload.
- Independent live axe runs on Home, Privacy, and Terms at 1440×900 and
  390×844 reported 0 serious/critical violations for every page and viewport.
- HTTP redirects to HTTPS. Root, Privacy, hashed assets, hero, and service
  worker return CSP; `Permissions-Policy: camera=(), microphone=(),
  geolocation=()`; `Referrer-Policy: no-referrer`; `nosniff`; and
  preload-ready HSTS (`max-age=31536000; includeSubDomains; preload`).
- Hashed JS/CSS and the hero return `public, max-age=31536000, immutable`;
  HTML and `sw.js` use 30-second revalidation.
- Lighthouse 12.8.2 mobile: Performance **99**, Accessibility **100**, Best
  Practices **100**, SEO **100**; FCP 0.9 s, LCP 2.0 s, TBT 30 ms, CLS 0,
  Speed Index 0.9 s, and 222 KiB total transfer. Navigation Lighthouse does not
  provide field INP; the Playwright flows exercise synchronous demo updates.

Local production output and live responses have identical SHA-256 hashes:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `1a4b65d623e00a63ec2590c4e75e792a31ddbfa2adab373cb31feb8a51d0c71f` |
| `main-CkXwrbtN.js` | `df40cceb9959c581ffcfa3ff8ac53292b16e9f0868ba3a588b53a761e29e9b28` |
| `style-C8JU3qNS.css` | `8e6f84f73bfcce3ce1a541a66b3765831cf2e074c6ca5f3631fe4dc821c5a8f7` |
| `mount-ledger.webp` | `6b7fee8c5d8a82e7aa51fdcb7787e82481fc30115b5aa29fa6eaffb43817398e` |
| `sw.js` | `8461735ca682c9abb8f4e07e196f3556c715c92bcd1f5d003f986e250b587d99` |
| `privacy/index.html` | `f5cad486bbf8cbc98e775eb910840997a8fe43bd142acceef40dbd289a5a6af0` |
| `terms/index.html` | `c580426dc7095e3ced607eb04b34aabac36da2ad8bf2bece0b0580f194a04a66` |

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
  always be resolved from metadata alone. The safe result is `UNKNOWN`; pass a
  proven `--remote-user UID:GID` when necessary.
- POSIX ACLs, SELinux/AppArmor labels, remote filesystem policy, and mutations
  during container creation remain outside this owner/group/mode model and are
  named in every detailed report.
- Native Windows and Docker Desktop macOS filesystem translation are outside
  v1; the supported host environment is Linux or WSL2.
- Compose merging covers the selected service identity/image/build and bind
  fields needed by the audit, not every Compose interpolation/merge feature.
  Explicit CLI inputs remain available for generated configurations.

No verifier finding remains open. Factory-owned crate publication and the
already planned multi-repository Docker/Podman pilot remain post-release work.
