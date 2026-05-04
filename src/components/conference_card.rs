use crate::components::calendar_popover::CalendarPopover;
use crate::components::checkbox_button::format_rank_label;
use crate::components::conf::{ConfItem, STATUS_FIN, STATUS_TBD};
use crate::components::countdown::CountDown;
use crate::components::timeline::TimeLine;
use chrono::{NaiveDate, NaiveDateTime};
use leptos::prelude::*;
use std::collections::HashSet;
use thaw::{Icon, Tag};

#[component]
pub fn ConferenceCard(
    conf: ConfItem,
    use_english: RwSignal<bool>,
    rank_list: RwSignal<HashSet<String>>,
    core_rank_list: RwSignal<HashSet<String>>,
    thcpl_rank_list: RwSignal<HashSet<String>>,
    like_list: RwSignal<HashSet<String>>,
    is_mobile: RwSignal<bool>,
    acceptance_rate: Memo<Option<String>>,
) -> impl IntoView {
    let is_finished = conf.status == STATUS_FIN;
    let is_tbd = conf.status == STATUS_TBD;
    let ccf_rank_value = conf.rank.clone();
    let ccf_rank_label = conf.displayrank.clone();
    let core_rank_value = conf.corerank.clone().unwrap_or_else(|| "N".to_string());
    let core_tag_label = format_rank_label("CORE", &core_rank_value);
    let thcpl_rank_value = conf.thcplrank.clone().unwrap_or_else(|| "N".to_string());
    let thcpl_tag_label = format_rank_label("THCPL", &thcpl_rank_value);
    let safe_website_link = safe_external_link(&conf.link).map(str::to_string);
    let website_host = safe_website_link
        .as_deref()
        .map(format_website_label)
        .unwrap_or_else(|| "Website unavailable".to_string());
    let website_link_view = if let Some(website_link) = safe_website_link {
        view! {
            <a
                href=website_link
                class="conference-link-pill interactive-link"
                target="_blank"
                rel="noopener noreferrer"
            >
                <span class="conference-link-pill-text">{website_host}</span>
                <Icon icon=icondata::BsArrowUpRight class="conference-link-pill-icon" />
            </a>
        }
        .into_any()
    } else {
        view! {
            <span
                class="conference-link-pill conference-link-pill-muted"
                aria-label="Conference website link unavailable"
            >
                <span class="conference-link-pill-text">{website_host}</span>
            </span>
        }
        .into_any()
    };
    let deadline_text = if is_tbd {
        "Deadline TBD".to_string()
    } else {
        format_deadline_label(&conf.deadline, &conf.timezone)
    };
    let favorite_label = format!("{} {}", conf.title, conf.year);
    let conf_id = conf.id.clone();
    let conf_id_for_favorite = conf_id.clone();
    let is_favorite =
        Memo::new(move |_| like_list.with(|list| list.contains(&conf_id_for_favorite)));

    view! {
        <article class="conference-card">
            <div class=move || {
                format!(
                    "conference-card-shell {}",
                    if is_finished { "conference-card-finished" } else { "" },
                )
            }>
                <button
                    type="button"
                    class="favorite-toggle conference-favorite-button"
                    aria-label=move || {
                        if is_favorite.get() {
                            format!("Remove {favorite_label} from favorites")
                        } else {
                            format!("Add {favorite_label} to favorites")
                        }
                    }
                    aria-pressed=move || is_favorite.get().to_string()
                    on:click=move |_| {
                        like_list.update(|list| {
                            if list.contains(&conf_id) {
                                list.remove(&conf_id);
                            } else {
                                list.insert(conf_id.clone());
                            }
                        });
                    }
                >
                    {move || {
                        let (icon, class) = if is_favorite.get() {
                            (icondata::BsStarFill, "favorite-icon favorite-icon-active")
                        } else {
                            (icondata::BsStar, "favorite-icon favorite-icon-inactive")
                        };

                        view! { <Icon icon=icon class=class /> }
                    }}
                </button>

                <div class="conference-card-main">
                    <div class="conference-card-left">
                        <div class="conference-card-title-row">
                            <a
                                href=format!("https://dblp.org/db/conf/{}", conf.dblp)
                                class="conference-card-title interactive-link"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                {conf.title.clone()} " " {conf.year}
                            </a>
                        </div>

                        <div class="conference-card-meta">
                            {conf.date.clone()} ", " {conf.place.clone()}
                        </div>

                        <div class="conference-card-description">{conf.description.clone()}</div>

                        <div class="conference-card-tags">
                            <span class=move || {
                                if rank_list.with(|list| list.contains(&ccf_rank_value)) {
                                    "tag-highlight"
                                } else {
                                    ""
                                }
                            }>
                                <Tag class="plain-tag">{ccf_rank_label.clone()}</Tag>
                            </span>
                            <span class=move || {
                                if core_rank_list.with(|list| list.contains(&core_rank_value)) {
                                    "tag-highlight"
                                } else {
                                    ""
                                }
                            }>
                                <Tag class="plain-tag">{core_tag_label.clone()}</Tag>
                            </span>
                            <span class=move || {
                                if thcpl_rank_list.with(|list| list.contains(&thcpl_rank_value)) {
                                    "tag-highlight"
                                } else {
                                    ""
                                }
                            }>
                                <Tag class="plain-tag">{thcpl_tag_label.clone()}</Tag>
                            </span>
                            <span class="conference-category-chip">
                                {move || {
                                    if use_english.get() {
                                        conf.subname_en.clone()
                                    } else {
                                        conf.subname.clone()
                                    }
                                }}
                            </span>
                        </div>

                        {move || {
                            conf.comment.as_ref().map(|comment| {
                                view! {
                                    <div class="conference-note">
                                        <b>"NOTE: "</b>
                                        {comment.clone()}
                                    </div>
                                }
                            })
                        }}

                        {move || {
                            acceptance_rate.get().map(|acc| {
                                view! { <div class="conference-supporting-text">{format!("Acc. Rate: {acc}")}</div> }
                            })
                        }}

                        {website_link_view}
                    </div>

                    <div class="conference-card-right">
                        <div class="conference-card-countdown-block">
                            {move || {
                                if is_tbd {
                                    view! { <span class="conference-card-countdown countdown-normal">"TBD"</span> }
                                        .into_any()
                                } else {
                                    view! {
                                        <span class="conference-card-countdown">
                                            <CountDown remain=conf.remain />
                                        </span>
                                    }
                                        .into_any()
                                }
                            }}

                            <div class="conference-card-deadline">{deadline_text.clone()}</div>
                        </div>

                        {move || {
                            if is_tbd {
                                view! {
                                    <div class="conference-card-action-line">
                                        <a
                                            href="https://github.com/ccfddl/ccf-deadlines/pulls"
                                            class="inline-muted-link interactive-link"
                                            target="_blank"
                                            rel="noopener noreferrer"
                                        >
                                            "Pull request to update deadline"
                                        </a>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <div class="conference-card-action-line">
                                        <span class="conference-card-local-deadline">
                                            {conf.local_ddl.clone().unwrap_or_default()}
                                        </span>
                                        <CalendarPopover
                                            google_calendar_url=conf.google_calendar_url.clone()
                                            icloud_calendar_url=conf.icloud_calendar_url.clone()
                                            is_mobile
                                        />
                                    </div>
                                }
                                    .into_any()
                            }
                        }}

                        <Show when=move || !is_finished && !is_tbd>
                            <TimeLine time_points=conf.ddls.clone() />
                        </Show>
                    </div>
                </div>
            </div>
        </article>
    }
}

