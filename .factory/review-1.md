# Adversarial first-read review 1 — Mount Identity Audit

**Verdict: FAIL**

Reviewed commit `190a4a1fe158c9728d9c38c7eeb466898cac8886` and the live site at
<https://devcontainer-identity-audit.sociobot.in> on 1 September 2026 UTC.
The live core files checked were byte-identical to the clean-clone build.

There is one blocking repeated finding and 24 minor findings. All 15 registered
claim commands pass, but ten public claim-like sentences still have no matching
entry in `.factory/claims.json`. A PASS requires zero findings and no unlisted
claim.

## Findings

### F-1-1 — BLOCKING — the earlier hero-cache finding is only partly fixed

- **Exact location:** live response for `/mount-ledger.webp`:
  `Cache-Control: public, must-revalidate, max-age=30`.
- **History:** `.factory/verification.md`, P2, required one-year immutable
  caching for the hashed assets and hero asset. The live hashed JavaScript now
  returns `max-age=31536000, immutable`; the 216,498-byte hero does not.
- **Why this matters:** the largest first-screen file is revalidated on repeat
  visits, and the earlier correction was not completed for every named target.
- **Concrete fix:** give the hero a content-hashed filename, update its HTML and
  service-worker references, and send `public, max-age=31536000, immutable` for
  that file. Add a live-header check for the hero as well as `/assets/*`.

### F-1-2 — Minor — route changes do not place focus on the new heading

- **Exact location:** selecting `Try it with sample data` changes `/` to
  `/demo/`; `document.activeElement` is `<body>`. Going Back also leaves focus
  on `<body>`. A direct `/#how` link scrolls correctly but leaves focus on
  `<body>`.
- **Why this matters:** a keyboard or screen-reader visitor does not receive the
  required focused heading as confirmation that the route changed.
- **Concrete fix:** make the route heading programmatically focusable, focus it
  after route load or history restoration, and add forward/back focus checks to
  Playwright.

### F-1-3 — Minor — `Copy` does not name the result

- **Exact quote/location:** landing install control, `Copy`.
- **Why this matters:** the label does not say what will be copied.
- **Concrete fix:** change it to `Copy install command` and keep the temporary
  success state explicit, such as `Install command copied`.

### F-1-4 — Minor — `preflight` adds jargon and a third name for the check

- **Exact quotes/locations:** landing eyebrow `Read-only mount preflight`, form
  action `Run preflight`, and result label `Preflight report`.
- **Why this matters:** the page otherwise calls the product an audit or check.
  A visitor must infer that all three terms mean the same operation.
- **Concrete fix:** use `Checks without starting a container`, `Check mount
  permissions`, and `Permission report`.

### F-1-5 — Minor — `Start for real` does not name its destination

- **Exact quote/location:** demo banner, `Start for real`.
- **Why this matters:** the destination is a blank browser calculator; it does
  not read a project or start a CLI audit.
- **Concrete fix:** change it to `Open blank browser check`.

### F-1-6 — Minor — `browser sample` and `browser demo` are inconsistent

- **Exact quotes/locations:** first-screen facts, `The browser sample works
  offline after the first visit.` and `The browser demo sends no project data.`
- **Why this matters:** `.factory/copy-audit.md` declares `browser sample` as the
  one term for this concept.
- **Concrete fix:** rewrite the second sentence as `The browser sample sends no
  project data.`

### F-1-7 — Minor — `Test an identity mapping` is a jargon-led heading

- **Exact quote/location:** landing browser-tool heading, `Test an identity
  mapping`.
- **Why this matters:** it names an implementation concept rather than the
  result the visitor wants.
- **Concrete fix:** use `Check numeric workspace access`.

### F-1-8 — Minor — `Read the map` is unclear out of context

- **Exact quote/location:** landing How it works step B, `Read the map`.
- **Why this matters:** a heading list does not identify which map is read.
- **Concrete fix:** use `Read the runtime identity map`.

### F-1-9 — Minor — `Read the inode` relies on unexplained jargon

- **Exact quote/location:** landing How it works step C, `Read the inode`.
- **Why this matters:** the useful operation is checking directory ownership
  and permissions, not naming the filesystem structure.
