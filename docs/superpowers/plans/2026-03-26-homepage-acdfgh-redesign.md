# Homepage A/C/D/F/G-H UI Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Redesign the homepage UI sections A/C/D/F/G/H to match the target reference while preserving all existing data flow and interactions.

**Architecture:** Keep `ShowTable` as the state owner and preserve existing signals/filters/pagination/favorite/storage pipeline. Implement the redesign by adjusting homepage component markup minimally where structure is needed, then centralizing visual changes in `public/styles.css` with new semantic utility classes. Add only tiny structural hooks in `showtable.rs` if required for CSS-safe composition.

**Tech Stack:** Rust 1.x, Leptos, `leptos`, `thaw`, `serde`, static CSS (`public/styles.css`), `cargo test`, `trunk build`.

---

## File Structure

### Primary files to modify
- `src/components/header.rs`
  - Reorganize hero, subtitle/disclaimer, and latest-update sections.
- `src/components/top_toolbar.rs`
  - Unify search/filter/subscribe into one toolbar layout.
- `src/components/results_meta.rs`
  - Rework summary row into timezone + result-count row.
- `src/components/conference_card.rs`
  - Update card-level structure/classes to support one-card visual and countdown column alignment.
- `public/styles.css`
  - Apply all theme, spacing, card, countdown, and responsive visual updates.

### Optional touchpoint only when class-safe styling requires a wrapper
- `src/components/showtable.rs`
  - Add non-functional wrapper/container classes without changing business logic.

### Files to inspect (no logic changes)
- `src/components/countdown.rs` (verify existing urgency thresholds/classes already exist)
- `src/components/category_chip.rs` (verify existing chip behavior unaffected)
- `src/components/checkbox_button.rs` (validate dropdown width/placement interactions still render)
- `src/pages/home.rs` (page composition remains `<Header />` + `<ShowTable />`)

### Files to avoid editing for this scope
- `src/components/conf.rs`, `src/components/timezone.rs`, `public/conference/*` generation, merge scripts, data files.

---

## Commit and rollback discipline

- After each major task block, create a checkpoint commit with one focused message.
- Rollback point for each block is the previous commit hash or working-tree restore:
  - `git restore src/components/<file>.rs public/styles.css`
  - For larger accidental changes: `git reset --mixed <prev-hash>` (only after explicit confirmation).
- Do **not** batch unrelated behavior changes with styling checkpoints.

## Verification commands

- Unit/style contract tests:
  - `cargo test header::tests`
  - `cargo test top_toolbar::tests`
  - `cargo test results_meta::tests`
  - `cargo test conference_card::tests`
  - `cargo test showtable::tests`
  - `cargo test`
- Build check (CI-equivalent flow):
  - `python scripts/merge.py conference --exclude "types.yml" > public/conference/allconf.yml && python scripts/merge.py accept_rates > public/conference/allacc.yml && cp conference/types.yml public/conference/types.yml && trunk build --release`

### Acceptance criteria (global)

- Header shows 3 clear blocks: brand/title, latest update, disclaimer.
- Top-right controls (theme/GitHub button/language switch) remain present, in the same order, and unchanged behavior.
- Toolbar has a prominent search input and aligned filters/subscription action row on desktop; wraps cleanly on narrow screens.
- Result summary row shows timezone left and count right; clear-filters behavior unchanged.
- Conference list reads as one coherent card per item with updated countdown panel hierarchy and no behavior changes.
- Card website link is visually compact/capsule-style and still opens the original URL.
- No overflow on mobile, no behavior regressions in search/filter/favorite/subscribe/language links.
- All tests and build pass.

---

## Task 1: Lock baseline and define design contracts

### Files
- Modify: none (verification-only prep)
- Test: `src/components/showtable.rs`, `src/components/header.rs`, `src/components/conference_card.rs`

- [ ] **Step 1: Record baseline behavior with focused test runs**

```bash
cargo test header::tests
cargo test showtable::tests
cargo test conference_card::tests
```