fn safe_external_link(link: &str) -> Option<&str> {
    let trimmed = link.trim();
    let lower_link = trimmed.to_ascii_lowercase();

    if lower_link.starts_with("https://") || lower_link.starts_with("http://") {
        Some(trimmed)
    } else {
        None
    }
}

fn format_website_label(link: &str) -> String {
    let without_protocol = link
        .trim_start_matches("https://")
        .trim_start_matches("http://");
    let label = without_protocol.chars().take(42).collect::<String>();

    if without_protocol.chars().count() > 42 {
        format!("{label}…")
    } else {
        label
    }
}

fn format_deadline_label(deadline: &str, timezone: &str) -> String {
    if let Ok(value) = NaiveDateTime::parse_from_str(deadline, "%Y-%m-%d %H:%M:%S") {
        return format!("Deadline: {} {}", value.format("%b %d, %H:%M"), timezone);
    }

    if let Ok(value) = NaiveDate::parse_from_str(deadline, "%Y-%m-%d") {
        return format!("Deadline: {} 23:59 {}", value.format("%b %d"), timezone);
    }

    format!("Deadline: {deadline} {timezone}")
}

#[cfg(test)]
mod tests {
    use super::{format_deadline_label, format_website_label, safe_external_link};

    #[test]
    fn accepts_http_and_https_links() {
        assert_eq!(
            safe_external_link("https://example.com/conf"),
            Some("https://example.com/conf")
        );
        assert_eq!(
            safe_external_link("http://example.com/conf"),
            Some("http://example.com/conf")
        );
    }

    #[test]
    fn rejects_scriptable_or_non_external_links() {
        for link in [
            "",
            "javascript:alert(1)",
            "JaVaScRiPt:alert(1)",
            "data:text/html,<script>alert(1)</script>",
            "/relative/path",
            "//example.com/conf",
        ] {
            assert_eq!(safe_external_link(link), None);
        }
    }

    #[test]
    fn trims_surrounding_space_before_validating_links() {
        assert_eq!(
            safe_external_link("  https://example.com/conf  "),
            Some("https://example.com/conf")
        );
    }

    #[test]
    fn formats_date_only_deadlines_as_end_of_day() {
        assert_eq!(
            format_deadline_label("2026-05-04", "UTC+0"),
            "Deadline: May 04 23:59 UTC+0"
        );
    }

    #[test]
    fn truncates_website_labels_on_character_boundaries() {
        let label = format_website_label(
            "https://例子测试例子测试例子测试例子测试例子测试例子测试例子测试例子测试例子测试.com/path",
        );

        assert!(label.ends_with('…'));
        assert_eq!(label.trim_end_matches('…').chars().count(), 42);
    }
}
