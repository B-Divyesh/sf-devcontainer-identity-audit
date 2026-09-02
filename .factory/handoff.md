# Mount Identity Audit — verification 14 handoff

## Status: FAIL

Candidate `9ff3773b87a1be0d05a34ab9d50306971f8d935c` was independently tested on 2 September 2026 UTC against <https://devcontainer-identity-audit.sociobot.in/>. Production is the candidate build: its footer reports `9ff3773b87a1`, and all 17 publicly served build files match fresh local output byte-for-byte.

Release is blocked by one high-severity false claim. A supported rootless Podman audit with no configured user, numeric image metadata, and live UID/GID maps makes four read-only runtime calls:

```text
info --format json
image inspect local/example:latest --format {{json .Config.User}}
unshare cat /proc/self/uid_map
unshare cat /proc/self/gid_map
```

The landing proof strip, README, and `read-only-safety` registry entry promise **at most three**. Its claim test passes because it supplies an explicit numeric `remoteUser`, skipping image inspection. Correct the maximum or combine calls, and add this image-only rootless case to the tagged claim test before release.

## Verification summary

- All 24 exact commands from `.factory/claims.json`: PASS.
- Cold first-read and one-click 390 px demo: PASS.
- `npm test`: PASS — 11 Rust unit, 23 Rust integration, 30 Vitest, and 80 Playwright checks; eight intentional skips.
- `npm run lint`, `npm run copy:audit:check`, `npm audit --audit-level=low`, and exact `npm run build`: PASS.
- Packed crate and clean consumer install: PASS; one executable, useful help, version `0.1.0`, isolated demo `FAIL`/1 as documented.
- Live Playwright: 36 passed, four intentional skips; keyboard, mobile, 200% text, reduced motion, recovery, service-worker update, and offline reload covered.
- Axe serious/critical findings: zero across all five public routes and both viewports.
- Privacy probe: same-origin GETs only; unique entered value absent from requests; no cookies or Web Storage/IndexedDB entries.
- Security headers, caching, HTTPS redirect, explicit 404, link crawl, and candidate deployment identity: PASS.
- Lighthouse mobile: 99 performance, 100 accessibility, 100 best practices, 100 SEO; LCP 2.0 s, TBT 90 ms, CLS 0.
- Initial assets: 7,246-byte JS, 17,047-byte CSS, no fonts, 216,498-byte hero.

Full evidence and the exact reproduction are in [`.factory/verification-14.md`](verification-14.md). Browser artifacts are in [`.factory/verification-14-evidence/`](verification-14-evidence/).

No product code, deployment, infrastructure, secrets, or out-of-scope resources were changed. Only this verification report, handoff, and local evidence were added.
