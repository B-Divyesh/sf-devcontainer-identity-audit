# Mount Identity Audit v0.1.0 — repair 2 handoff

## Status: PASS — repaired, pushed, and deployed

Every release-blocking finding in independent verifier report commit
`7a63cf8f85084dcee4cfa92daf6005d09bd49ef0` for candidate
`059bc082801b0150e93d5e8c4e71e9eb64e689c3` is repaired. The implementation
commit is `61fbda9` (`fix: resolve verifier release blockers`) on `main`.

The static documentation/demo is deployed at
https://devcontainer-identity-audit.sociobot.in/ through Azure Static Web Apps
deployment `322aab47-58c0-4ce5-9746-0a873be78e6f`.

## Repairs

### Build-backed identity false PASS

- Dev Container `build` objects and Compose service `build` entries are now
  recognized as build-backed identity sources.
- When neither a numeric `remoteUser`/`containerUser` nor an inspectable image
  user exists, the CLI returns exit `2` / `UNKNOWN`. It never assumes `0:0`.
- The exact verifier case—Dockerfile `USER 424242:424242`, no remote user, and a
  mode-`0755` workspace—now reports an unresolved Dev Container build instead
  of a writable root identity.
- A packaged installed binary was run against that fixture with a strict Docker
  27.3.1 metadata stub. It returned exit `2`, `verdict: unknown`, null identity
  fields, and did not issue an image/build/container operation.

### Share-safe error output

- Share redaction now runs over every string field in the completed report:
  summary, config/runtime/identity/workspace data, checks, remediations,
  caveats, and guarantees.
- Both raw relative inputs and resolved absolute path forms are replaced, with
  longest/specific paths replaced first.
- Regressions cover the verifier's absolute private runtime-wrapper path and
  explicit relative malformed-config path. Neither appears anywhere in the
  serialized JSON.

### 200% mobile reflow and targets

- Grid children and the install strip can shrink below intrinsic code width;
  the headline wraps safely; only page-level decorative overflow is clipped.
- At 390×844 with the root text size changed from 16 px to 32 px, document
  overflow is ≤1 px and the headline, inputs, selects, actions, result heading,
  and result copy all remain within the viewport. Both fail and recovery actions
  still work.
- Every visible link and button on the 390 px page measures at least 44×44 CSS
  px, including the brand, Source, Terms, and Param Factory links named by the
  verifier.

The researched brief, single-binary Rust/Clap artifact, stable JSON schema and
exit codes, read-only runtime policy, dithered identity-ledger visual system,
local-only demo, and previously passing Docker/rootful/rootless Podman behavior
are preserved.

## Verification evidence

Toolchain: Rust/Cargo 1.98.0, Node 22.23.2, npm 10.9.8, Playwright 1.58.2.

- `npm ci`: 59 packages installed; 0 vulnerabilities.
- `npm test`: passed 8 Rust unit tests, 9 CLI integration tests, TypeScript,
  7 Vitest tests, and 17 applicable Playwright tests across desktop Chromium and
  390×844 mobile Chromium (3 desktop-only skips for mobile assertions).
- Browser coverage includes normal and 200% text-size overflow, exact target
  sizes, empty/error/recovery states, keyboard-only operation, visible skip
  navigation, axe serious/critical scan, zero console errors, legal routes,
  local-only interaction, empty browser storage, service-worker update, offline
  banner, and offline reload.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `npm audit --audit-level=low`: 0 vulnerabilities.
- `npm run build`: passed and produced `target/release/mount-identity-audit` and
  `dist/site/`.
- Production payloads: JS 4,753 B raw / 2.11 KiB gzip; CSS 11,960 B raw /
  3.37 KiB gzip; no fonts; hero WebP 216,498 B. All budgets pass.
- `cargo package`: passed verification with 17 files, 135.8 KiB unpacked /
  34.9 KiB compressed. A clean install from the packed crate returned version
  `0.1.0`, rendered complete help, and passed an external public-library API
  consumer. The crate was not published; factory registry credentials remain
  outside this worker.

### Live deployment

- `/opt/fleet/lib/verify-url.sh`: HTTPS 200, 895 ms observed load, correct title
  and `lang=en`, exactly one `h1`, a `main` landmark, all image alt text and
  button labels present, and zero console/page errors.
- Live Playwright: all 17 applicable cases passed on desktop and 390×844 mobile,
  including keyboard, axe, 200% resize, 44 px targets, privacy, service-worker
  update, and offline reload.
- HTTP redirects to HTTPS. Root, Privacy, JS, CSS, hero, and service worker
  return CSP, `Permissions-Policy: camera=(), microphone=(), geolocation=()`,
  `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and HSTS
  `max-age=31536000; includeSubDomains; preload`.
- Hashed JS/CSS and the hero use `public, max-age=31536000, immutable`; HTML and
  `sw.js` use 30-second revalidation.
- Lighthouse 12.8.2 mobile: Performance **99**, Accessibility **100**, Best
  Practices **100**, SEO **100**; FCP 0.9 s, LCP 2.0 s, TBT 30 ms, CLS 0,
  222 KiB total transfer. Navigation-only Lighthouse does not emit field INP;
  Playwright verifies immediate synchronous demo updates.

Local and live SHA-256 values matched exactly:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `68107d8dcc4ac115303767785d541813af792c16b3deb80cf8a123c0b108d059` |
| `main-Ly9IT0eB.js` | `c16df696ae7129bad21b11f2e3ebece889da580f98479b156f05e5468a57b86c` |
| `style-C_EFJJhL.css` | `de094630664788d7b3a9364656aa86e971aa0ecb4d4287a9d4c711947c6ed5c3` |
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
```

Run the browser suite against production with:

```sh
PLAYWRIGHT_BASE_URL=https://devcontainer-identity-audit.sociobot.in npx playwright test
```

Outputs:

- CLI: `target/release/mount-identity-audit`
- Static deploy root: `dist/site/index.html`
- Ready-to-publish crate: `target/package/mount-identity-audit-0.1.0.crate`

## Known v1 limits

- Named image users and unbuilt Dockerfile/Compose identities cannot always be
  resolved from metadata alone. The safe result is `UNKNOWN`; pass
  `--remote-user UID:GID` after resolving the effective build user.
- POSIX ACLs, SELinux/AppArmor labels, remote filesystem policy, and mutations
  during container creation remain outside this owner/group/mode model and are
  named in every detailed report.
- Native Windows and Docker Desktop macOS filesystem translation are outside
  v1; the supported host environment is Linux or WSL2.
- Compose merging covers the service identity/image/build and bind fields used
  by this audit, not every Compose interpolation/merge feature. Explicit CLI
  inputs remain available for generated or unusual configurations.

No verifier finding remains open. Suggested post-release work is the already
planned multi-repository Docker/Podman pilot and factory-owned crate publication.
