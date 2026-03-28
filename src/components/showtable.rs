use crate::components::category_chip::CategoryChip;
use crate::components::checkbox_button::*;
use crate::components::conf::ConfItem;
use crate::components::conf::*;
use crate::components::conference_card::ConferenceCard;
use crate::components::results_meta::ResultsMeta;
use crate::components::subscription_modal::*;
use crate::components::timezone::*;
use crate::components::top_toolbar::TopToolbar;
use chrono::{DateTime, FixedOffset};
use leptos::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::OnceLock;
use thaw::*;
use urlencoding::encode;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, window};

fn are_all_categories_selected(categories: &[Category], selected: &HashSet<String>) -> bool {
    !categories.is_empty() && categories.iter().all(|category| selected.contains(&category.sub))
}

fn all_category_subs(categories: &[Category]) -> HashSet<String> {
    categories.iter().map(|category| category.sub.clone()).collect()
}

fn toggle_category_selection(selected: &mut HashSet<String>, sub: &str) {
    if !selected.insert(sub.to_string()) {
        selected.remove(sub);
    }
}

fn has_active_phase1_filters(
    input_value: &str,
    check_list: &HashSet<String>,
    rank_list: &HashSet<String>,
    core_rank_list: &HashSet<String>,
    thcpl_rank_list: &HashSet<String>,
) -> bool {
    !input_value.trim().is_empty()
        || !check_list.is_empty()
        || !rank_list.is_empty()
        || !core_rank_list.is_empty()
        || !thcpl_rank_list.is_empty()
}

fn clear_phase1_filters(
    input_value: RwSignal<String>,
    check_list: RwSignal<HashSet<String>>,
    rank_list: RwSignal<HashSet<String>>,
    core_rank_list: RwSignal<HashSet<String>>,
    thcpl_rank_list: RwSignal<HashSet<String>>,
) {
    input_value.set(String::new());
    check_list.set(HashSet::new());
    rank_list.set(HashSet::new());
    core_rank_list.set(HashSet::new());
    thcpl_rank_list.set(HashSet::new());
}

