# Adversarial first-read review 5 — Mount Identity Audit

**Verdict: FAIL**

Reviewed commit `bd6b50d7299628387bdca1ee582829f97a106e58` and the live
site at <https://devcontainer-identity-audit.sociobot.in/> on 2 September 2026
UTC. The first-read, demo, privacy, claims, accessibility, routing, and build
checks pass. One minor factual defect remains. The work order permits `PASS`
only with zero findings.

## Findings

### F-5-1 — Minor — every footer presents a stale build label

- **Exact quote/location:** the footer on `/`, `/demo/`, `/privacy/`, `/terms/`,
  `/404.html`, and the designed unknown-route response says
  `Built by Param Factory · v0.1.0 · polish-3`.
- **Code location:** `site/index.html:228`, `site/demo/index.html:101`,
  `site/privacy/index.html:45`, `site/terms/index.html:45`, and
  `site/404.html:39`. `site/e2e/site.spec.ts:335` and
  `site/src/site-structure.test.ts:20` require the stale literal rather than a
  current build value.
- **Evidence:** `.factory/polish-4.md` records a later deployed polish-4 build.
  The current product also includes the later Docker user-namespace repair and
  service-worker cache revision. Eighteen deployable files from the current
  clean build match production byte-for-byte, so this is not an old live
  deployment; it is an old label in the current build.
- **Why this matters:** the required footer build identifier gives visitors
  and support staff the wrong release identity. The existing tests preserve
  the error instead of checking freshness.
- **Concrete fix:** inject the package version and current short commit or
  factory build ID into every footer during the Vite build. Replace the two
  hard-coded `polish-3` assertions with one test that checks every route has the
  injected current ID and rejects a stale literal.

## Cold first-screen check

I opened the live site in fresh Chromium contexts at 390×844 and 1440×900 and
did not scroll before recording this interpretation:

| Question | My first-read answer | Exact first-screen evidence |
| --- | --- | --- |
| What does this do? | It checks whether a mounted workspace will be writable before a Dev Container or rootless Podman container starts. | `Check mount permissions before container startup` |
| For whom? | Developers using Dev Containers or rootless Podman. | `For developers using Dev Containers or rootless Podman who need a writable workspace on the first open.` |
| What should I click first? | `Try it with sample data`. | The adjacent text says `Runs a known rootless Podman mismatch.` |

The action and all three facts are above the fold at both widths. This gate
passes.

## Copy audit

Counts use the repository's documented tokenizer: split on whitespace and
count each token containing a Unicode letter or number. Code blocks are
excluded. `npm run copy:audit:check` passed in the clean clone. No sentence is
over 22 words and no attached plain-words banned term appears. F-5-1 flags the
only misleading factual label. Headings and action labels otherwise name their
sections or results, and terminology remains consistent.

### Landing-page sentences

