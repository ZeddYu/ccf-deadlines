use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use thaw::Icon;
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
pub fn Header(theme_controller: ThemeController) -> impl IntoView {
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

    // Effect to fetch GitHub commits data on mount
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
        <section>
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
                </div>
                {move || {
                    show_latest_conf
                        .get()
                        .then(|| {
                            view! {
                                <span class="header-latest-badge">
                                    "Latest: " {show_str.get()} " !!!"
                                </span>
                            }
                        })
                }}
            </div>
            <div class="el-row subtitle">
                "Worldwide Conference Deadline Countdowns. To add/edit a conference,\u{00a0}"
                <a
                    class="header-muted-link interactive-link"
                    href="https://github.com/ccfddl/ccf-deadlines/pulls"
                    target="_blank"
                >
                    "send a pull request"
                </a> "."
            </div>
            <div class="el-row subtitle">
                "Preview tabular portal:\u{00a0}"
                <a class="header-muted-link interactive-link" href="https://ccfddl.cn/" target="_blank">
                    "https://ccfddl.cn/"
                </a> ", or scan to try\u{00a0}"
                <a
                    class="header-muted-link interactive-link"
                    href="https://github.com/ccfddl/ccf-deadlines/blob/main/.readme_assets/applet_qrcode.jpg"
                    target="_blank"
                >
                    "wechat applet"
                </a> "."
            </div>
            <div class="el-row subtitle">
                "*Disclaimer: The data provided by ccfddl is manually collected and for reference purposes only."
            </div>
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
