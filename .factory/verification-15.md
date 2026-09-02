# Independent verification 15 — Mount Identity Audit

## Verdict: FAIL

Candidate commit `9c814a9cb8e4bb69f48bb888adeb8971a620752e` was independently verified on 2 September 2026 UTC against <https://devcontainer-identity-audit.sociobot.in/>. The live footer identifies build `9c814a9cb8e4`, and all 17 publicly served build files are byte-identical to a fresh candidate build.

The candidate fails the acceptance contract. On a supported rootless Podman `keep-id` path, the packed CLI reports `PASS` and exit code 0 for a workspace that the configured remote user cannot write. The live browser check gives the opposite, correct result for the same IDs, so the registered browser/CLI parity claim is also false despite its listed test passing.

No product code or infrastructure was changed during verification.

## Release-blocking defect

### High — rootless Podman `keep-id` can produce an unsafe false `PASS`

The CLI reads the outer rootless Podman map using:

```text
podman unshare cat /proc/self/uid_map
podman unshare cat /proc/self/gid_map
```

Under `--userns=keep-id`, Podman adds a second mapping layer. For a caller `1000:1000`, the relevant UID mapping is:

```text
keep-id inner map
container 0..999   -> intermediate 1..1000
container 1000     -> intermediate 0
container 1001...  -> intermediate 1001...

podman unshare outer map
intermediate 0     -> host 1000
intermediate 1...  -> host 100000...
```

Therefore container root maps through intermediate ID 1 to host ID 100000. It does not map to host ID 1000. This is also the mapping construction in Podman's `getRootlessKeepIDMapping`: container range 0 starts at parent/intermediate ID 1, while the kept container UID maps to parent/intermediate ID 0.

The candidate handles only the exact kept UID/GID as a special case in `src/runtime.rs` lines 176–189. Every other ID is passed directly to the outer `podman unshare` map at lines 196–199, skipping the keep-id layer.

I reproduced this with the freshly packed and installed release executable. The project had:

- `remoteUser: "0:0"`;
- `runArgs: ["--userns=keep-id"]`;
- a workspace owned by host `1000:1000`, mode `0755`;
- caller identity `1000:1000`;
- live outer UID/GID maps `0 1000 1` and `1 100000 65536`.

The recording adapter received only these read-only calls:

```text
1  info --format json
2  unshare cat /proc/self/uid_map
3  unshare cat /proc/self/gid_map
```

The installed candidate returned:

```json
{
  "verdict": "pass",
  "summary": "The intended remote user can read and write this bind mount.",
  "identity": {
    "container_uid": 0,
    "container_gid": 0,
    "host_uid": 1000,
    "host_gid": 1000
  },
  "workspace": {
    "owner_uid": 1000,
    "owner_gid": 1000,
    "mode": "0755",
    "writable": true
  }
}
```

It exited 0. Correct two-layer composition maps `0:0` to `100000:100000`, which has no write bit on that workspace and must return `FAIL`/1.

The live browser calculator, given those same owner, caller, subordinate-range, remote-user, mode, runtime, and `keep-id` inputs, returned:

```text
FAIL
100000:100000 · keep-id mapping
1000:1000 · 0755
read · no write · traverse
```

This directly contradicts claim `browser-parity`: “The browser sample follows the CLI owner, group, mode, and rootless mapping rules.” Its tagged test uses remote ID 2000 and an outer map beginning at 100001 while the browser input says the subordinate range starts at 100000. That fixture avoids the keep-id hole and does not test container root, an ID below the kept user, or genuinely identical maps.

Required repair: compose the keep-id inner UID/GID map with the live outer maps before checking permissions. Cover IDs below, equal to, and above the kept UID/GID, including container root, using identical range inputs in browser and packed-CLI tests. The browser formula should also be reconciled with Podman's actual mapping for non-kept IDs.

## Mandatory first-read and claim gates

The cold live first screen passes the comprehension gate at desktop and 390 × 844:

- What it does: **“Check mount permissions before container startup.”**
- Who it serves: developers using Dev Containers or rootless Podman who need a writable workspace.
- First action: **“Try it with sample data,”** followed by “Runs a known rootless Podman mismatch.”

The action opens the computed sample in one click. On mobile, the result title, mapped host identity, and access branch all appear in the first post-click viewport. The persistent banner exposes Reset demo and Open blank browser check.

`.factory/claims.json` exists with 24 entries. I ran every listed command separately and exactly as written from the candidate checkout. All 24 commands passed:

| Claims | Listed test result |
| --- | --- |
| `cli-demo`, `browser-demo`, `permission-verdicts`, `read-only-safety` | PASS |
| `config-support`, `compose-user-precedence`, `share-redaction`, `report-contract` | PASS |
| `runtime-mapping`, `docker-userns-remap`, `read-only-remediation`, `conservative-identities` | PASS |
| `browser-private`, `cli-private`, `offline-reload`, `browser-parity` | PASS |
| `mit-license`, `browser-report-details`, `config-discovery`, `runtime-optional` | PASS |
| `report-limits`, `compose-build-image`, `install-binary`, `build-artifacts` | PASS |

