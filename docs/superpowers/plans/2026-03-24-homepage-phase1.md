# Homepage Phase 1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement SPEC.md Phase 1 by restructuring the homepage header and top controls without changing the existing data flow, filtering semantics, localStorage keys, or conference list business logic.

**Architecture:** Keep state ownership in `src/components/showtable.rs` and keep `src/pages/home.rs` as the composition point for `Header` plus `ShowTable`. The current branch already has the three-row header, the language switch in `Header`, and the top-area class structure in `ShowTable`; Phase 1 work should now normalize tests, extract only small presentational components from the existing top area, and preserve conference fetching, filtering, pagination, favorites, subscription modal, timeline, and calendar behavior.

**Tech Stack:** Rust, Leptos CSR, Thaw components, existing CSS in `public/styles.css`

---

## File Map

### Existing files to modify
- `src/components/header.rs`
  - Keep latest-commit fetch behavior.
  - Preserve the existing three-layer header layout: brand/actions row, subtitle/action-links row, announcement row.
  - Keep the existing language switch in `Header` and only adjust action ordering/copy if needed to match SPEC.
- `src/components/showtable.rs`
  - Keep all current signals, effects, localStorage persistence, filtering, fetch, sorting, pagination, favorites, subscription modal, and top-level list orchestration logic.
  - Keep conference row rendering delegated to `src/components/conference_card.rs`.
  - Preserve the existing top-area structure (`primary-toolbar`, `secondary-meta-row`, `category-chip-section`) while refactoring it into smaller presentational units.
  - Pass existing state/callbacks into any extracted Phase 1 toolbar/meta components.
- `src/components/mod.rs`
  - Export any newly added Phase 1 components.
- `public/styles.css`
  - Adjust the already-present layout/styling for the three-row header, primary toolbar, meta row, and category chip section.
  - Preserve existing theme token usage and avoid introducing a new styling system.

### New files to create
- `src/components/top_toolbar.rs`
  - Small presentational component for the primary toolbar.
  - Render the large search input, desktop/mobile rank filter controls, and Subscribe button using callbacks/signals supplied by `ShowTable`.
- `src/components/results_meta.rs`
  - Small presentational component for the secondary meta row.
  - Render timezone text, result count, and clear-filters action while keeping logic in `ShowTable`.

### Existing files to inspect during implementation
- `src/pages/home.rs`
  - Verify `Header` and `ShowTable` composition still matches the intended Phase 1 ownership split and that `use_english` continues to be passed to both components.
- `src/components/checkbox_button.rs`
  - Reuse existing rank dropdown behavior and props.
- `src/components/category_chip.rs`
  - Keep the current category chip component/rendering unchanged unless a tiny Phase 1 prop adjustment is truly required.
- `src/components/conference_card.rs`
  - Keep the existing extracted conference row rendering untouched functionally in Phase 1 except for any compile-fix caused by prop plumbing.

### Verification commands
- Fast Rust test pass for source-order/layout guards:
  - `cargo test showtable::tests::top_section_order_matches_phase1_layout`
- Focused helper/Phase 1 behavior checks:
  - `cargo test showtable::tests`
  - `cargo test header::tests`
- Full Rust test pass:
  - `cargo test`
- Frontend build verification:
  - `python scripts/merge.py conference --exclude "types.yml" > public/conference/allconf.yml && python scripts/merge.py accept_rates > public/conference/allacc.yml && cp conference/types.yml public/conference/types.yml && trunk build --release`

## Implementation Notes
- Do not change localStorage keys: `use_english`, `types`, `ranks`, `core_ranks`, `thcpl_ranks`, `likes`.
- Do not move business logic out of `ShowTable` in Phase 1; only move view structure.
- The current branch already contains the `primary-toolbar`, `secondary-meta-row`, and `category-chip-section` structure in `showtable.rs`; treat those classes as the baseline contract to preserve while extracting components.
- The current branch already contains the three header rows plus the language switch in `header.rs`; treat Phase 1 header work as normalization/reordering rather than introducing that structure from scratch.
- Keep existing mobile filter behavior, but relocate it visually only if needed inside the extracted primary toolbar.
- Add result count and clear-filters using existing signals in `ShowTable`; do not invent new persistence rules.
- Prefer minimal new props and callbacks over new shared abstractions.
- Normalize the duplicate `top_section_order_matches_phase1_layout` coverage in `showtable.rs` into one canonical test module rather than preserving duplicate test blocks.