- **Concrete fix:** use `Read workspace ownership and mode`.

### F-1-10 — Minor — `What it understands` is not a self-contained README heading

- **Exact quote/location:** `README.md`, `## What it understands`.
- **Why this matters:** a reader navigating by headings cannot tell whether the
  section concerns file formats, runtimes, or permissions.
- **Concrete fix:** use `## Supported configuration and permission inputs`.

### F-1-11 — Minor — `precise` is an unsupported marketing adjective

- **Exact quote/location:** README introduction, `a precise ownership report`.
- **Why this matters:** the adjective does not tell the visitor what evidence
  the report contains.
- **Concrete fix:** rewrite the sentence as `It is for developers and CI
  maintainers who want a numeric ownership report before startup.`

### F-1-12 — Minor — `first-class` is a slogan rather than usable information

- **Exact quote/location:** README Usage, `Scriptable JSON and share-safe output
  are first-class:`.
- **Why this matters:** the sentence makes a qualitative assertion without
  explaining the two options.
- **Concrete fix:** use `Use --json for scripts. Add --share to replace local
  paths with neutral labels.`

### F-1-13 — Minor — `scripting contract` is unexplained jargon

- **Exact quote/location:** README bundled sample, `this preserves the normal
  scripting contract.`
- **Why this matters:** it does not say what the exit code means for a script.
- **Concrete fix:** use `The command returns exit code 1, the same as a project
  with a confirmed mismatch.`

### F-1-14 — Minor — the terminal-recording provenance is an unlisted claim

- **Exact quote/location:** landing CLI demo, `Recorded from the real binary
  with mount-identity-audit --demo.`
- **Why this matters:** `cli-demo` runs the binary but does not compare the
  visible SVG transcript with current normalized output.
- **Concrete fix:** add a `terminal-transcript` claim and a test that compares
  the visible transcript with the current bundled-demo output, or label it
  `Example output from mount-identity-audit --demo`.

### F-1-15 — Minor — the browser report-detail promise is unlisted

- **Exact quote/location:** landing empty result, `The report will explain the
  effective host identity and exact permission branch.`
- **Why this matters:** no claim entry promises or checks both report details.
- **Concrete fix:** add a `browser-report-details` claim whose test confirms the
  mapped host identity and the selected owner/group/other permission branch.

### F-1-16 — Minor — the configuration-discovery promise is unlisted

- **Exact quote/location:** README Usage, `The CLI discovers
  .devcontainer/devcontainer.json, .devcontainer.json, or devcontainer.json.`
- **Why this matters:** `config-support` checks one `.devcontainer` path, not all
  three discovery locations named to users.
- **Concrete fix:** add a `config-discovery` claim with one clean fixture for
  each path and an explicit precedence check.

### F-1-17 — Minor — optional-runtime behavior is an unlisted claim

- **Exact quote/location:** README Install, `Docker or Podman is optional when
  every identity is supplied numerically.`
- **Why this matters:** no claim entry states this setup promise.
- **Concrete fix:** add a `runtime-optional` claim that runs the packed CLI with
  numeric identities and no installed runtime.

### F-1-18 — Minor — the report-limit statement is an unlisted claim

- **Exact quote/location:** README limits, `Every detailed report states these
  limits.`
- **Why this matters:** no registered claim checks pass, fail, and unknown
  reports for the stated limit text or fields.
- **Concrete fix:** add a `report-limits` claim covering human and JSON output,
  or remove `Every` and describe where the limits are documented.

### F-1-19 — Minor — stale-image handling is an unlisted claim

- **Exact quote/location:** README limits, `The CLI never trusts a possibly
  stale image tag as current build evidence.`
- **Why this matters:** a local regression exists, but no claim entry names this
  behavior and the `conservative-identities` tagged test does not use a Compose
  `build` plus `image` fixture.
- **Concrete fix:** register `compose-build-image` and tag the existing clean
  integration case that confirms an `UNKNOWN` result.

### F-1-20 — Minor — mapping-name output is an unlisted claim

- **Exact quote/location:** landing Runtime differences, `The report names the
  mapping it used.`
- **Why this matters:** the registered runtime claim checks different numeric
  mappings, but its claim text does not promise the visible mapping label.
