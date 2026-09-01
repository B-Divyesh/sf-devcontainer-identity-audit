# Adversarial first-read review 2 — Mount Identity Audit

**Verdict: FAIL**

Reviewed commit `5ccbfa2eb78b647d3dc12c3c92cccda7abd29771` and the live site at
<https://devcontainer-identity-audit.sociobot.in/> on 1 September 2026 UTC.
There are two blocking findings and nine minor findings. All 22 registered
claim commands pass, but the one-click demo does not show its result in the
first mobile screen. One finding from review 1 has also regressed.

## Findings

### F-2-1 — BLOCKING — the one-click mobile demo hides the result below the form

- **Exact location:** select `Try it with sample data` at 390×844. The resulting
  `/?demo=1#demo` viewport contains the three-row sticky demo banner, a clipped
  `Check numeric workspace access` heading, introductory copy, and the start of
  `Host workspace`. It does not contain `Mount mismatch predicted`, the mapped
  identity, or the permission result. On `/demo/`, `#audit-result` begins at
  approximately 2,371 CSS pixels from the top.
- **Why this blocks a first-time visitor:** the required first screen after the
  click must already show the product being used with realistic sample data. A
  phone visitor instead sees a long input form and must discover that a result
  already exists below it.
- **Concrete fix:** on demo entry, put a compact sample summary and the computed
  `FAIL` result before the editable form at mobile widths. Keep the banner to at
  most two rows, apply a sticky-header-aware scroll offset so the heading is not
  clipped, and add a 390×844 assertion that `#result-title`, `#mapped-id`, and
  `#access-id` are in the initial viewport after the one click.

### F-1-25 — BLOCKING (repeated) — the copy-audit evidence still has an incorrect count

- **Exact location:** `.factory/copy-audit.md` declares whitespace-delimited
  tokens, but records `10` for `Use numeric identities—the CLI refuses to guess
  named users.` The declared tokenizer counts `identities—the` as one token, so
  the reproducible count is `9`.
- **History:** review 1 required reproducible counts under F-1-25. `polish-1.md`
  marks that finding fixed, but the current file still contains this mismatch.
- **Why this blocks acceptance:** the work order says every earlier finding must
  be verified rather than trusted, and any unfixed or regressed finding returns
  as blocking under the same ID.
- **Concrete fix:** change the count to `9`, generate both landing and README
  tables from the documented tokenizer, and test the generated counts so prose
  edits cannot make the evidence stale.

### F-2-2 — Minor — legal route changes are neither focused nor announced

- **Exact location:** select `Privacy` or `Terms` from the live site. After each
  navigation, `document.activeElement` is `<body>`. Neither page contains an
  `[aria-live]` route-status element.
- **Why this matters:** a keyboard or screen-reader visitor gets no explicit
  focus or live announcement for the new page heading, as required by the site
  structure contract.
- **Concrete fix:** focus the destination `<h1>` after navigation and announce
  its text through one visually hidden `aria-live="polite"` route-status region.
  Cover `/privacy/`, `/terms/`, `/demo/`, Back, and Forward in Playwright.

### F-2-3 — Minor — the illustration caption uses a metaphor

- **Exact quote/location:** landing hero caption, `Host identity and container
  identity meet at one bind mount.`
- **Why this matters:** `meet` makes the reader interpret a metaphor instead of
  stating the mapping operation.
- **Concrete rewrite:** `A bind mount maps the container identity to a host
  identity.`

### F-2-4 — Minor — `Read config` abbreviates the established term

- **Exact quote/location:** How it works, step A, `Read config`; the section
  heading and README use `configuration`.
- **Why this matters:** the same concept has two names, and `config` is jargon
  when the full word fits.
- **Concrete rewrite:** `Read the configuration`.

### F-2-5 — Minor — the browser uses four names for the same person

- **Exact quotes/locations:** form legend `Container process`, labels `Remote
  UID` and `Remote GID`, result label `Container`, summary `The mapped identity
  ...`, and remedy `the developer identity`.
- **Why this matters:** a first-time visitor must infer that the container
  process, remote user, container identity, mapped identity, and developer
  identity refer to one identity before and after mapping.
- **Concrete fix:** use `remote user` for the person throughout: `Remote user`,
  `Remote UID`, `Remote GID`, `Remote user`, and `The mapped remote user ...`.
  Reserve `host identity` for the post-mapping value.

