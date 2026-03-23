use crate::components::header::Header;
use crate::components::showtable::ShowTable;
use crate::components::theme::{create_thaw_theme, ThemeController};
use leptos::prelude::*;

use thaw::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let theme_controller = ThemeController::new();
    let mode = theme_controller.mode();
    let theme = RwSignal::new(create_thaw_theme(mode.get_untracked()));

    Effect::new(move |_| {
        theme.set(create_thaw_theme(mode.get()));
    });

    view! {
        <ConfigProvider theme=theme>
            <div class="home">
                <Header theme_controller=theme_controller.clone() />
                <ShowTable />
            </div>
        </ConfigProvider>
    }
}