### Task 1: Stabilize the current Phase 1 baseline and define the extraction seam

**Files:**
- Modify: `src/components/showtable.rs`
- Inspect: `src/components/checkbox_button.rs:1-220` (exact lines to confirm during implementation)
- Inspect: `src/components/category_chip.rs:1-220` (exact lines to confirm during implementation)
- Test: `src/components/showtable.rs`

- [ ] **Step 1: Add or normalize the top-area contract test**

Add/normalize a single test module in `src/components/showtable.rs` that asserts all of the following source-order invariants:
- `class="primary-toolbar"` exists
- `class="secondary-meta-row"` exists
- `class="category-chip-section"` exists
- primary toolbar appears before secondary meta row
- secondary meta row appears before category chip section

If there are duplicate copies of this test module, replace them with one canonical module.

- [ ] **Step 2: Run the focused test to verify the baseline**

Run: `cargo test showtable::tests::top_section_order_matches_phase1_layout`
Expected: PASS on the current branch after duplicate test coverage is normalized, confirming the current top-area ordering guard still works before refactoring.

- [ ] **Step 3: Add clear helper functions in `ShowTable` for Phase 1-only view state**

Implement minimal helpers in `src/components/showtable.rs` for:
- computing whether any filters/search are active
- clearing search + category + CCF + CORE + THCPL filters in one action
- computing current filtered result count from the same filtered list used for pagination

Keep these helpers local to `showtable.rs`; do not create a utility module.

- [ ] **Step 4: Add a unit test for the clear-filters helper**

Add a focused test that initializes non-empty search/category/rank selections and verifies the clear helper resets them all to the empty/default state without touching unrelated state like likes or pagination.

- [ ] **Step 5: Run all `showtable` tests**

Run: `cargo test showtable`
Expected: PASS, proving the prep refactor did not break the local module.

- [ ] **Step 6: Commit the baseline prep (optional checkpoint)**

If you want an incremental checkpoint:
```bash
git add src/components/showtable.rs
git commit -m "refactor: prepare homepage phase 1 toolbar state"
```

### Task 2: Extract the primary toolbar presentation from `ShowTable`

**Files:**
- Create: `src/components/top_toolbar.rs`
- Modify: `src/components/showtable.rs`
- Modify: `src/components/mod.rs`
- Test: `src/components/showtable.rs`

- [ ] **Step 1: Write the failing source-structure test for extracted toolbar usage**

Add a test in `src/components/showtable.rs` that asserts `showtable.rs` now references `<TopToolbar` and still contains `class="category-chip-section"` after the extracted toolbar call site.

- [ ] **Step 2: Run the focused test to verify it fails before extraction**

Run: `cargo test showtable::tests::top_toolbar_component_is_used`
Expected: FAIL because `TopToolbar` does not exist yet.

- [ ] **Step 3: Create `src/components/top_toolbar.rs` with a minimal presentational component**

Implement a small component that receives the existing search signal, mobile state, filter-panel toggle state, rank filter signals, dropdown state, subscribe callback, and `use_english`. It should render:
- search input with the existing search icon and class hooks
- desktop rank dropdowns on non-mobile
- mobile filter toggle + dropdown panel on mobile
- Subscribe button

Do not move persistence or filtering logic into this file.

- [ ] **Step 4: Replace the inline primary toolbar markup in `ShowTable` with `<TopToolbar ... />`**

Update `src/components/showtable.rs` to:
- import `TopToolbar`
- pass the existing signals/callbacks into it
- keep the generated DOM class `primary-toolbar` at the rendered output level

- [ ] **Step 5: Export the new component**

Update `src/components/mod.rs` with:
```rust
pub mod top_toolbar;
```