fn filter_conferences(
    conferences: Vec<ConfItem>,
    check_list: &HashSet<String>,
    input_value: &str,
    rank_list: &HashSet<String>,
    core_rank_list: &HashSet<String>,
    thcpl_rank_list: &HashSet<String>,
) -> Vec<ConfItem> {
    let mut filtered_list = conferences;

    if !check_list.is_empty() {
        filtered_list.retain(|item| check_list.contains(&item.sub.to_uppercase()));
    }

    if !rank_list.is_empty() {
        filtered_list.retain(|item| rank_list.contains(&item.rank));
    }

    if !core_rank_list.is_empty() {
        filtered_list.retain(|item| {
            let core_rank = item.corerank.as_deref().unwrap_or("N");
            core_rank_list.contains(core_rank)
        });
    }

    if !thcpl_rank_list.is_empty() {
        filtered_list.retain(|item| {
            let thcpl_rank = item.thcplrank.as_deref().unwrap_or("N");
            thcpl_rank_list.contains(thcpl_rank)
        });
    }

    if !input_value.is_empty() {
        let input_lower = input_value.to_lowercase();
        filtered_list.retain(|item| {
            item.id.to_lowercase().contains(&input_lower)
                || item.title.to_lowercase().contains(&input_lower)
        });
    }

    filtered_list
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::FixedOffset;

    fn categories() -> Vec<Category> {
        vec![
            Category {
                name: "人工智能".to_string(),
                name_en: "Artificial Intelligence".to_string(),
                sub: "AI".to_string(),
            },
            Category {
                name: "数据库".to_string(),
                name_en: "Database".to_string(),
                sub: "DB".to_string(),
            },
        ]
    }

    fn sample_conf(id: &str, status: &str, is_like: bool) -> ConfItem {
        ConfItem {
            title: format!("Conf {id}"),
            description: "A conference".to_string(),
            sub: "AI".to_string(),
            rank: "A".to_string(),
            corerank: Some("A*".to_string()),
            thcplrank: Some("B".to_string()),
            displayrank: "CCF A".to_string(),
            dblp: "sigmod".to_string(),
            year: 2026,
            id: id.to_string(),
            link: "https://example.com".to_string(),
            abstract_deadline: None,
            deadline: "2026-04-01 23:59:59".to_string(),
            comment: None,
            timezone: "UTC+0".to_string(),
            date: "Apr 1-3, 2026".to_string(),
            place: "Paris".to_string(),
            status: status.to_string(),
            is_like,
            remain: 42,
            local_ddl: Some("2026-04-01 23:59:59 UTC+0".to_string()),
            origin_ddl: Some("2026-04-01 23:59:59 UTC+0".to_string()),
            subname: "人工智能".to_string(),
            subname_en: "Artificial Intelligence".to_string(),
            google_calendar_url: Some("https://calendar.google.com".to_string()),
            icloud_calendar_url: Some("https://icloud.com".to_string()),
            acc_str: Some("25%".to_string()),
            ddls: vec![TimePoint {
                timepoint: chrono::DateTime::parse_from_rfc3339("2026-04-01T23:59:59+00:00")
                    .unwrap()
                    .with_timezone(&FixedOffset::east_opt(0).unwrap()),
                r#type: 1,
            }],
        }
    }

    #[test]
    fn detects_when_all_categories_are_selected() {
        let selected = HashSet::from(["AI".to_string(), "DB".to_string()]);

        assert!(are_all_categories_selected(&categories(), &selected));
    }

    #[test]
    fn phase2_category_chip_section_uses_category_chip_component() {
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

        let chip_section = index_of(component_source, "class=\"category-chip-section\"");
        let actions_row = index_of(component_source, "class=\"category-actions-row\"");
        let chip_grid = index_of(component_source, "class=\"category-chip-grid\"");
        let action_chip = index_of(component_source, "\"category-chip category-chip-action\"");
        let category_chip = index_of(component_source, "<CategoryChip");
        let table_container = index_of(component_source, "class=\"table-container\"");

        assert!(chip_section < table_container);
        assert!(chip_section < actions_row);
        assert!(actions_row < chip_grid);
        assert!(actions_row < category_chip);
        assert!(action_chip < category_chip);
    }

    #[test]
    fn phase2_conference_list_uses_conference_card_component() {
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

        let filtered_list = index_of(component_source, "let filtered_list = Memo::new");
        let paginated_list = index_of(component_source, "let paginated_list = Memo::new");
        let empty_state = index_of(component_source, "No data available.");
        let conference_card = index_of(component_source, "<ConferenceCard");
        let table_body = index_of(component_source, "<TableBody>");

        assert!(filtered_list < paginated_list);
        assert!(paginated_list < table_body);
        assert!(table_body < conference_card);
        assert!(empty_state < conference_card);
    }

    #[test]
    fn category_selection_helpers_preserve_toggle_all_and_empty_state_contract() {
        let categories = categories();
        let mut selected = HashSet::new();

        assert!(selected.is_empty());
        assert!(!are_all_categories_selected(&categories, &selected));

        toggle_category_selection(&mut selected, "AI");
        assert_eq!(selected, HashSet::from(["AI".to_string()]));
        assert!(!are_all_categories_selected(&categories, &selected));

        toggle_category_selection(&mut selected, "AI");
        assert!(selected.is_empty());

        selected = all_category_subs(&categories);
        assert!(are_all_categories_selected(&categories, &selected));
    }

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
        let secondary_meta = index_of(component_source, "class=\"secondary-meta-row\"");
        let category_chips = index_of(component_source, "class=\"category-chip-section\"");
        let table_container = index_of(component_source, "class=\"table-container\"");

        assert!(primary_toolbar < secondary_meta);
        assert!(secondary_meta < category_chips);
        assert!(category_chips < table_container);
    }

    #[test]
    fn results_meta_component_is_used() {
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

        let results_meta = index_of(component_source, "<ResultsMeta");
        let secondary_meta = index_of(component_source, "class=\"secondary-meta-row\"");
        let category_chips = index_of(component_source, "class=\"category-chip-section\"");

        assert!(results_meta < secondary_meta);
        assert!(secondary_meta < category_chips);
    }

    #[test]
    fn top_toolbar_component_is_used() {
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

        let top_toolbar = index_of(component_source, "<TopToolbar");
        let category_chips = index_of(component_source, "class=\"category-chip-section\"");

        assert!(top_toolbar < category_chips);
    }

    #[test]
    fn phase1_toolbar_meta_and_chip_sections_have_distinct_css_hierarchy() {
        const STYLES_SOURCE: &str = include_str!("../../public/styles.css");

        assert!(STYLES_SOURCE.contains(".primary-toolbar {"));
        assert!(STYLES_SOURCE.contains(".secondary-meta-row {"));
        assert!(STYLES_SOURCE.contains(".category-chip-section {"));
        assert!(STYLES_SOURCE.contains(".results-meta-actions {"));
        assert!(STYLES_SOURCE.contains(".clear-filters-button {"));
    }

    #[test]
    fn phase3_conference_card_contract_preserves_deadline_and_timeline_structure() {
        const CARD_SOURCE: &str = include_str!("conference_card.rs");

        assert!(CARD_SOURCE.contains("<CountDown remain compact=true />"));
        assert!(CARD_SOURCE.contains("class=\"conference-deadline-panel"));
        assert!(CARD_SOURCE.contains("<CalendarPopover"));
        assert!(CARD_SOURCE.contains("format!(\"Deadline: {}\", show_ddl_str)"));
        assert!(CARD_SOURCE.contains("class=\"conference-website-label\">\"Website\"</span>"));
        assert!(CARD_SOURCE.contains("<TimeLine time_points=ddls />"));
    }

    #[test]
    fn phase3_css_hierarchy_is_defined() {
        const STYLES_SOURCE: &str = include_str!("../../public/styles.css");

        assert!(STYLES_SOURCE.contains(".hero-header {"));
        assert!(STYLES_SOURCE.contains(".primary-toolbar {"));
        assert!(STYLES_SOURCE.contains(".primary-toolbar-main {"));
        assert!(STYLES_SOURCE.contains(".timezone-search {"));
        assert!(STYLES_SOURCE.contains(".secondary-meta-row {"));
        assert!(STYLES_SOURCE.contains(".results-meta-actions {"));
        assert!(STYLES_SOURCE.contains(".category-chip-section {"));
        assert!(STYLES_SOURCE.contains(".conference-card-shell {"));
        assert!(STYLES_SOURCE.contains(".conference-card-main,"));
        assert!(STYLES_SOURCE.contains(".conference-tag-groups {"));
        assert!(STYLES_SOURCE.contains(".conference-deadline-panel,"));
        assert!(STYLES_SOURCE.contains(".table-no-results {"));
        assert!(STYLES_SOURCE.contains(".table-no-results.empty-state-text {"));
        assert!(STYLES_SOURCE.contains(".countdown-compact {"));
        assert!(STYLES_SOURCE.contains("@media (max-width: 640px) {"));
        assert!(STYLES_SOURCE.contains(".primary-toolbar-actions > * {"));
        assert!(STYLES_SOURCE.contains(".conference-deadline-panel .countdown-display,"));
    }

    #[test]
    fn clears_search_category_and_rank_filters_without_touching_likes_or_pagination() {
        let input_value = RwSignal::new("vision".to_string());
        let check_list = RwSignal::new(HashSet::from(["AI".to_string()]));
        let rank_list = RwSignal::new(HashSet::from(["A".to_string()]));
        let core_rank_list = RwSignal::new(HashSet::from(["A*".to_string()]));
        let thcpl_rank_list = RwSignal::new(HashSet::from(["B".to_string()]));
        let like_list = RwSignal::new(HashSet::from(["liked-conf".to_string()]));
        let page = RwSignal::new(3);

        clear_phase1_filters(
            input_value,
            check_list,
            rank_list,
            core_rank_list,
            thcpl_rank_list,
        );

        assert!(input_value.get().is_empty());
        assert!(check_list.get().is_empty());
        assert!(rank_list.get().is_empty());
        assert!(core_rank_list.get().is_empty());
        assert!(thcpl_rank_list.get().is_empty());
        assert_eq!(like_list.get(), HashSet::from(["liked-conf".to_string()]));
        assert_eq!(page.get(), 3);
    }

    #[test]
    fn detects_when_phase1_filters_are_active() {
        let empty = HashSet::new();
        let selected = HashSet::from(["AI".to_string()]);

        assert!(!has_active_phase1_filters("", &empty, &empty, &empty, &empty));
        assert!(has_active_phase1_filters("vision", &empty, &empty, &empty, &empty));
        assert!(has_active_phase1_filters("", &selected, &empty, &empty, &empty));
    }

    #[test]
    fn secondary_meta_row_exposes_result_count_and_clear_filters_action() {
        const RESULTS_META_SOURCE: &str = include_str!("results_meta.rs");

        assert!(RESULTS_META_SOURCE.contains("class=\"results-count-message\""));
        assert!(RESULTS_META_SOURCE.contains("class=\"clear-filters-button\""));
    }

    #[test]
    fn current_result_count_uses_filtered_list_without_cloning_for_len() {
        const SHOWTABLE_SOURCE: &str = include_str!("showtable.rs");

        let showtable_start = SHOWTABLE_SOURCE
            .rfind("pub fn ShowTable(use_english: RwSignal<bool>) -> impl IntoView {")
            .expect("expected ShowTable component definition");
        let showtable_end = SHOWTABLE_SOURCE
            .rfind("static UTC_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();")
            .expect("expected end of ShowTable component source");
        let component_source = &SHOWTABLE_SOURCE[showtable_start..showtable_end];

        assert!(component_source.contains(
            "filtered_list.with(|filtered_list| filtered_list.len())"
        ));
    }

    #[test]
    fn filtered_result_count_helper_is_removed() {
        const SHOWTABLE_SOURCE: &str = include_str!("showtable.rs");
        let tests_start = SHOWTABLE_SOURCE
            .find("#[cfg(test)]")
            .expect("expected test module");
        let production_source = &SHOWTABLE_SOURCE[..tests_start];

        assert!(!production_source.contains("fn filtered_result_count("));
    }

    #[test]
    fn computes_filtered_result_count_from_same_filtered_list_used_for_pagination() {
        let conferences = vec![
            sample_conf("liked-run", "RUN", true),
            sample_conf("other-run", "RUN", false),
            sample_conf("finished", "FIN", false),
        ];
        let empty = HashSet::new();
        let query = String::from("liked");
        let filtered = filter_conferences(
            conferences,
            &empty,
            query.as_str(),
            &empty,
            &empty,
            &empty,
        );

        assert_eq!(filtered.len(), 1);
    }
}