- **Concrete fix:** add the visible label to `runtime-mapping` and assert it in
  the tagged test, or remove this sentence.

### F-1-21 — Minor — the single-binary build statement is unlisted

- **Exact quote/location:** README Install, `Build the single binary with a
  stable Rust toolchain:`.
- **Why this matters:** this is an installation outcome a visitor can rely on,
  but it has no claim entry.
- **Concrete fix:** add an `install-binary` claim that installs the packed crate
  in a clean prefix and confirms the one public executable and its help output.

### F-1-22 — Minor — the documented `npm test` coverage is unlisted

- **Exact quote/location:** README Develop, test, and build, `npm test runs Rust
  unit, integration, claim, and browser tests.`
- **Why this matters:** the command passed in this review, but the public
  statement is not represented in the claims registry.
- **Concrete fix:** register a development-gate claim or replace the sentence
  with a non-claiming command label such as `Run all repository checks:`.

### F-1-23 — Minor — the documented build outputs are unlisted

- **Exact quote/location:** README Develop, test, and build, `npm run build
  produces the release binary and the deployable site at dist/site.`
- **Why this matters:** the command passed in this review, but no registered
  claim checks both named outputs.
- **Concrete fix:** register a `build-artifacts` claim that checks the release
  executable and `dist/site`, or introduce the command without an output claim.

### F-1-24 — Minor — the README says the registry is complete when it is not

- **Exact quote/location:** README Privacy and security, `Every public promise
  is listed in .factory/claims.json.`
- **Why this matters:** F-1-14 through F-1-23 identify public promises with no
  matching entry.
- **Concrete fix:** complete the registry and tagged tests, then keep this
  sentence; otherwise remove it.

### F-1-25 — Minor — the repository copy-audit evidence has incorrect counts

- **Exact location:** `.factory/copy-audit.md` says counts are
  whitespace-separated, but it records 7 words for `The demo still works
  locally; install links may not.`; the sentence has 9. It also records 11 for
  the 10-word octal hint and 17 for the 18-word limits sentence.
- **Why this matters:** the handoff evidence cannot be reproduced using its
  stated counting method.
- **Concrete fix:** regenerate the audit with one documented tokenizer and add
  the README sentence table required by this review process.

## First screen check

Fresh Chromium contexts were opened without scrolling at 390×844 and 1440×900.

| Question | Mobile | Desktop | Evidence |
| --- | --- | --- | --- |
| What does this do? | Confirmed | Confirmed | `Check mount permissions before container startup` |
| For whom? | Confirmed | Confirmed | `For developers using Dev Containers or rootless Podman...` |
| What should be selected first? | Confirmed | Confirmed | `Try it with sample data`, followed by `Runs a known rootless Podman mismatch.` |

The primary action and all three plain facts are in the initial viewport at both
sizes. No first-screen blocking finding was recorded.

## Copy audit

Counts use whitespace-separated tokens that contain a letter or number.
Punctuation-only separators are not words. No sentence exceeds 22 words, and no
banned plain-words term appears. Findings F-1-3 through F-1-13 cover every
button, terminology, jargon, adjective, and heading issue found.

### Landing page sentences

| Words | Sentence |
| ---: | --- |
| 2 | You’re offline. |
| 9 | The demo still works locally; install links may not. |
| 6 | Check mount permissions before container startup. |
| 17 | For developers using Dev Containers or rootless Podman who need a writable workspace on the first open. |
| 6 | Runs a known rootless Podman mismatch. |
| 5 | Free under the MIT License. |
| 9 | The browser sample works offline after the first visit. |
| 7 | The browser demo sends no project data. |
| 10 | Host identity and container identity meet at one bind mount. |
| 14 | This browser check uses the same owner, group, and mode rules as the CLI. |
| 7 | It stores and sends nothing you enter. |
| 10 | Three or four octal digits, such as 0755 or 0775. |
| 9 | Use numeric identities—the CLI refuses to guess named users. |
| 11 | Enter the caller identity and range starts from `/etc/subuid` and `/etc/subgid`. |
| 9 | Run the sample mismatch or load the safe example. |
| 12 | The report will explain the effective host identity and exact permission branch. |
| 12 | Find the remote user and workspace bind in JSONC or Compose metadata. |
| 12 | Ask Docker or Podman for runtime metadata and rootless user namespace ranges. |
| 12 | Compare the effective host UID/GID with owner, group, and other permission bits. |
| 6 | Docker and Podman are separate cases. |
| 7 | The report names the mapping it used. |
| 12 | It never treats “container user 1000” as “host user 1000” without evidence. |
| 18 | The audit does not inspect ACLs, security labels, remote filesystem policy, or changes made while a container starts. |
| 6 | It never changes ownership or permissions. |
| 11 | The browser sample stays in memory and sends no project data. |
| 8 | Recorded from the real binary with `mount-identity-audit --demo`. |
| 11 | Demo — bundled sample data; your project was not read or changed. |
| 11 | The mapped remote identity can read but cannot write the workspace. |
| 6 | Local identity evidence for mounted workspaces. |