- [ ] **Step 6: Run the focused toolbar test**

Run: `cargo test showtable::tests::top_toolbar_component_is_used`
Expected: PASS.

- [ ] **Step 7: Run the full Rust test suite**

Run: `cargo test`
Expected: PASS.

- [ ] **Step 8: Commit the toolbar extraction (optional checkpoint)**

If you want an incremental checkpoint:
```bash
git add src/components/showtable.rs src/components/top_toolbar.rs src/components/mod.rs
git commit -m "refactor: extract homepage primary toolbar"
```

### Task 3: Extract the secondary meta row and preserve filter behavior

**Files:**
- Create: `src/components/results_meta.rs`
- Modify: `src/components/showtable.rs`
- Modify: `src/components/mod.rs`
- Test: `src/components/showtable.rs`

- [ ] **Step 1: Write the failing source-structure test for the meta component**

Add a test in `src/components/showtable.rs` that asserts:
- `showtable.rs` references `<ResultsMeta`
- `class="secondary-meta-row"` still exists in the rendered structure
- `class="category-chip-section"` remains below the meta component

- [ ] **Step 2: Run the focused test to verify it fails before extraction**

Run: `cargo test showtable::tests::results_meta_component_is_used`
Expected: FAIL because `ResultsMeta` does not exist yet.

- [ ] **Step 3: Create `src/components/results_meta.rs`**

Implement a small presentational component that receives:
- timezone string signal or derived string
- filtered result count signal/value
- whether filters are active
- clear-filters callback
- `use_english`

Render the secondary row with:
- timezone text
- result count text
- clear-filters button/action
- placeholder-friendly structure for future toggles, but no new behavior

Keep all state mutation in `ShowTable`.

- [ ] **Step 4: Replace the inline secondary row in `ShowTable` with `<ResultsMeta ... />`**

Pass the derived values/callbacks from `ShowTable` while preserving current timezone behavior.

- [ ] **Step 5: Export the new component**

Update `src/components/mod.rs` with:
```rust
pub mod results_meta;
```

- [ ] **Step 6: Run focused tests for top-area structure**

Run: `cargo test showtable`
Expected: PASS.

- [ ] **Step 7: Commit the meta-row extraction (optional checkpoint)**

If you want an incremental checkpoint:
```bash
git add src/components/showtable.rs src/components/results_meta.rs src/components/mod.rs
git commit -m "refactor: extract homepage results meta row"
```

### Task 4: Normalize the existing `Header` Phase 1 structure and finalize action ordering

**Files:**
- Modify: `src/components/header.rs`
- Modify: `public/styles.css`
- Inspect: `src/pages/home.rs:10-25`
- Test: `src/components/header.rs`

- [ ] **Step 1: Add or normalize the source-layout test for the header contract**

Add a small source-based test module in `src/components/header.rs` that asserts the rendered source contains:
- `class="header-brand-row"`
- `class="header-subtitle-row"`
- `class="header-announcement-row"`
- `class="header-language-switch"`
- `class="header-brand-actions"`

And asserts the rows appear in the order brand → subtitle → announcement.

- [ ] **Step 2: Run the focused header test to verify the current baseline**

Run: `cargo test header::tests::header_rows_match_phase1_structure`
Expected: PASS on the current branch if the existing header structure already satisfies the source contract. If it fails, fix the source contract first before changing action ordering.

- [ ] **Step 3: Finalize the `Header` component’s action ordering and copy**

Update `src/components/header.rs` so the right action group renders in SPEC order:
- theme toggle
- GitHub button
- language switch

Keep the language switch in `Header` and preserve the same `use_english` signal.

Keep:
- latest update fetch behavior
- PR link
- tabular portal link
- WeChat applet link
- disclaimer text

Do not move any of this back into `ShowTable`.

- [ ] **Step 4: Add a focused behavior check for the existing language switch plumbing**

Add a small test that verifies `Header` still accepts `use_english: RwSignal<bool>` and that the source contains the switch bound to that signal, so the existing persisted preference path remains wired after the action-order refactor.

- [ ] **Step 5: Run the header-focused test suite**

