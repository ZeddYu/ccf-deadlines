# Homepage Phase 3 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.
>
> **Execution constraint:** Implement directly in the current workspace. Do not create or use a worktree for this plan.

**Goal:** Implement SPEC.md Phase 3 polish and responsive refinement so the redesigned homepage feels cleaner, calmer, and more coherent on desktop and mobile without changing homepage behavior or core data flow.

**Architecture:** Keep `src/components/showtable.rs` as the homepage state owner and preserve the Phase 1/2 component boundaries already established in the workspace: `Header`, `TopToolbar`, `ResultsMeta`, `CategoryChip`, and `ConferenceCard`. Phase 3 should be primarily a CSS-and-view-polish pass, with only small Rust changes where necessary to expose better structure for styling or to add a compact homepage countdown mode, while keeping filtering, persistence, timeline, calendar popover, and pagination logic intact.

**Tech Stack:** Rust, Leptos CSR, Thaw components, existing homepage CSS in `public/styles.css`, existing YAML merge pipeline, Trunk build

---

## File Structure

### Primary files to modify
- `public/styles.css`
  - Main Phase 3 work file.
  - Refine header, toolbar, chip section, conference card, deadline panel, and responsive behavior.
  - Reduce dark-mode border stacking and rely more on layered backgrounds/surfaces.
- `src/components/countdown.rs`
  - Add a compact homepage display mode if needed so conference cards emphasize day/hour instead of noisy seconds.
  - Keep urgency thresholds and countdown tick behavior unchanged.
- `src/components/conference_card.rs`
  - Make only minimal view changes needed to consume the compact countdown mode and preserve the final card hierarchy for styling.
  - Keep favorite toggle, website link, timeline, calendar popover, deadline text, and helper behavior intact.
- `src/components/top_toolbar.rs`
  - Make only small structural tweaks if CSS needs a stronger search entry point or better mobile stacking/focus affordances.
- `src/components/results_meta.rs`
  - Make only small structural tweaks if needed to reserve room for future toggles and improve mobile wrapping/alignment.
- `src/components/header.rs`
  - Make only small markup or copy-preserving structure tweaks if needed for cleaner row balance, announcement strip spacing, or de-emphasized secondary links.
- `src/components/showtable.rs`
  - Keep logic ownership unchanged.
  - Add/adjust focused regression tests for Phase 3 structural/style contracts.

### Existing files to inspect during implementation
- `src/pages/home.rs`
  - Confirm page composition remains `Header` + `ShowTable`.
- `src/components/category_chip.rs`
  - Confirm chip markup still supports the intended Phase 3 styling without behavior changes.
- `src/components/mod.rs`
  - Confirm module exports stay correct after any supporting edits.
- `SPEC.md`
  - Source-of-truth acceptance checklist for Phase 3.
- `docs/superpowers/plans/2026-03-25-homepage-phase2.md`
  - Reference for what was intentionally preserved from Phase 2.

### Files not to change unless absolutely required
- `src/components/conf.rs`
- conference YAML / accept-rates data files
- merge scripts / fetch pipeline
- localStorage keys and persistence schema
- calendar URL generation logic
- timeline data generation logic
- timezone computation logic

## Implementation constraints to preserve
- Preserve localStorage compatibility for:
  - `use_english`
  - `types`
  - `ranks`
  - `core_ranks`
  - `thcpl_ranks`
  - `likes`
- Do not move business logic out of `ShowTable`.
- Do not rewrite conference filtering, sorting, favorites, pagination, or subscription behavior.
- Keep `CategoryChip` and `ConferenceCard` presentation-oriented.
- Do not alter calendar popover desktop-hover/mobile-click semantics.
- Do not alter timeline generation or deadline status rules.
- Prefer focused Rust/source guards plus real build/manual verification over inventing new test infrastructure.

## Verification commands
- Focused Rust tests:
  - `cargo test countdown::tests`
  - `cargo test conference_card::tests`
  - `cargo test showtable::tests`
  - `cargo test header::tests`
- Full Rust tests:
  - `cargo test`
- Full build flow used by CI:
  - `python scripts/merge.py conference --exclude "types.yml" > public/conference/allconf.yml && python scripts/merge.py accept_rates > public/conference/allacc.yml && cp conference/types.yml public/conference/types.yml && trunk build --release`
- Manual/browser verification targets:
  - desktop width homepage
  - mobile-width homepage
  - dark mode and light mode

### Task 1: Lock the Phase 3 surface and countdown contracts

**Files:**
- Modify: `src/components/showtable.rs`
- Modify: `src/components/countdown.rs`
- Modify: `src/components/conference_card.rs`
- Test: `src/components/showtable.rs`
- Test: `src/components/countdown.rs`
- Test: `src/components/conference_card.rs`

- [ ] **Step 1: Add failing source/behavior tests for the Phase 3 contract**

Add or update focused tests so they guard the smallest stable contract needed for Phase 3:
- homepage still composes the card list through `ConferenceCard`
- the card still contains the countdown area, deadline text, website row, timeline, and calendar popover
- `CountDown` can support a compact homepage display mode without changing urgency thresholds
- Phase 3 stylesheet/source guards exist for the key layout selectors rather than relying only on eyeballing