### F-2-6 — Minor — the footer is a slogan rather than a useful one-liner

- **Exact quote/location:** every footer, `Local identity evidence for mounted
  workspaces.`
- **Why this matters:** the noun phrase does not say what the visitor can do or
  what result the product returns.
- **Concrete rewrite:** `Check whether a remote user can write to a mounted
  workspace.`

### F-2-7 — Minor — the README packs mapping jargon into one 22-word sentence

- **Exact quote/location:** README, Try the bundled sample: `These values let it
  distinguish the identity kept by Podman keep-id from other remote users,
  which still map through the subordinate ranges.`
- **Why this matters:** `identity kept`, `keep-id`, and `subordinate ranges`
  require three concepts to be decoded at once. The sentence sits at the
  22-word hard limit.
- **Concrete rewrite:** `Podman keep-id preserves the host caller’s ID. Other
  remote users map through the subordinate ID ranges.`

### F-2-8 — Minor — the README describes limits as a `permission model`

- **Exact quote/location:** README, Supported configuration and permission
  inputs: `POSIX ACLs, security labels, remote filesystems, and startup
  mutations remain outside the v1 permission model.`
- **Why this matters:** `startup mutations` and `permission model` are abstract
  terms where a direct limitation is clearer.
- **Concrete rewrite:** `Version 1 does not check POSIX ACLs, security labels,
  remote filesystems, or changes made during startup.`

### F-2-9 — Minor — the README publishing instruction is needlessly indirect

- **Exact quote/location:** README, Develop, test, and build: `Prepare the
  unpublished registry artifact with cargo package; publishing is intentionally
  left to the Param Factory.`
- **Why this matters:** `registry artifact` and `intentionally left` obscure the
  two actions and actors.
- **Concrete rewrite:** `Run cargo package to prepare the crate. Param Factory
  handles publishing.`

### F-2-10 — Minor — the copy failure state neither explains the error nor names the action

- **Exact quote/location:** landing install button after a rejected Clipboard
  API call, `Select command`.
- **Why this matters:** the button still performs a copy attempt, but its label
  tells the visitor to perform a different action. It does not say that copying
  failed, why it may have failed, or what to do next.
- **Concrete fix:** keep the button label `Copy install command` and place a
  status beside it: `Couldn’t copy. Select the command and copy it manually.`

## First-screen check

Fresh Chromium contexts were opened without scrolling at 390×844 and 1440×900.

| Question | Mobile | Desktop | Evidence |
| --- | --- | --- | --- |
| What does this do? | Confirmed | Confirmed | `Check mount permissions before container startup` |
| For whom? | Confirmed | Confirmed | `For developers using Dev Containers or rootless Podman ...` |
| What should be selected first? | Confirmed | Confirmed | `Try it with sample data`, followed by `Runs a known rootless Podman mismatch.` |

In my own words: it predicts whether a Dev Container or rootless Podman user can
write to the mounted workspace before startup; it is for developers using those
runtimes; the first action is the sample-data link. The cold landing screen
passes. Finding F-2-1 concerns the screen produced by that action.

## Copy audit

Counts use the repository's stated tokenizer: every whitespace-delimited token
containing a Unicode letter or number counts once. Code blocks are excluded from
sentence counts. Sentence-like labels and prose list items are included. No
sentence exceeds 22 words and no banned marketing adjective appears.

### Landing-page sentences

This table includes static landing copy, the one-click sample result, alternate
result messages, and error sentence templates reachable on the landing page.