#[component]
pub fn ShowTable(use_english: RwSignal<bool>) -> impl IntoView {
    // mobile
    let is_mobile = RwSignal::new(false);
    let show_filters = RwSignal::new(false);

    // checkbox
    let sub_list = RwSignal::new(get_categories());
    let cached_check_list: HashSet<String> = get_from_local_storage("types")
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_else(|| HashSet::new());
    let check_list = RwSignal::new(cached_check_list);
    let is_all_checked_memo = Memo::new(move |_| {
        are_all_categories_selected(&sub_list.get(), &check_list.get())
    });

    let is_all_checked = RwSignal::new(false);

    Effect::new(move |_| {
        is_all_checked.set(is_all_checked_memo.get());
    });

    let handle_check_all = move |_| {
        if is_all_checked_memo.get_untracked() {
            check_list.set(HashSet::new());
        } else {
            let all_subs = all_category_subs(&sub_list.get_untracked());
            check_list.set(all_subs);
        }
    };

    // input
    let input_value = RwSignal::new(String::new());

    // checkboxbutton
    let mut cached_rank_list: HashSet<String> = get_from_local_storage("ranks")
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_else(|| HashSet::new());
    normalize_rank_filter_selection(&mut cached_rank_list);
    let rank_list = RwSignal::new(cached_rank_list);
    let mut cached_core_rank_list: HashSet<String> = get_from_local_storage("core_ranks")
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_else(|| HashSet::new());
    normalize_rank_filter_selection(&mut cached_core_rank_list);
    let core_rank_list = RwSignal::new(cached_core_rank_list);
    let mut cached_thcpl_rank_list: HashSet<String> = get_from_local_storage("thcpl_ranks")
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_else(|| HashSet::new());
    normalize_rank_filter_selection(&mut cached_thcpl_rank_list);
    let thcpl_rank_list = RwSignal::new(cached_thcpl_rank_list);
    let open_dropdown = RwSignal::new(None::<String>);

    // liked
    let cached_like_list: HashSet<String> = get_from_local_storage("likes")
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_else(|| HashSet::new());
    let like_list = RwSignal::new(cached_like_list);

    let show_subscription_modal = RwSignal::new(false);

    // pagination
    let page = RwSignal::new(1);
    let page_size = RwSignal::new(10);
    let page_count = RwSignal::new(1);
    let is_filter_change = RwSignal::new(false);

    // table
    let all_conf_list = RwSignal::new(Vec::<ConfItem>::new());

    let time_zone = RwSignal::new(String::new());

    Effect::new(move |_| {
        let _ = check_list.get();
        let _ = input_value.get();
        let _ = rank_list.get();
        let _ = core_rank_list.get();
        let _ = thcpl_rank_list.get();

        if is_filter_change.get_untracked() {
            page.set(1);
        } else {
            is_filter_change.set(true);
        }
    });

    Effect::new(move |_| {
        set_in_local_storage("use_english", &use_english.get().to_string());
        set_in_local_storage("types", &serde_json::to_string(&check_list.get()).unwrap());
        set_in_local_storage("ranks", &serde_json::to_string(&rank_list.get()).unwrap());
        set_in_local_storage(
            "core_ranks",
            &serde_json::to_string(&core_rank_list.get()).unwrap(),
        );
        set_in_local_storage(
            "thcpl_ranks",
            &serde_json::to_string(&thcpl_rank_list.get()).unwrap(),
        );
    });

    Effect::new(move |_| {
        set_in_local_storage("likes", &serde_json::to_string(&like_list.get()).unwrap());
    });

    Effect::new(move |_| {
        let _ = check_list.get();
        let _ = input_value.get();
        let _ = rank_list.get();
        let _ = core_rank_list.get();
        let _ = thcpl_rank_list.get();
        let _ = page.get();

        let (current_time, _) = get_browser_time_and_timezone();
        let utc_map = load_utc_map();

        all_conf_list.update(|conferences| {
            for item in conferences.iter_mut() {
                if item.deadline != "TBD" {
                    let tz_str = normalize_timezone(&item.timezone);

                    if let Some(tz_offset) = utc_map.get(&tz_str) {
                        let ddl_str = parse_deadline_to_rfc3339(&item.deadline, tz_offset);

                        if let Ok(ddl_datetime) = DateTime::parse_from_rfc3339(&ddl_str) {
                            let diff = ddl_datetime.signed_duration_since(current_time);
                            if diff.num_milliseconds() <= 0 {
                                item.remain = 0;
                                item.status = "FIN".to_string();
                            } else {
                                item.remain = diff.num_milliseconds() as u64;
                                item.status = "RUN".to_string();
                            }
                        }
                    }
                }
            }
        });
    });

    Effect::new(move || {
        // mobile check
        is_mobile.set(is_mobile_device());

        // timezone
        time_zone.set(get_timezone_name().unwrap());

        spawn_local(async move {
            let utc_map = load_utc_map();
            let rank_options: HashMap<&str, &str> = RANK_OPTIONS.iter().cloned().collect();

            let (current_time, current_timezone) = get_browser_time_and_timezone();

            // base_url
            let window = web_sys::window().unwrap();
            let location = window.location();
            let base_url = location.origin().unwrap();

            match fetch_all_conf(&base_url).await {
                Ok(conferences) => {
                    let mut conf_vec = Vec::new();

                    for conf in conferences {
                        let conf_items = conf.confs.iter().map(|year_conf| {
                            let mut flag = false;
                            let len = year_conf.timeline.len();
                            let mut cur_deadline = year_conf.timeline[len - 1].deadline.clone();
                            let mut cur_abstract_deadline =
                                year_conf.timeline[len - 1].abstract_deadline.clone();
                            let mut cur_comment = year_conf.timeline[len - 1].comment.clone();
                            let mut ddl_vec = Vec::<TimePoint>::new();

                            for timeline_item in year_conf.timeline.iter() {
                                let tz_offset = utc_map.get(&year_conf.timezone).unwrap();

                                let ddl_str = parse_deadline_to_rfc3339(&timeline_item.deadline, tz_offset);

                                // abstract type:0 submission type:1
                                if let Some(abs_ddl) = timeline_item.abstract_deadline.clone() {
                                    let abs_ddl_str = parse_deadline_to_rfc3339(&abs_ddl, tz_offset);

                                    if let Ok(abs_ddl_datetime) =
                                        DateTime::parse_from_rfc3339(&abs_ddl_str)
                                    {
                                        ddl_vec.push(TimePoint {
                                            timepoint: abs_ddl_datetime
                                                .with_timezone(&current_timezone)
                                                .clone(),
                                            r#type: 0,
                                        });
                                    }
                                }

                                if let Ok(ddl_datetime) = DateTime::parse_from_rfc3339(&ddl_str) {
                                    ddl_vec.push(TimePoint {
                                        timepoint: ddl_datetime
                                            .with_timezone(&current_timezone)
                                            .clone(),
                                        r#type: 1,
                                    });

                                    let diff = ddl_datetime.signed_duration_since(current_time);
                                    if !flag && diff.num_milliseconds() > 0 {
                                        cur_deadline = timeline_item.deadline.clone();
                                        cur_abstract_deadline =
                                            timeline_item.abstract_deadline.clone();
                                        cur_comment = timeline_item.comment.clone();
                                        flag = true;
                                    }
                                }
                            }

                            ConfItem {
                                title: conf.title.clone(),
                                description: conf.description.clone(),
                                sub: conf.sub.clone(),
                                rank: conf.rank.ccf.clone(),
                                corerank: conf.rank.core.clone(),
                                thcplrank: conf.rank.thcpl.clone(),
                                displayrank: rank_options
                                    .get(conf.rank.ccf.as_str())
                                    .unwrap()
                                    .to_string(),
                                dblp: conf.dblp.clone(),
                                year: year_conf.year,
                                id: year_conf.id.clone(),
                                link: year_conf.link.clone(),
                                abstract_deadline: cur_abstract_deadline,
                                deadline: cur_deadline,
                                comment: cur_comment,
                                timezone: year_conf.timezone.clone(),
                                date: year_conf.date.clone(),
                                place: year_conf.place.clone(),
                                status: "".to_string(), // Placeholder, should be determined based on current date
                                is_like: like_list.get_untracked().contains(&year_conf.id),
                                remain: 0,
                                local_ddl: None,
                                origin_ddl: None,
                                subname: "".to_string(),
                                subname_en: "".to_string(),
                                google_calendar_url: None,
                                icloud_calendar_url: None,
                                acc_str: None,
                                ddls: ddl_vec,
                            }
                        });
                        conf_vec.extend(conf_items);
                    }

                    for item in conf_vec.iter_mut() {
                        // subname
                        if let Some(matched_category) = sub_list
                            .get_untracked()
                            .iter()
                            .find(|sub_item| sub_item.sub == item.sub)
                        {
                            item.subname = matched_category.name.clone();
                            item.subname_en = matched_category.name_en.clone();
                        }

                        if item.deadline == "TBD" {
                            item.remain = 0;
                            item.status = "TBD".to_string();
                            continue;
                        }

                        let tz_str = normalize_timezone(&item.timezone);

                        // 4. Calculate deadlines and remaining time
                        if let Some(tz_offset) = utc_map.get(&tz_str) {
                            let ddl_str = parse_deadline_to_rfc3339(&item.deadline, tz_offset);

                            if let Ok(ddl_datetime) = DateTime::parse_from_rfc3339(&ddl_str) {
                                // Convert to browser local time and format
                                let local_ddl_datetime =
                                    ddl_datetime.with_timezone(&current_timezone);
                                let formatted_date_time =
                                    local_ddl_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                                let offset_seconds = local_ddl_datetime.offset().local_minus_utc();
                                let offset_hours = offset_seconds / 3600;
                                let formatted_timezone = format!("UTC{:+}", offset_hours);

                                item.local_ddl =
                                    Some(format!("{} {}", formatted_date_time, formatted_timezone));
                                item.origin_ddl =
                                    Some(format!("{} {}", item.deadline, item.timezone));

                                // Handle abstract deadline
                                if let Some(abs_ddl) = &item.abstract_deadline {
                                    let abs_ddl_str = parse_deadline_to_rfc3339(abs_ddl, tz_offset);
                                    if let Ok(abs_datetime) =
                                        DateTime::parse_from_rfc3339(&abs_ddl_str)
                                    {
                                        let formatted_abs_ddl = abs_datetime
                                            .with_timezone(&current_timezone)
                                            .format("%b %e, %Y")
                                            .to_string();
                                        if item.comment.is_none() {
                                            item.comment = Some(format!(
                                                "abstract deadline on {}.",
                                                formatted_abs_ddl
                                            ));
                                        }
                                    }
                                }

                                let diff = ddl_datetime.signed_duration_since(current_time);
                                if diff.num_milliseconds() <= 0 {
                                    item.remain = 0;
                                    item.status = "FIN".to_string();
                                } else {
                                    item.remain = diff.num_milliseconds() as u64;
                                    item.status = "RUN".to_string();
                                }

                                let iso_string =
                                    local_ddl_datetime.format("%Y%m%dT%H%M%S").to_string();

                                item.google_calendar_url = Some(format!(
                                    "https://www.google.com/calendar/render?action=TEMPLATE&text={}&dates={}/{}&details={:?}&location=Online&ctz={}&sf=true&output=xml",
                                    encode(&format!("{} {}", item.title, item.year)),
                                    iso_string,
                                    iso_string,
                                    encode(&format!(
                                        "{} {}",
                                        item.comment.as_ref().map_or("".to_string(), |c| c.clone()),
                                        "provided by @ccfddl".to_string()
                                    )),
                                    time_zone.get_untracked(),
                                ));

                                item.icloud_calendar_url = Some(format!(
                                    "data:text/calendar;charset=utf8,BEGIN:VCALENDAR\n\
                                    VERSION:2.0\n\
                                    BEGIN:VEVENT\n\
                                    URL:{}\n\
                                    DTSTART:{}\n\
                                    DTEND:{}\n\
                                    SUMMARY:{}\n\
                                    DESCRIPTION:{}\n\
                                    LOCATION:{}\n\
                                    END:VEVENT\n\
                                    END:VCALENDAR",
                                    encode("https://ccfddl.github.io/"),
                                    iso_string,
                                    iso_string,
                                    encode(&format!("{} {} Deadline", item.title, item.year)),
                                    encode(item.comment.as_ref().map_or("", |c| c.as_str())),
                                    encode(""),
                                ));
                            }
                        }
                    }
                    all_conf_list.set(conf_vec);
                }
                Err(e) => {
                    console::error_1(&format!("Error: {:?}", e).into());
                }
            }

            match fetch_all_acc(&base_url).await {
                Ok(all_acc) => {
                    for acc_item in all_acc {
                        for cur_acc in &acc_item.accept_rates {
                            all_conf_list.update(|conferences| {
                                for item in conferences.iter_mut() {
                                    for y in 1..=3 {
                                        if item.title == acc_item.title
                                            && item.year == cur_acc.year + y
                                        {
                                            item.acc_str = Some(cur_acc.str.clone());
                                        }
                                    }
                                }
                            });
                        }
                    }
                }
                Err(e) => {
                    console::error_1(&format!("Error: {:?}", e).into());
                }
            }
        });
    });

    let filtered_list = Memo::new(move |_| {
        filter_conferences(
            all_conf_list.get(),
            &check_list.get(),
            &input_value.get(),
            &rank_list.get(),
            &core_rank_list.get(),
            &thcpl_rank_list.get(),
        )
    });

    let has_active_filters = Memo::new(move |_| {
        has_active_phase1_filters(
            &input_value.get(),
            &check_list.get(),
            &rank_list.get(),
            &core_rank_list.get(),
            &thcpl_rank_list.get(),
        )
    });

    let current_result_count =
        Memo::new(move |_| filtered_list.with(|filtered_list| filtered_list.len()));

    let paginated_list = Memo::new(move |_| {
        let filtered_list = filtered_list.get();

        // Sorting and Grouping
        let mut run_list: Vec<_> = filtered_list
            .iter()
            .filter(|item| item.status == "RUN".to_string())
            .cloned()
            .collect();
        let tbd_list: Vec<_> = filtered_list
            .iter()
            .filter(|item| item.status == "TBD".to_string())
            .cloned()
            .collect();
        let mut fin_list: Vec<_> = filtered_list
            .iter()
            .filter(|item| item.status == "FIN".to_string())
            .cloned()
            .collect();

        run_list.sort_by(|a, b| a.remain.cmp(&b.remain));
        fin_list.sort_by(|a, b| b.year.cmp(&a.year));

        let mut all_list = Vec::new();
        all_list.extend(run_list);
        all_list.extend(tbd_list);
        all_list.extend(fin_list);

        let (liked_list, unliked_list): (Vec<_>, Vec<_>) =
            all_list.into_iter().partition(|conf| conf.is_like);

        let mut final_list = liked_list;
        final_list.extend(unliked_list);

        // Pagination
        let total_count = final_list.len();
        let page_val = page.get();
        let page_size_val = page_size.get();
        let start = (page_val - 1) as usize * page_size_val as usize;
        let end = (start + page_size_val as usize).min(total_count);
        page_count.set((total_count + page_size_val - 1) / page_size_val);

        let paginated_list: Vec<ConfItem> = if start < total_count {
            final_list[start..end].to_vec()
        } else {
            Vec::new()
        };

        paginated_list
    });

    let select_all_name = Memo::new(move |_| {
        if use_english.get() {
            "Select All".to_string()
        } else {
            "全选".to_string()
        }
    });

    view! {
        <section>
            <TopToolbar
                input_value=input_value
                rank_list=rank_list
                core_rank_list=core_rank_list
                thcpl_rank_list=thcpl_rank_list
                open_dropdown=open_dropdown
                show_filters=show_filters
                show_subscription_modal=show_subscription_modal
                is_mobile=is_mobile
                use_english=use_english
            />

            <ResultsMeta
                class="secondary-meta-row"
                timezone_label=Signal::derive(move || time_zone.get())
                result_count=Signal::derive(move || current_result_count.get())
                has_active_filters=Signal::derive(move || has_active_filters.get())
                on_clear_filters=Callback::new(move |_| {
                    clear_phase1_filters(
                        input_value,
                        check_list,
                        rank_list,
                        core_rank_list,
                        thcpl_rank_list,
                    );
                })
                use_english=use_english
            />

            <div class="category-chip-section">
                <div class="category-actions-row">
                    <button
                        type="button"
                        class=move || {
                            if is_all_checked.get() {
                                "category-chip category-chip-selected category-chip-action"
                            } else {
                                "category-chip category-chip-action"
                            }
                        }
                        aria-pressed=move || is_all_checked.get().to_string()
                        on:click=handle_check_all
                    >
                        {move || select_all_name.get()}
                    </button>
                </div>

                <div class="category-chip-grid">
                    <For
                        each=move || { sub_list.get().into_iter().collect::<Vec<Category>>() }
                        key=|item| item.sub.clone()
                        children=move |item| {
                            let sub = item.sub.clone();

                            view! {
                                <CategoryChip
                                    category=item
                                    selected=Signal::derive(move || check_list.get().contains(&sub))
                                    use_english=use_english.into()
                                    is_mobile=is_mobile.into()
                                    on_toggle=Callback::new(move |value: String| {
                                        check_list.update(|selected| {
                                            toggle_category_selection(selected, &value);
                                        });
                                    })
                                />
                            }
                        }
                    />
                </div>
            </div>

            <SubscriptionModal
                show=show_subscription_modal
                use_english=use_english
                check_list=check_list
                rank_list=rank_list
                core_rank_list=core_rank_list
                thcpl_rank_list=thcpl_rank_list
            />

            <div class="zonedivider" />
            <div class="table-container">
                <Table>
                    <TableBody>
                        {move || {
                            if paginated_list.get().is_empty() {
                                view! {
                                    <TableRow>
                                        <TableCell>
                                            <div class="table-no-results empty-state-text">
                                                "No data available."
                                            </div>
                                        </TableCell>
                                    </TableRow>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <For
                                        each=move || paginated_list.get()
                                        key=|conf| {
                                            format!("{}{}", conf.title.clone(), conf.year.clone())
                                        }
                                        children=move |conf| {
                                            let conf_title = conf.title.clone();
                                            let conf_year = conf.year;
                                            let ccf_rank_value = conf.rank.clone();
                                            let core_rank_value = conf
                                                .corerank
                                                .clone()
                                                .unwrap_or_else(|| "N".to_string());
                                            let thcpl_rank_value = conf
                                                .thcplrank
                                                .clone()
                                                .unwrap_or_else(|| "N".to_string());

                                            view! {
                                                <ConferenceCard
                                                    conf=conf
                                                    use_english=use_english.into()
                                                    is_mobile=is_mobile
                                                    ccf_rank_selected=Signal::derive(move || {
                                                        rank_list.get().contains(&ccf_rank_value)
                                                    })
                                                    core_rank_selected=Signal::derive(move || {
                                                        core_rank_list.get().contains(&core_rank_value)
                                                    })
                                                    thcpl_rank_selected=Signal::derive(move || {
                                                        thcpl_rank_list.get().contains(&thcpl_rank_value)
                                                    })
                                                    on_toggle_favorite=Callback::new(move |_| {
                                                        all_conf_list.update(|conferences| {
                                                            for item in conferences.iter_mut() {
                                                                if item.title == conf_title
                                                                    && item.year == conf_year
                                                                {
                                                                    item.is_like = !item.is_like;
                                                                    like_list.update(|list| {
                                                                        if item.is_like {
                                                                            list.insert(item.id.clone());
                                                                        } else {
                                                                            list.remove(&item.id);
                                                                        }
                                                                    });
                                                                    break;
                                                                }
                                                            }
                                                        });
                                                    })
                                                />
                                            }
                                        }
                                    />
                                }
                                    .into_any()
                            }
                        }}
                    </TableBody>
                </Table>
            </div>

            <div class="footer">
                <div class="footer-text">
                    <span>
                        "Maintained by @ccfddl. If you find it useful, star or follow "
                        <a class="footer-link interactive-link" href="https://github.com/ccfddl" target="_blank">
                            "@ccfddl"
                        </a> " on Github."
                    </span>
                </div>
                <div class="footer-pagination">
                    <Pagination page page_count />
                </div>
            </div>

        </section>
    }
}

