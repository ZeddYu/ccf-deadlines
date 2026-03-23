use leptos::prelude::*;
use serde::Deserialize;
use thaw::Icon;
use wasm_bindgen_futures::spawn_local;

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
        spawn_local(async move {
            if let Ok(repo) = fetch_repo_data().await {
                set_star_count.set(Some(repo.stargazers_count));
            }
        });
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
        <a
            href=REPO_URL
            target="_blank"
            rel="noreferrer"
            class="header-github-link header-github-compact"
            aria-label=star_label
            title=star_label
        >
            <Icon icon=icondata::BsGithub class="header-github-icon" />
            <span class="header-github-divider" aria-hidden="true"></span>
            <Icon icon=icondata::BsStarFill class="header-github-star-icon" />
            <span class="header-github-count">{short_star_count}</span>
        </a>
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

async fn fetch_repo_data() -> Result<RepoData, Box<dyn std::error::Error>> {
    let response = reqwest::get(REPO_API_URL).await?;
    let repo: RepoData = response.json().await?;
    Ok(repo)
}