- [ ] **Step 2: Run the focused tests to capture the baseline**

Run:
- `cargo test conference_card::tests`
- `cargo test showtable::tests`

Expected: PASS for existing guards, with any new Phase 3-specific tests failing narrowly until the implementation is added.

- [ ] **Step 3: Add a compact homepage countdown mode**

In `src/components/countdown.rs`, add the smallest API change needed for calmer conference-card countdowns, for example an optional compact mode prop that:
- keeps the same ticking interval
- keeps the same urgency classification thresholds
- displays day/hour-first text on cards
- de-emphasizes seconds when days or hours are present
- keeps the existing default output intact for any non-homepage callers

Avoid introducing a second countdown component.

- [ ] **Step 4: Update `ConferenceCard` to consume the compact countdown mode**

In `src/components/conference_card.rs`, switch the homepage card countdown rendering to the compact mode while preserving:
- the same `remain` source
- calendar popover placement in the deadline area
- deadline exact-text line and timezone clarity
- timeline visibility rules for `FIN`/`TBD`

- [ ] **Step 5: Add focused helper tests for compact countdown formatting**

Add small deterministic tests in `src/components/countdown.rs` for the formatter/helper introduced in Step 3, covering at least:
- `days + hours`
- `hours + minutes`
- sub-hour fallback
- unchanged urgency thresholds

- [ ] **Step 6: Run focused tests again**

Run:
- `cargo test countdown::tests`
- `cargo test conference_card::tests`

Expected: PASS.

- [ ] **Step 7: Commit the countdown contract checkpoint (optional)**

```bash
git add src/components/countdown.rs src/components/conference_card.rs src/components/showtable.rs
git commit -m "feat: add compact homepage countdown display"
```

### Task 2: Polish the header and toolbar hierarchy without changing behavior

**Files:**
- Modify: `src/components/header.rs`
- Modify: `src/components/top_toolbar.rs`
- Modify: `src/components/results_meta.rs`
- Modify: `public/styles.css`
- Test: `src/components/header.rs`
- Test: `src/components/showtable.rs`

- [ ] **Step 1: Add failing structural/style guards for top-of-page polish**

Add or update tests so they assert the homepage still exposes the intended top-of-page layers while allowing visual refinement:
- header brand row
- header subtitle row
- header announcement row
- primary toolbar
- secondary meta row
- category chip section
- search input remains in the primary toolbar and above the chip section

- [ ] **Step 2: Run the focused top-section tests**

Run:
- `cargo test header::tests`
- `cargo test showtable::tests::top_section_order_matches_phase1_layout`

Expected: PASS for preserved ordering, with any new Phase 3 styling guards failing only if the CSS contract is missing.

- [ ] **Step 3: Strengthen search as the primary visual entry point**

In `public/styles.css` and only minimally in `src/components/top_toolbar.rs` if needed:
- increase visual weight of the search field
- improve focus ring clarity
- ensure better width behavior on desktop
- ensure stacked layout/readable spacing on mobile
- keep existing search behavior and current input wiring intact

If necessary, add a clear-button slot only if it can be done with very small view changes and without altering search semantics; otherwise keep this strictly visual.

- [ ] **Step 4: Refine header row emphasis and secondary-link de-emphasis**

Adjust `header.rs` and `public/styles.css` only as needed so:
- the brand row stays dominant
- the subtitle row reads as supporting information
- portal/wechat/disclaimer links are visibly secondary
- the announcement row reads as a standalone strip
- theme/GitHub/language controls stay aligned and responsive

Preserve existing links, actions, and language-switch behavior.

- [ ] **Step 5: Refine the secondary meta row layout**

In `src/components/results_meta.rs` and `public/styles.css`, make only small layout changes needed so the row cleanly supports:
- timezone text
- result count
- clear filters action
- spare room/alignment for future controls
- readable wrapping on narrow screens

Do not add new filter behavior.

- [ ] **Step 6: Run focused tests after top-section polish**

Run:
- `cargo test header::tests`
- `cargo test showtable::tests`

Expected: PASS.

- [ ] **Step 7: Commit the top-section polish checkpoint (optional)**

```bash
git add src/components/header.rs src/components/top_toolbar.rs src/components/results_meta.rs public/styles.css src/components/showtable.rs
git commit -m "feat: polish homepage header and toolbar hierarchy"
```

### Task 3: Refine chip and card surfaces for cleaner light/dark presentation

**Files:**
- Modify: `public/styles.css`
- Modify: `src/components/conference_card.rs`
- Modify: `src/components/top_toolbar.rs`
- Modify: `src/components/results_meta.rs`
- Modify: `src/components/showtable.rs` (tests only if needed for empty-state guards)
- Inspect: `src/components/category_chip.rs`
- Test: `src/components/showtable.rs`
- Test: `src/components/conference_card.rs`

- [ ] **Step 1: Add failing CSS hierarchy guards for Phase 3 polish selectors**

