# Changelog

## Unreleased

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
