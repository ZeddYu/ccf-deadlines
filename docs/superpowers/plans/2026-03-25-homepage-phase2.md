# Homepage Phase 2 Implementation Plan

> **Execution constraint:** Implement directly in the current workspace. Do not create or use a worktree for this plan.

**Goal:** Implement SPEC.md Phase 2 in the current workspace by replacing the checkbox-heavy category area with chip-style controls and finalizing conference row extraction into a clearer card-based presentation without changing homepage behavior.

**Architecture:** Keep `src/components/showtable.rs` as the state owner and orchestration layer for fetch, filter, persistence, sorting, pagination, favorites, timezone handling, and modal control. Phase 2 should only finalize presentational boundaries already visible in the branch (`CategoryChip`, `ConferenceCard`, `TopToolbar`, `ResultsMeta`), tighten their layout contracts, and extend `public/styles.css` so the homepage reads as chip filters + structured cards while preserving existing data flow.

**Tech Stack:** Rust, Leptos CSR, Thaw components, existing CSS in `public/styles.css`

---

## File Map

### Verified current workspace baseline
- `src/components/showtable.rs`
  - Already owns homepage state, filtering, persistence, pagination, favorites, and conference-card call sites.
- `src/components/category_chip.rs`
  - Already exists as a presentation-oriented chip component with mobile/desktop label logic.
- `src/components/conference_card.rs`
  - Already exists as a presentation-oriented conference row/card component with favorite, timeline, calendar, and metadata rendering.
- `src/components/top_toolbar.rs`
  - Already exists as the extracted primary toolbar component from Phase 1.
- `src/components/results_meta.rs`
  - Already exists as the extracted secondary meta row component from Phase 1.
- `src/components/mod.rs`
  - Already exports the current homepage component modules.

### Existing files to modify
- `src/components/showtable.rs`
  - Keep ownership of category selection, search input, CCF/CORE/THCPL filters, favorites persistence, pagination, timezone label, and subscription modal state.
  - Keep `filter_conferences`, pagination semantics, localStorage keys, fetch flow, deadline computation, timeline generation, and calendar URL generation unchanged.
  - Finalize the category-chip section and conference-card call sites, plus add/extend regression tests that guard the Phase 2 structure.
- `src/components/category_chip.rs`
  - Keep this as a presentation-only button component.
  - Finalize its label/selection contract only if needed to match the approved chip UI and keep the mobile-vs-desktop label behavior.
- `src/components/conference_card.rs`
  - Keep this as the single-row presentation boundary.
  - Finalize the card hierarchy so it clearly exposes title/year, favorite toggle, date/location, description, rank tags, category/meta tags, note, website row, countdown, deadline text, timeline, and calendar popover.
  - Preserve the existing helper functions and add targeted tests only for render-adjacent helpers if needed.
- `public/styles.css`
  - Extend the existing homepage token-based styling with Phase 2 chip, utility action, card, metadata, deadline, and responsive layout rules.
  - Reuse current variables and theme tokens; do not introduce a parallel styling system.

### Existing files to inspect during implementation
- `src/components/mod.rs`
  - Confirm the component exports remain correct after any file-level adjustments.
- `src/components/results_meta.rs`
  - Confirm the secondary meta row still fits above the chip section and does not need behavior changes.
- `src/components/top_toolbar.rs`
  - Confirm the primary toolbar remains unchanged functionally and still composes cleanly with the Phase 2 layout.
- `src/pages/home.rs`
  - Verify the page composition remains `Header` + `ShowTable` and that `use_english` continues to be passed into `ShowTable`.
- `SPEC.md`
  - Treat as the source-of-truth acceptance checklist.
- `docs/superpowers/specs/2026-03-25-homepage-phase2-design.md`
  - Treat as the approved design document for boundaries, hierarchy, and verification scope.