### README sentences and sentence-like prose lines

Code blocks are excluded. Short lead-ins and list items are included so no
reader-facing prose is omitted.

| Words | Sentence or prose line |
| ---: | --- |
| 15 | Mount Identity Audit checks bind-mount ownership before a Dev Container or rootless Podman workspace starts. |
| 10 | It compares host permissions with the mapped numeric container identity. |
| 22 | It is for developers and CI maintainers who would rather see a precise ownership report than discover a `Permission denied` after startup. |
| 6 | Live documentation and one-click sample: `devcontainer-identity-audit.sociobot.in/demo/` |
| 9 | Build the single binary with a stable Rust toolchain: |
| 5 | The CLI needs no account. |
| 7 | It includes no HTTP or telemetry client. |
| 11 | Docker or Podman is optional when every identity is supplied numerically. |
| 10 | Run one command without pointing the audit at your project: |
| 9 | The command copies `examples/mismatch/` into a unique temporary directory. |
| 8 | It prints that path and audits the copy. |
| 16 | The known write mismatch returns `FAIL` with exit code `1`; this preserves the normal scripting contract. |
| 17 | The browser sample also accepts the host caller identity and the allocated `/etc/subuid` and `/etc/subgid` range starts. |
| 22 | These values let it distinguish the identity kept by Podman `keep-id` from other remote users, which still map through the subordinate ranges. |
| 8 | The CLI reads the live runtime map instead. |
| 5 | Run it from a repository. |
| 7 | The CLI discovers `.devcontainer/devcontainer.json`, `.devcontainer.json`, or `devcontainer.json`. |
| 10 | It follows Compose metadata when the configuration names a service. |
| 2 | Typical report: |
| 18 | Use an explicit numeric identity when an image stores only a user name or is not available locally: |
| 7 | Scriptable JSON and share-safe output are first-class: |
| 12 | `--share` replaces host paths, repository names, and config paths with neutral labels. |
| 7 | JSON output is versioned with `schema_version: 1`. |
| 4 | Exit codes are stable: |
| 8 | `0`: the mount is predicted readable and writable; |
| 9 | `1`: a definite permission or read-only failure was found; |
| 15 | `2`: the audit could not reach a safe conclusion (configuration, runtime, or named-user resolution error). |
| 16 | The process never runs `chown`, edits configuration, pulls an image, creates a container, or starts one. |
| 9 | An audit makes at most three read-only runtime calls. |
| 11 | Those calls use `info`, `image inspect`, or rootless Podman identity maps. |
| 4 | JSON-with-comments Dev Container files; |
| 10 | `remoteUser`, `containerUser`, `image`, `build`, `workspaceFolder`, `workspaceMount`, `runArgs`, `dockerComposeFile`, and `service`; |
| 7 | Compose `services.<name>.user`, `image`, `build`, `volumes`, and `read_only`; |
| 5 | Docker’s direct Linux ID mapping; |
| 21 | rootful Podman and rootless Podman’s live UID/GID maps, including both `--userns=keep-id` / `--userns=host` and split `--userns keep-id` / `--userns host` intent; |
| 6 | host owner/group/mode and read-only mount declarations. |
| 10 | An explicit Dev Container `remoteUser` is the intended editor identity. |
| 12 | It stays authoritative when a selected Compose service declares a different `user`. |
| 7 | Otherwise, that Compose service `user` overrides `containerUser`. |
| 12 | The CLI uses `containerUser` only when neither of those values is present. |
| 16 | Named image users and UID-only values do not prove a primary GID without running a container. |
| 14 | The audit returns `UNKNOWN` and requests `--remote-user UID:GID`; it never invents a same-number group. |
| 11 | Linux reserves ID `4294967295`, so the CLI rejects it as `UNKNOWN`. |
| 15 | POSIX ACLs, security labels, remote filesystems, and startup mutations remain outside the v1 permission model. |
| 6 | Every detailed report states these limits. |
| 10 | Build-backed configurations without an explicit numeric user also return `UNKNOWN`. |
| 13 | The CLI never trusts a possibly stale image tag as current build evidence. |
| 12 | Requirements: a Linux host (or WSL2), stable Rust, Node 20+, and npm. |
| 12 | Docker Desktop filesystem translation on native macOS/Windows is outside the v1 model. |
| 10 | `npm test` runs Rust unit, integration, claim, and browser tests. |
| 13 | `npm run build` produces the release binary and the deployable site at `dist/site`. |
| 8 | Run the site locally with `npm run dev`. |
| 5 | To build only one artifact: |
| 16 | Prepare the unpublished registry artifact with `cargo package`; publishing is intentionally left to the Param Factory. |
| 8 | The CLI includes no network or telemetry client. |
| 12 | The browser sample stores no entered values and sends no project data. |
| 7 | After the first visit, it reloads offline. |
| 8 | Use `--share` before attaching reports to public issues. |
| 7 | Every public promise is listed in `.factory/claims.json`. |
| 14 | Run one claim with its listed command or run all coverage with `npm test`. |
| 6 | MIT © 2026 Sociobot (Param Factory). |

