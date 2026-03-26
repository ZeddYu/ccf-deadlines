# Homepage Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Update the homepage UI to match the design reference more closely while preserving the existing data flow, search/filter/subscribe/favorite interactions, and top-right control behavior.

**Architecture:** Keep the current Leptos component boundaries centered around `Header`, `ShowTable`, `TopToolbar`, `ResultsMeta`, and `ConferenceCard`, but reshape their rendered hierarchy and shared CSS tokens so the page reads as a deliberate homepage rather than a stitched set of sections. Concentrate most of the visual redesign in `public/styles.css`, keep Rust changes focused on markup structure and copy placement, and preserve existing signal/data plumbing.

**Tech Stack:** Rust, Leptos, Thaw, CSS, Trunk/WASM

---

## File map

### Existing files to modify
- `public/styles.css` — homepage design tokens, hero layout, toolbar styling, chip styling, result row styling, unified card styling, countdown panel styling, responsive rules.
- `src/components/header.rs` — restructure hero copy, split latest update bar from disclaimer line, preserve theme/GitHub/language controls, update structure tests.
- `src/components/showtable.rs` — reorder homepage sections so toolbar, chips, and result summary match the new hierarchy; keep filtering logic unchanged; update structure tests.
- `src/components/top_toolbar.rs` — refine toolbar markup so search, subscribe, and filters align as one unified control row on desktop and wrap cleanly on narrow screens.
- `src/components/results_meta.rs` — simplify summary row content so timezone text and result count are positioned as a dedicated row between chips and conference cards.
- `src/components/conference_card.rs` — render each conference as one integrated card, move the favorite action into a cleaner card action position, keep deadline/countdown/timeline behavior intact, and convert the website link to a compact capsule style.
- `src/components/countdown.rs` — only if needed to support refined countdown emphasis classes without changing countdown logic.
- `src/components/mod.rs` — only if component exports need adjustment after homepage-focused refactors.

### Existing files to verify during implementation
- `src/pages/home.rs` — ensure the page shell remains `Header` + `ShowTable` and does not need additional restructuring.
- `src/components/header.rs` tests — source-order assertions will need to track the new hierarchy.
- `src/components/showtable.rs` tests — section-order assertions will need to match the new chips/result-summary/list flow.
- `src/components/conference_card.rs` tests — preserve card contract assertions while updating class names/order where necessary.
- `src/components/results_meta.rs` tests — keep tests aligned with the simplified summary row markup.

### Commands to use for verification
- `cargo test`
- `trunk build --release`

---

### Task 1: Lock down the new homepage hierarchy in tests

**Files:**
- Modify: `src/components/header.rs`
- Modify: `src/components/showtable.rs`
- Modify: `src/components/results_meta.rs`
- Modify: `src/components/conference_card.rs`
- Test: existing inline `#[cfg(test)]` blocks in those files

- [ ] **Step 1: Update header structure tests to describe the new hero layout**

Replace the current phase-1 order assertions in `src/components/header.rs` so they validate this sequence inside the component source:

```rust
#[test]
fn header_rows_match_redesign_structure() {
    const HEADER_SOURCE: &str = include_str!("header.rs");

    fn index_of(haystack: &str, needle: &str) -> usize {
        haystack
            .find(needle)
            .unwrap_or_else(|| panic!("expected to find `{needle}` in Header source"))
    }

    let brand_row = index_of(HEADER_SOURCE, "class=\"header-brand-row\"");
    let description_row = index_of(HEADER_SOURCE, "class=\"header-description-row\"");
    let announcement_row = index_of(HEADER_SOURCE, "class=\"header-announcement-row\"");
    let disclaimer_row = index_of(HEADER_SOURCE, "class=\"header-disclaimer-row\"");

    assert!(brand_row < description_row);
    assert!(description_row < announcement_row);
    assert!(announcement_row < disclaimer_row);
}
```

- [ ] **Step 2: Update showtable structure tests to reflect the new section order**

Adjust the `top_section_order_matches_phase1_layout` style test in `src/components/showtable.rs` to assert this order:

