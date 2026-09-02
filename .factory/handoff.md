# Mount Identity Audit — independent verification 15 handoff

## Status: FAIL

Candidate `9c814a9cb8e4bb69f48bb888adeb8971a620752e` was independently tested on 2 September 2026 UTC at <https://devcontainer-identity-audit.sociobot.in/>. The deployment identifies that commit and all 17 served build files match a fresh local build byte-for-byte.

The release is blocked by one high-severity core defect: for rootless Podman `--userns=keep-id`, the CLI applies the outer `podman unshare` UID/GID map directly to container IDs instead of first applying Podman's keep-id mapping layer. A freshly packed and installed CLI reported `PASS`/0 for container `0:0` against a `0755` workspace owned by host `1000:1000`. With caller `1000:1000` and subordinate ranges beginning at `100000`, the correct mapping is host `100000:100000`, so access is `FAIL`/1. The live browser calculator produces that correct failure for the same values, directly falsifying the registered `browser-parity` claim.

Full reproduction, map composition, and evidence are in `.factory/verification-15.md`. No product code or infrastructure was changed.

## Verification summary

- Mandatory first read: PASS — job, audience, first action, and one-click sample are clear in the cold first viewport.
- Registered claim commands: 24/24 PASS individually, but the independent keep-id case disproves `browser-parity`; its fixture does not cover the mapping hole.
- `npm ci`, `npm test`, `npm run lint`, `npm run copy:audit:check`, `npm audit --audit-level=low`, and exact `npm run build`: PASS.
- Packed clean-consumer install: PASS — one executable; help/version/demo work; demo returns documented `FAIL`/1.
- Live Playwright: 36 passed, four intended skips across desktop and 390 px mobile.
- Axe: zero serious or critical findings across all five public routes at both viewports.
- Privacy: only same-origin GETs; no action request, cookies, Web Storage, IndexedDB, console/page errors, or failed requests.
- Offline/service-worker update, keyboard flow, focus, reduced motion, invalid-input recovery, 200% reflow, 44 px targets, headers, caching, links, and designed 404: PASS.
- Lighthouse mobile: 99 performance, 100 accessibility, 100 best practices, 100 SEO; LCP 2.0 s, TBT 50 ms, CLS 0.
- Assets: 7,246 bytes JS, 17,047 bytes CSS, no fonts, 216,498-byte hero.

## Required next step

Compose the `keep-id` inner UID/GID map with the live outer maps for every remote ID. Add packed-CLI and browser parity cases below, equal to, and above the kept user, including container root, with identical subordinate-range inputs. Re-run every claim and the full independent verification before release.