### Heading and control check

- Headline: 6 words; names the job.
- Audience sentence: 17 words; names the intended users and outcome.
- No sentence exceeds 22 words.
- No banned term from the supplied plain-words list appears.
- Result-naming controls that already pass: `Try it with sample data`, `Load
  safe example`, `Reset demo`, `Read the source`, and `See every option`.
- Findings F-1-3 through F-1-13 list every control, heading, adjective,
  inconsistent term, and jargon issue found.

## Demo and sandbox check

- One selection from the cold first screen opens `/demo/`.
- The destination immediately contains the required banner, populated numeric
  sample, rootless Podman selection, and computed `FAIL` state. On desktop the
  failure report is visible in the first viewport. On mobile the populated
  sample begins in the first viewport and the heading states that the report
  explains the write failure.
- The sample reports mapped identity `100999:100999 · rootless subuid map` and
  explains that it can read but cannot create or edit entries.
- `Reset demo` restores owner `1000`, the default namespace, and the `FAIL`
  result after edits.
- `Start for real` returns to `/#demo`, removes demo mode, and leaves the normal
  calculator at `Ready`.
- After entering the unique value `3141592`, checking, and resetting, cookies,
  localStorage, sessionStorage, and IndexedDB remained empty. No request carried
  the value. All observed requests were same-origin static GETs.
- The CLI `--demo` registered test confirmed a unique OS temporary directory,
  unchanged bundled input, expected `FAIL`, and exit code `1`.

## Registered claims

Every exact `test` value in `.factory/claims.json` was run separately after
`npm ci` in a fresh clone at the reviewed commit.

| Claim ID | Result |
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

There are no failing registered tests. Findings F-1-14 through F-1-24 record
the public claim-like sentences that are not fully represented in the registry.

## Earlier-finding confirmation

No earlier `.factory/review-*.md` or `.factory/polish-*.md` files exist. The
existing handoff reports PASS. For completeness, every defect recorded in the
available independent verification files was checked again.

