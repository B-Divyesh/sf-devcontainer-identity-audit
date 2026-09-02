# Independent verification 12 — Mount Identity Audit

## Verdict: FAIL

Candidate commit `90190fb5224abf8f0a30c097ad8570a36ec83e61` was
independently verified on 2 September 2026 UTC against
<https://devcontainer-identity-audit.sociobot.in/>. The live static site is the
candidate build and the declared claims pass, but the CLI can return a false
`PASS` when Docker reports daemon-level user namespace remapping. That is a
release-blocking error in the product's core mount-identity decision.

No product code or infrastructure was changed during this verification.

## Release-blocking finding

### V12-1 — High — Docker `userns-remap` is treated as direct identity mapping

The default Docker path recognizes only a `SecurityOptions` entry containing
`rootless`. A normal Docker daemon reporting `name=userns` is therefore marked
as non-rootless, and every container UID/GID is treated as the same host
UID/GID. Under Docker user namespace remapping, that direct mapping is false.

Fresh packed-consumer reproduction:

1. Create a mode-`0755` workspace owned by host `1000:1000` with
   `{"remoteUser":"1000:1000"}`.
2. Supply a deterministic Docker adapter whose read-only `info --format` call
   returns:

   ```json
   {"ServerVersion":"27.3.1","SecurityOptions":["name=userns"]}
   ```

3. Run the installed package:

   ```sh
   mount-identity-audit PROJECT \
     --runtime docker --runtime-bin DOCKER_ADAPTER --json
   ```

Observed exit code: `0`. Observed report:

```text
verdict: pass
runtime: docker 27.3.1; rootless false; inspected true
container identity: 1000:1000
mapped host identity: 1000:1000
workspace: 0755 1000:1000
summary: The intended remote user can read and write this bind mount.
```

This can tell a developer the workspace is writable even though Docker maps the
container identity through the daemon's subordinate-ID range. The correct safe
result is `UNKNOWN` unless that map is proven. The implementation evidence is
`src/runtime.rs`: Docker inspection only searches serialized security options
for `rootless` at lines 105–113, while Docker mapping returns the input IDs
directly at lines 156–160. The README lists only “Docker's direct Linux ID
mapping,” but the CLI does not detect or reject the unsupported remapped case,
and its limitations omit it.

Required repair: detect Docker user namespace remapping from runtime metadata
and either resolve its mapping or return `UNKNOWN` with Docker-specific advice.
Add a registered claim regression that fails if `name=userns` produces `PASS`.

## Other finding

### V12-2 — Medium — read-only failures offer no effective recovery

A packed-CLI fixture used a workspace owned by `0:0`, mode `0755`, remote user
`0:0`, and this explicit mount:

```json
{"workspaceMount":"source=${localWorkspaceFolder},target=/workspace,type=bind,readonly"}
```

The CLI correctly returned exit `1` and “The workspace bind mount is declared
read-only.” Its only next steps were to choose the already-selected owner
identity `0:0` or change host group/mode. Neither can make a declared read-only
mount writable. The browser sample gives the useful instruction—remove the
read-only flag only if edits are intended—but the real CLI does not.

Required repair: for a read-only verdict, lead with the exact configuration
change to review (`readonly`, `read_only`, or `ro`) and do not recommend an
identity or mode change as a fix for that branch. Add a CLI regression.

## Mandatory first-read and demo gate

The cold 1440×900 page answers all three required questions in plain words:

- What it does: “Check mount permissions before container startup.”
- Who it is for: developers using Dev Containers or rootless Podman who need a
  writable workspace on first open.
- What to select first: “Try it with sample data,” followed by “Runs a known
  rootless Podman mismatch.”

One click opened `/?demo=1#demo`. At 390×844 the focused sample heading began at
82 CSS px; `Mount mismatch predicted`, mapped host identity
`100999:100999`, and `read · no write · traverse` ended at 430, 617, and 694
CSS px. Horizontal overflow was zero. The persistent banner says “Demo — sample
data, nothing is saved” and exposes Reset demo and Open blank browser check.

## Declared claims

`.factory/claims.json` exists with 22 entries. After locked installation with
`npm ci`, every listed command was run separately, including its exact grep:

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
| `browser-parity` | PASS |
| `mit-license` | PASS |
| `browser-report-details` | PASS |
| `config-discovery` | PASS |
| `runtime-optional` | PASS |
| `report-limits` | PASS |
| `compose-build-image` | PASS |
| `install-binary` | PASS |
| `build-artifacts` | PASS |