| Words | Sentence |
| ---: | --- |
| 6 | Demo — sample data, nothing is saved |
| 6 | ● Checks without starting a container · v0.1.0 |
| 6 | Check mount permissions before container startup |
| 17 | For developers using Dev Containers or rootless Podman who need a writable workspace on the first open. |
| 11 | A bind mount maps the container identity to a host identity. |
| 2 | Browser sample |
| 14 | This browser check uses the same owner, group, and mode rules as the CLI. |
| 7 | It stores and sends nothing you enter. |
| 10 | Three or four octal digits, such as 0755 or 0775. |
| 9 | Use numeric identities—the CLI refuses to guess named users. |
| 11 | Enter the caller identity and range starts from /etc/subuid and /etc/subgid. |
| 2 | Permission report |
| 9 | Run the sample mismatch or load the safe example. |
| 12 | The report will explain the effective host identity and exact permission branch. |
| 3 | Safe next step |
| 3 | How it works |
| 12 | Find the remote user and workspace bind in JSONC or Compose metadata. |
| 12 | Ask Docker or Podman for runtime metadata and rootless user namespace ranges. |
| 12 | Compare the effective host UID/GID with owner, group, and other permission bits. |
| 2 | Runtime differences |
| 7 | The report names the mapping it used. |
| 12 | It never treats “container user 1000” as “host user 1000” without evidence. |
| 3 | Limits and privacy |
| 18 | The audit does not inspect ACLs, security labels, remote filesystem policy, or changes made while a container starts. |
| 6 | It never changes ownership or permissions. |
| 11 | The browser sample stays in memory and sends no project data. |
| 2 | CLI demo |
| 5 | Example output from mount-identity-audit --demo. |
| 11 | Check whether a remote user can write to a mounted workspace. |
| 6 | Built by Param Factory · v0.1.0 · polish-3 |
| 2 | You’re offline. |
| 9 | The demo still works locally; install links may not. |
| 6 | Runs a known rootless Podman mismatch. |
| 5 | Free under the MIT License. |
| 9 | The browser sample works offline after the first visit. |
| 7 | The browser sample sends no project data. |
| 5 | The demo could not complete. |
| 4 | Reload and try again. |
| 3 | Install command copied. |
| 2 | Couldn’t copy. |
| 7 | Select the command and copy it manually. |
| 12 | Directory mode must be three or four octal digits, such as 0755. |
| 11 | The mapped remote user can read, write, and traverse this workspace. |
| 5 | No ownership change is indicated. |
| 9 | Confirm with the CLI against the real runtime map. |
| 9 | The mount is explicitly read-only, regardless of matching ownership. |
| 11 | Remove the read-only mount flag only if workspace edits are intended. |
| 11 | The mapped remote user cannot read and traverse the workspace directory. |
| 13 | Use Podman keep-id so the remote user maps back to the host owner. |
| 14 | Choose a remote UID:GID that maps to the workspace owner or intended project group. |
| 14 | The mapped remote user can read this directory but cannot create or edit entries. |
| 13 | Match the workspace owner or deliberately grant group write access on the host. |

### README sentences and sentence-like prose