static UTC_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();

fn normalize_timezone(tz: &str) -> String {
    match tz {
        "AoE" => "UTC-12".to_string(),
        "UTC" => "UTC+0".to_string(),
        _ => tz.to_string(),
    }
}

fn parse_deadline_to_rfc3339(deadline: &str, tz_offset: &str) -> String {
    if deadline.contains(' ') {
        format!(
            "{}T{}{}",
            deadline.split(' ').nth(0).unwrap_or(""),
            deadline.split(' ').nth(1).unwrap_or("00:00:00"),
            tz_offset
        )
    } else {
        format!("{}T23:59:59{}", deadline, tz_offset)
    }
}

const RANK_OPTIONS: &[(&str, &str)] = &[("A", "CCF A"), ("B", "CCF B"), ("C", "CCF C"), ("N", "Non-CCF")];

const MOBILE_KEYWORDS: &[&str] = &[
    "phone", "pad", "pod", "iphone", "ipod", "ios", "ipad", "android", "mobile",
    "blackberry", "iemobile", "mqqbrowser", "juc", "fennec", "wosbrowser",
    "browserng", "webos", "symbian", "windows phone",
];

fn get_utc_map() -> &'static HashMap<String, String> {
    UTC_MAP.get_or_init(|| {
        let mut utc_map = HashMap::new();
        for i in -12..=12 {
            let offset_str = if i >= 0 {
                format!("+{:02}:00", i)
            } else {
                format!("-{:02}:00", -i)
            };
            let key = if i >= 0 {
                format!("UTC+{}", i)
            } else {
                format!("UTC{}", i)
            };
            utc_map.insert(key, offset_str);
        }
        utc_map.insert("AoE".to_string(), "-12:00".to_string());
        utc_map.insert("UTC".to_string(), "+00:00".to_string());
        utc_map
    })
}