Expected: all existing tests pass before UI edits.

- [ ] **Step 2: Freeze plan checkpoints for this phase**

```bash
git status --short
git add -N src/components/header.rs src/components/top_toolbar.rs src/components/results_meta.rs src/components/conference_card.rs public/styles.css src/components/showtable.rs
```

Expected: staged intent-only entries for targeted files (no behavior change yet).

- [ ] **Step 3: Commit baseline marker before UI edits**

```bash
git add src/components/header.rs src/components/top_toolbar.rs src/components/results_meta.rs src/components/conference_card.rs public/styles.css src/components/showtable.rs docs/superpowers/plans/2026-03-26-homepage-acdfgh-redesign.md

git commit -m "chore: plan checkpoint for homepage A/C/D/F/G-H redesign"
```

Rollback point: revert this commit only if scope diverges.

---

## Task 2: Implement Section A/C header reflow in `src/components/header.rs`

### Files
- Modify: `src/components/header.rs`
- Modify: `public/styles.css`
- Modify: `src/components/header.rs` tests

#### Step 1 — Rework header markup structure for section separation
- Keep brand row (title + top-right actions) unchanged in IA.
- Move disclaimer to its own row.
- Keep latest update bar always full-width and conditionally rendered only when text exists.
- Keep fetch logic and `show_latest_conf/show_str` behavior untouched.

```rust
// src/components/header.rs (new/updated structure)
<div class="header-disclaimer-row">
  <div class="header-disclaimer-message">
    "*Disclaimer: "
    "... for reference only."
  </div>
</div>
<div class="header-announcement-row">
  <Show when=move || show_latest_conf.get()>...</Show>
</div>
```

- [ ] **Step 2: Add missing row/container class hooks and replace disclaimer position**
  - Add `class="header-disclaimer-row"`, `class="header-disclaimer-message"`.
  - Add `class="header-announcement-content"`, `class="header-announcement-label"`, `class="header-announcement-text"`, optional badge class `class="header-announcement-badge"`.
  - Ensure info icon element uses a dedicated class.

- [ ] **Step 3: Update hero/announcement tests for explicit structure**

```rust
#[test]
fn header_disclaimer_row_is_present_and_disclaimer_text_is_isolated() {
    const HEADER_SOURCE: &str = include_str!("header.rs");
    assert!(HEADER_SOURCE.contains("class=\"header-disclaimer-row\""));
    assert!(HEADER_SOURCE.contains("header-disclaimer-message"));
    assert!(HEADER_SOURCE.contains("class=\"header-announcement-row\""));
}
```

- [ ] **Step 4: Run targeted tests and fix any source-based assertions**

```bash
cargo test header::tests::header_rows_match_phase1_structure
cargo test header::tests::header_disclaimer_row_is_present_and_disclaimer_text_is_isolated
```

Expected: pass; if `header_rows_have_phase1_surface_hierarchy_in_styles` fails, add required selectors once in CSS.

- [ ] **Step 5: Commit Task 2 checkpoint**

```bash
git add src/components/header.rs public/styles.css

git commit -m "feat: restructure hero header A/C blocks for announcement and disclaimer"
```

Rollback point: revert the above commit if rows or top IA regress.

---

## Task 3: Unify toolbar row in `src/components/top_toolbar.rs` (Section D)

### Files
- Modify: `src/components/top_toolbar.rs`
- Modify: `public/styles.css`
- Test: `src/components/top_toolbar.rs`

#### Step 1 — Tighten copy and class-level structure

- Change search placeholder to `"Search conferences..."`.
- Keep `Input` binding to `input_value` untouched.
- Ensure search, filters, subscribe are in one visual toolbar block:
  - Desktop: inline `primary-toolbar-main` + `primary-toolbar-actions`
  - Mobile: search row + optional filter panel under existing toggle semantics.

```rust
// src/components/top_toolbar.rs
<Input
  value=input_value
  placeholder="Search conferences..."
  size=InputSize::Small
  class="custom-search-input"
>
```

