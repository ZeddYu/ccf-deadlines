Task: Redesign the homepage UI of this repo to match the provided target screenshot as closely as reasonable, while preserving existing data flow and interactions.

Important constraint:
Do NOT change the top-right controls’ information architecture or behavior:
- language switcher
- GitHub button
- GitHub star area
You may only adjust their spacing/alignment slightly so they fit the new layout better.

Goal
Transform the current homepage from the existing layout into a cleaner structure closer to the target reference:
1. hero header with title + short description
2. separate latest update announcement bar
3. separate disclaimer line
4. unified search/filter/subscribe toolbar
5. cleaner field chips section
6. result summary / tools row
7. conference list as integrated cards, not visually split blocks
8. better countdown panel styling
9. improved dark theme consistency, spacing, hierarchy, and responsiveness

Scope
Please inspect and modify the homepage-related files only. Likely relevant files include:
- public/styles.css
- src/components/showtable.rs
- src/components/countdown.rs
- src/lib.rs
- any homepage/header/search/filter/card related components
If the actual file structure differs, locate the correct files yourself.

Do not:
- change backend/data source behavior
- change conference schema
- break search/filter/subscribe/favorite/language switching/link behavior
- introduce heavy new dependencies
- redesign unrelated pages

Required UI changes

A. Header / hero area
- Keep the main title: CCFDDL® Open Deadlines
- Under it, show a short description line similar to:
  Worldwide Conference Deadline Countdowns. To add or edit a conference, send a pull request.
- Remove the current overloaded top banner style where multiple unrelated texts are crammed together.
- Restructure the top into:
  1) title + short description
  2) latest update bar
  3) disclaimer line
- Preserve the top-right language/GitHub/star controls, only improve spacing/alignment.

B. Latest Update bar
- Convert the current latest update area into a full-width announcement bar similar to the target screenshot.
- Include:
  - subtle info icon
  - label: Latest Update:
  - update content text
  - optional NEW badge
- Style:
  - dark translucent card
  - subtle border
  - soft shadow
  - rounded corners
- Gracefully hide or simplify if no update text exists.

C. Disclaimer
- Move disclaimer out of the main banner.
- Render it as a lighter, secondary line below the update bar.
- Keep it visually weak and unobtrusive.

D. Search / filter toolbar
- Rebuild the main control row into a more unified toolbar:
  - left: wide search input with search icon
  - right: filter dropdowns
  - subscribe button as primary action
- Search input:
  - placeholder like Search conferences...
  - clearer focus ring
  - consistent height with dropdowns/buttons
- Dropdowns:
  - unified sizing
  - visually consistent with search input
- Subscribe button:
  - preserve current behavior
  - visually closer to the target screenshot
- Desktop first, but keep responsive wrapping on narrow screens.

E. Field chips section
- Make the topic/field pills more consistent:
  - consistent height
  - consistent spacing
  - clearer selected state
  - weaker unselected state
- Add a small action area on the right side if feasible:
  - Clear all
  - optional Select common fields entry if easy to implement
- If Select common fields logic does not exist, do not invent a large feature; either omit it or leave a small non-disruptive placeholder only if appropriate.
- Ensure both Chinese and English labels wrap gracefully.

F. Result summary row
- Create a row between chips and the conference list:
  - left: Deadlines are shown in Asia/Shanghai time.
  - right: result count, e.g. 52 conferences found.
- If view-toggle icons already exist or are easy to support, align them here; otherwise keep this area simple.
- Reposition the current result count from its existing awkward placement into this row.

G. Conference list card redesign
- Each conference entry should become one unified card, visually similar to the target screenshot.
- Avoid the current feeling of separate left/right disconnected blocks.
- Card layout on desktop:
  - left: conference metadata
  - right: countdown panel
- Card behavior:
  - subtle border
  - slightly larger radius
  - cleaner padding
  - mild hover highlight only
- Left side content hierarchy:
  1) conference short name + year
  2) date + location
  3) full conference name
  4) rank tags like CCF / CORE / THCPL
  5) field tags
  6) website link in a compact readable form
- Right side:
  1) prominent countdown number
  2) deadline text below
  3) progress/timeline bar below
- Favorite/star icon:
  - keep existing functionality
  - place it more like a top-right corner action inside the card
- Website link:
  - replace raw long inline text style with a more compact pill/inline capsule style
  - truncate long URLs visually but preserve click target

H. Countdown panel styling
- Redesign the countdown box to be closer to the target screenshot:
  - subtle gradient/dark panel
  - refined border
  - more polished progress bar
- Countdown number should have strongest emphasis
- Deadline text secondary
- Timezone/fine-print tertiary
- Keep existing urgency logic if present, but remap the visuals more elegantly
- Avoid overusing harsh red

I. Dark theme refinement
- Improve theme consistency:
  - deeper page background
  - slightly elevated card surfaces
  - low-contrast borders
  - controlled glow/focus treatment
- Introduce or refine design tokens if the project supports them:
  - background colors
  - card colors
  - border colors
  - primary/secondary text
  - accent color
  - warning color
  - radius values
  - shadow values
  - control heights
- Clean up current issues:
  - crowded hero area
  - inconsistent control styling
  - loose chip layout
  - poor relationship between timezone text and result count
  - conference item feels stitched together instead of designed as one card

J. Responsive behavior
- Desktop should be the main target matching the reference.
- Tablet:
  - toolbar may wrap to two lines
  - conference card may become stacked
- Mobile:
  - no horizontal scrolling
  - search bar on its own row
  - filters/buttons wrap cleanly
  - countdown panel stacks below metadata
  - chips wrap naturally and remain tappable

Implementation guidance
- Prefer minimal but clean refactors.
- Reuse existing components where possible.
- If needed, split homepage into clearer sections/components:
  - Hero/Header
  - AnnouncementBar
  - SearchToolbar
  - FilterChips
  - ResultToolbar
  - ConferenceCard
  - CountdownPanel
- Centralize styles/tokens instead of scattering magic numbers.
- Do not chase pixel-perfect duplication if it harms maintainability.
- Use the target screenshot as style/layout direction, but fix any obvious issues in the mockup instead of copying its bugs.

Acceptance criteria
- Homepage structure and visual hierarchy are clearly closer to the target screenshot than the current version.
- Top-right language/GitHub/star area remains functionally unchanged.
- Latest update, disclaimer, search toolbar, chips, result summary, and conference cards are clearly separated and visually coherent.
- Conference entries read as unified cards.
- Search, filters, subscribe, favorite, language switch, and links still work.
- No obvious text overflow, broken alignment, or inconsistent chip/button heights.
- Responsive layout remains usable.

Expected output
1. Implement the code changes directly.
2. Summarize which files were changed.
3. Explain what each major change improved.
4. Note any parts that could not fully match the reference and why.
5. Provide a short manual test checklist covering:
   - desktop
   - mobile
   - Chinese/English
   - long conference names
   - long URLs
   - expired vs upcoming deadlines

Execution process
- First inspect the homepage code and write a short implementation plan.
- Then make the changes.
- Keep changes focused on homepage UI only.
- Verify existing behavior is preserved after styling/layout refactor.
