You are implementing a homepage redesign for the repository ZeddYu/ccf-deadlines.

Context
- This project is a Leptos CSR app.
- The current home page is composed from Header + ShowTable.
- Most homepage state, filtering, search, favorites, pagination, subscription modal, and conference list rendering currently live in src/components/showtable.rs.
- Current styles are centralized in public/styles.css.
- The redesign goal is to keep the existing data flow and filtering behavior, but upgrade the homepage information architecture and UI to match a more product-oriented conference deadline search experience.

Important constraints
1. Do not rewrite the data model or the conference-fetching pipeline unless absolutely necessary.
2. Preserve existing localStorage keys and compatibility:
   - use_english
   - types
   - ranks
   - core_ranks
   - thcpl_ranks
   - likes
3. Reuse existing theme infrastructure and existing components where possible.
4. Prefer extracting small view components over making ShowTable longer.
5. Keep current behavior working:
   - search
   - category filtering
   - CCF / CORE / THCPL filtering
   - favorites
   - pagination
   - subscription modal
   - timeline
   - calendar popover
   - language switch
   - dark/light theme

High-level redesign goals
1. Reorganize the first screen into three levels:
   - header brand/actions
   - primary toolbar
   - category filters
2. Make search the primary visual entry.
3. Replace the current category checkbox-heavy feel with chip/pill-style filters.
4. Improve conference card hierarchy:
   - clearer title area
   - rank tags grouped separately from domain tags
   - cleaner deadline area on the right
5. Reduce heavy dark-mode border feel and rely more on surfaces/background layers.
6. Keep the redesign implementable without changing the core conference logic.

Target visual structure

A. Header
Refactor the current header into three rows:

1) brand row
- left:
  - title: CCFDDL Open Deadlines
- right:
  - theme toggle
  - GitHub button
  - language switch

2) subtitle row
- concise subtitle
- keep existing “send a pull request” type action
- keep tabular portal / wechat applet links if already present, but visually de-emphasize them

3) announcement row
- show latest update / latest commit information as a standalone announcement strip
- do not keep it mixed inline with action buttons

B. Main toolbar
Create a primary toolbar directly under the header:
- large search input
- CCF dropdown
- CORE dropdown
- THCPL dropdown
- Subscribe button

Create a secondary toolbar / meta row:
- timezone text
- result count
- clear filters action
- reserve room for future toggles like favorites only or upcoming only

C. Category filter section
- render categories as selectable chips/pills instead of visually dominant form-style checkboxes
- keep the same underlying behavior using the current category state
- add utility actions:
  - Clear all
  - Select common fields (can be stubbed as a utility action if needed)

D. Conference list
Keep the current list logic and pagination logic, but render each conference as a cleaner card-like row with clearer information hierarchy.

Conference card structure

Left side
1. title row
- conference short name / year
- favorite star aligned to the right

2. meta row
- date
- location

3. full conference name

4. rank tag group
- CCF / CORE / THCPL grouped together

5. domain/meta tag group
- category tag
- optional acceptance rate / note if available

6. website link row

Right side
1. large countdown summary
- prefer compact format such as day + hour
- avoid making seconds the dominant display

2. exact deadline line
- compact, readable format
- keep AoE / timezone clarity

3. existing timeline visualization
- keep it, but visually subordinate it to the countdown

4. keep calendar popover integration

Files to modify

Primary files
- src/components/header.rs
- src/components/showtable.rs
- public/styles.css

Suggested new components
- src/components/top_toolbar.rs
- src/components/category_chips.rs
- src/components/conference_card.rs
- src/components/results_meta.rs

Optional supporting edits
- src/components/countdown.rs
- src/components/checkbox_button.rs
- src/pages/home.rs

Detailed implementation tasks

Phase 1: structural refactor without changing business logic
1. Refactor Header into:
   - brand row
   - subtitle row
   - announcement row
2. Move language switch from the current ShowTable area into Header’s right-side action group.
3. Reorganize ShowTable top area into:
   - primary toolbar
   - secondary meta row
   - category chip section
4. Keep all current filtering/search logic intact.
5. Keep current list rendering working even before card extraction is complete.

Acceptance for Phase 1
- app compiles
- header is visually split into 3 layers
- search and main filters are above category filters
- language switch still works
- current filters still work

Phase 2: category chips + conference card extraction
1. Extract a category chip component that uses existing category data/state.
2. Replace the visually dominant checkbox section with chip/pill buttons.
3. Extract conference card rendering from ShowTable into a separate component.
4. Group tags into:
   - rank tags
   - category/meta tags
5. Keep favorites, website links, timeline, and calendar popover functional.

Acceptance for Phase 2
- category filtering behavior matches previous behavior
- conference rows render as structured cards
- favorite toggle still works and persists
- timeline and calendar popover still work
- pagination still works

Phase 3: style polish and responsive refinement
1. Add/adjust CSS for:
   - header layout
   - primary toolbar
   - chip-style category filters
   - conference card
   - deadline panel
   - utility/meta row
   - empty states
2. Reduce harsh border stacking in dark mode.
3. Improve hover/focus/selected states consistency.
4. Ensure responsive behavior:
   - mobile: stacked layout
   - desktop: aligned toolbars and two-column conference card feel
5. Make the search input the strongest visual entry point.

Acceptance for Phase 3
- dark mode looks cleaner and less border-heavy
- mobile layout does not overflow or collapse awkwardly
- desktop alignment is consistent
- selected/hover/focus states are visually coherent

Implementation notes

State ownership
- Keep state in ShowTable for now.
- Pass signals/callbacks into extracted components rather than moving business logic immediately.

Search
- Keep current search behavior unless a very small extension is easy.
- UI should be upgraded with:
  - search icon
  - clear button
  - stronger focus state
  - wider input

Category chips
- Entire chip must be clickable.
- Use selected visual treatment with either filled or emphasized outlined state.
- Preserve current underlying category filter semantics.

Dropdowns
- Reuse existing dropdown logic where possible.
- Only improve trigger layout and styling so they align with the new toolbar.

Countdown
- If practical, add a compact display mode to CountDown so the homepage card can show a calmer version.
- Do not break existing urgency logic.

Styling rules
- Reuse existing theme/tokens where possible.
- Do not add a new styling framework.
- Prefer extending/reworking public/styles.css.
- Avoid excessive shadows.
- Use surface/background hierarchy more than stacked borders.

Do not change unless required
- conference fetching logic
- timezone computation logic
- calendar URL generation logic
- timeline data generation logic
- existing localStorage schema

Deliverables
1. Working code changes
2. New extracted components
3. Updated styles
4. Brief implementation notes in the final PR description explaining:
   - what was changed
   - what was intentionally preserved
   - any follow-up opportunities

Testing checklist
Before finishing, verify:
- search works
- CCF/CORE/THCPL filters work
- category filters work
- favorites persist
- subscription modal opens
- timeline renders
- calendar popover works
- pagination works
- language switch works
- dark/light theme works
- homepage looks acceptable on mobile and desktop

Preferred PR split
- PR1: Header + toolbar restructure
- PR2: Category chips + conference card extraction
- PR3: Style polish + responsive refinement