- [ ] **Step 2: Add/normalize semantic classes for uniform control height and alignment**
  - Ensure actions use `class="toolbar-action-button"`, `class="subscribe-button"`, `class="filter-toggle-button"` (or existing names if already present).
  - Keep `size=ButtonSize::Small` and `ButtonAppearance::Subtle` intact.

- [ ] **Step 3: Add/refresh source tests to guard D structure**

```rust
#[cfg(test)]
mod tests {
  #[test]
  fn top_toolbar_uses_expected_placeholder() {
      const SOURCE: &str = include_str!("top_toolbar.rs");
      assert!(SOURCE.contains("Search conferences..."));
  }

  #[test]
  fn top_toolbar_keeps_filter_and_subscribe_in_primary_actions() {
      const SOURCE: &str = include_str!("top_toolbar.rs");
      assert!(SOURCE.contains("class=\"primary-toolbar-actions\""));
      assert!(SOURCE.contains("on_click=move |_| show_subscription_modal.set(true)"));
  }
}
```

- [ ] **Step 4: Run focused tests and adjust selectors**

```bash
cargo test top_toolbar::tests
```

- [ ] **Step 5: Commit Task 3 checkpoint**

```bash
git add src/components/top_toolbar.rs public/styles.css

git commit -m "feat: refine toolbar structure and search entry for section D"
```

Rollback point: restore `top_toolbar.rs` and `public/styles.css` before this commit if mobile behavior changes.

---

## Task 4: Rework Section F in `src/components/results_meta.rs`

### Files
- Modify: `src/components/results_meta.rs`
- Modify: `public/styles.css`
- Modify: `src/components/showtable.rs` tests (source-order contract)

#### Step 1 — Rearrange composition into left/right summary row
- Keep both timezone and result count text semantics exactly.
- Keep clear filters callback identical.
- Keep English/Chinese labels exactly as existing strings.

```rust
// src/components/results_meta.rs
<div class="results-meta-summary">
  <div class="results-meta-timezone">...</div>
  <div class="results-meta-actions">...
    <span class="results-count-message">...</span>
    <Show when=move || has_active_filters.get()>...</Show>
  </div>
</div>
```

- [ ] **Step 2: Add row-specific classes to maintain flexible wrapping**
  - Add classes: `results-meta-left`, `results-meta-right`, `results-meta-count-group`.
  - Keep `clear-filters-button` class for behavior tests.

- [ ] **Step 3: Add source tests for the new result row contract**

```rust
#[test]
fn results_meta_has_timezone_and_results_count_rows() {
  const SOURCE: &str = include_str!("results_meta.rs");
  assert!(SOURCE.contains("results-meta-left"));
  assert!(SOURCE.contains("results-meta-right"));
  assert!(SOURCE.contains("clear-filters-button"));
}
```

- [ ] **Step 4: Verify section contract from table composition**

```bash
cargo test showtable::tests::results_meta_component_is_used
cargo test results_meta::tests
```

Expected: row still rendered through `ShowTable` with same call sites.

- [ ] **Step 5: Commit Task 4 checkpoint**

```bash
git add src/components/results_meta.rs public/styles.css src/components/showtable.rs

git commit -m "feat: rework result summary row with timezone/count layout"
```

Rollback point: restore `results_meta.rs` and revert test additions if summary semantics drift.

---

## Task 5: Make conference cards feel unified + countdown panel polish (Sections G/H)

### Files
- Modify: `src/components/conference_card.rs`
- Modify: `public/styles.css`
- Test: `src/components/conference_card.rs`

#### Step 1 — Add non-behavioral card composition hooks
- Keep `conf` property and all callback wiring unchanged.
- Keep `<CountDown remain compact=true />` and `TimeLine` usage unchanged.
- Add structural classes to support a single-card visual using existing two-cell markup:
  - `conference-card-shell`
  - `conference-card-row`, `conference-card-main-wrap`, `conference-card-meta-row`
  - `conference-website-link-wrap`, `conference-website-link`, `countdown-panel`, `countdown-line`.