Run: `cargo test header`
Expected: PASS.

- [ ] **Step 6: Commit the header finalization (optional checkpoint)**

If you want an incremental checkpoint:
```bash
git add src/components/header.rs
git commit -m "refactor: finalize homepage phase 1 header"
```

### Task 5: Apply Phase 1 CSS for header, toolbar, and meta hierarchy

**Files:**
- Modify: `public/styles.css`
- Inspect: `src/components/header.rs`
- Inspect: `src/components/top_toolbar.rs`
- Inspect: `src/components/results_meta.rs`
- Inspect: `src/components/showtable.rs`

- [ ] **Step 1: Add/adjust CSS for the three header layers**

Style the header so the three rows read as distinct layers using existing theme variables:
- brand row aligns title left and actions right
- subtitle row supports concise supporting text and de-emphasized utility links
- announcement row stands apart as a strip

Do not introduce a new token system.

- [ ] **Step 2: Add/adjust CSS for the primary toolbar**

Ensure the search input becomes the strongest visual entry point and that:
- desktop layout keeps search + rank filters + Subscribe aligned
- mobile layout stacks cleanly
- mobile filter panel remains usable

- [ ] **Step 3: Add/adjust CSS for the secondary meta row**

Style timezone text, result count, and clear-filters action so the row reads as low-emphasis metadata between the primary toolbar and category chips.

- [ ] **Step 4: Add/adjust spacing rules for the category-chip section following the new hierarchy**

Only adjust spacing/surface hierarchy needed for Phase 1. Do not do the full Phase 3 polish here.

- [ ] **Step 5: Run a release build for CSS + Rust integration verification**

Run: `python scripts/merge.py conference --exclude "types.yml" > public/conference/allconf.yml && python scripts/merge.py accept_rates > public/conference/allacc.yml && cp conference/types.yml public/conference/types.yml && trunk build --release`
Expected: build succeeds.

- [ ] **Step 6: Commit the Phase 1 styling pass (optional checkpoint)**

If you want an incremental checkpoint:
```bash
git add public/styles.css
git commit -m "style: apply homepage phase 1 layout polish"
```

### Task 6: Final verification against Phase 1 acceptance criteria

**Files:**
- Inspect: `src/components/header.rs`
- Inspect: `src/components/showtable.rs`
- Inspect: `src/components/top_toolbar.rs`
- Inspect: `src/components/results_meta.rs`
- Inspect: `public/styles.css`

- [ ] **Step 1: Run the full Rust test suite**

Run: `cargo test`
Expected: PASS.

- [ ] **Step 2: Run the full local frontend build flow used by CI**

Run: `python scripts/merge.py conference --exclude "types.yml" > public/conference/allconf.yml && python scripts/merge.py accept_rates > public/conference/allacc.yml && cp conference/types.yml public/conference/types.yml && trunk build --release`
Expected: PASS.

- [ ] **Step 3: Manually verify the homepage acceptance checklist in the browser**

Confirm all of the following:
- header is visually split into 3 layers
- search and main filters are above category filters
- language switch still works
- search still works
- CCF/CORE/THCPL filters still work
- category filtering still works
- favorites still persist
- subscription modal still opens
- timeline still renders
- calendar popover still works
- pagination still works
- dark/light theme still works
- mobile and desktop layouts remain acceptable

- [ ] **Step 4: Prepare PR notes**

Draft concise PR notes covering:
- what changed: header normalization/action reorder, primary toolbar extraction, meta row extraction, Phase 1 CSS adjustments
- what was preserved: filtering/data flow/localStorage/favorites/pagination/subscription/timeline/calendar logic
- follow-up opportunities: Phase 2 polish around card/category-chip presentation, Phase 3 responsive/style cleanup

- [ ] **Step 5: Commit final verification fixes if needed (optional checkpoint)**

If verification produced follow-up edits, capture them with:
```bash
git add src/components/header.rs src/components/showtable.rs src/components/top_toolbar.rs src/components/results_meta.rs src/components/mod.rs public/styles.css
git commit -m "test: verify homepage phase 1 integration"
```
