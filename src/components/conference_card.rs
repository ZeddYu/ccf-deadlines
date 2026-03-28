use crate::components::calendar_popover::CalendarPopover;
use crate::components::conf::ConfItem;
use crate::components::countdown::CountDown;
use crate::components::timeline::TimeLine;
use leptos::either::Either;
use leptos::prelude::*;
use thaw::{Icon, TableCell, TableCellLayout, TableRow, Tag};

pub fn rank_group_label(prefix: &str, value: &str) -> String {
    if value == "N" {
        format!("Non-{prefix}")
    } else {
        format!("{prefix} {value}")
    }
}

pub fn conference_category_label(conf: &ConfItem, use_english: bool) -> String {
    if use_english {
        conf.subname_en.clone()
    } else {
        conf.subname.clone()
    }
}

pub fn acceptance_rate_label(conf: &ConfItem) -> Option<String> {
    conf.acc_str
        .as_ref()
        .map(|acc| format!("Acc. Rate: {acc}"))
}

#[component]
pub fn ConferenceCard(
    conf: ConfItem,
    use_english: Signal<bool>,
    is_mobile: RwSignal<bool>,
    ccf_rank_selected: Signal<bool>,
    core_rank_selected: Signal<bool>,
    thcpl_rank_selected: Signal<bool>,
    on_toggle_favorite: Callback<()>,
) -> impl IntoView {
    let is_finished = conf.status == "FIN";
    let is_tbd = conf.status == "TBD";
    let is_like = conf.is_like;
    let title = conf.title.clone();
    let year = conf.year;
    let date = conf.date.clone();
    let place = conf.place.clone();
    let description = conf.description.clone();
    let comment = conf.comment.clone();
    let acceptance_rate = acceptance_rate_label(&conf);
    let category_conf = conf.clone();
    let category_label = Signal::derive(move || conference_category_label(&category_conf, use_english.get()));
    let remain = conf.remain;
    let google_calendar_url = conf.google_calendar_url.clone();
    let icloud_calendar_url = conf.icloud_calendar_url.clone();
    let link = conf.link.clone();
    let display_link = link.clone();
    let ddls = conf.ddls.clone();
    let ccf_rank_label = conf.displayrank.clone();
    let core_rank_label = rank_group_label("CORE", conf.corerank.as_deref().unwrap_or("N"));
    let thcpl_rank_label = rank_group_label("THCPL", conf.thcplrank.as_deref().unwrap_or("N"));
    let show_ddl_str = if is_tbd {
        "TBD".to_string()
    } else {
        format!(
            "{} ({})",
            conf.local_ddl.clone().unwrap(),
            conf.origin_ddl.clone().unwrap(),
        )
    };
    let dblp_url = format!("https://dblp.org/db/conf/{}", conf.dblp);

    view! {
        <TableRow>
            <TableCell>
                <TableCellLayout>
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
                                        {if is_like {
                                            Either::Left(view! {
                                                <Icon icon=icondata::BsStarFill class="favorite-icon favorite-icon-active" />
                                            })
                                        } else {
                                            Either::Right(view! {
                                                <Icon icon=icondata::BsStar class="favorite-icon favorite-icon-inactive" />
                                            })
                                        }}
                                    </button>
                                </div>
                            </div>

                            <div class="conference-meta-text conference-card-meta-row">{date} " " {place}</div>

                            <div class="conference-meta-text conference-card-meta-row">{description}</div>

                            <div class="conference-tag-groups conference-card-meta-row">
                                <div class="tag-container conference-tag-group conference-rank-tags">
                                    <span class=move || if ccf_rank_selected.get() { "tag-highlight" } else { "" }>
                                        <Tag class="plain-tag">{ccf_rank_label.clone()}</Tag>
                                    </span>
                                    " "
                                    <span class=move || if core_rank_selected.get() { "tag-highlight" } else { "" }>
                                        <Tag class="plain-tag">{core_rank_label.clone()}</Tag>
                                    </span>
                                    " "
                                    <span class=move || if thcpl_rank_selected.get() { "tag-highlight" } else { "" }>
                                        <Tag class="plain-tag">{thcpl_rank_label.clone()}</Tag>
                                    </span>
                                </div>

                                <div class="tag-container conference-tag-group conference-meta-tags">
                                    {acceptance_rate.map(|acc| {
                                        view! {
                                            <Tag class="plain-tag">{acc}</Tag>
                                        }
                                    })}
                                    <Tag class="plain-tag">{move || category_label.get()}</Tag>
                                </div>
                            </div>

                            {comment.map(|comment| {
                                view! {
                                    <div class="conference-note conference-card-meta-row">
                                        <b>"NOTE: "</b>
                                        {comment}
                                    </div>
                                }
                            })}

                            <div class="conference-meta-text conference-card-meta-row conference-website-link-wrap">
                                <span class="conference-website-label">"Website"</span>
                                <a
                                    href=link.clone()
                                    title=link.clone()
                                    class="inline-muted-link interactive-link inline-break-link conference-website-link"
                                    target="_blank"
                                >
                                    {display_link}
                                </a>
                            </div>
                        </div>

                        <div class="conference-deadline-panel countdown-panel">
                            <div class:conf-fin=is_finished>
                                {if is_tbd {
                                    Either::Left(view! {
                                        <div class="countdown-container countdown-line">
                                            <div class="countdown-display countdown-value-wrap">
                                                <span class="countdown-value">"TBD"</span>
                                            </div>
                                        </div>
                                    })
                                } else {
                                    Either::Right(view! {
                                        <div class="countdown-container countdown-line">
                                            <div class="countdown-display countdown-value-wrap">
                                                <span class="countdown-value">
                                                    <CountDown remain compact=true />
                                                    <CalendarPopover
                                                        google_calendar_url=google_calendar_url
                                                        icloud_calendar_url=icloud_calendar_url
                                                        is_mobile
                                                    />
                                                </span>
                                            </div>
                                        </div>
                                    })
                                }}

                                <div class="conference-meta-text countdown-panel-meta">
                                    {if is_tbd {
                                        Either::Left(view! {
                                            <span>
                                                "Deadline: "
                                                <a
                                                    href="https://github.com/ccfddl/ccf-deadlines/pulls"
                                                    class="inline-muted-link interactive-link"
                                                    target="_blank"
                                                >
                                                    "pull request to update"
                                                </a>
                                            </span>
                                        })
                                    } else {
                                        Either::Right(view! {
                                            <span>{format!("Deadline: {}", show_ddl_str)}</span>
                                        })
                                    }}
                                </div>

                                {if is_finished || is_tbd {
                                    Either::Left(view! { <></> })
                                } else {
                                    Either::Right(view! {
                                        <div class="countdown-timeline-wrap">
                                            <TimeLine time_points=ddls />
                                        </div>
                                    })
                                }}
                            </div>
                        </div>
                    </div>
                </TableCellLayout>
            </TableCell>
        </TableRow>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::conf::ConfItem;

    fn sample_conf() -> ConfItem {
        ConfItem {
            title: "Conf".to_string(),
            description: "A conference".to_string(),
            sub: "AI".to_string(),
            rank: "A".to_string(),
            corerank: Some("A*".to_string()),
            thcplrank: Some("B".to_string()),
            displayrank: "CCF A".to_string(),
            dblp: "sigmod".to_string(),
            year: 2026,
            id: "conf-2026".to_string(),
            link: "https://example.com".to_string(),
            abstract_deadline: None,
            deadline: "2026-04-01 23:59:59".to_string(),
            comment: Some("Bring camera-ready changes".to_string()),
            timezone: "UTC+0".to_string(),
            date: "Apr 1-3, 2026".to_string(),
            place: "Paris".to_string(),
            status: "RUN".to_string(),
            is_like: false,
            remain: 42,
            local_ddl: Some("2026-04-01 23:59:59".to_string()),
            origin_ddl: Some("2026-04-01 23:59:59 UTC+0".to_string()),
            subname: "人工智能".to_string(),
            subname_en: "Artificial Intelligence".to_string(),
            google_calendar_url: Some("https://calendar.google.com".to_string()),
            icloud_calendar_url: Some("https://icloud.com".to_string()),
            acc_str: Some("25%".to_string()),
            ddls: vec![],
        }
    }

    #[test]
    fn formats_non_rank_labels_for_meta_groups() {
        assert_eq!(rank_group_label("CORE", "N"), "Non-CORE");
        assert_eq!(rank_group_label("THCPL", "N"), "Non-THCPL");
    }

    #[test]
    fn formats_named_rank_labels_for_meta_groups() {
        assert_eq!(rank_group_label("CORE", "A*"), "CORE A*");
        assert_eq!(rank_group_label("THCPL", "B"), "THCPL B");
    }

    #[test]
    fn prefers_english_category_label_when_requested() {
        let conf = sample_conf();

        assert_eq!(conference_category_label(&conf, true), "Artificial Intelligence");
        assert_eq!(conference_category_label(&conf, false), "人工智能");
    }

    #[test]
    fn formats_acceptance_rate_text_only_when_present() {
        let conf = sample_conf();

        assert_eq!(acceptance_rate_label(&conf), Some("Acc. Rate: 25%".to_string()));

        let mut without_rate = conf;
        without_rate.acc_str = None;
        assert_eq!(acceptance_rate_label(&without_rate), None);
    }

    #[test]
    fn conference_card_keeps_unified_card_and_capsule_link_classes() {
        const SOURCE: &str = include_str!("conference_card.rs");

        assert!(SOURCE.contains("conference-card-shell conference-card-row"));
        assert!(SOURCE.contains("conference-card-main conference-card-main-wrap"));
        assert!(SOURCE.contains("conference-card-meta-row"));
        assert!(SOURCE.contains("conference-website-link-wrap"));
        assert!(SOURCE.contains("conference-website-link"));
        assert!(SOURCE.contains("conference-deadline-panel countdown-panel"));
        assert!(SOURCE.contains("conference-favorite-anchor"));
        assert!(SOURCE.contains("countdown-value-wrap"));
        assert!(SOURCE.contains("countdown-panel-meta"));
        assert!(SOURCE.contains("countdown-timeline-wrap"));
    }

    #[test]
    fn website_row_stays_in_main_card_column_before_deadline_panel() {
        const SOURCE: &str = include_str!("conference_card.rs");

        let main_column = SOURCE
            .find("<div class=\"conference-card-main conference-card-main-wrap\"")
            .expect("conference card main column should exist");
        let website_row = SOURCE
            .find("<span class=\"conference-website-label\">\"Website\"</span>")
            .expect("conference card should render a website row");
        let deadline_panel = SOURCE
            .find("<div class=\"conference-deadline-panel countdown-panel\">")
            .expect("conference deadline panel should exist");

        assert!(
            main_column < website_row && website_row < deadline_panel,
            "website row should be rendered in the main card column before the deadline panel"
        );
    }

    #[test]
    fn conference_card_uses_single_table_cell_for_integrated_surface() {
        const SOURCE: &str = include_str!("conference_card.rs");
        let tests_start = SOURCE.find("#[cfg(test)]").expect("expected test module");
        let production_source = &SOURCE[..tests_start];

        assert_eq!(production_source.matches("<TableCell>").count(), 1);
        assert!(production_source.contains("<div class=\"conference-card-shell conference-card-row\">"));
    }
}