- Keep note order and website row position before deadline panel.

```rust
// src/components/conference_card.rs
<div class="conference-card-shell conference-card-row">
  <div class="conference-card-main conference-card-main-wrap">
    ...
    <div class="conference-meta-text conference-website-line">
      <a href=link class="inline-muted-link conference-website-link inline-break-link" ...>{display_link}</a>
    </div>
  </div>
</div>
```

- [ ] **Step 2: Move favorite placement to card-anchored top-right style hook (behavior preserved)**
- Keep button event `on_toggle_favorite` unchanged.
- Wrap favorite button with a container class for styling as top-right action.

```rust
// src/components/conference_card.rs
<div class="conference-favorite-anchor">
  <button ... class="favorite-toggle">...</button>
</div>
```

- [ ] **Step 3: Refresh deadline panel structure for stronger hierarchy**
- Introduce `class="countdown-value-wrap"`, `class="countdown-panel-meta"`, and `class="countdown-timeline-wrap"` wrappers.
- Keep exact deadline text behavior (`Deadline: ...` + fallback pull request link) untouched.

- [ ] **Step 4: Add regression tests for card contract**

```rust
#[test]
fn website_link_uses_styled_compact_wrapper_class() {
  const CARD_SOURCE: &str = include_str!("conference_card.rs");
  assert!(CARD_SOURCE.contains("conference-website-link"));
  assert!(CARD_SOURCE.contains("conference-card-main-wrap"));
}
```

- [ ] **Step 5: Run tests for unchanged behavior guardrails**

```bash
cargo test conference_card::tests
```

Expected: existing behavior contracts pass + new class presence checks pass.

- [ ] **Step 6: Commit Task 5 checkpoint**

```bash
git add src/components/conference_card.rs public/styles.css

git commit -m "feat: add unified conference card styling hooks for section G/H"
```

Rollback point: revert this commit if website row order or deadline/timeline visibility shifts unexpectedly.

---

## Task 6: Design token and responsive styling pass in `public/styles.css`

### Files
- Modify: `public/styles.css`

#### Step 1 — Introduce/standardize homepage tokens
- Tune or add root tokens:
  - `--bg-deep`, `--surface-1`, `--surface-2`, `--surface-border`, `--text-soft`, `--text-muted`, `--accent`, `--warning`, `--radius-card`, `--shadow-soft`, `--control-height`.
- Keep unrelated page styles untouched.

- [ ] **Step 2 — Redesign hero/header blocks (Section A/C)**
  - `hero-header`: stronger vertical rhythm.
  - `header-announcement-row`: translucent card-like strip with rounded corners/shadow.
  - `header-announcement-label`: include icon spacing and small badge style.
  - `header-disclaimer-row`: muted secondary tone.

- [ ] **Step 3 — Refine toolbar and summary row controls (Section D/F)**
  - `.primary-toolbar`, `.primary-toolbar-main`, `.timezone-search`, `.custom-search-input`, `.search-prefix-icon`
  - `.primary-toolbar-actions`, `.desktop-filter-actions`, `.mobile-filter-menu`, `.subscribe-button`
  - `.secondary-meta-row`, `.results-meta-left`, `.results-meta-right`, `.results-meta-actions`
  - Ensure consistent control height and focus ring treatment.

- [ ] **Step 4 — Refine conference card + countdown visual hierarchy (Section G/H)**
  - `.conference-card-shell`, `.conference-card-main`, `.conference-deadline-panel`, `.conference-favorite-anchor`
  - `.conference-tag-groups`, `.conference-meta-text`, `.conference-note`
  - `.conference-website-link` as pill/truncation (`max-width`, `overflow`, `text-overflow`).
  - `.conference-deadline-panel .countdown-display`, `.countdown-compact`, `.countdown-value`, `.countdown-urgent`, `.countdown-warning`, `.countdown-attention`, `.countdown-normal`.
  - Ensure timeline appears as secondary/progressive visual under deadline.