```rust
#[test]
fn top_section_order_matches_redesign_layout() {
    const SHOWTABLE_SOURCE: &str = include_str!("showtable.rs");

    let showtable_start = SHOWTABLE_SOURCE
        .rfind("pub fn ShowTable(use_english: RwSignal<bool>) -> impl IntoView {")
        .expect("expected ShowTable component definition");
    let showtable_end = SHOWTABLE_SOURCE
        .rfind("static UTC_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();")
        .expect("expected end of ShowTable component source");
    let component_source = &SHOWTABLE_SOURCE[showtable_start..showtable_end];

    fn index_of(haystack: &str, needle: &str) -> usize {
        haystack
            .find(needle)
            .unwrap_or_else(|| panic!("expected to find `{needle}` in ShowTable component"))
    }

    let primary_toolbar = index_of(component_source, "<TopToolbar");
    let category_chips = index_of(component_source, "class=\"category-chip-section\"");
    let secondary_meta = index_of(component_source, "class=\"secondary-meta-row\"");
    let table_container = index_of(component_source, "class=\"table-container\"");

    assert!(primary_toolbar < category_chips);
    assert!(category_chips < secondary_meta);
    assert!(secondary_meta < table_container);
}
```

- [ ] **Step 3: Update results-meta and conference-card source tests for the redesigned classes**

In `src/components/results_meta.rs`, assert the new row class names stay present:

```rust
#[test]
fn results_meta_has_timezone_and_results_count_rows() {
    const SOURCE: &str = include_str!("results_meta.rs");

    assert!(SOURCE.contains("results-meta-summary"));
    assert!(SOURCE.contains("results-meta-timezone"));
    assert!(SOURCE.contains("results-meta-count-group"));
    assert!(SOURCE.contains("results-count-message"));
    assert!(SOURCE.contains("clear-filters-button"));
}
```

In `src/components/conference_card.rs`, keep the existing contract checks but update them to include the redesigned capsule link and unified card shell:

```rust
#[test]
fn conference_card_keeps_unified_card_and_capsule_link_classes() {
    const SOURCE: &str = include_str!("conference_card.rs");

    assert!(SOURCE.contains("conference-card-shell conference-card-row"));
    assert!(SOURCE.contains("conference-card-main conference-card-main-wrap"));
    assert!(SOURCE.contains("conference-card-meta-row"));
    assert!(SOURCE.contains("conference-website-link-wrap"));
    assert!(SOURCE.contains("conference-website-link"));
    assert!(SOURCE.contains("conference-deadline-panel countdown-panel"));
}
```

- [ ] **Step 4: Run tests and confirm they fail for the expected old structure**

Run: `cargo test header_rows_match_redesign_structure top_section_order_matches_redesign_layout results_meta_has_timezone_and_results_count_rows conference_card_keeps_unified_card_and_capsule_link_classes`

Expected: FAIL because the current component markup still uses the old subtitle/disclaimer ordering and old section flow.

- [ ] **Step 5: Commit the failing-test checkpoint**

```bash
git add src/components/header.rs src/components/showtable.rs src/components/results_meta.rs src/components/conference_card.rs
git commit -m "test: define homepage redesign structure"
```

---

### Task 2: Reshape the hero area into title, description, update bar, and disclaimer

**Files:**
- Modify: `src/components/header.rs`
- Modify: `public/styles.css`
- Test: `src/components/header.rs`

- [ ] **Step 1: Rewrite the header markup to match the new information hierarchy**

Replace the current subtitle block in `src/components/header.rs` with a dedicated description row, preserve the existing control block, and keep the update bar/disclaimer as separate siblings:

```rust
view! {
    <section class="hero-header">
        <div class="header-brand-row">
            <div class="header-brand-copy">
                <a href="/" class="title">
                    "CCFDDL"
                    <sup>"®"</sup>
                    "\u{00a0}Open Deadlines"
                </a>
                <div class="header-description-row">
                    <span class="header-description-primary">
                        "Worldwide Conference Deadline Countdowns."
                    </span>
                    <span class="header-description-secondary">
                        "To add or edit a conference, "
                        <a
                            class="header-muted-link interactive-link"
                            href="https://github.com/ccfddl/ccf-deadlines/pulls"
                            target="_blank"
                        >
                            "send a pull request"
                        </a>
                        "."
                    </span>
                </div>
            </div>
            <div class="header-brand-actions">
                // keep the existing theme button, GitButton, and language switch here unchanged
            </div>
        </div>

        {move || {
            show_latest_conf.get().then(|| {
                view! {
                    <div class="header-announcement-row" role="status" aria-live="polite">
                        <div class="header-announcement-content">
                            <Icon icon=icondata::FiInfo class="header-announcement-icon" />
                            <span class="header-announcement-label">"Latest Update:"</span>
                            <span class="header-announcement-text">{show_str.get()}</span>
                            <span class="header-announcement-badge">"NEW"</span>
                        </div>
                    </div>
                }
            })
        }}

        <div class="header-disclaimer-row">
            <p class="header-disclaimer-message">
                "*Disclaimer: The data provided by ccfddl is manually collected and for reference purposes only."
            </p>
        </div>
    </section>
}
```

- [ ] **Step 2: Add hero-specific surface and spacing styles**

Add or replace the relevant header rules in `public/styles.css` with a tighter surface hierarchy:

```css
.hero-header {
    display: flex;
    flex-direction: column;
    gap: 14px;
    margin-top: 24px;
}

.header-brand-row {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 20px;
    padding: 8px 0 0;
}

.header-brand-copy {
    display: flex;
    flex-direction: column;
    gap: 12px;
    min-width: 0;
}

.header-description-row {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    color: var(--color-text-secondary);
    line-height: 1.6;
}

.header-announcement-row {
    border: 1px solid var(--color-border-soft);
    border-radius: var(--radius-xl);
    background: linear-gradient(180deg, var(--surface-elevated), var(--surface-panel));
    box-shadow: var(--shadow-md);
    padding: 14px 16px;
}

.header-disclaimer-row {
    padding: 0 2px;
}

.header-disclaimer-message {
    margin: 0;
    color: var(--color-text-tertiary);
    font-size: 13px;
}
```

- [ ] **Step 3: Preserve top-right control behavior while improving alignment only**

Keep the existing theme button, GitHub button, and language switch logic untouched in `src/components/header.rs`; only refine the action-row CSS in `public/styles.css`:

```css
.header-brand-actions {
    display: inline-flex;
    align-items: center;
    gap: 12px;
    margin-left: auto;
    flex-wrap: wrap;
    justify-content: flex-end;
}

.header-language-switch {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 6px 10px;
    border: 1px solid var(--color-border-light);
    border-radius: 999px;
    background: var(--surface-panel);
}
```

- [ ] **Step 4: Run the focused header tests**

Run: `cargo test header_rows_match_redesign_structure header_language_switch_still_uses_use_english_signal header_brand_actions_follow_phase1_order header_disclaimer_and_announcement_structure_is_present`

Expected: PASS.

- [ ] **Step 5: Commit the hero update**

```bash
git add src/components/header.rs public/styles.css
git commit -m "feat: redesign homepage hero hierarchy"
```

---

### Task 3: Rebuild the search/filter/subscribe area as one unified toolbar

**Files:**
- Modify: `src/components/top_toolbar.rs`
- Modify: `public/styles.css`
- Test: `src/components/top_toolbar.rs`
- Verify: `src/components/showtable.rs`

- [ ] **Step 1: Adjust `TopToolbar` markup so the search area and actions read as one surface**

Update `src/components/top_toolbar.rs` to keep the same props and behavior but use a clearer internal grouping:

```rust
view! {
    <div class="primary-toolbar">
        <div class="primary-toolbar-main">
            <div class="toolbar-search-shell timezone-search">
                <Input
                    value=input_value
                    placeholder="Search conferences..."
                    size=InputSize::Small
                    class="custom-search-input"
                >
                    <InputPrefix slot>
                        <Icon icon=icondata::FiSearch class="search-prefix-icon" />
                    </InputPrefix>
                </Input>
            </div>
        </div>

        <div class="primary-toolbar-actions">
            <div class="desktop-filter-actions">
                // existing CCF / CORE / THCPL dropdowns unchanged
            </div>
            <Button
                class="toolbar-action-button subscribe-button"
                size=ButtonSize::Small
                appearance=ButtonAppearance::Subtle
                on_click=move |_| show_subscription_modal.set(true)
            >
                <Icon icon=icondata::AiCalendarOutlined class="calendar-button-icon" />
                {move || if use_english.get() { "Subscribe" } else { "订阅" }}
            </Button>
            // keep the existing mobile filter toggle branch and show_filters behavior
        </div>
    </div>
}
```