### Verification commands
- Focused Rust tests for Phase 2 structure:
  - `cargo test showtable::tests::top_section_order_matches_phase1_layout`
  - `cargo test showtable::tests::phase2_category_chip_section_uses_category_chip_component`
  - `cargo test showtable::tests::phase2_conference_list_uses_conference_card_component`
  - `cargo test conference_card::tests`
  - `cargo test category_chip::tests`
- Full Rust test pass:
  - `cargo test`
- Frontend build verification:
  - `python scripts/merge.py conference --exclude "types.yml" > public/conference/allconf.yml && python scripts/merge.py accept_rates > public/conference/allacc.yml && cp conference/types.yml public/conference/types.yml && trunk build --release`

## Implementation Notes
- Do not use a worktree for this plan.
- Do not change localStorage keys: `use_english`, `types`, `ranks`, `core_ranks`, `thcpl_ranks`, `likes`.
- Do not move state ownership out of `ShowTable`.
- Do not rewrite conference fetching, acceptance-rate fetch flow, timezone normalization, deadline computation, timeline creation, calendar URL generation, or pagination semantics.
- Keep `CategoryChip` and `ConferenceCard` presentation-only; pass existing signals/callbacks from `ShowTable`.
- Preserve current language-switch behavior: desktop chips use localized names, mobile chips use the short `sub` label.
- Preserve card metadata coverage: rank tags, category tag, optional acceptance-rate tag, and optional note.
- Prefer source-based regression guards in Rust tests over adding heavy UI test infrastructure.
- Keep Phase 2 styling to “behavior unchanged + basic polish”; do not broaden into unrelated Phase 3 restyling.

### Task 1: Lock the Phase 2 integration seam in `ShowTable`

**Files:**
- Modify: `src/components/showtable.rs`
- Inspect: `src/components/category_chip.rs:1-120` (exact lines to confirm during implementation)
- Inspect: `src/components/conference_card.rs:1-260` (exact lines to confirm during implementation)
- Test: `src/components/showtable.rs`

- [ ] **Step 1: Add or normalize the Phase 2 source-structure tests**

Add or update a single canonical test module in `src/components/showtable.rs` that asserts all of the following:
- `class="category-chip-section"` exists
- `class="category-chip-grid"` exists
- `showtable.rs` references `<CategoryChip`
- `showtable.rs` references `<ConferenceCard`
- the category-chip section appears above the table container
- the conference list rendering inside the table body still delegates rows through `ConferenceCard`

- [ ] **Step 2: Run the focused `ShowTable` source tests and verify the current baseline**

Run: `cargo test showtable::tests`
Expected: PASS on the current branch after the Phase 2 source guards are normalized, proving the plan is working from the real integration seam rather than stale assumptions.

- [ ] **Step 3: Keep category state helpers local to `ShowTable` and remove any accidental ownership drift**

Inspect the category-related helpers in `src/components/showtable.rs` and make only minimal edits needed so that:
- category state remains a `HashSet<String>` owned by `ShowTable`
- “Select All” and single-chip toggles are both implemented in `ShowTable`
- there is no business logic migration into `category_chip.rs`
- the current filter pipeline still consumes `check_list` exactly once upstream of pagination

- [ ] **Step 4: Add a focused regression for the category selection contract**

Add a small unit/source test in `src/components/showtable.rs` that proves the category helpers still support:
- toggling one category on/off
- computing the “all selected” state from the real category list
- preserving the existing empty-set meaning of “no category filter”

- [ ] **Step 5: Run focused tests again**

Run: `cargo test showtable`
Expected: PASS.

- [ ] **Step 6: Commit the Phase 2 seam prep (optional checkpoint)**

If you want an incremental checkpoint:
```bash
git add src/components/showtable.rs
git commit -m "test: lock homepage phase 2 integration seam"
```

### Task 2: Finalize chip-style category filters without changing filter semantics

