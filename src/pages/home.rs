use crate::components::header::Header;
use crate::components::showtable::ShowTable;
use crate::components::theme::{create_thaw_theme, ThemeController};
use leptos::prelude::*;

use thaw::*;
use web_sys::window;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let theme_controller = ThemeController::new();
    let mode = theme_controller.mode();
    let theme = RwSignal::new(create_thaw_theme(mode.get_untracked()));
    let use_english = RwSignal::new(load_use_english_preference());

    Effect::new(move |_| {
        theme.set(create_thaw_theme(mode.get()));
    });

    view! {
        <ConfigProvider theme=theme>
            <div class="home">
                <Header theme_controller=theme_controller.clone() use_english=use_english />
                <ShowTable use_english=use_english />
            </div>
        </ConfigProvider>
    }
}

fn load_use_english_preference() -> bool {
    window()
        .and_then(|window| window.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item("use_english").ok().flatten())
        .and_then(|value| value.parse::<bool>().ok())
        .unwrap_or(false)
}