| Words | Sentence |
| ---: | --- |
| 15 | Mount Identity Audit checks bind-mount ownership before a Dev Container or rootless Podman workspace starts. |
| 10 | It compares host permissions with the mapped numeric container identity. |
| 15 | It is for developers and CI maintainers who want a numeric ownership report before startup. |
| 6 | Live documentation and one-click sample: devcontainer-identity-audit.sociobot.in/?demo=1#demo |
| 8 | Build one binary with a stable Rust toolchain: |
| 5 | The CLI needs no account. |
| 7 | It includes no HTTP or telemetry client. |
| 11 | Docker or Podman is optional when every identity is supplied numerically. |
| 10 | Run one command without pointing the audit at your project: |
| 9 | The command copies examples/mismatch/ into a unique temporary directory. |
| 8 | It prints that path and audits the copy. |
| 19 | The known write mismatch returns FAIL with exit code 1, the same as a project with a confirmed mismatch. |
| 17 | The browser sample also accepts the host caller identity and the allocated /etc/subuid and /etc/subgid range starts. |
| 7 | Podman keep-id preserves the host caller’s ID. |
| 9 | Other remote users map through the subordinate ID ranges. |
| 8 | The CLI reads the live runtime map instead. |
| 5 | Run it from a repository. |
| 7 | The CLI discovers .devcontainer/devcontainer.json, .devcontainer.json, or devcontainer.json. |
| 10 | It follows Compose metadata when the configuration names a service. |
| 2 | Typical report: |
| 18 | Use an explicit numeric identity when an image stores only a user name or is not available locally: |
| 4 | Use --json for scripts. |
| 9 | Add --share to replace local paths with neutral labels: |
| 12 | --share replaces host paths, repository names, and config paths with neutral labels. |
| 7 | JSON output is versioned with schema_version: 1. |
| 4 | Exit codes are stable: |
| 8 | 0: the mount is predicted readable and writable; |
| 9 | 1: a definite permission or read-only failure was found; |
| 15 | 2: the audit could not reach a safe conclusion (configuration, runtime, or named-user resolution error). |
| 16 | The process never runs chown, edits configuration, pulls an image, creates a container, or starts one. |
| 9 | An audit makes at most three read-only runtime calls. |
| 11 | Those calls use info, image inspect, or rootless Podman identity maps. |
| 4 | JSON-with-comments Dev Container files; |
| 10 | remoteUser, containerUser, image, build, workspaceFolder, workspaceMount, runArgs, dockerComposeFile, and service; |
| 7 | Compose services.&lt;name&gt;.user, image, build, volumes, and read_only; |
| 10 | Docker’s direct Linux ID mapping when daemon userns-remap is inactive; |
| 19 | rootful Podman and rootless Podman’s live UID/GID maps, including both --userns=keep-id / --userns=host and split --userns keep-id / --userns host intent; |
| 6 | host owner/group/mode and read-only mount declarations. |
| 10 | An explicit Dev Container remoteUser is the intended editor identity. |
| 12 | It stays authoritative when a selected Compose service declares a different user. |
| 7 | Otherwise, that Compose service user overrides containerUser. |
| 12 | The CLI uses containerUser only when neither of those values is present. |
| 16 | Named image users and UID-only values do not prove a primary GID without running a container. |
| 14 | The audit returns UNKNOWN and requests --remote-user UID:GID; it never invents a same-number group. |
| 11 | Linux reserves ID 4294967295, so the CLI rejects it as UNKNOWN. |
| 10 | Docker daemon userns-remap changes the host IDs behind container IDs. |
| 12 | The CLI detects name=userns and returns UNKNOWN until that remap is resolved. |
| 13 | It never reports matching container and host IDs as safe in this case. |
| 15 | For a read-only workspace, the CLI recommends reviewing the mount's readonly, read_only, or ro setting. |
| 12 | It does not suggest identity or host mode changes for that failure. |
| 16 | Version 1 does not check POSIX ACLs, security labels, remote filesystems, or changes made during startup. |
| 6 | Every detailed report states these limits. |
| 10 | Build-backed configurations without an explicit numeric user also return UNKNOWN. |
| 13 | The CLI never trusts a possibly stale image tag as current build evidence. |
| 12 | Requirements: a Linux host (or WSL2), stable Rust, Node 20+, and npm. |
| 12 | Docker Desktop filesystem translation on native macOS/Windows is outside the v1 model. |
| 4 | Run all repository checks: |
| 13 | npm run build produces the release binary and the deployable site at dist/site. |
| 8 | Run the site locally with npm run dev. |
| 5 | To build only one artifact: |
| 7 | Run cargo package to prepare the crate. |
| 4 | Param Factory handles publishing. |
| 8 | The CLI includes no network or telemetry client. |
| 12 | The browser sample stores no entered values and sends no project data. |
| 7 | After the first visit, it reloads offline. |
| 8 | Use --share before attaching reports to public issues. |
| 7 | Every public promise is listed in .factory/claims.json. |
| 14 | Run one claim with its listed command or run all coverage with npm test. |
| 5 | MIT © 2026 Sociobot (Param Factory). |

### Labels, jargon, and terminology

- Result-naming actions pass: `Try it with sample data`, `Copy install command`,
  `Check mount permissions`, `Load safe example`, `Reset demo`, and
  `Open blank browser check`.
- Section headings pass in context: `Check numeric workspace access`, `Check
  the configuration, map, and workspace`, `Docker and Podman are separate
  cases`, `Know what the audit does not check`, and `Run the same check in a
  terminal`.
- The same concepts consistently use `workspace`, `remote user`, `identity
  map`, `Dev Container`, `keep-id`, `browser sample`, and `bundled sample`.
- `JSONC`, UID/GID, `subuid`, `keep-id`, and the runtime names are necessary
  technical terms for this developer-facing CLI and are explained or shown in
  context. No marketing adjective or information-free slogan was found.

## Demo, sandbox, and privacy

- One click on `Try it with sample data` opens `/?demo=1#demo`. At 390×844,
  `Mount mismatch predicted`, mapped host identity
  `100999:100999 · rootless subuid map`, and
  `read · no write · traverse` are all inside the first viewport.
- The persistent banner says `Demo — sample data, nothing is saved` and includes
  `Reset demo` and `Open blank browser check`.
- After editing a seeded value, Reset restored `Owner UID` to `1000` and the
  original failing report. The safe example and exit-to-blank paths also passed
  in the live suite.
- Before and after the demo, cookies, Local Storage, Session Storage, and
  IndexedDB were empty. The request log contained only the production origin.
  No entered value appeared in a request.
- The offline-reload claim passed in a dedicated browser context after service
  worker activation.