**Files:**
- Modify: `src/components/showtable.rs`
- Modify: `src/components/category_chip.rs`
- Modify: `public/styles.css`
- Test: `src/components/showtable.rs`
- Test: `src/components/category_chip.rs`

- [ ] **Step 1: Write the failing source-style test for the chip section contract**

Add a test in `src/components/showtable.rs` that asserts the category section uses the expected Phase 2 class hierarchy and component call sites, including:
- `class="category-chip-section"`
- `class="category-actions-row"`
- `class="category-chip-grid"`
- `<CategoryChip`
- the select-all action rendered as a chip-style action button rather than a checkbox block

- [ ] **Step 2: Run the focused test to verify the starting point**

Run: `cargo test showtable::tests::phase2_category_chip_section_uses_category_chip_component`
Expected: either PASS if the branch already satisfies the contract or FAIL in a narrowly useful way that identifies the remaining integration mismatch.

- [ ] **Step 3: Finalize `CategoryChip` as a pure chip/pill presentation component**

In `src/components/category_chip.rs`, make only the minimal changes needed so the component clearly expresses the Phase 2 contract:
- button-based chip rendering
- selected vs unselected class handling
- stable `aria-pressed`
- current label behavior (`sub` on mobile, localized names on desktop)
- no “select all” semantics and no direct state mutation beyond invoking the provided callback

- [ ] **Step 4: Finalize the category-chip section in `ShowTable`**

Update `src/components/showtable.rs` so the section cleanly renders:
- one chip-style utility action for select-all / clear-all semantics driven by current selection state
- the `For` loop over categories using `<CategoryChip ... />`
- the existing `check_list` toggle callback
- no fallback to the old checkbox-heavy presentation

Do not change `filter_conferences` or localStorage persistence.

- [ ] **Step 5: Extend `public/styles.css` for the Phase 2 chip UI**

Add or refine the minimal CSS needed for:
- chip container spacing
- chip wrap behavior
- selected, hover, and focus states
- utility/action chip styling
- mobile stacking and readable spacing

Keep the rules scoped to the homepage chip area and reuse existing tokens.

- [ ] **Step 6: Add or update chip helper tests**

In `src/components/category_chip.rs`, keep the existing label tests and add only small helper coverage if needed for any new label or class helper introduced during finalization.

- [ ] **Step 7: Run focused chip tests**

Run:
- `cargo test category_chip::tests`
- `cargo test showtable::tests::phase2_category_chip_section_uses_category_chip_component`
Expected: PASS.

- [ ] **Step 8: Commit the chip finalization (optional checkpoint)**

If you want an incremental checkpoint:
```bash
git add src/components/showtable.rs src/components/category_chip.rs public/styles.css
git commit -m "feat: finalize homepage category chip filters"
```

### Task 3: Finalize conference-card presentation while preserving card behavior

**Files:**
- Modify: `src/components/showtable.rs`
- Modify: `src/components/conference_card.rs`
- Modify: `public/styles.css`
- Test: `src/components/showtable.rs`
- Test: `src/components/conference_card.rs`

- [ ] **Step 1: Write the failing source-structure test for conference-card delegation**

Add a test in `src/components/showtable.rs` that asserts:
- the table body conference branch renders `<ConferenceCard`
- the empty state still exists separately from the card rendering path
- the card path remains downstream of filtering and pagination

- [ ] **Step 2: Run the focused delegation test**

Run: `cargo test showtable::tests::phase2_conference_list_uses_conference_card_component`
Expected: either PASS if already wired or FAIL with a clear structural mismatch to fix.

- [ ] **Step 3: Finalize the `ConferenceCard` hierarchy**

In `src/components/conference_card.rs`, make only minimal layout-oriented changes needed so one card clearly exposes:
- title + year + favorite toggle
- date/location row
- full conference description row
- rank tag group
- category/meta tag group with category tag plus optional acceptance-rate tag
- optional note block when present
- website row
- countdown / deadline summary area
- exact deadline text with timezone clarity
- timeline and calendar popover in the right-side deadline area