- [ ] **Step 2: Replace toolbar surface styles with unified control sizing**

Update the toolbar rules in `public/styles.css`:

```css
.primary-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 14px;
    flex-wrap: wrap;
    margin-top: 18px;
    padding: 14px;
    border: 1px solid var(--color-border-soft);
    border-radius: var(--radius-xl);
    background: var(--surface-panel);
    box-shadow: var(--shadow-md);
}

.primary-toolbar-main {
    flex: 1 1 320px;
    min-width: min(100%, 320px);
}

.toolbar-search-shell,
.desktop-filter-actions,
.toolbar-action-button {
    min-height: var(--control-height);
}

.primary-toolbar-actions {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
    justify-content: flex-end;
}
```

- [ ] **Step 3: Add stronger search focus treatment and consistent control styling**

Extend `public/styles.css` with control-level rules:

```css
.custom-search-input,
.toolbar-action-button,
.desktop-filter-actions .thaw-button,
.desktop-filter-actions .thaw-select {
    border-radius: var(--radius-lg);
}

.custom-search-input:focus-within {
    border-color: var(--color-accent);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--color-accent) 24%, transparent);
}

.subscribe-button {
    background: color-mix(in srgb, var(--color-accent) 14%, var(--surface-elevated));
    border-color: color-mix(in srgb, var(--color-accent) 28%, var(--color-border-light));
    color: var(--color-text-primary);
}
```

- [ ] **Step 4: Run the toolbar-focused tests**

Run: `cargo test top_toolbar_uses_expected_placeholder top_toolbar_keeps_filter_and_subscribe_in_primary_actions top_section_order_matches_redesign_layout`

Expected: PASS.

- [ ] **Step 5: Commit the toolbar changes**

```bash
git add src/components/top_toolbar.rs src/components/showtable.rs public/styles.css
git commit -m "feat: unify homepage search toolbar"
```

---

### Task 4: Move chips above the result summary and tighten chip styling

**Files:**
- Modify: `src/components/showtable.rs`
- Modify: `public/styles.css`
- Test: `src/components/showtable.rs`

- [ ] **Step 1: Reorder the `ShowTable` markup to match toolbar → chips → summary → list**

Inside `src/components/showtable.rs`, place the category chip section before the `secondary-meta-row` block while keeping all existing signals, filtering callbacks, and chip behavior intact:

```rust
view! {
    <div class="showtable-shell">
        <TopToolbar
            input_value
            rank_list
            core_rank_list
            thcpl_rank_list
            open_dropdown
            show_filters
            show_subscription_modal
            is_mobile
            use_english
        />

        <div class="category-chip-section">
            <div class="category-actions-row">
                <button
                    type="button"
                    class="category-chip category-chip-action"
                    on:click=handle_check_all
                >
                    {move || if is_all_checked.get() {
                        if use_english.get() { "Clear all" } else { "清空领域" }
                    } else if use_english.get() {
                        "Select all"
                    } else {
                        "选择全部"
                    }}
                </button>
            </div>
            <div class="category-chip-grid">
                // existing CategoryChip rendering stays here
            </div>
        </div>

        <ResultsMeta
            class="secondary-meta-row"
            timezone_label=timezone_label
            result_count=result_count
            has_active_filters=has_active_filters
            on_clear_filters=clear_filters
            use_english=use_english
        />

        <div class="table-container">
            // existing table/list rendering
        </div>
    </div>
}
```

- [ ] **Step 2: Tighten category-chip layout and selected-state visuals**

Update the chip styles in `public/styles.css`:

```css
.category-chip-section {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 14px;
    flex-wrap: wrap;
    margin-top: 16px;
    padding: 14px;
    border: 1px solid var(--color-border-soft);
    border-radius: var(--radius-xl);
    background: var(--surface-panel);
}

.category-chip-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    flex: 1 1 640px;
}

.category-chip {
    min-height: var(--control-height);
    border-radius: 999px;
    padding: 0 14px;
}

.category-chip-selected {
    background: color-mix(in srgb, var(--color-accent) 18%, var(--surface-elevated));
    border-color: color-mix(in srgb, var(--color-accent) 42%, var(--color-border-light));
    color: var(--color-text-primary);
}
```

- [ ] **Step 3: Keep the action area small and non-invasive**

If `ShowTable` already has only toggle-all support, do not add new “common fields” logic. Keep only the existing all/clear action and make that explicit in the code by using one action button, not a new data feature.

Use this button content pattern:

```rust
{move || {
    if is_all_checked.get() {
        if use_english.get() { "Clear all" } else { "清空领域" }
    } else if use_english.get() {
        "Select all"
    } else {
        "选择全部"
    }
}}
```

- [ ] **Step 4: Run the chip/layout tests**

Run: `cargo test phase2_category_chip_section_uses_category_chip_component top_section_order_matches_redesign_layout phase1_toolbar_meta_and_chip_sections_have_distinct_css_hierarchy`

Expected: PASS.

- [ ] **Step 5: Commit the chip-section update**

```bash
git add src/components/showtable.rs public/styles.css
git commit -m "feat: refine homepage field chips"
```

---

### Task 5: Simplify the result summary row

**Files:**
- Modify: `src/components/results_meta.rs`
- Modify: `public/styles.css`
- Test: `src/components/results_meta.rs`
- Verify: `src/components/showtable.rs`

- [ ] **Step 1: Keep `ResultsMeta` focused on timezone, count, and clear-filters action only**

Update `src/components/results_meta.rs` so the rendered copy matches the redesigned summary row language:

```rust
view! {
    <div class=class>
        <div class="results-meta-summary">
            <div class="results-meta-timezone">
                {move || format!("Deadlines are shown in {} time.", timezone_label.get())}
            </div>
            <div class="results-meta-actions results-meta-count-group">
                <div class="results-count-message">
                    {move || {
                        if use_english.get() {
                            format!("{} conferences found", result_count.get())
                        } else {
                            format!("共找到 {} 个会议", result_count.get())
                        }
                    }}
                </div>
                <Show when=move || has_active_filters.get()>
                    <button
                        type="button"
                        class="clear-filters-button"
                        on:click=move |_| on_clear_filters.run(())
                    >
                        {move || if use_english.get() { "Clear filters" } else { "清除筛选" }}
                    </button>
                </Show>
            </div>
        </div>
    </div>
}
```

- [ ] **Step 2: Apply summary-row layout styles that separate it from the chips and cards**

Add or replace these rules in `public/styles.css`:

```css
.secondary-meta-row {
    margin-top: 14px;
    padding: 0 2px;
}

.results-meta-summary {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    flex-wrap: wrap;
}

.results-meta-timezone {
    color: var(--color-text-secondary);
    font-size: 14px;
}

.results-count-message {
    color: var(--color-text-primary);
    font-weight: 500;
}
```

- [ ] **Step 3: Preserve the existing clear-filters behavior and only restyle the action**

Do not change `on_clear_filters`. Only refine the button style in `public/styles.css`:

```css
.clear-filters-button {
    min-height: 30px;
    padding: 0 12px;
    border-radius: 999px;
    border: 1px solid var(--color-border-light);
    background: var(--surface-elevated);
    color: var(--color-text-secondary);
}
```

- [ ] **Step 4: Run the results-meta tests**

Run: `cargo test results_meta_has_timezone_and_results_count_rows results_meta_component_is_used`

Expected: PASS.

- [ ] **Step 5: Commit the result-summary row**

```bash
git add src/components/results_meta.rs src/components/showtable.rs public/styles.css
git commit -m "feat: redesign homepage result summary row"
```

---

### Task 6: Convert conference entries into unified cards with a refined countdown panel

**Files:**
- Modify: `src/components/conference_card.rs`
- Modify: `src/components/countdown.rs` (only if class hooks are needed)
- Modify: `public/styles.css`
- Test: `src/components/conference_card.rs`
- Verify: `src/components/showtable.rs`