- The clean-clone release binary ran `mount-identity-audit --demo`, copied
  `examples/mismatch/` to a unique `/tmp/mount-identity-audit-demo-*` directory,
  returned the expected `FAIL` with exit 1, and left the repository unchanged.

## Claims

I ran every exact command in `.factory/claims.json` independently from clean
clone `/tmp/mia-review5-clean-EU3HwK/repo` after `npm ci`.

| Claim | Result | Observable evidence |
| --- | --- | --- |
| `cli-demo` | PASS | Isolated bundled sample and temporary path |
| `browser-demo` | PASS | One-click 390×844 mismatch result |
| `permission-verdicts` | PASS | Exit codes 0, 1, and 2 |
| `read-only-safety` | PASS | No project changes or container startup; call cap |
| `config-support` | PASS | JSONC and selected Compose metadata |
| `compose-user-precedence` | PASS | Packed-CLI precedence fixtures |
| `share-redaction` | PASS | All supplied path classes removed |
| `report-contract` | PASS | Schema version and neutral labels |
| `runtime-mapping` | PASS | Separate named Docker and rootless Podman mappings |
| `docker-userns-remap` | PASS | Unresolved remap returns UNKNOWN |
| `read-only-remediation` | PASS | Mount-setting recovery for all three spellings |
| `conservative-identities` | PASS | Unproved identities return UNKNOWN |
| `browser-private` | PASS | Same-origin requests and empty browser storage |
| `cli-private` | PASS | No account, network, or telemetry client |
| `offline-reload` | PASS | Dedicated offline context reload |
| `browser-parity` | PASS | Browser result matches packed CLI |
| `mit-license` | PASS | Landing, metadata, and LICENSE agree |
| `browser-report-details` | PASS | Mapped identity and access branch visible |
| `config-discovery` | PASS | Three paths and precedence |
| `runtime-optional` | PASS | Numeric project with `--no-runtime` |
| `report-limits` | PASS | Pass, fail, and unknown reports list limits |
| `compose-build-image` | PASS | Stale tag is not trusted |
| `install-binary` | PASS | Packed crate installs one documented executable |
| `build-artifacts` | PASS | Release CLI and `dist/site` exist |

The live landing page and README were cross-checked sentence by sentence
against the registry. All behavioral, privacy, compatibility, and quantitative
claims have entries. No untested product claim remains. F-5-1 concerns an
incorrect release label, not an untested product behavior.

## Earlier finding verification

I read all prior `review-*.md`, `polish-*.md`, verification reports, and the
handoff. Each earlier finding was checked on production and in the current
code; none is repeated.

| Earlier finding | Status | Fresh evidence |
| --- | --- | --- |
| F-1-1 | Fixed | Live hashed hero returns one-year immutable caching. |
| F-1-2 | Fixed | Live demo, fragment, Back, and Forward navigation focus the destination heading. |
| F-1-3 | Fixed | The action says `Copy install command`; success and failure text name the result. |
| F-1-4 | Fixed | No public `preflight` wording remains; check/report terms are used. |
| F-1-5 | Fixed | Demo exit says `Open blank browser check`. |
| F-1-6 | Fixed | `browser sample` is used consistently. |
| F-1-7 | Fixed | Browser section says `Check numeric workspace access`. |
| F-1-8 | Fixed | Step B says `Read the runtime identity map`. |
| F-1-9 | Fixed | Step C says `Read workspace ownership and mode`. |
| F-1-10 | Fixed | README heading names supported configuration and permission inputs. |
| F-1-11 | Fixed | README names the numeric report without `precise`. |
| F-1-12 | Fixed | README directly explains `--json` and `--share`. |
| F-1-13 | Fixed | Bundled sample states that a confirmed mismatch returns exit 1. |
| F-1-14 | Fixed | Terminal caption says `Example output`; it makes no recording-provenance claim. |
| F-1-15 | Fixed | Registered browser report test verifies mapped identity and permission branch. |
| F-1-16 | Fixed | All three discovery paths and precedence pass. |
| F-1-17 | Fixed | Numeric identities work without Docker or Podman. |
| F-1-18 | Fixed | Pass, fail, and unknown reports list the limits. |
| F-1-19 | Fixed | Compose `build` plus `image` returns UNKNOWN. |
| F-1-20 | Fixed | Docker and rootless Podman reports name distinct mappings. |
| F-1-21 | Fixed | The packed crate installs one documented executable. |
| F-1-22 | Fixed | README uses the non-claiming `Run all repository checks` label. |
| F-1-23 | Fixed | The registered build test confirms the CLI and `dist/site`. |
| F-1-24 | Fixed | Registry validation and all 24 exact claim commands pass. |
| F-1-25 | Fixed | Generated counts preserve Markdown paths, links, placeholders, and the nine-word em-dash case. |
| F-2-1 | Fixed | Mobile one-click demo shows the verdict, mapped ID, and access branch in the first viewport. |
| F-2-2 | Fixed | Privacy, Terms, Demo, Back, and Forward focus and announce headings. |
| F-2-3 | Fixed | Hero caption literally explains the bind-mount identity mapping. |
| F-2-4 | Fixed | Step A says `Read the configuration`. |
| F-2-5 | Fixed | Forms use `remote user`; mapped output uses `host identity`. |
| F-2-6 | Fixed | Footer one-liner says what the tool checks. |
| F-2-7 | Fixed | README separates `keep-id` and subordinate-range explanations. |
| F-2-8 | Fixed | README directly states version-1 limits. |
| F-2-9 | Fixed | README directly assigns crate preparation and publishing responsibility. |
| F-2-10 | Fixed | Clipboard failure retains the action name and gives manual recovery. |
| F-4-1 | Fixed | The generated audit lists all six formerly combined UI sentences separately. |

