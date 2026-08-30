# Mount Identity Audit — visual thesis

## Direction

**Dithered identity ledger.** The product borrows from two places that belong to
its job: Unix permission tables and two-colour technical print manuals. Coarse
halftone fields make identity boundaries visible—host on one plate, container on
the other—while hard rules and registration marks suggest an audit rather than a
launcher. The result should feel diagnostic, local, and calmly exact. It must not
look like a glossy cloud dashboard or a generic developer gradient.

This is intentionally a **single light treatment**. The warm paper ground is an
essential part of the print metaphor, is painted explicitly, and avoids an
unreadable faux-paper dark mode. The browser advertises `color-scheme: light`.

## Palette

| Token | Value | Role |
| --- | --- | --- |
| Paper | `#F3EBDD` | page background, like uncoated stock |
| Sheet | `#FFFDF7` | raised working surfaces |
| Ink | `#171A17` | primary type and borders |
| Muted ink | `#55584F` | secondary copy (7:1+ on paper) |
| Podman green | `#17624A` | primary action and pass state |
| Signal orange | `#B43C24` | fail state and registration accent |
| Proof yellow | `#E6C84F` | warnings and selected marks, always with ink text |
| Rule | `#A9A18F` | non-text dividers only |

Green and orange are never the only status signal: every result includes a word,
symbol, and explanation. Primary buttons use green with `#FFFDF7` text (contrast
above 7:1); body text uses ink on paper (above 14:1).

## Type

- **Display:** `Arial Narrow`, `Aptos Narrow`, `Roboto Condensed`, system
  sans-serif. Condensed caps evoke equipment labels without downloading a font.
- **Data and body:** `ui-monospace`, `SFMono-Regular`, `Cascadia Code`, `Roboto
  Mono`, `Liberation Mono`, monospace. Permission bits, IDs, and commands align
  naturally. Body copy stays at 16–18 px with 1.55 leading.
- Scale: 14, 16, 20, 28, 44, and a responsive 72 px display size. Numeric tables
  use tabular figures. Reading measure tops out around 68 characters.

No font files or third-party requests are required; the pairing is fully local.

## Spacing and shape

An 8 px base rhythm with 4 px for tight label gaps: `4 / 8 / 16 / 24 / 32 / 48 /
72 / 96`. Content is constrained to 1180 px. Borders are 2 px ink rules; corners
are deliberately small (0–4 px), like cut paper rather than software cards.
Sections group by open space before adding a box. Touch targets are at least
44×44 px. At 390 px, the audit table becomes labelled rows and the illustration
becomes a narrow crop; no essential content is removed.

## Interaction grammar

- Actions depress 2 px like a letterpress key and immediately update a polite
  status region.
- The demo follows a three-part physical sequence: configuration slip → identity
  comparison → verdict stamp. Keyboard focus is a 3 px green ring with a paper
  offset.
- Status changes use a brief stamp-in (180 ms opacity + scale) and rows reveal in
  order over 240 ms. Nothing loops.
- Under `prefers-reduced-motion: reduce`, transforms and smooth scrolling are
  removed; state changes are instant opacity swaps. The halftone texture is
  static, with no flicker.

## Original asset plan and provenance

`site/public/mount-ledger.webp` is the primary raster illustration: a text-free editorial
cutaway of a host folder and container workspace aligned through UID/GID
registration plates. It clarifies the mapping concept, not merely the mood. The
source PNG is not shipped.

- Generator: factory `gen-image.sh`, deployment metadata retained in the adjacent
  `mount-ledger.source.json` file.
- Prompt: “Dithered two-colour technical editorial illustration for a developer
  tool landing page. A cutaway desktop workstation on the left and a rootless
  container/workspace box on the right, connected by one physical bind-mount
  ribbon passing through aligned circular identity tokens. Show paired brass ID
  tags and permission punch holes, with one clean registration alignment. Warm
  uncoated paper, near-black ink, deep forest green and restrained signal orange,
  coarse halftone dots, screenprint misregistration, 1960s Unix manual diagram,
  flat orthographic composition, generous negative space. No people, no logos,
  no brands, no gradients, no readable text, no letters, no numbers, no
  watermark.”
- License: original work generated for this MIT-licensed project, 2026-08-28.
- CSS halftone fields and registration marks are hand-authored, deterministic
  gradients used as layout texture; they are not stock assets.

Derived release assets keep the same provenance. `social-card.webp` is a
deterministic 1200×630 center crop of `mount-ledger.webp` for link previews.
`apple-touch-icon.png` is a deterministic square crop of that same art.
`demo-terminal.svg` is a hand-authored, text-accessible recording frame based on
the real `mount-identity-audit --demo` output. No stock or third-party asset was
introduced.

## Accessibility and performance constraints

The hero has a concise meaningful alt and explicit dimensions; the WebP must be
≤300 KB. Texture never sits behind long text without a solid paper backing.
Focus, selected, pass, warning, and fail states all retain text labels. Initial JS
stays under 200 KB and CSS under 50 KB. Only the hero is eager/high priority;
below-fold visuals are CSS or lazy.