- [ ] **Step 5 — Mobile/tablet breakpoints without overflow**
  - Update `@media (max-width: 768px)` and `@media (max-width: 640px)` sections:
    - Search bar on its own row.
    - filter/actions wrap cleanly.
    - card content stacks with countdown below metadata.
    - no horizontal scrolling on wide cards.

- [ ] **Step 6: Run style and full suite checks**

```bash
cargo test
cargo test header::tests
cargo test showtable::tests
cargo test conference_card::tests
python scripts/merge.py conference --exclude "types.yml" > public/conference/allconf.yml && python scripts/merge.py accept_rates > public/conference/allacc.yml && cp conference/types.yml public/conference/types.yml && trunk build --release
```

- [ ] **Step 7: Commit Task 6 checkpoint**

```bash
git add public/styles.css

git commit -m "feat: refine homepage section A/C/D/F/G-H visual system and responsiveness"
```

Rollback point: keep a CSS-only revert safe by resetting `public/styles.css`.

---

## Task 7: Optional structure-safe `showtable.rs` hook refinement

### Files
- Modify: `src/components/showtable.rs`
- Test: `src/components/showtable.rs`

#### Step 1 — Add minimal wrapper classes only if CSS cannot target current structure safely
- Only if needed, add class hooks around the sections that are already semantically present (no behavior changes):
  - Existing order: `TopToolbar`, `ResultsMeta`, `category-chip-section`, `SubscriptionModal`, `table-container` unchanged.
  - Add optional `class="conference-table-wrap"` wrapper around `<Table>` or `class="conference-list-wrapper"` on existing container.

- [ ] **Step 2: Add/adjust source tests for structural hooks**

```rust
#[test]
fn top_section_order_and_hook_classes_remain_stable() {
    const SHOWTABLE_SOURCE: &str = include_str!("showtable.rs");
    assert!(SHOWTABLE_SOURCE.contains("class=\"table-container\""));
}
```

- [ ] **Step 3: Re-run top-order and full tests**

```bash
cargo test showtable::tests::top_section_order_matches_phase1_layout
cargo test
```

- [ ] **Step 4: Commit Task 7 checkpoint**

```bash
git add src/components/showtable.rs

git commit -m "chore: add optional structural classes for stable homepage styling"
```

Rollback point: revert this commit if order or behavior assertions unexpectedly fail.

---

## Task 8: Final validation and manual checklist

### Files
- Inspect only, no code changes

- [ ] **Step 1: Run full project validation**

```bash
cargo test
python scripts/merge.py conference --exclude "types.yml" > public/conference/allconf.yml && python scripts/merge.py accept_rates > public/conference/allacc.yml && cp conference/types.yml public/conference/types.yml && trunk build --release
```

- [ ] **Step 2: Manual checklist (required)**
  - Desktop (≥1200px): sections A/C/D/F/G/H hierarchy and spacing.
  - Tablet (900x1024): toolbar wraps to two lines cleanly.
  - Mobile (≤640px): no horizontal scroll, search own row, filters wrap.
  - Language toggle: English/中文 updates all texts while actions still work.
  - Long conference names: no clipping overflow.
  - Long URLs: visually truncated/capsule clickable and opens correct target.
  - Expired vs upcoming: FIN/TBD states still render same messages and ranking timeline behavior.

- [ ] **Step 3: Close-out commit for completed redesign pass**

```bash
git status --short

git commit -am "feat: finalize homepage A/C/D/F/G-H redesign pass"
```

Expected outcome: complete redesign passes visual and behavior acceptance criteria while preserving all existing interactions.

---

Plan complete and saved to `docs/superpowers/plans/2026-03-26-homepage-acdfgh-redesign.md`. Two execution options:

1. Subagent-Driven (recommended) - dispatch one subagent per task and review after each task
2. Inline Execution - execute tasks in this session using superpowers:executing-plans with checkpoints