## Structure, accessibility, links, and visual identity

- Titles are route-specific and follow the required pattern: `Mount Identity
  Audit — Check mount permissions`, `Demo — Mount Identity Audit`, `Privacy —
  Mount Identity Audit`, `Terms — Mount Identity Audit`, and `Page not found —
  Mount Identity Audit`.
- Every route has `lang="en"`, one h1, one main landmark, a description,
  canonical URL, Open Graph/Twitter metadata, SVG favicon, apple-touch icon,
  header, footer, Privacy, and Terms. `robots.txt` and `sitemap.xml` return 200.
- An unknown path returns HTTP 404 with the designed page. Direct routes,
  fragment links, Back, and Forward preserve the correct route, focus the new
  heading, and update the polite live region.
- Every discovered public link returned 200, excluding the intentional HTTP
  404 status of the unknown route itself. No console error was recorded.
- Live Playwright: 36 passed and four intentional desktop skips. Axe found zero
  serious or critical issues on Home, Demo, Privacy, Terms, and 404 at desktop
  and mobile widths. Keyboard flow, reduced motion, 200% reflow, and 44-pixel
  mobile targets passed.
- CSP, HSTS, `Referrer-Policy`, `X-Content-Type-Options`, and Permissions Policy
  are present. `frame-ancestors 'none'` is correctly sent as a response header.
- Hashed JS/CSS and the hashed hero have one-year immutable caching. Initial JS
  is 7,246 bytes raw across two files, far below the 200 KB product limit.
- Eighteen public build files match the live responses byte-for-byte.
  `staticwebapp.config.json` is deployment configuration and correctly is not a
  public URL.
- The dithered two-colour identity ledger, warm paper palette, hard rules,
  registration marks, and stamped result are specific to mount identity
  auditing. This is not a generic SaaS template.

## Quality gates

The clean clone passed:

- `npm test`: 11 Rust unit tests, 23 Rust integration tests, 28 Vitest tests,
  and 80 Playwright tests; eight intentional project-specific skips.
- `npm run lint`.
- `npm run build`, producing the release executable and `dist/site`.
- `npm run copy:audit:check`.
- `cargo package --locked --allow-dirty`.

## Missed leverage

No missing AI feature is expected. The brief requires a deterministic local
permission calculation; model output would weaken a numeric access verdict.
The tool already provides the implied machine export and sharing path through
JSON and `--share`, plus isolated browser and CLI sample paths. No decorative
AI or provider key is present.

## What would make this perfect

Replace the stale `polish-3` footer text with a build-time version and commit or
factory build ID, and test that current value across every route. Then rerun the
review. No other finding remains.
