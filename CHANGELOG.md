# Changelog

## Unreleased — 2026-09-02

- Compose Podman `keep-id`'s inner namespace with the live rootless UID/GID
  maps, including container IDs below, equal to, and above the kept user.
- Align the browser calculator with Podman's two-layer `keep-id` mapping and
  verify parity against an installed packed CLI using identical ranges.
- Correct the documented maximum to four read-only runtime calls for an
  image-only rootless Podman audit, with an exact call-recording regression.
- Return `UNKNOWN` when Docker reports daemon `userns-remap` instead of
  assuming container IDs map directly to the host.
- Give read-only mounts a configuration-specific recovery step without
  suggesting unrelated identity or host permission changes.

## Unreleased - 2026-08-30

- Corrected browser `keep-id` mapping for remote users that differ from the
  host caller, with a packed-CLI parity regression.
- Made the safe-example action replace every input after validation errors.
- Enforced 44 px link and button targets across every public route.
- Added isolated CLI and browser sample modes with claim-tagged regression tests.
- Rejected Linux's reserved UID/GID value `4294967295` across CLI and browser paths.
- Added complete route metadata, a designed 404 response, legal-page shells, and build identity.

## Unreleased

- Keep an explicit Dev Container `remoteUser` authoritative when the selected
  Compose service declares a different `user`.
- Reflow the Docker/Podman comparison into labelled rows at 390 px instead of
  requiring a 700 px horizontal scroll.
- Return `UNKNOWN` for UID-only users from config, CLI overrides, Compose, or
  image metadata instead of inventing a same-number primary GID.
- Do not trust a possibly stale Compose `image` tag when the service is backed
  by current `build` inputs and no explicit numeric user is available.
- Reject browser-demo rootless mappings that overflow Linux IDs, and keep the
  skip link hidden before focus when reduced motion is requested.
- Return `UNKNOWN` instead of assuming root when build-backed configuration has
  no safely resolvable numeric user.
- Redact configured paths from every string field in share-safe error reports.
- Reflow the documentation at 200% mobile text size and enforce 44 px targets.

All notable changes to Mount Identity Audit are documented here. The project
uses semantic versioning.

## [0.1.0] — 2026-08-28

- Initial read-only Docker and rootless Podman mount identity preflight.
- JSONC and Compose metadata discovery.
- Human, JSON, quiet, and share-safe reports.
- Static documentation site with an in-browser audit explainer.
