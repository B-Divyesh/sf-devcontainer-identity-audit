# Mount Identity Audit — polish 4 handoff

## Status: PASS

Repair commit: `b01b201` (`fix: split copy audit source sentences`).
Deployment: Static Web Apps deployment `1120a0a0-d53e-4f3d-95b1-0b974f5ff462` to
<https://devcontainer-identity-audit.sociobot.in/> on 2 September 2026 UTC.

## What changed

The copy-audit generator now sends quoted UI messages in `site/src/main.ts` and
`site/src/audit.ts` through `splitSentences()`. Its inventory records each
reader-visible sentence separately. Regression coverage locks all six rows from
the three multi-sentence messages. This repairs F-4-1 without changing product
copy or runtime behaviour.

The catalog description is a verb-first, 75-character sentence:
`Check workspace mount access before Dev Container or rootless Podman startup.`

## Exact verification evidence

- Fresh clone: `/tmp/mia-polish4-clean-u5zYbI/repo`, cloned from `b01b201`.
  `npm ci` completed with no vulnerabilities.
- Every one of the 22 exact commands in `.factory/claims.json` passed
  independently from that clone: `cli-demo`, `browser-demo`,
  `permission-verdicts`, `read-only-safety`, `config-support`,
  `compose-user-precedence`, `share-redaction`, `report-contract`,
  `runtime-mapping`, `conservative-identities`, `browser-private`,
  `cli-private`, `offline-reload`, `browser-parity`, `mit-license`,
  `browser-report-details`, `config-discovery`, `runtime-optional`,
  `report-limits`, `compose-build-image`, `install-binary`, and
  `build-artifacts`.
- The same clone passed `npm test` (10 Rust unit tests, 21 Rust integration
  tests, 27 Vitest tests, and 78 Playwright passes; six intentional
  project-specific skips), `npm run lint`, `npm run build`,
  `npm run copy:audit:check`, and `cargo package --locked --allow-dirty`.
  The package is `target/package/mount-identity-audit-0.1.0.crate`.
- The deploy build contains `target/release/mount-identity-audit` and
  `dist/site`. Vite reports 2.99 KB gzip JavaScript and 4.30 KB gzip CSS,
  below the static first-load budgets.
- Live cold URL verification passed at
  <https://devcontainer-identity-audit.sociobot.in/>: 799 ms navigation,
  no console errors, title `Mount Identity Audit — Check mount permissions`,
  `lang=en`, one `h1`, one `main`, no missing image alternatives, and no
  unnamed buttons. The report is
  [`evidence/polish-4-live/verify.json`](evidence/polish-4-live/verify.json).
- Live Playwright route suite: 36 passed and four intentional desktop-only
  skips across Home, Demo, Privacy, Terms, and 404. It includes keyboard,
  focus/history announcements, 390 px demo viewport, 200% reflow, privacy,
  offline reload, metadata, and Axe serious/critical checks.
- `npx @axe-core/cli` passed on all five live routes. The live hero response
  has `Cache-Control: public, max-age=31536000, immutable`; an unknown route
  returns HTTP 404.
- Live Lighthouse (mobile) scored Performance 99, Accessibility 100, Best
  Practices 100, and SEO 100 (FCP 1.0 s, LCP 2.0 s, TBT 20 ms, CLS 0).
- Visual checks: [desktop first screen](evidence/polish-4-live-desktop-home.png)
  and [390×844 query demo](evidence/polish-4-live-mobile-demo.png). The demo
  screenshot shows `FAIL`, `100999:100999`, and `read · no write · traverse`
  before the editable form.

## Privacy and demo checks

The fresh-context browser claims confirmed only same-origin static requests,
no cookies or browser storage, an offline reload after service-worker
activation, a one-click `?demo=1#demo` path, persistent reset/exit controls,
and a CLI `--demo` that works from an isolated temporary copy.

## Known gaps and next steps

None. The crate is ready for the factory-owned registry publishing action:
`cargo package --locked`.