Do not change favorite persistence, timeline generation, or calendar link generation.

- [ ] **Step 4: Keep `ShowTable` responsible for all prepared card props and callbacks**

Update `src/components/showtable.rs` only as needed to keep `ConferenceCard` call sites clean while preserving:
- rank-selected highlight signals
- favorite toggle callback ownership
- the same filtered and paginated list source
- no new transformation layer between `paginated_list` and the card component

- [ ] **Step 5: Extend `public/styles.css` for the Phase 2 card layout**

Add or refine styles for:
- card spacing and grouping
- title/favorite row
- rank/meta tag groups
- note styling
- website/deadline text rows
- countdown/deadline panel hierarchy
- responsive stacking for mobile

Avoid global table restyling outside the homepage list area.

- [ ] **Step 6: Add or update `conference_card.rs` helper tests**

Keep and extend small helper tests so they verify:
- rank labels still format correctly
- category label still respects language choice
- acceptance-rate text only appears when present
- any new helper extracted for note/meta grouping behaves deterministically

- [ ] **Step 7: Run focused conference-card tests**

Run:
- `cargo test conference_card::tests`
- `cargo test showtable::tests::phase2_conference_list_uses_conference_card_component`
Expected: PASS.

- [ ] **Step 8: Commit the card finalization (optional checkpoint)**

If you want an incremental checkpoint:
```bash
git add src/components/showtable.rs src/components/conference_card.rs public/styles.css
git commit -m "feat: finalize homepage conference cards"
```

### Task 4: Add Phase 2 CSS guards and run full verification before completion

**Files:**
- Modify: `src/components/showtable.rs`
- Modify: `src/components/conference_card.rs` (tests only if needed)
- Modify: `public/styles.css`
- Inspect: `src/components/results_meta.rs`
- Inspect: `src/components/top_toolbar.rs`

- [ ] **Step 1: Add source-based CSS hierarchy guards for Phase 2**

Add or update tests in `src/components/showtable.rs` that assert `public/styles.css` contains the key Phase 2 selectors needed for this layout, such as:
- `.category-chip-section {`
- `.category-chip-grid {`
- `.category-chip-action {`
- `.conference-tag-groups {`
- `.conference-rank-tags {`
- `.conference-meta-tags {`
- `.conference-note {`
- `.countdown-container {`

Use the smallest stable set of selectors that captures the Phase 2 contract.

- [ ] **Step 2: Run the focused style guard tests**

Run: `cargo test showtable::tests::phase2_css_hierarchy_is_defined`
Expected: PASS after the stylesheet and source guards agree.

- [ ] **Step 3: Run the full Rust test suite**

Run: `cargo test`
Expected: PASS.

- [ ] **Step 4: Run the full frontend build flow used by CI**

Run:
`python scripts/merge.py conference --exclude "types.yml" > public/conference/allconf.yml && python scripts/merge.py accept_rates > public/conference/allacc.yml && cp conference/types.yml public/conference/types.yml && trunk build --release`

Expected: successful asset generation and successful frontend build.

- [ ] **Step 5: Perform manual browser verification before any completion claim**

Manually verify all of the following on desktop and mobile-width layouts:
- search still works
- CCF / CORE / THCPL filters still work
- category filters still work
- favorites still persist
- subscription modal still opens
- timeline still renders
- calendar popover still works
- pagination still works
- language switch still works
- dark/light theme still works
- chip section and conference cards have no obvious overflow or broken alignment

Record any failures and fix them before marking the work complete.

- [ ] **Step 6: Commit the finished Phase 2 work**

When all verification passes:
```bash
git add src/components/showtable.rs src/components/category_chip.rs src/components/conference_card.rs public/styles.css
git commit -m "feat: implement homepage phase 2 chip and card layout"
```