| Words | Sentence |
| ---: | --- |
| 2 | You’re offline. |
| 9 | The demo still works locally; install links may not. |
| 5 | Checks without starting a container. |
| 6 | Check mount permissions before container startup. |
| 17 | For developers using Dev Containers or rootless Podman who need a writable workspace on the first open. |
| 6 | Runs a known rootless Podman mismatch. |
| 5 | Free under the MIT License. |
| 9 | The browser sample works offline after the first visit. |
| 7 | The browser sample sends no project data. |
| 10 | Host identity and container identity meet at one bind mount. |
| 6 | Demo — sample data, nothing is saved. |
| 14 | This browser check uses the same owner, group, and mode rules as the CLI. |
| 7 | It stores and sends nothing you enter. |
| 10 | Three or four octal digits, such as 0755 or 0775. |
| 9 | Use numeric identities—the CLI refuses to guess named users. |
| 11 | Enter the caller identity and range starts from /etc/subuid and /etc/subgid. |
| 9 | Run the sample mismatch or load the safe example. |
| 12 | The report will explain the effective host identity and exact permission branch. |
| 13 | The mapped identity can read this directory but cannot create or edit entries. |
| 13 | Use Podman keep-id so the developer identity maps back to the host owner. |
| 10 | The mapped identity can read, write, and traverse this workspace. |
| 5 | No ownership change is indicated. |
| 9 | Confirm with the CLI against the real runtime map. |
| 9 | The mount is explicitly read-only, regardless of matching ownership. |
| 11 | Remove the read-only mount flag only if workspace edits are intended. |
| 10 | The mapped identity cannot read and traverse the workspace directory. |
| 14 | Choose a remote UID:GID that maps to the workspace owner or intended project group. |
| 13 | Match the workspace owner or deliberately grant group write access on the host. |
| 7 | Owner UID must be a non-negative number. |
| 9 | Owner UID must be below Linux's reserved 4294967295 value. |
| 12 | Directory mode must be three or four octal digits, such as 0755. |
| 5 | The demo could not complete. |
| 4 | Reload and try again. |
| 12 | Find the remote user and workspace bind in JSONC or Compose metadata. |
| 12 | Ask Docker or Podman for runtime metadata and rootless user namespace ranges. |
| 12 | Compare the effective host UID/GID with owner, group, and other permission bits. |
| 6 | Docker and Podman are separate cases. |
| 7 | The report names the mapping it used. |
| 12 | It never treats container user 1000 as host user 1000 without evidence. |
| 18 | The audit does not inspect ACLs, security labels, remote filesystem policy, or changes made while a container starts. |
| 6 | It never changes ownership or permissions. |
| 11 | The browser sample stays in memory and sends no project data. |
| 5 | Example output from mount-identity-audit --demo. |
| 11 | DEMO — bundled sample data; your project was not read or changed. |
| 11 | The mapped remote identity can read but cannot write the workspace. |
| 6 | Local identity evidence for mounted workspaces. |

Flags from this table are F-2-3 through F-2-6. F-2-10 covers a sentence-like
error state. `Owner UID` represents the two error templates; the other current
two-word field labels have the same counts.

### README sentences and sentence-like prose