The independent keep-id counterexample shows that the `browser-parity` test does not prove its public claim. A false registered claim is release-blocking under the claims contract.

## Clean quality gates and packaged CLI

- `npm ci`: PASS — 61 packages; zero audit vulnerabilities.
- `npm test`: PASS — 11 Rust unit tests, 23 Rust integration tests, 30 Vitest tests, and 80 Playwright tests; eight intentional cross-project skips.
- `npm run lint`: PASS — rustfmt, warnings-as-errors Clippy, and TypeScript typechecking.
- `npm run copy:audit:check`: PASS.
- `npm audit --audit-level=low`: PASS — zero vulnerabilities.
- Exact `npm run build`: PASS — produced `target/release/mount-identity-audit` and `dist/site/`.
- `cargo package --locked --allow-dirty`: PASS — 20 files, 172.5 KiB unpacked and 42.9 KiB compressed.
- Clean consumer install: PASS — exactly one 1,141,400-byte executable; `--version` returned `0.1.0`; `--help` documented the public CLI; `--demo` copied bundled data to a unique temporary directory, returned the documented `FAIL`, and exited 1.

Independent installed-CLI checks produced: owner match `PASS`/0; ordinary mismatch `FAIL`/1; reserved ID, UID-only user, named user, missing project, and invalid `--no-runtime` combination `UNKNOWN`/2; group and other-mode branches `PASS`/0. Twenty-four concurrent read-only audits all returned `PASS`. The repaired image-only rootless path made exactly four read-only calls and returned the expected mapped mismatch.

## Live browser, accessibility, privacy, and recovery

- Factory `verify-url.sh`: HTTPS 200 in 601 ms; correct title and `lang=en`; one H1 and one main; no missing image alternatives, unnamed buttons, or console errors.
- Production Playwright: PASS — 36 checks with four intentional desktop-only skips. Coverage included desktop and 390 px mobile, keyboard-only operation, focus/history announcements, 200% text reflow, 44 px targets, reduced motion, invalid input and recovery, demo reset/exit, legal routes, service-worker update, and offline reload.
- Axe through Playwright found zero serious or critical findings on Home, Demo, Privacy, Terms, and 404 at both viewports.
- The first Tab reveals and focuses the skip link at `y=8` with a 3 px green focus ring. Under reduced motion, no element retained an animation or transition longer than 0.01 ms.
- A fresh live privacy probe entered unique value `3141592` and exercised invalid-input recovery, safe-example loading, and demo reset. The action generated no request. All ten observed navigation/resource requests were same-origin GETs; there were no request failures, page errors, cookies, localStorage, sessionStorage, or IndexedDB databases.
- The browser document response includes a self-only CSP with `frame-ancestors 'none'`, preload HSTS, `Referrer-Policy: no-referrer`, `nosniff`, and a restrictive Permissions Policy.
- HTML and `sw.js` revalidate after 30 seconds; a conditional document request returned 304. Hashed JS/CSS and the hero use one-year immutable caching. HTTP redirects to HTTPS. An unknown route serves the designed page with HTTP 404.
- All 16 discovered internal and external links returned 200 after redirects.

This is a static local-first site plus CLI. It has no server-side product endpoint, account, product-unlock request, payment, or sign-in. API rate-limit, backend persistence/health, and Entra authority checks are not applicable. The job is deterministic permission calculation; no AI feature is warranted.

## Deployment identity and performance

All 17 publicly served build files matched fresh local `dist/site` bytes by SHA-256, including all HTML routes, hashed assets, images, service worker, robots file, and sitemap. The two remaining build files are deployment configuration rather than public content. The live footer reports `v0.1.0 · 9c814a9cb8e4`.

| Budget or metric | Fresh result |
| --- | ---: |
| JavaScript | 7,246 bytes raw / 3,022 bytes gzip |
| CSS | 17,047 bytes raw / 4,327 bytes gzip |
| Fonts | 0 bytes |
| Hero WebP | 216,498 bytes |
| Lighthouse performance | 99 |
| Lighthouse accessibility | 100 |
| Lighthouse best practices | 100 |
| Lighthouse SEO | 100 |
| FCP / LCP | 1.2 s / 2.0 s |
| Total blocking time / CLS | 50 ms / 0 |
| Calculator action to DOM update | 0.60 ms |
| Transfer | 226 KiB |

## Defects by severity

- Critical: none.
- High: one — rootless Podman `keep-id` mapping skips Podman's inner mapping layer, producing an unsafe false `PASS` and falsifying browser/CLI parity.
- Medium: none.
- Low: none.