#[allow(dead_code)]
fn load_utc_map() -> HashMap<String, String> {
    get_utc_map().clone()
}

#[cfg(target_arch = "wasm32")]
fn get_browser_time_and_timezone() -> (DateTime<FixedOffset>, FixedOffset) {
    let utc_now = chrono::Utc::now();
    let js_date = web_sys::js_sys::Date::new_0();
    let offset_minutes = -(js_date.get_timezone_offset() as i32);

    let timezone = FixedOffset::east_opt(offset_minutes * 60)
        .unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());

    let current_time = utc_now.with_timezone(&timezone);

    (current_time, timezone)
}

#[cfg(not(target_arch = "wasm32"))]
fn get_browser_time_and_timezone() -> (DateTime<FixedOffset>, FixedOffset) {
    use chrono::Local;
    let local_time = Local::now();
    let timezone = *local_time.offset();
    (local_time.with_timezone(&timezone), timezone)
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = navigator, getter, js_name = userAgent)]
    fn user_agent() -> String;
}

fn is_client_device() -> bool {
    web_sys::window().is_some()
}

fn is_mobile_device() -> bool {
    if !is_client_device() {
        return false;
    }

    let window = web_sys::window().expect("no global window exists");
    let navigator = window.navigator();
    let user_agent = navigator
        .user_agent()
        .expect("user agent not available")
        .to_lowercase();

    MOBILE_KEYWORDS
        .iter()
        .any(|&keyword| user_agent.contains(keyword))
}

fn get_from_local_storage(key: &str) -> Option<String> {
    let window = window().unwrap();
    let local_storage = window.local_storage().ok().flatten().unwrap();
    local_storage.get_item(key).unwrap()
}

fn set_in_local_storage(key: &str, value: &str) {
    let window = window().unwrap();
    let local_storage = window.local_storage().ok().flatten().unwrap();
    local_storage.set_item(key, value).unwrap();
}