The registered direct-Docker mapping test does not exercise Docker
`userns-remap`, so its passing result does not cover V12-1.

## Clean quality gates and package consumer

- `npm ci`: PASS — 61 packages installed; zero audit vulnerabilities.
- `npm audit --audit-level=low`: PASS — zero vulnerabilities.
- `npm test`: PASS — 10 Rust unit tests, 21 Rust integration tests, 27 Vitest
  tests, and 78 Playwright tests passed; six intentional project/viewport skips.
- `npm run lint`: PASS — Rust format, warnings-as-errors Clippy, and TypeScript.
- `npm run copy:audit:check`: PASS.
- Exact `npm run build`: PASS — produced the release executable and `dist/site`.
- `cargo package --locked --allow-dirty`: PASS — 20 files, 165.3 KiB unpacked,
  41.6 KiB compressed.

The packaged crate was installed into a fresh temporary Cargo prefix. It
installed exactly one `mount-identity-audit` executable, printed useful help and
version `0.1.0`, and ran the isolated bundled demo with expected `FAIL`/exit 1.
Additional installed-package cases produced:

| Case | Result |
| --- | --- |
| owner `0:0`, mode `0755`, direct Docker | `PASS`, exit 0 |
| non-owner `1000:1000`, mode `0755`, direct Docker | `FAIL`, exit 1 |
| non-owner `1000:1000`, mode `0777`, direct Docker | `PASS`, exit 0 |
| invalid JSONC | `UNKNOWN`, exit 2 with parse detail |
| reserved UID `4294967295` | `UNKNOWN`, exit 2 with recovery advice |
| named/invalid remote identity | `UNKNOWN`, exit 2 with recovery advice |
| declared read-only mount | `FAIL`, exit 1; see V12-2 |
| `--share` on a private fixture path | no private path appeared |

## Live browser, accessibility, privacy, and offline behavior

- Factory URL verification passed in 587 ms: HTTP 200, expected title,
  `lang=en`, one H1, a main landmark, no missing image alternatives, no unnamed
  buttons, and no console errors.
- The live Playwright route suite passed 36 tests with four intentional
  desktop-only skips. It covered Home, Demo, Privacy, Terms, and 404 on desktop
  and 390 px mobile.
- Playwright Axe found zero serious or critical issues across all five routes.
  Keyboard-only use reached and operated the primary flow. The focused skip
  link had a visible 3 px `#17624a` outline. Mobile targets met 44 px, the page
  reflowed at 200% without clipping, and reduced motion reduced the result
  transition to effectively zero.
- A fresh live demo flow used the unique input `3141592`. Every observed request
  was same-origin GET traffic, and the value appeared in no URL or body. Cookies,
  localStorage, sessionStorage, and IndexedDB remained empty. There were no
  console or page errors.
- Service-worker update succeeded with active cache `mia-site-v9`; `/demo/`
  reloaded offline under the controlling worker and retained the sample `FAIL`.
- Same-origin routes and the linked GitHub repository, usage section, and
  license returned 200. A random unknown route returned the designed page with
  HTTP 404.

The site is static and has no API, account, sign-in, product-unlock call,
server-side state, or payment flow. Rate-limit, concurrency, persistence, and
Entra checks are therefore not applicable. AI is appropriately absent from this
deterministic local audit.

## Headers, caching, deployment identity, and budgets

HTML responses include a self-only CSP with `frame-ancestors 'none'`, HSTS
preload, `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and
a restrictive permissions policy. HTML and `sw.js` use 30-second revalidation;
an `If-None-Match` request returned 304. Hashed JS/CSS and the hashed hero use
one-year immutable caching.

All 18 browser-served files in the candidate `dist/site` output were
byte-identical to live production. `staticwebapp.config.json` is correctly
consumed by the host rather than publicly served. The candidate itself is the
requested commit and matched `origin/main` before report changes.

Initial JavaScript is 7,246 bytes raw and 2.99 KiB gzip; CSS is 17,008 bytes raw
and 4.30 KiB gzip; there are no font files; the hero is 216,498 bytes. Fresh
mobile Lighthouse results were Performance 99, Accessibility 100, Best
Practices 100, and SEO 100, with FCP 1.1 s, LCP 2.0 s, TBT 0 ms, CLS 0, and
226 KiB total transfer.

## Defects by severity

- Critical: none.
- High: V12-1, Docker `userns-remap` can produce a false `PASS`.
- Medium: V12-2, read-only failures do not provide an effective remediation.
- Low: none.
