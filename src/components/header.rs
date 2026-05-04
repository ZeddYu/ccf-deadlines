use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use thaw::Icon;

use crate::components::dom::schedule_non_critical_async_request;
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

    let language_button_label = move || {
        if use_english.get() {
            "Switch language to Chinese"
        } else {
            "Switch language to English"
        }
    };

    Effect::new(move |_| {
        fetch_latest_commit_after_delay(set_show_latest_conf, set_show_str);
    });

    view! {
        <section class="home-masthead" aria-labelledby="site-title">
            <div class="home-masthead-shell">
                <div class="home-masthead-copy">
                    <h1 id="site-title" class="home-masthead-heading">
                        <a href="/" class="home-masthead-title">
                            "CCFDDL"
                            <sup>"®"</sup>
                            "\u{00a0}Open Deadlines"
                        </a>
                    </h1>
                    <p class="home-masthead-subtitle">
                        "Global conference deadline countdowns for fast search and tracking."
                    </p>
                </div>

                <div class="home-masthead-actions" aria-label="Homepage actions">
                    <button
                        type="button"
                        class="header-theme-button"
                        aria-label=theme_button_label
                        title=theme_button_label
                        on:click=move |_| theme_controller.cycle_preference()
                    >
                        {move || {
                            let icon = match theme_preference.get() {
                                ThemePreference::System => icondata::BsDisplayFill,
                                ThemePreference::Light => icondata::BsSunFill,
                                ThemePreference::Dark => icondata::BsMoonStarsFill,
                            };

                            view! { <Icon icon=icon class="header-theme-icon" /> }
                        }}
                    </button>
                    <div class="header-git-button">
                        <GitButton />
                    </div>
                    <button
                        type="button"
                        class="header-language-pill"
                        aria-label=language_button_label
                        title=language_button_label
                        on:click=move |_| use_english.update(|value| *value = !*value)
                    >
                        <Icon icon=icondata::BsGlobe2 class="header-language-icon" />
                        <span>{move || if use_english.get() { "English" } else { "中文" }}</span>
                    </button>
                </div>
            </div>

            <div class="home-trust-row">
                {move || {
                    show_latest_conf
                        .get()
                        .then(|| {
                            view! {
                                <div
                                    class="home-update-note"
                                    role="status"
                                    aria-live="polite"
                                    aria-atomic="true"
                                >
                                    <Icon icon=icondata::BsInfoCircleFill class="home-update-icon" />
                                    <span class="home-update-text">
                                        "Latest Update: " {show_str.get()}
                                    </span>
                                </div>
                            }
                        })
                }}

                <div class="home-data-note">
                    "Data is manually collected and is for reference purposes only."
                    <Icon icon=icondata::BsInfoCircle class="hero-disclaimer-icon" />
                </div>

                <a
                    class="home-contribute-link interactive-link"
                    href="https://github.com/ccfddl/ccf-deadlines/pulls"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    "Contribute updates"
                </a>
            </div>
        </section>
    }
}

fn fetch_latest_commit_after_delay(
    set_show_latest_conf: WriteSignal<bool>,
    set_show_str: WriteSignal<String>,
) {
    schedule_non_critical_async_request(async move {
        if let Ok((show_conf, conf_str)) = fetch_latest_commit().await {
            set_show_latest_conf.set(show_conf);
            set_show_str.set(conf_str);
        }
    });
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
