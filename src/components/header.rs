use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use thaw::{Icon, Switch};
use wasm_bindgen_futures::spawn_local;

use crate::components::gitbutton::GitButton;
use crate::components::theme::{ThemeController, ThemePreference};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct CommitData {
    commit: CommitInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct CommitInfo {
    message: String,
}

#[component]
pub fn Header(theme_controller: ThemeController, use_english: RwSignal<bool>) -> impl IntoView {
    let (show_latest_conf, set_show_latest_conf) = signal(false);
    let (show_str, set_show_str) = signal(String::new());
    let theme_preference = theme_controller.preference();

    let current_theme_label = move || match theme_preference.get() {
        ThemePreference::System => "System",
        ThemePreference::Light => "Light",
        ThemePreference::Dark => "Dark",
    };

    let next_theme_label = move || match theme_preference.get().next() {
        ThemePreference::System => "System",
        ThemePreference::Light => "Light",
        ThemePreference::Dark => "Dark",
    };

    let theme_button_label = move || {
        format!(
            "Theme: {}. Click to switch to {}.",
            current_theme_label(),
            next_theme_label()
        )
    };

    Effect::new(move |_| {
        spawn_local(async move {
            match fetch_latest_commit().await {
                Ok((show_conf, conf_str)) => {
                    set_show_latest_conf.set(show_conf);
                    set_show_str.set(conf_str);
                }
                Err(_) => {}
            }
        });
    });

    view! {
        <section class="hero-header">
            <div class="header-brand-row">
                <a href="/" class="title">
                    "CCFDDL"
                    <sup>"®"</sup>
                    "\u{00a0}Open Deadlines"
                </a>
                <div class="header-brand-actions">
                    <button
                        type="button"
                        class="header-theme-button"
                        aria-label=theme_button_label
                        title=theme_button_label
                        on:click=move |_| theme_controller.cycle_preference()
                    >
                        {move || {
                            let icon = match theme_preference.get() {
                                ThemePreference::System => icondata::BsCircleHalf,
                                ThemePreference::Light => icondata::BsSun,
                                ThemePreference::Dark => icondata::BsMoonStars,
                            };

                            view! { <Icon icon=icon class="header-theme-icon" /> }
                        }}
                    </button>
                    <div class="header-git-button">
                        <GitButton />
                    </div>
                    <div class="header-language-switch">
                        <span class=("is_active", move || !use_english.get())>"中文"</span>
                        <Switch checked=use_english />
                        <span class=("is_active", move || use_english.get())>"English"</span>
                    </div>
                </div>
            </div>

            <div class="header-subtitle-row">
                <div class="header-subtitle-primary">
                    "Worldwide conference deadline search and countdowns."
                </div>
                <div class="header-subtitle-links subtitle">
                    <span>
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
                    <span>
                        "Tabular portal: "
                        <a class="header-muted-link interactive-link" href="https://ccfddl.cn/" target="_blank">
                            "ccfddl.cn"
                        </a>
                    </span>
                    <span>
                        "WeChat applet: "
                        <a
                            class="header-muted-link interactive-link"
                            href="https://github.com/ccfddl/ccf-deadlines/blob/main/.readme_assets/applet_qrcode.jpg"
                            target="_blank"
                        >
                            "scan to try"
                        </a>
                    </span>
                </div>
            </div>

            <div class="header-disclaimer-row">
                <p class="header-disclaimer-message">
                    "*Disclaimer: The data provided by ccfddl is manually collected and for reference purposes only."
                </p>
            </div>

            {move || {
                show_latest_conf
                    .get()
                    .then(|| {
                        view! {
                            <div class="header-announcement-row" role="status" aria-live="polite">
                                <div class="header-announcement-content">
                                    <span class="header-announcement-label">"Latest update"</span>
                                    <span class="header-announcement-text">{show_str.get()}</span>
                                    <span class="header-announcement-badge">"New"</span>
                                </div>
                            </div>
                        }
                    })
            }}
        </section>
    }
}

async fn fetch_latest_commit() -> Result<(bool, String), Box<dyn std::error::Error>> {
    let url = "https://api.github.com/repos/ccfddl/ccf-deadlines/commits?page=1&per_page=10";

    let response = reqwest::get(url).await?;
    let commits: Vec<CommitData> = response.json().await?;

    for commit in commits {
        let message = commit.commit.message;
        let words: Vec<&str> = message.split_whitespace().collect();

        if !words.is_empty() {
            let first_word: String = words[0].to_lowercase();
            if first_word == "update" || first_word == "add" {
                let mut result_str: String = message[..].to_string();
                if let Some(idx) = message.find('(') {
                    result_str = message[..idx].to_string();
                }
                return Ok((true, result_str));
            }
        }
    }

    Ok((false, String::new()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn header_rows_match_phase1_structure() {
        const HEADER_SOURCE: &str = include_str!("header.rs");

        fn index_of(haystack: &str, needle: &str) -> usize {
            haystack
                .find(needle)
                .unwrap_or_else(|| panic!("expected to find `{needle}` in Header source"))
        }

        let brand_row = index_of(HEADER_SOURCE, "class=\"header-brand-row\"");
        let subtitle_row = index_of(HEADER_SOURCE, "class=\"header-subtitle-row\"");
        let disclaimer_row = index_of(HEADER_SOURCE, "class=\"header-disclaimer-row\"");
        let announcement_row = index_of(HEADER_SOURCE, "class=\"header-announcement-row\"");
        index_of(HEADER_SOURCE, "class=\"header-language-switch\"");
        index_of(HEADER_SOURCE, "class=\"header-brand-actions\"");

        assert!(brand_row < subtitle_row);
        assert!(subtitle_row < disclaimer_row);
        assert!(disclaimer_row < announcement_row);
    }

    #[test]
    fn header_language_switch_still_uses_use_english_signal() {
        const HEADER_SOURCE: &str = include_str!("header.rs");

        assert!(HEADER_SOURCE.contains("pub fn Header(theme_controller: ThemeController, use_english: RwSignal<bool>) -> impl IntoView"));
        assert!(HEADER_SOURCE.contains("<Switch checked=use_english />"));
        assert!(HEADER_SOURCE.contains("class=\"header-language-switch\""));
    }

    #[test]
    fn header_brand_actions_follow_phase1_order() {
        const HEADER_SOURCE: &str = include_str!("header.rs");
        let actions_start = HEADER_SOURCE
            .find("<div class=\"header-brand-actions\">")
            .expect("expected header-brand-actions block");
        let actions_end = HEADER_SOURCE[actions_start..]
            .find("<div class=\"header-subtitle-row\">")
            .map(|offset| actions_start + offset)
            .expect("expected end of header-brand-actions block");
        let actions_source = &HEADER_SOURCE[actions_start..actions_end];

        fn index_of(haystack: &str, needle: &str) -> usize {
            haystack
                .find(needle)
                .unwrap_or_else(|| panic!("expected to find `{needle}` in header-brand-actions block"))
        }

        let theme_button = index_of(actions_source, "class=\"header-theme-button\"");
        let git_button = index_of(actions_source, "class=\"header-git-button\"");
        let language_switch = index_of(actions_source, "class=\"header-language-switch\"");

        assert!(theme_button < git_button);
        assert!(git_button < language_switch);
    }

    #[test]
    fn header_disclaimer_and_announcement_structure_is_present() {
        const HEADER_SOURCE: &str = include_str!("header.rs");

        assert!(HEADER_SOURCE.contains("class=\"header-disclaimer-row\""));
        assert!(HEADER_SOURCE.contains("class=\"header-disclaimer-message\""));
        assert!(HEADER_SOURCE.contains("class=\"header-announcement-content\""));
        assert!(HEADER_SOURCE.contains("class=\"header-announcement-badge\""));
    }

    #[test]
    fn header_rows_have_phase1_surface_hierarchy_in_styles() {
        const STYLES_SOURCE: &str = include_str!("../../public/styles.css");

        assert!(STYLES_SOURCE.contains(".hero-header {"));
        assert!(STYLES_SOURCE.contains(".header-brand-row {"));
        assert!(STYLES_SOURCE.contains(".header-subtitle-row {"));
        assert!(STYLES_SOURCE.contains(".header-disclaimer-row {"));
        assert!(STYLES_SOURCE.contains(".header-announcement-row {"));
        assert!(STYLES_SOURCE.contains(".header-language-switch {"));
    }

    #[test]
    fn header_brand_actions_base_rule_is_defined_once_in_styles() {
        const STYLES_SOURCE: &str = include_str!("../../public/styles.css");
        const HEADER_BRAND_ACTIONS_BASE_RULE: &str = ".header-brand-actions {\n    display: inline-flex;\n    align-items: center;\n    gap: 10px;\n    margin-left: auto;\n}";
        let normalized_styles = STYLES_SOURCE.replace("\r\n", "\n");

        assert_eq!(normalized_styles.matches(HEADER_BRAND_ACTIONS_BASE_RULE).count(), 1);
    }
}