| Earlier finding | Current confirmation |
| --- | --- |
| Split `--userns keep-id` parsing | Fixed; CLI regression and registered runtime checks pass. |
| Missing CSP and Permissions Policy | Fixed; both are present on live responses. |
| Static caching | **Partly fixed; F-1-1 repeats the unresolved hero-file part.** |
| Short HSTS age | Fixed; live value is `max-age=31536000; includeSubDomains; preload`. |
| Dockerfile-backed false safe result | Fixed; build-backed identity check returns `UNKNOWN`. |
| Share output path disclosure | Fixed; registered redaction check passes. |
| 200% mobile clipping | Fixed; live mobile reflow check passes. |
| Small mobile targets | Fixed for all checked public routes; live target check passes. |
| UID-only invented GID | Fixed; conservative-identity checks pass. |
| Compose `build` plus `image` stale evidence | Behavior fixed in the integration suite; registry gap remains F-1-19. |
| Reserved-ID browser result | Fixed; validation check passes. |
| Reduced-motion skip-link display | Fixed; reduced-motion keyboard check passes. |
| Compose/Dev Container identity precedence | Fixed; packed-CLI claim passes. |
| Mobile runtime comparison layout | Fixed; labelled-row check passes. |
| Unpublished candidate | Fixed; nine core live files match the clean build byte for byte. |
| Missing claims registry | Fixed; 15 entries exist and all commands pass. |
| Missing one-click sample | Fixed; the cold action opens a populated sample. |
| Missing route metadata and designed 404 | Fixed; route and live-404 checks pass. |
| Missing copy audit | Present, but its count accuracy remains F-1-25. |
| Claim commands failing in a clean clone | Fixed; every exact command passes independently. |
| First-screen facts below viewport | Fixed at 390×844 and 1440×900. |
| Browser `keep-id` mismatch | Fixed; browser and packed CLI both report the checked `102000:102000` case as `FAIL`. |
| Safe-example recovery after invalid input | Fixed; live recovery check passes. |

## Structure, accessibility, links, and identity

- `/`, `/demo/`, `/privacy/`, and `/terms/` return 200. An unknown path returns
  the designed 404 document with status 404.
- Titles follow the required pattern and are under 60 characters. Every checked
  route has `lang=en`, one H1, one main landmark, a description, canonical URL,
  Open Graph image, Twitter card, SVG favicon, and apple-touch icon.
- `robots.txt` and `sitemap.xml` are present; the sitemap lists all four public
  routes.
- Every link found across the home, demo, legal, and 404 pages was requested.
  All intended destinations returned 200; the deliberately unknown review URL
  returned the designed 404.
- Header and footer content is consistent across routes and includes Privacy,
  Terms, Param Factory, version, and build ID.
- The factory URL verifier passed in 550 ms with zero console errors, one H1,
  one main landmark, complete image alternatives, and labelled buttons.
- The live Playwright suite passed 57 applicable checks with 5 configured
  project skips. Integrated Axe checks found zero serious or critical issues.
- Keyboard operation, skip-link focus, 200% text reflow, 44 px mobile targets,
  reduced motion, offline reload, and no horizontal overflow all pass.
- The 6,085-byte JavaScript bundle is well below the static-product budget.
- The dithered two-colour technical-ledger identity is specific to this product
  and matches `.factory/design.md`; it does not present as a generic template.
- Route focus remains the exception recorded as F-1-2.

## Missed leverage

No additional feature finding is recorded. JSON output and share-redacted
export already cover the useful handoff path implied by the brief. Sync would
conflict with the local, read-only job. A model-assisted step would make a
deterministic UID/GID and mode calculation less explainable, so no AI feature is
expected here.

## Quality-gate evidence

- `npm ci`: PASS, 0 vulnerabilities.
- All 15 exact claim commands in a fresh clone: PASS.
- `npm test`: PASS — 10 Rust unit tests, 21 Rust integration tests, 23 Vitest
  checks, and 57 applicable Playwright checks.
- `npm run lint`: PASS.
- `npm run build`: PASS; release CLI and `dist/site/` produced.
- Live Playwright run: PASS, 57 applicable checks and 5 configured skips.
- Factory live URL verifier: PASS.
- Core build/live byte comparison: 9 of 9 checked files match.

## What would make this perfect

Complete F-1-1 through F-1-25, then repeat the cold mobile and desktop review,
every registered claim command from a fresh clone, the live route-focus check,
the full link crawl, and the live hero-header check. At that point there should
be no remaining copy flag, unlisted claim, historical regression, or untested
statement.
