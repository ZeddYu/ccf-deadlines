# Homepage Phase 2 Design

**Goal:** Implement SPEC.md Phase 2 in the current workspace by replacing the checkbox-heavy category area with chip-style filters and extracting conference list rows into a clearer card-based presentation, while preserving the existing homepage data flow and interactions.

**Constraints:**
- Do not use a worktree.
- Keep existing state ownership in `src/components/showtable.rs`.
- Preserve localStorage keys and compatibility.
- Do not change conference fetching, timezone computation, calendar URL generation, timeline data generation, or pagination semantics.
- Keep search, rank filters, favorites, subscription modal, timeline, calendar popover, language switch, and theme behavior working.

## Current context

Current homepage redesign context in this workspace:
- `SPEC.md` defines Phase 2 as category chips + conference card extraction on top of the Phase 1 homepage restructure.
- `src/components/showtable.rs` remains the homepage orchestration layer and is the state owner to preserve for this phase.
- `src/components/header.rs`, `src/components/top_toolbar.rs`, and `src/components/results_meta.rs` define the top-of-page structure that Phase 2 must build on without changing the underlying behavior.
- `src/components/category_chip.rs` and `src/components/conference_card.rs` are the intended presentation boundaries for this phase; the implementation plan must verify their current state and either create or finish wiring them as needed.
- `public/styles.css` already contains homepage styling and will be extended for chip and card presentation.

## Recommended approach

Use a **component extraction + moderate cleanup** approach:
- Keep `ShowTable` as the orchestration layer for all homepage state and business behavior.
- Extract or finish focused presentation components for category chips and conference cards.
- Allow small render-adjacent helper cleanup in `showtable.rs`, but do not restructure the fetching/filtering pipeline.
- Add basic polish for hover, selected states, spacing, and mobile stacking, but stop short of full Phase 3 visual refinement.

This is the best fit because it delivers Phase 2 cleanly without pushing the refactor into deeper architectural changes.

## Component design

### `src/components/showtable.rs`
Responsibilities:
- Continue to own homepage state and derived state.
- Keep search, category filter, CCF/CORE/THCPL filter, favorites, pagination, timezone label, and subscription modal control.
- Pass `ConfItem`, selected category state, and callbacks/signals into presentational components.
- Retain current localStorage and signal semantics.

Boundaries:
- No new source of truth for category state.
- No relocation of fetch logic or deadline computation logic.
- Only light cleanup of helper functions that are directly coupled to the extracted rendering.

### `src/components/category_chip.rs`
Responsibilities:
- Render a single clickable chip/pill.
- Accept display label, selected state, click handler, and language-related props as needed.
- Provide selected/hover/focus presentation only.

Boundaries:
- No business state ownership.
- No knowledge of “select all” or “clear all” semantics.
- `showtable.rs` remains responsible for toggling the `HashSet<String>` and utility actions.

### `src/components/conference_card.rs`
Responsibilities:
- Render one conference item as a structured card.
- Group content into title/meta/rank/meta/link/deadline sections.
- Expose favorite action, website link area, timeline, and calendar popover in the new layout.
- Include the required domain/meta group content from `SPEC.md`: category tag plus optional acceptance rate and optional note when available.

Boundaries:
- No filtering, sorting, pagination, or persistence ownership.
- No rewrite of timeline/calendar generation.
- Receives already-prepared item data and callbacks from `showtable.rs`.

### `public/styles.css`
Responsibilities:
- Add chip, utility action, card, tag group, and deadline panel styles.
- Support selected, hover, focus, and mobile stacked layouts for Phase 2.
- Reuse current variables/tokens and extend the existing style system.

Boundaries:
- Avoid global restyling outside the homepage areas touched by Phase 2.
- Do not front-load full Phase 3 polish.

## Data flow and interaction design

### Category chips
- Category selection continues to be represented by the existing category `HashSet<String>` in `showtable.rs`.
- Each chip toggles the matching `sub` value.
- Utility actions such as “Clear all” and any “select common fields” stub remain controlled by `showtable.rs`.
- Filtering semantics stay identical to the current implementation.

### Conference cards
- Card rendering consumes the same filtered, paginated conference list already produced in `showtable.rs`.
- Favorite toggles continue to update the current likes/localStorage flow.
- Timeline and calendar popover stay attached to each item using the existing logic.
- Search, rank filtering, and category filtering all remain upstream from the card component.

### Information hierarchy
Left side:
1. Conference short name / year with favorite control
2. Date and location
3. Full conference name
4. Rank tag group
5. Category/meta tag group, including the category tag and optional acceptance rate / note if available
6. Website link row

Right side:
1. Countdown summary
2. Exact deadline text with timezone clarity
3. Timeline visualization
4. Calendar popover

This is a presentation change only; deadline/time logic remains unchanged.

## Testing strategy

Use the existing lightweight regression style already present in the repo:
- Add/extend source-based assertions in Rust tests where appropriate.
- Prefer structural assertions that confirm extracted component usage and key layout hierarchy.
- Add small unit tests only if extraction naturally creates testable helper functions.

Coverage focus:
- `showtable.rs` renders `CategoryChip` and `ConferenceCard` in the expected sections.
- The old checkbox-heavy category presentation is no longer the dominant homepage category UI.
- `conference_card.rs` still exposes favorite/timeline/calendar-related entry points.
- `conference_card.rs` preserves the required category / acceptance-rate / note metadata area when available.
- `public/styles.css` contains key chip/card/deadline class hierarchy.
- Existing Phase 1 structural tests continue to pass.

Behavioral verification required before claiming completion:
- Search still works.
- CCF / CORE / THCPL filters still work.
- Category filters still work.
- Favorites still persist.
- Subscription modal still opens.
- Timeline still renders.
- Calendar popover still works.
- Pagination still works.
- Language switch still works.
- Dark/light theme still works.
- Homepage looks acceptable on both mobile and desktop.

## Acceptance criteria

Phase 2 is done when:
- Category filtering behavior matches the previous behavior.
- Category filters render as chip/pill controls instead of the old checkbox-heavy section.
- Conference rows render as structured cards with clearer hierarchy.
- Favorite toggle still works and persists.
- Timeline and calendar popover still work within the new card layout.
- Pagination behavior remains unchanged.
- Mobile layout stacks without obvious overflow or broken alignment.
- Visual polish reaches “behavior unchanged + basic polish,” but does not expand into full Phase 3 restyling.

## Implementation notes for the next planning step

The implementation plan should break the work into:
1. Category chip extraction/finalization and ShowTable integration
2. Conference card extraction/finalization and ShowTable integration
3. Phase 2 CSS layering and regression test updates
4. Verification commands before any completion claim

The implementation should happen directly in the current workspace, not in a worktree.