- [ ] **Step 1: Keep the card logic intact but restructure the card markup into one cohesive surface**

Update the card shell in `src/components/conference_card.rs` so the main metadata and countdown panel sit inside one integrated card container:

```rust
<div class="conference-card-shell conference-card-row">
    <div class="conference-card-main conference-card-main-wrap" class:conf-fin=is_finished>
        <div class="conference-card-header-row">
            <div class="conf-title">
                <a href=dblp_url class="table-link interactive-link" target="_blank">
                    {title}
                </a>
                " "
                {year}
            </div>
            <div class="conference-favorite-anchor">
                <button
                    type="button"
                    class="favorite-toggle"
                    aria-label=if is_like {
                        "Remove conference from favorites"
                    } else {
                        "Add conference to favorites"
                    }
                    on:click=move |_| on_toggle_favorite.run(())
                >
                    // existing star icon rendering
                </button>
            </div>
        </div>

        <div class="conference-meta-text conference-card-meta-row">{date} " " {place}</div>
        <div class="conference-meta-text conference-card-meta-row">{description}</div>
        // existing rank tags + category tags
        // existing optional note row

        <div class="conference-meta-text conference-card-meta-row conference-website-link-wrap">
            <span class="conference-website-label">"Website"</span>
            <a
                href=link.clone()
                class="inline-muted-link interactive-link inline-break-link conference-website-link"
                target="_blank"
                title=link.clone()
            >
                {display_link}
            </a>
        </div>
    </div>

    <div class="conference-deadline-panel countdown-panel">
        // existing countdown / deadline / timeline behavior preserved
    </div>
</div>
```

- [ ] **Step 2: Replace the card and countdown styles with a single-card visual system**

Add or replace these rules in `public/styles.css`:

```css
.conference-card-row {
    display: grid;
    grid-template-columns: minmax(0, 1.7fr) minmax(260px, 0.95fr);
    gap: 18px;
    padding: 18px;
    border: 1px solid var(--color-border-soft);
    border-radius: calc(var(--radius-xl) + 2px);
    background: linear-gradient(180deg, var(--surface-panel), var(--surface-elevated));
    box-shadow: var(--shadow-md);
    transition: border-color var(--transition-fast), transform var(--transition-fast), box-shadow var(--transition-fast);
}

.conference-card-row:hover {
    border-color: color-mix(in srgb, var(--color-accent) 28%, var(--color-border-light));
    box-shadow: var(--shadow-lg);
    transform: translateY(-1px);
}

.countdown-panel {
    border: 1px solid var(--color-border-light);
    border-radius: var(--radius-lg);
    background: linear-gradient(180deg, var(--surface-elevated), color-mix(in srgb, var(--surface-panel) 82%, black));
    padding: 14px;
}

.countdown-value {
    font-size: clamp(24px, 3vw, 34px);
    font-weight: 700;
    letter-spacing: -0.02em;
}
```

- [ ] **Step 3: Convert the website link into a readable capsule without changing navigation behavior**

Add CSS for the compact website link:

```css
.conference-website-link-wrap {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
}

.conference-website-label {
    color: var(--color-text-tertiary);
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
}

.conference-website-link {
    display: inline-flex;
    align-items: center;
    max-width: min(100%, 420px);
    min-height: 30px;
    padding: 0 12px;
    border: 1px solid var(--color-border-light);
    border-radius: 999px;
    background: var(--surface-elevated);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}
```

- [ ] **Step 4: Run the conference-card tests**

Run: `cargo test conference_card_keeps_unified_card_and_capsule_link_classes website_row_stays_in_main_card_column_before_deadline_panel phase3_conference_card_contract_preserves_deadline_and_timeline_structure`

Expected: PASS.

- [ ] **Step 5: Commit the unified-card redesign**

```bash
git add src/components/conference_card.rs src/components/countdown.rs public/styles.css
git commit -m "feat: redesign homepage conference cards"
```

---

### Task 7: Finish design-token cleanup and responsive behavior

**Files:**
- Modify: `public/styles.css`
- Verify: `src/components/header.rs`
- Verify: `src/components/top_toolbar.rs`
- Verify: `src/components/showtable.rs`
- Verify: `src/components/conference_card.rs`

