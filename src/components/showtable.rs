use crate::components::category_filter_chips::CategoryFilterChips;
use crate::components::checkbox_button::*;
use crate::components::conf::*;
use crate::components::conference_card::ConferenceCard;
use crate::components::deadline_utils::*;
use crate::components::loading::LoadingSkeleton;
use crate::components::storage::{
    get_json_from_local_storage, set_in_local_storage, set_json_in_local_storage,
};
use crate::components::subscription_modal::*;
use crate::components::timezone::*;
use chrono::DateTime;
use leptos::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use thaw::*;
use urlencoding::encode;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

const DEFAULT_CONFERENCE_CARD_LIST_CLASS: &str =
    "conference-card-list conference-card-list-comfortable";

fn conference_matches_filters(
    item: &ConfItem,
    categories: &HashSet<String>,
    ccf_ranks: &HashSet<String>,
    core_ranks: &HashSet<String>,
    thcpl_ranks: &HashSet<String>,
    input_lower: &str,
) -> bool {
    if !categories.is_empty() && !categories.contains(&item.sub.to_uppercase()) {
        return false;
    }
    if !ccf_ranks.is_empty() && !ccf_ranks.contains(&item.rank) {
        return false;
    }
    if !core_ranks.is_empty() && !core_ranks.contains(item.corerank.as_deref().unwrap_or("N")) {
        return false;
    }
    if !thcpl_ranks.is_empty() && !thcpl_ranks.contains(item.thcplrank.as_deref().unwrap_or("N")) {
        return false;
    }

    input_lower.is_empty()
        || item.id.to_lowercase().contains(input_lower)
        || item.title.to_lowercase().contains(input_lower)
}