| Words | Sentence or prose line |
| ---: | --- |
| 15 | Mount Identity Audit checks bind-mount ownership before a Dev Container or rootless Podman workspace starts. |
| 10 | It compares host permissions with the mapped numeric container identity. |
| 15 | It is for developers and CI maintainers who want a numeric ownership report before startup. |
| 5 | Live documentation and one-click sample. |
| 8 | Build one binary with a stable Rust toolchain. |
| 5 | The CLI needs no account. |
| 7 | It includes no HTTP or telemetry client. |
| 11 | Docker or Podman is optional when every identity is supplied numerically. |
| 10 | Run one command without pointing the audit at your project. |
| 9 | The command copies examples/mismatch/ into a unique temporary directory. |
| 8 | It prints that path and audits the copy. |
| 19 | The known write mismatch returns FAIL with exit code 1, the same as a project with a confirmed mismatch. |
| 17 | The browser sample also accepts the host caller identity and the allocated /etc/subuid and /etc/subgid range starts. |
| 22 | These values let it distinguish the identity kept by Podman keep-id from other remote users, which still map through the subordinate ranges. |
| 8 | The CLI reads the live runtime map instead. |
| 5 | Run it from a repository. |
| 7 | The CLI discovers .devcontainer/devcontainer.json, .devcontainer.json, or devcontainer.json. |
| 10 | It follows Compose metadata when the configuration names a service. |
| 2 | Typical report. |
| 18 | Use an explicit numeric identity when an image stores only a user name or is not available locally. |
| 4 | Use --json for scripts. |
| 9 | Add --share to replace local paths with neutral labels. |
| 12 | --share replaces host paths, repository names, and config paths with neutral labels. |
| 7 | JSON output is versioned with schema_version: 1. |
| 4 | Exit codes are stable. |
| 8 | 0: the mount is predicted readable and writable. |
| 9 | 1: a definite permission or read-only failure was found. |
| 15 | 2: the audit could not reach a safe conclusion (configuration, runtime, or named-user resolution error). |
| 16 | The process never runs chown, edits configuration, pulls an image, creates a container, or starts one. |
| 9 | An audit makes at most three read-only runtime calls. |
| 11 | Those calls use info, image inspect, or rootless Podman identity maps. |
| 4 | JSON-with-comments Dev Container files. |
| 10 | remoteUser, containerUser, image, build, workspaceFolder, workspaceMount, runArgs, dockerComposeFile, and service. |
| 7 | Compose services.&lt;name&gt;.user, image, build, volumes, and read_only. |
| 5 | Docker’s direct Linux ID mapping. |
| 19 | rootful Podman and rootless Podman’s live UID/GID maps, including both --userns=keep-id / --userns=host and split --userns keep-id / --userns host intent. |
| 6 | host owner/group/mode and read-only mount declarations. |
| 10 | An explicit Dev Container remoteUser is the intended editor identity. |
| 12 | It stays authoritative when a selected Compose service declares a different user. |
| 7 | Otherwise, that Compose service user overrides containerUser. |
| 12 | The CLI uses containerUser only when neither of those values is present. |
| 16 | Named image users and UID-only values do not prove a primary GID without running a container. |
| 14 | The audit returns UNKNOWN and requests --remote-user UID:GID; it never invents a same-number group. |
| 11 | Linux reserves ID 4294967295, so the CLI rejects it as UNKNOWN. |
| 15 | POSIX ACLs, security labels, remote filesystems, and startup mutations remain outside the v1 permission model. |
| 6 | Every detailed report states these limits. |
| 10 | Build-backed configurations without an explicit numeric user also return UNKNOWN. |
| 13 | The CLI never trusts a possibly stale image tag as current build evidence. |
| 12 | Requirements: a Linux host (or WSL2), stable Rust, Node 20+, and npm. |
| 12 | Docker Desktop filesystem translation on native macOS/Windows is outside the v1 model. |
| 4 | Run all repository checks. |
| 13 | npm run build produces the release binary and the deployable site at dist/site. |
| 8 | Run the site locally with npm run dev. |
| 5 | To build only one artifact. |
| 16 | Prepare the unpublished registry artifact with cargo package; publishing is intentionally left to the Param Factory. |
| 8 | The CLI includes no network or telemetry client. |
| 12 | The browser sample stores no entered values and sends no project data. |
| 7 | After the first visit, it reloads offline. |
| 8 | Use --share before attaching reports to public issues. |
| 7 | Every public promise is listed in .factory/claims.json. |
| 14 | Run one claim with its listed command or run all coverage with npm test. |

Flags from this table are F-2-7 through F-2-9. All README headings (`Install`,
`Try the bundled sample`, `Usage`, `Supported configuration and permission
inputs`, `Develop, test, and build`, `Privacy and security`, and `License`) name
their sections directly.

### Landing headings and actions

The job headline is 6 words. Section headings describe their sections. Normal
actions use result-naming verbs: `Try it with sample data`, `Copy install
command`, `Check mount permissions`, `Load safe example`, `Reset demo`, `Open
blank browser check`, `Read the source`, and `See every option`. The sole
heading terminology flag is F-2-4. The Clipboard failure label is F-2-10.

## Demo and sandbox verification

- Home to `/?demo=1#demo` takes one click and immediately computes the realistic
  rootless Podman mismatch (`1000:1000` maps to `100999:100999`, mode `0755`).
- The persistent banner says `Demo — sample data, nothing is saved` and exposes
  `Reset demo` plus `Open blank browser check`.
- Editing the owner and mode, running the check, and selecting Reset restored
  `1000`, `0755`, default rootless mapping, and the original `FAIL` result.
- Leaving demo mode produced the blank `Waiting for an identity` state.
- Before, during, and after the demo, cookies, Local Storage, Session Storage,
  and IndexedDB were empty. The request log contained only same-origin static
  GET requests and no submission request.
- The release CLI was run with `--demo` from an empty temporary directory. It
  copied the sample to `/tmp/mount-identity-audit-demo-...`, printed the path,
  returned `FAIL` with exit code 1, and left the invoking directory empty.
- Isolation and Reset pass. The mobile first-result presentation fails under
  F-2-1.

## Claims

Every exact command from `.factory/claims.json` was run separately after
`git clone --local /work/repo ...` and `npm ci`. Logs are at
`/tmp/mia-review2-<claim-id>.log` in this worker.

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

