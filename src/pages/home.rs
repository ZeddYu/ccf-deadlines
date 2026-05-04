use crate::components::header::Header;
use crate::components::showtable::ShowTable;
use crate::components::storage::get_from_local_storage;
use crate::components::theme::{ThemeController, create_thaw_theme};
use leptos::prelude::*;

use thaw::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let theme_controller = ThemeController::new();
    let mode = theme_controller.mode();
    let theme = RwSignal::new(create_thaw_theme(mode.get_untracked()));
    let use_english = RwSignal::new(
        get_from_local_storage("use_english")
            .as_deref()
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(false),
    );

    Effect::new(move |_| {
        theme.set(create_thaw_theme(mode.get()));
    });

    view! {
        <ConfigProvider theme=theme>
            <div class="home">
                <a class="skip-link" href="#main-content">"Skip to main content"</a>
                <header>
                    <Header theme_controller=theme_controller.clone() use_english />
                </header>
                <main id="main-content" tabindex="-1">
                    <ShowTable use_english />
                </main>
            </div>
        </ConfigProvider>
    }
}