- [ ] **Step 1: Add or normalize homepage design tokens near the existing CSS variable section**

Define or refine shared tokens in `public/styles.css`:

```css
:root {
    --surface-panel: #121a24;
    --surface-elevated: #182230;
    --color-border-soft: rgba(255, 255, 255, 0.08);
    --color-border-light: rgba(255, 255, 255, 0.12);
    --color-text-primary: rgba(255, 255, 255, 0.96);
    --color-text-secondary: rgba(255, 255, 255, 0.72);
    --color-text-tertiary: rgba(255, 255, 255, 0.5);
    --color-accent: #7aa2ff;
    --radius-lg: 14px;
    --radius-xl: 20px;
    --shadow-md: 0 12px 32px rgba(0, 0, 0, 0.18);
    --shadow-lg: 0 18px 40px rgba(0, 0, 0, 0.24);
    --control-height: 38px;
}
```

If equivalent variables already exist, update them instead of duplicating them.

- [ ] **Step 2: Refine dark-theme page background and section spacing**

Update the homepage shell styles:

```css
.home {
    max-width: 1240px;
    margin: 0 auto;
    padding: 0 20px 40px;
}

html[data-theme="dark"] body {
    background:
        radial-gradient(circle at top, rgba(74, 107, 255, 0.14), transparent 34%),
        #0b1118;
}
```

- [ ] **Step 3: Add responsive breakpoints for tablet and mobile stacking**

Add responsive rules covering toolbar wrap, card stacking, and chip layout:

```css
@media (max-width: 980px) {
    .conference-card-row {
        grid-template-columns: 1fr;
    }

    .primary-toolbar-actions {
        width: 100%;
        justify-content: flex-start;
    }
}

@media (max-width: 720px) {
    .header-brand-row,
    .results-meta-summary,
    .category-chip-section {
        flex-direction: column;
        align-items: stretch;
    }

    .primary-toolbar,
    .primary-toolbar-actions,
    .category-chip-grid {
        width: 100%;
    }

    .conference-website-link {
        max-width: 100%;
    }
}
```

- [ ] **Step 4: Run the full homepage test/build verification**

Run: `cargo test && trunk build --release`

Expected: all Rust tests pass and the frontend production build succeeds.

- [ ] **Step 5: Commit the finish pass**

```bash
git add public/styles.css src/components/header.rs src/components/showtable.rs src/components/top_toolbar.rs src/components/results_meta.rs src/components/conference_card.rs src/components/countdown.rs src/components/mod.rs
git commit -m "feat: polish homepage redesign responsiveness"
```

---

## Manual test checklist

- [ ] Desktop: header shows title, short description, announcement bar, then disclaimer in that order.
- [ ] Desktop: top-right theme/GitHub/language controls still work and remain in the top-right cluster.
- [ ] Desktop: search input, filters, and subscribe button align as one toolbar.
- [ ] Desktop: field chips wrap cleanly and selected chips are visually distinct.
- [ ] Desktop: timezone text and result count appear in a dedicated summary row below the chips.
- [ ] Desktop: each conference entry reads as one card, with metadata on the left and the countdown panel on the right.
- [ ] Desktop: long conference names do not overlap the favorite button.
- [ ] Desktop: long URLs truncate visually inside the website capsule but still open correctly.
- [ ] Desktop: expired, TBD, and upcoming conferences still render their current countdown/deadline states correctly.
- [ ] Mobile/tablet: toolbar wraps cleanly without horizontal scrolling.
- [ ] Mobile/tablet: conference cards stack vertically and the countdown panel moves below metadata.
- [ ] Chinese/English: language switch still updates copy and chip labels without layout breakage.

## Notes for the implementing agent

- Do not change fetch logic, filter logic, local-storage behavior, favorite toggling, or subscription behavior unless a markup move strictly requires passing an unchanged prop through.
- Do not add a new “common fields” feature unless an equivalent helper already exists; this redesign is visual/structural, not a new product feature.
- Prefer editing existing classes over adding parallel one-off classes when a shared token or reusable surface rule can solve the problem.
- If `src/components/countdown.rs` already exposes sufficient hooks, leave it untouched and skip staging it in Tasks 6 and 7.
