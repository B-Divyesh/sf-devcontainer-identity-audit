# Changelog

## Unreleased - 2026-08-30

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