#[component]
pub fn ShowTable(use_english: RwSignal<bool>) -> impl IntoView {
    // mobile
    let is_mobile = RwSignal::new(false);
    let is_loading = RwSignal::new(true);
    let load_error = RwSignal::new(None::<String>);

    // checkbox
    let sub_list = RwSignal::new(get_categories());
    let cached_check_list: HashSet<String> =
        get_json_from_local_storage("types").unwrap_or_default();
    let check_list = RwSignal::new(cached_check_list);
    // input
    let input_value = RwSignal::new(String::new());

    // checkboxbutton
    let mut cached_rank_list: HashSet<String> =
        get_json_from_local_storage("ranks").unwrap_or_default();
    normalize_rank_filter_selection(&mut cached_rank_list);
    let rank_list = RwSignal::new(cached_rank_list);
    let mut cached_core_rank_list: HashSet<String> =
        get_json_from_local_storage("core_ranks").unwrap_or_default();
    normalize_rank_filter_selection(&mut cached_core_rank_list);
    let core_rank_list = RwSignal::new(cached_core_rank_list);
    let mut cached_thcpl_rank_list: HashSet<String> =
        get_json_from_local_storage("thcpl_ranks").unwrap_or_default();
    normalize_rank_filter_selection(&mut cached_thcpl_rank_list);
    let thcpl_rank_list = RwSignal::new(cached_thcpl_rank_list);
    let open_dropdown = RwSignal::new(None::<String>);
    let show_mobile_filters = RwSignal::new(false);

    // liked
    let cached_like_list: HashSet<String> =
        get_json_from_local_storage("likes").unwrap_or_default();
    let like_list = RwSignal::new(cached_like_list);

    let show_subscription_modal = RwSignal::new(false);

    // pagination
    let page = RwSignal::new(1);
    let page_size = RwSignal::new(10);
    let is_filter_change = RwSignal::new(false);

    // table
    let all_conf_list = RwSignal::new(Vec::<ConfItem>::new());
    let acceptance_rates = RwSignal::new(HashMap::<String, String>::new());

    let time_zone = RwSignal::new(String::new());

    Effect::new(move |_| {
        check_list.with(|_| ());
        input_value.with(|_| ());
        rank_list.with(|_| ());
        core_rank_list.with(|_| ());
        thcpl_rank_list.with(|_| ());

        if is_filter_change.get_untracked() {
            page.set(1);
        } else {
            is_filter_change.set(true);
        }
    });

    Effect::new(move |_| {
        set_in_local_storage("use_english", &use_english.get().to_string());
    });

    Effect::new(move |_| {
        check_list.with(|list| set_json_in_local_storage("types", list));
    });

    Effect::new(move |_| {
        rank_list.with(|list| set_json_in_local_storage("ranks", list));
    });

    Effect::new(move |_| {
        core_rank_list.with(|list| set_json_in_local_storage("core_ranks", list));
    });

    Effect::new(move |_| {
        thcpl_rank_list.with(|list| set_json_in_local_storage("thcpl_ranks", list));
    });

    Effect::new(move |_| {
        like_list.with(|list| set_json_in_local_storage("likes", list));
    });

    Effect::new(move |_| {
        check_list.with(|_| ());
        input_value.with(|_| ());
        rank_list.with(|_| ());
        core_rank_list.with(|_| ());
        thcpl_rank_list.with(|_| ());
        let _ = page.get();

        let (current_time, _) = get_browser_time_and_timezone();
        let utc_map = load_utc_map();

        all_conf_list.update(|conferences| {
            for item in conferences.iter_mut() {
                if item.deadline != STATUS_TBD {
                    let tz_str = normalize_timezone(&item.timezone);

                    if let Some(tz_offset) = utc_map.get(&tz_str) {
                        let ddl_str = parse_deadline_to_rfc3339(&item.deadline, tz_offset);

                        if let Ok(ddl_datetime) = DateTime::parse_from_rfc3339(&ddl_str) {
                            let diff = ddl_datetime.signed_duration_since(current_time);
                            if diff.num_milliseconds() <= 0 {
                                item.remain = 0;
                                item.status = STATUS_FIN.to_string();
                            } else {
                                item.remain = diff.num_milliseconds() as u64;
                                item.status = STATUS_RUN.to_string();
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
        time_zone.set(get_timezone_name().unwrap_or_else(|| "UTC".to_string()));

        spawn_local(async move {
            let utc_map = load_utc_map();
            let (current_time, current_timezone) = get_browser_time_and_timezone();

            let Some(base_url) = current_origin() else {
                let message = "Unable to read browser location origin";
                console::error_1(&message.into());
                load_error.set(Some(message.to_string()));
                is_loading.set(false);
                return;
            };

            let conferences = match fetch_all_conf(&base_url).await {
                Ok(conferences) => conferences,
                Err(e) => {
                    console::error_1(&format!("Error: {:?}", e).into());
                    load_error.set(Some("Unable to load conference data.".to_string()));
                    is_loading.set(false);
                    return;
                }
            };

            let mut conf_vec = Vec::new();

            for conf in conferences {
                let conf_items = conf.confs.iter().filter_map(|year_conf| {
                    let last_timeline_item = year_conf.timeline.last()?;
                    let mut flag = false;
                    let mut cur_deadline = last_timeline_item.deadline.clone();
                    let mut cur_abstract_deadline = last_timeline_item.abstract_deadline.clone();
                    let mut cur_comment = last_timeline_item.comment.clone();
                    let mut ddl_vec = Vec::<TimePoint>::new();
                    let tz_str = normalize_timezone(&year_conf.timezone);
                    let Some(tz_offset) = utc_map.get(&tz_str) else {
                        console::warn_1(
                            &format!(
                                "Unknown timezone for {} {}: {}",
                                conf.title, year_conf.year, year_conf.timezone
                            )
                            .into(),
                        );
                        return None;
                    };

                    for timeline_item in year_conf.timeline.iter() {
                        let ddl_str = parse_deadline_to_rfc3339(&timeline_item.deadline, tz_offset);

                        if let Some(abs_ddl) = timeline_item.abstract_deadline.clone() {
                            let abs_ddl_str = parse_deadline_to_rfc3339(&abs_ddl, tz_offset);

                            if let Ok(abs_ddl_datetime) = DateTime::parse_from_rfc3339(&abs_ddl_str)
                            {
                                ddl_vec.push(TimePoint {
                                    timepoint: abs_ddl_datetime.with_timezone(&current_timezone),
                                    r#type: 0,
                                });
                            }
                        }

                        if let Ok(ddl_datetime) = DateTime::parse_from_rfc3339(&ddl_str) {
                            ddl_vec.push(TimePoint {
                                timepoint: ddl_datetime.with_timezone(&current_timezone),
                                r#type: 1,
                            });

                            let diff = ddl_datetime.signed_duration_since(current_time);
                            if !flag && diff.num_milliseconds() > 0 {
                                cur_deadline = timeline_item.deadline.clone();
                                cur_abstract_deadline = timeline_item.abstract_deadline.clone();
                                cur_comment = timeline_item.comment.clone();
                                flag = true;
                            }
                        }
                    }

                    Some(ConfItem {
                        title: conf.title.clone(),
                        description: conf.description.clone(),
                        sub: conf.sub.clone(),
                        rank: conf.rank.ccf.clone(),
                        corerank: conf.rank.core.clone(),
                        thcplrank: conf.rank.thcpl.clone(),
                        displayrank: format_rank_label("CCF", &conf.rank.ccf),
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
                        status: String::new(),
                        is_like: false,
                        remain: 0,
                        local_ddl: None,
                        origin_ddl: None,
                        subname: "".to_string(),
                        subname_en: "".to_string(),
                        google_calendar_url: None,
                        icloud_calendar_url: None,
                        ddls: ddl_vec,
                    })
                });
                conf_vec.extend(conf_items);
            }

            for item in conf_vec.iter_mut() {
                if let Some(matched_category) = sub_list
                    .get_untracked()
                    .iter()
                    .find(|sub_item| sub_item.sub == item.sub)
                {
                    item.subname = matched_category.name.clone();
                    item.subname_en = matched_category.name_en.clone();
                }

                if item.deadline == STATUS_TBD {
                    item.remain = 0;
                    item.status = STATUS_TBD.to_string();
                    continue;
                }

                let tz_str = normalize_timezone(&item.timezone);

                if let Some(tz_offset) = utc_map.get(&tz_str) {
                    let ddl_str = parse_deadline_to_rfc3339(&item.deadline, tz_offset);

                    if let Ok(ddl_datetime) = DateTime::parse_from_rfc3339(&ddl_str) {
                        let local_ddl_datetime = ddl_datetime.with_timezone(&current_timezone);
                        let formatted_date_time =
                            local_ddl_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                        let offset_seconds = local_ddl_datetime.offset().local_minus_utc();
                        let offset_hours = offset_seconds / 3600;
                        let formatted_timezone = format!("UTC{:+}", offset_hours);

                        item.local_ddl =
                            Some(format!("{} {}", formatted_date_time, formatted_timezone));
                        item.origin_ddl = Some(format!("{} {}", item.deadline, item.timezone));

                        if let Some(abs_ddl) = &item.abstract_deadline {
                            let abs_ddl_str = parse_deadline_to_rfc3339(abs_ddl, tz_offset);
                            if let Ok(abs_datetime) = DateTime::parse_from_rfc3339(&abs_ddl_str) {
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
                            item.status = STATUS_FIN.to_string();
                        } else {
                            item.remain = diff.num_milliseconds() as u64;
                            item.status = STATUS_RUN.to_string();
                        }

                        let iso_string = local_ddl_datetime.format("%Y%m%dT%H%M%S").to_string();

                        item.google_calendar_url = Some(build_google_calendar_url(
                            &item.title,
                            item.year,
                            &iso_string,
                            &format!(
                                "{} provided by @ccfddl",
                                item.comment.as_deref().unwrap_or("")
                            ),
                            &time_zone.get_untracked(),
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
                            encode_ical_text(&format!("{} {} Deadline", item.title, item.year)),
                            encode_ical_text(item.comment.as_ref().map_or("", |c| c.as_str())),
                            encode_ical_text(""),
                        ));
                    }
                }
            }
            let conf_acc_keys = conf_vec
                .iter()
                .map(|item| (item.id.clone(), item.title.clone(), item.year))
                .collect::<Vec<_>>();

            load_error.set(None);
            all_conf_list.set(conf_vec);
            is_loading.set(false);

            match fetch_all_acc(&base_url).await {
                Ok(all_acc) => {
                    let mut acc_by_title_year: HashMap<String, HashMap<i32, String>> =
                        HashMap::new();
                    for acc_item in all_acc {
                        let year_map = acc_by_title_year.entry(acc_item.title).or_default();
                        for cur_acc in acc_item.accept_rates {
                            for year_offset in 1..=3 {
                                year_map.insert(cur_acc.year + year_offset, cur_acc.str.clone());
                            }
                        }
                    }

                    let mut acc_by_conf_id = HashMap::new();
                    for (id, title, year) in &conf_acc_keys {
                        if let Some(year_map) = acc_by_title_year.get(title)
                            && let Some(acc_str) = year_map.get(year)
                        {
                            acc_by_conf_id.insert(id.clone(), acc_str.clone());
                        }
                    }
                    acceptance_rates.set(acc_by_conf_id);
                }
                Err(e) => {
                    console::error_1(&format!("Error: {:?}", e).into());
                }
            }
        });
    });

    let filtered_list = Memo::new(move |_| {
        let input_lower = input_value.get().to_lowercase();
        let liked_ids = like_list.get();
        let mut run_list = Vec::new();
        let mut tbd_list = Vec::new();
        let mut fin_list = Vec::new();

        check_list.with(|categories| {
            rank_list.with(|ccf_ranks| {
                core_rank_list.with(|core_ranks| {
                    thcpl_rank_list.with(|thcpl_ranks| {
                        all_conf_list.with(|conferences| {
                            for item in conferences {
                                if !conference_matches_filters(
                                    item,
                                    categories,
                                    ccf_ranks,
                                    core_ranks,
                                    thcpl_ranks,
                                    &input_lower,
                                ) {
                                    continue;
                                }

                                match item.status.as_str() {
                                    STATUS_RUN => run_list.push(item.clone()),
                                    STATUS_TBD => tbd_list.push(item.clone()),
                                    STATUS_FIN => fin_list.push(item.clone()),
                                    _ => {}
                                }
                            }
                        });
                    });
                });
            });
        });

        run_list.sort_by_key(|item| item.remain);
        fin_list.sort_by_key(|item| std::cmp::Reverse(item.year));

        let mut sorted_list = Vec::with_capacity(run_list.len() + tbd_list.len() + fin_list.len());
        sorted_list.extend(run_list);
        sorted_list.extend(tbd_list);
        sorted_list.extend(fin_list);

        let (mut liked_list, unliked_list): (Vec<_>, Vec<_>) = sorted_list
            .into_iter()
            .partition(|conf| liked_ids.contains(&conf.id));
        liked_list.extend(unliked_list);

        liked_list
    });

    let page_count = Memo::new(move |_| {
        let total_count = filtered_list.with(Vec::len);
        total_count.div_ceil(page_size.get()).max(1)
    });

    let paginated_list = Memo::new(move |_| {
        let page_val = page.get();
        let page_size_val = page_size.get();

        filtered_list.with(|final_list| {
            let total_count = final_list.len();
            let start = (page_val - 1) * page_size_val;
            let end = (start + page_size_val).min(total_count);

            if start < total_count {
                final_list[start..end].to_vec()
            } else {
                Vec::new()
            }
        })
    });

    let clear_all_label = Memo::new(move |_| {
        if use_english.get() {
            "Clear all".to_string()
        } else {
            "清空".to_string()
        }
    });
    let compact_result_label = Memo::new(move |_| {
        let count = filtered_list.with(Vec::len);

        if use_english.get() {
            format!("{count} conferences")
        } else {
            format!("{count} 个会议")
        }
    });
    let search_placeholder = Memo::new(move |_| {
        if use_english.get() {
            "Search conferences...".to_string()
        } else {
            "搜索会议...".to_string()
        }
    });

    let empty_state_label = Memo::new(move |_| {
        if use_english.get() {
            "No conferences match the current filters.".to_string()
        } else {
            "当前筛选条件下暂无会议。".to_string()
        }
    });
    let timezone_message = Memo::new(move |_| {
        let current_timezone = time_zone.get();
        if use_english.get() {
            format!("Deadlines are shown in {current_timezone} time.")
        } else {
            format!("当前截止时间基于 {current_timezone} 时区显示。")
        }
    });
    let result_status_label =
        Memo::new(move |_| format!("{} {}", compact_result_label.get(), timezone_message.get()));
    let active_filter_summary = Memo::new(move |_| {
        let count = check_list.with(HashSet::len)
            + rank_list.with(HashSet::len)
            + core_rank_list.with(HashSet::len)
            + thcpl_rank_list.with(HashSet::len)
            + usize::from(!input_value.with(String::is_empty));

        if count == 0 {
            if use_english.get() {
                "No active filters".to_string()
            } else {
                "未启用筛选".to_string()
            }
        } else if use_english.get() {
            format!("{count} active filters")
        } else {
            format!("已选 {count} 项")
        }
    });
    let mobile_filter_button_label =
        Memo::new(
            move |_| match (use_english.get(), show_mobile_filters.get()) {
                (true, true) => "Hide filters".to_string(),
                (true, false) => "Show filters".to_string(),
                (false, true) => "收起筛选".to_string(),
                (false, false) => "展开筛选".to_string(),
            },
        );

    view! {
    <section class="home-content">
        <SubscriptionModal
            show=show_subscription_modal
            use_english=use_english
            check_list=check_list
            rank_list=rank_list
            core_rank_list=core_rank_list
            thcpl_rank_list=thcpl_rank_list
        />

        <div class="home-task-console">
            <div class="home-task-bar home-control-bar">
                <div class="home-control-search">
                    <label class="sr-only" for="conference-search">
                        {move || if use_english.get() { "Search conferences" } else { "搜索会议" }}
                    </label>
                    <Input
                        id="conference-search"
                        value=input_value
                        placeholder=search_placeholder
                        size=InputSize::Small
                        class="custom-search-input home-search-input"
                    >
                        <InputPrefix slot>
                            <Icon icon=icondata::FiSearch class="search-prefix-icon" />
                        </InputPrefix>
                    </Input>
                </div>

                <div class="home-control-filters home-control-filters-desktop">
                    <MultiSelectDropdown
                        dropdown_id="ccf".to_string()
                        title="CCF".to_string()
                        options=ccf_filter_options()
                        selected_values=rank_list
                        use_english=use_english
                        panel_width="184px".to_string()
                        open_dropdown=open_dropdown
                    />
                    <MultiSelectDropdown
                        dropdown_id="core".to_string()
                        title="CORE".to_string()
                        options=core_filter_options()
                        selected_values=core_rank_list
                        use_english=use_english
                        panel_width="192px".to_string()
                        open_dropdown=open_dropdown
                    />
                    <MultiSelectDropdown
                        dropdown_id="thcpl".to_string()
                        title="THCPL".to_string()
                        options=thcpl_filter_options()
                        selected_values=thcpl_rank_list
                        use_english=use_english
                        panel_width="196px".to_string()
                        open_dropdown=open_dropdown
                    />
                </div>

                <Button
                    size=ButtonSize::Small
                    appearance=ButtonAppearance::Primary
                    class="home-subscribe-button"
                    on_click=move |_| show_subscription_modal.set(true)
                >
                    <Icon icon=icondata::AiCalendarOutlined class="calendar-button-icon" />
                    {move || if use_english.get() { "Subscribe" } else { "订阅" }}
                </Button>
            </div>

            <div class="mobile-filter-bar">
                <button
                    type="button"
                    class="home-chip-action home-mobile-filter-toggle"
                    aria-expanded=move || show_mobile_filters.get().to_string()
                    aria-controls="mobile-filter-panel"
                    on:click=move |_| {
                        show_mobile_filters.set(!show_mobile_filters.get_untracked());
                        open_dropdown.set(None);
                    }
                >
                    {move || mobile_filter_button_label.get()}
                </button>
                <span class="active-filter-count">{move || active_filter_summary.get()}</span>
            </div>

            <Show when=move || show_mobile_filters.get()>
                <div id="mobile-filter-panel" class="mobile-filter-panel">
                    <div class="mobile-filter-menu">
                        <MultiSelectDropdown
                            dropdown_id="ccf-mobile".to_string()
                            title="CCF".to_string()
                            options=ccf_filter_options()
                            selected_values=rank_list
                            use_english=use_english
                            panel_width="100%".to_string()
                            open_dropdown=open_dropdown
                        />
                        <MultiSelectDropdown
                            dropdown_id="core-mobile".to_string()
                            title="CORE".to_string()
                            options=core_filter_options()
                            selected_values=core_rank_list
                            use_english=use_english
                            panel_width="100%".to_string()
                            open_dropdown=open_dropdown
                        />
                        <MultiSelectDropdown
                            dropdown_id="thcpl-mobile".to_string()
                            title="THCPL".to_string()
                            options=thcpl_filter_options()
                            selected_values=thcpl_rank_list
                            use_english=use_english
                            panel_width="100%".to_string()
                            open_dropdown=open_dropdown
                        />
                    </div>

                    <CategoryFilterChips
                        sub_list=sub_list
                        check_list=check_list
                        use_english=use_english
                        clear_all_label=clear_all_label
                        input_value=input_value
                        rank_list=rank_list
                        core_rank_list=core_rank_list
                        thcpl_rank_list=thcpl_rank_list
                        open_dropdown=open_dropdown
                    />
                </div>
            </Show>

            <div class="home-chip-row home-chip-row-desktop">
                <CategoryFilterChips
                    sub_list=sub_list
                    check_list=check_list
                    use_english=use_english
                    clear_all_label=clear_all_label
                    input_value=input_value
                    rank_list=rank_list
                    core_rank_list=core_rank_list
                    thcpl_rank_list=thcpl_rank_list
                    open_dropdown=open_dropdown
                />
            </div>
        </div>

        <div class="home-result-context">
            <span
                class="home-context-chip"
                role="status"
                aria-live="polite"
                aria-atomic="true"
            >
                {move || result_status_label.get()}
            </span>
        </div>

        {move || {
            if is_loading.get() {
                view! { <LoadingSkeleton /> }.into_any()
            } else if let Some(message) = load_error.get() {
                view! {
                    <div class="empty-state-panel" role="alert">
                        <p class="empty-state-text">{message}</p>
                    </div>
                }
                .into_any()
            } else if paginated_list.get().is_empty() {
                view! {
                    <div class="empty-state-panel">
                        <p class="empty-state-text">{move || empty_state_label.get()}</p>
                    </div>
                }
                .into_any()
            } else {
                view! {
                    <div class=DEFAULT_CONFERENCE_CARD_LIST_CLASS>
                        <For
                            each=move || paginated_list.get()
                            key=|conf| conf.id.clone()
                            children=move |conf| {
                                let conf_id = conf.id.clone();
                                let acceptance_rate = Memo::new(move |_| {
                                    acceptance_rates.with(|rates| rates.get(&conf_id).cloned())
                                });

                                view! {
                                    <ConferenceCard
                                        conf
                                        use_english=use_english
                                        rank_list=rank_list
                                        core_rank_list=core_rank_list
                                        thcpl_rank_list=thcpl_rank_list
                                        like_list=like_list
                                        is_mobile=is_mobile
                                        acceptance_rate=acceptance_rate
                                    />
                                }
                            }
                        />
                    </div>
                }
                    .into_any()
            }
        }}

        <div class="footer">
            <div class="footer-text">
                <span>
                    "Maintained by @ccfddl. If you find it useful, star or follow "
                    <a
                        class="footer-link interactive-link"
                        href="https://github.com/ccfddl"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
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

#[cfg(test)]
mod tests {
    use super::{ConfItem, DEFAULT_CONFERENCE_CARD_LIST_CLASS, conference_matches_filters};
    use std::collections::HashSet;

    fn values(items: &[&str]) -> HashSet<String> {
        items.iter().map(|item| item.to_string()).collect()
    }

    fn test_conference() -> ConfItem {
        ConfItem {
            title: "TestConf".to_string(),
            description: "Test conference".to_string(),
            sub: "AI".to_string(),
            rank: "A".to_string(),
            corerank: Some("N".to_string()),
            thcplrank: Some("T1".to_string()),
            displayrank: "CCF A".to_string(),
            dblp: "testconf".to_string(),
            year: 2026,
            id: "testconf-2026".to_string(),
            link: "https://example.com".to_string(),
            abstract_deadline: None,
            deadline: "2026-05-04".to_string(),
            comment: None,
            timezone: "UTC".to_string(),
            date: "May 2026".to_string(),
            place: "Online".to_string(),
            status: "RUN".to_string(),
            is_like: false,
            remain: 0,
            local_ddl: None,
            origin_ddl: None,
            subname: "Artificial Intelligence".to_string(),
            subname_en: "Artificial Intelligence".to_string(),
            google_calendar_url: None,
            icloud_calendar_url: None,
            ddls: Vec::new(),
        }
    }

    #[test]
    fn defaults_to_comfortable_view_only() {
        assert_eq!(
            DEFAULT_CONFERENCE_CARD_LIST_CLASS,
            "conference-card-list conference-card-list-comfortable"
        );
    }

    #[test]
    fn requires_each_active_filter_to_match_conference() {
        let conference = test_conference();
        let empty = HashSet::new();

        assert!(conference_matches_filters(
            &conference,
            &values(&["AI"]),
            &values(&["A"]),
            &values(&["N"]),
            &values(&["T1"]),
            "testconf"
        ));
        assert!(!conference_matches_filters(
            &conference,
            &values(&["DB"]),
            &empty,
            &empty,
            &empty,
            ""
        ));
        assert!(!conference_matches_filters(
            &conference,
            &empty,
            &values(&["B"]),
            &empty,
            &empty,
            ""
        ));
        assert!(!conference_matches_filters(
            &conference,
            &empty,
            &empty,
            &values(&["A*"]),
            &empty,
            ""
        ));
        assert!(!conference_matches_filters(
            &conference,
            &empty,
            &empty,
            &empty,
            &values(&["T2"]),
            ""
        ));
        assert!(!conference_matches_filters(
            &conference,
            &empty,
            &empty,
            &empty,
            &empty,
            "missing"
        ));
    }
}