Extend the existing source-based CSS tests in `src/components/showtable.rs` so they cover the stable selectors required by the polished design, for example:
- `.hero-header {`
- `.primary-toolbar {`
- `.secondary-meta-row {`
- `.category-chip-section {`
- `.conference-card-shell {`
- `.conference-card-main {`
- `.conference-deadline-panel {`
- `.table-no-results {` or the chosen empty-state selector
- any new compact-countdown or responsive helper selectors introduced in this phase

Keep the selector list minimal and stable.

- [ ] **Step 2: Run the focused CSS guard test**

Run: `cargo test showtable::tests`
Expected: the new guard fails until the stylesheet contains the intended Phase 3 selectors.

- [ ] **Step 3: Reduce border heaviness and improve surface layering in `public/styles.css`**

Refine the homepage styles so:
- dark mode relies more on surface/background contrast than stacked borders
- cards and top controls feel calmer and more layered
- hover/focus/selected states remain clear but not harsh
- chips, toolbar controls, and cards share a coherent visual language
- excessive shadows are avoided

Prefer token updates and homepage-scoped selector refinement over one-off color overrides.

- [ ] **Step 4: Refine conference card hierarchy and spacing**

In `public/styles.css` and only minimally in `src/components/conference_card.rs` if needed:
- make the title row and favorite alignment cleaner
- visually separate rank tags from category/meta tags
- subordinate the timeline to the countdown block
- improve spacing for note and website rows
- ensure the right-side deadline panel reads as a distinct but integrated surface

Do not change favorite behavior or the underlying data displayed.

- [ ] **Step 5: Style the homepage empty state explicitly**

Using the existing no-results rendering path in `src/components/showtable.rs`, add or refine the minimal markup/test hook and CSS needed so the empty state:
- is visually coherent with the polished homepage surfaces
- remains readable in light and dark mode
- has comfortable spacing on desktop and mobile
- does not introduce any new behavior or alternate data path

Do not replace the current empty-state logic; only polish its presentation.

- [ ] **Step 6: Refine toolbar/chip interaction states for consistency**

Polish the CSS for:
- chip hover/selected/focus states
- search hover/focus states
- clear-filters button states
- theme button and other top-level action states

The goal is consistency, not new functionality.

- [ ] **Step 7: Run focused tests after style refinement**

Run:
- `cargo test showtable::tests`
- `cargo test conference_card::tests`

Expected: PASS.

- [ ] **Step 8: Commit the surface-polish checkpoint (optional)**

```bash
git add public/styles.css src/components/conference_card.rs src/components/top_toolbar.rs src/components/results_meta.rs src/components/showtable.rs
git commit -m "feat: refine homepage surfaces and card hierarchy"
```

### Task 4: Finish responsive refinement and full verification

**Files:**
- Modify: `public/styles.css`
- Modify: `src/components/showtable.rs` (tests only if needed)
- Inspect: `src/components/header.rs`
- Inspect: `src/components/top_toolbar.rs`
- Inspect: `src/components/results_meta.rs`
- Inspect: `src/components/conference_card.rs`

- [ ] **Step 1: Add final responsive/layout guard tests if needed**

If implementation introduces new stable helper selectors or wrapper classes for responsive behavior, add the smallest source-based tests needed to keep them from regressing. Do not add brittle tests for exact CSS property values.

- [ ] **Step 2: Complete mobile and desktop responsive CSS refinement**

In `public/styles.css`, ensure:
- mobile layout stacks cleanly without horizontal overflow
- desktop toolbar alignment remains consistent
- conference cards preserve a strong two-column feel on larger screens
- the deadline panel collapses cleanly beneath metadata on narrow screens
- header actions wrap without awkward collisions

- [ ] **Step 3: Run the full Rust test suite**

Run: `cargo test`
Expected: PASS.

- [ ] **Step 4: Run the full frontend build flow used by CI**

Run:
`python scripts/merge.py conference --exclude "types.yml" > public/conference/allconf.yml && python scripts/merge.py accept_rates > public/conference/allacc.yml && cp conference/types.yml public/conference/types.yml && trunk build --release`

Expected: successful merge output and successful `trunk build --release`.

- [ ] **Step 5: Perform manual browser verification before claiming completion**

Verify on desktop and mobile-width layouts:
- search works
- CCF / CORE / THCPL filters work
- category filters work
- favorites persist
- subscription modal opens
- timeline renders
- calendar popover works
- pagination works
- language switch works
- dark/light theme works
- search is visually the strongest entry point
- dark mode feels less border-heavy
- mobile layout does not overflow or collapse awkwardly
- desktop alignment is consistent
- selected/hover/focus states are visually coherent

Record any failures and fix them before completion.

- [ ] **Step 6: Commit the finished Phase 3 work**

```bash
git add public/styles.css src/components/countdown.rs src/components/conference_card.rs src/components/top_toolbar.rs src/components/results_meta.rs src/components/header.rs src/components/showtable.rs
git commit -m "feat: polish homepage phase 3 responsive UI"
```