The live landing page and README were cross-checked against the registry. No
unlisted functional or privacy claim was found. The passing `browser-demo` test
asserts that the result exists in the DOM; it does not contradict F-2-1 because
it does not assert that the result is in the initial mobile viewport.

## Earlier finding verification

Every finding in `.factory/review-1.md` was checked against the live site and
repository, rather than accepted from `.factory/polish-1.md`.

| Earlier ID | Status in review 2 | Evidence |
| --- | --- | --- |
| F-1-1 | Fixed | Hashed hero and JS both return `max-age=31536000, immutable`. |
| F-1-2 | Fixed for its named demo/Back/fragment cases | Live Playwright focus assertions pass. F-2-2 covers the separate legal-route gap. |
| F-1-3 | Fixed | Live label is `Copy install command`. |
| F-1-4 | Fixed | No public `preflight` copy remains. |
| F-1-5 | Fixed | Demo exit is `Open blank browser check`. |
| F-1-6 | Fixed | First-screen facts use `browser sample` consistently. |
| F-1-7 | Fixed | Heading is `Check numeric workspace access`. |
| F-1-8 | Fixed | Step B is `Read the runtime identity map`. |
| F-1-9 | Fixed | Step C is `Read workspace ownership and mode`. |
| F-1-10 | Fixed | README heading is `Supported configuration and permission inputs`. |
| F-1-11 | Fixed | Unsupported `precise` adjective is gone. |
| F-1-12 | Fixed | `first-class` slogan is gone. |
| F-1-13 | Fixed | The sample now explains exit code 1 directly. |
| F-1-14 | Fixed | Caption says `Example output`; it no longer claims a current recording. |
| F-1-15 | Fixed | `browser-report-details` is registered and passes. |
| F-1-16 | Fixed | `config-discovery` is registered and passes. |
| F-1-17 | Fixed | `runtime-optional` is registered and passes. |
| F-1-18 | Fixed | `report-limits` is registered and passes. |
| F-1-19 | Fixed | `compose-build-image` is registered and passes. |
| F-1-20 | Fixed | `runtime-mapping` covers distinct Docker/Podman mapping outputs and passes. |
| F-1-21 | Fixed | `install-binary` installs the packed crate and passes. |
| F-1-22 | Fixed | README now says `Run all repository checks`. |
| F-1-23 | Fixed | `build-artifacts` is registered and passes. |
| F-1-24 | Fixed | The current landing/README cross-check found no unlisted functional claim. |
| F-1-25 | **Regressed; BLOCKING** | The declared 10-word count is reproducibly 9. |

## Structure, accessibility, and identity

- `/`, `/demo/`, `/privacy/`, `/terms/`, `/404.html`, and an unknown path were
  loaded live. Each has `lang="en"`, one `<main>`, one `<h1>`, route-specific
  title, description, canonical, OG/Twitter data, favicon, and apple-touch icon.
- The unknown path returns HTTP 404 with the designed page and a home action.
- A crawl of every discovered link returned 200 except the intentional unknown
  route's self URL, which correctly remained 404. No product link is dead.
- Header and footer shells are consistent. Privacy and Terms are present.
- Desktop and mobile live suites pass: 32 executed checks total, with no serious
  or critical Axe findings, no console errors, keyboard and 200% reflow checks,
  reduced-motion handling, touch targets, offline reload, metadata, and routing.
- F-2-2 records the route focus/announcement omission not covered by those
  passing assertions.
- The dithered two-colour identity ledger is product-specific and matches
  `.factory/design.md`; it does not read as a generic SaaS template.
- Initial live JavaScript is 6,805 bytes. All observed requests are first-party.

## Missed leverage

No missed-leverage finding was recorded. The brief calls for configuration and
runtime inspection, pass/fail/unknown output, safe remediation, redacted sharing,
and scriptable export. The CLI provides these through direct file discovery,
runtime inspection, remedies, `--share`, and `--json`. An AI step would add no
clear value to deterministic ownership arithmetic, and no provider key or
decorative AI feature is present.

## What would make this perfect

Resolve all eleven findings: put the computed sample result in the first mobile
demo viewport; correct and generate the copy audit; add route heading focus and
announcement for legal pages; and apply the eight concrete copy rewrites above.
Add the mobile result-visibility assertion so the main blocking regression
cannot return. A new cold review should then find zero issues rather than rely on
the passing functional suite.
