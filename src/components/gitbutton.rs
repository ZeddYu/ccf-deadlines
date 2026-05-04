use leptos::prelude::*;
use serde::Deserialize;
use thaw::Icon;

use crate::components::dom::schedule_non_critical_async_request;

const REPO_URL: &str = "https://github.com/ccfddl/ccf-deadlines";
const REPO_API_URL: &str = "https://api.github.com/repos/ccfddl/ccf-deadlines";

#[derive(Debug, Deserialize, Clone)]
struct RepoData {
    stargazers_count: u64,
}

#[component]
pub fn GitButton() -> impl IntoView {
    let (star_count, set_star_count) = signal(None::<u64>);

    Effect::new(move |_| {
        fetch_repo_data_after_delay(set_star_count);
    });

    let short_star_count = move || {
        star_count
            .get()
            .map(format_star_count)
            .unwrap_or_else(|| "--".to_string())
    };

    let star_label = move || match star_count.get() {
        Some(count) => format!("Open the ccf-deadlines GitHub repository ({count} stars)"),
        None => "Open the ccf-deadlines GitHub repository".to_string(),
    };

    view! {
        <div class="header-github-group">
            <a
                href=REPO_URL
                target="_blank"
                rel="noopener noreferrer"
                class="header-github-link"
                aria-label="Open the ccf-deadlines GitHub repository"
                title="Open the ccf-deadlines GitHub repository"
            >
                <Icon icon=icondata::BsGithub class="header-github-icon" />
                <span>"GitHub"</span>
            </a>
            <a
                href=REPO_URL
                target="_blank"
                rel="noopener noreferrer"
                class="header-github-link header-github-star-pill"
                aria-label=star_label
                title=star_label
            >
                <Icon icon=icondata::BsStarFill class="header-github-star-icon" />
                <span class="header-github-count">{short_star_count}</span>
            </a>
        </div>
    }
}

fn format_star_count(count: u64) -> String {
    if count < 1_000 {
        count.to_string()
    } else {
        let value = count as f64 / 1_000.0;
        let rounded = (value * 10.0).round() / 10.0;

        if rounded >= 10.0 {
            format!("{rounded:.0}k")
        } else {
            format!("{rounded:.1}k")
        }
    }
}

fn fetch_repo_data_after_delay(set_star_count: WriteSignal<Option<u64>>) {
    schedule_non_critical_async_request(async move {
        if let Ok(repo) = fetch_repo_data().await {
            set_star_count.set(Some(repo.stargazers_count));
        }
    });
}

async fn fetch_repo_data() -> Result<RepoData, Box<dyn std::error::Error>> {
    let response = reqwest::get(REPO_API_URL).await?;
    let repo: RepoData = response.json().await?;
    Ok(repo)
}
