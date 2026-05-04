use crate::components::storage::{get_from_local_storage, set_in_local_storage};
use leptos::prelude::*;
use std::cell::RefCell;
use thaw::Theme;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{MediaQueryList, MediaQueryListEvent, window};

const THEME_STORAGE_KEY: &str = "theme";
const THEME_LIGHT: &str = "light";
const THEME_DARK: &str = "dark";
const THEME_SYSTEM: &str = "system";
const BRAND_COLOR_LIGHT: &str = "#409eff";
const BRAND_COLOR_LIGHT_HOVER: &str = "#66b1ff";
const BRAND_COLOR_DARK: &str = "#7aa2ff";
const BRAND_COLOR_DARK_HOVER: &str = "#92b2ff";
const STROKE_LIGHT: &str = "#dcdfe6";
const STROKE_LIGHT_SUBTLE: &str = "#ebeef5";
const STROKE_DARK: &str = "rgba(255, 255, 255, 0.16)";
const STROKE_DARK_SUBTLE: &str = "rgba(255, 255, 255, 0.10)";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemePreference {
    Light,
    Dark,
    System,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Clone)]
pub struct ThemeController {
    preference: RwSignal<ThemePreference>,
    mode: RwSignal<ThemeMode>,
}

thread_local! {
    static SYSTEM_THEME_LISTENER: RefCell<Option<SystemThemeListener>> = const { RefCell::new(None) };
}

struct SystemThemeListener {
    media_query: MediaQueryList,
    closure: Closure<dyn FnMut(MediaQueryListEvent)>,
}

impl ThemePreference {
    pub fn as_storage_value(self) -> &'static str {
        match self {
            Self::Light => THEME_LIGHT,
            Self::Dark => THEME_DARK,
            Self::System => THEME_SYSTEM,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::System => Self::Light,
            Self::Light => Self::Dark,
            Self::Dark => Self::System,
        }
    }

    pub fn from_storage_value(value: Option<&str>) -> Self {
        match value {
            Some(THEME_LIGHT) => Self::Light,
            Some(THEME_DARK) => Self::Dark,
            Some(THEME_SYSTEM) | None => Self::System,
            _ => Self::System,
        }
    }
}

impl ThemeMode {
    pub fn as_data_theme(self) -> &'static str {
        match self {
            Self::Light => THEME_LIGHT,
            Self::Dark => THEME_DARK,
        }
    }
}

impl ThemeController {
    pub fn new() -> Self {
        let preference = RwSignal::new(ThemePreference::from_storage_value(
            get_from_local_storage(THEME_STORAGE_KEY).as_deref(),
        ));
        let mode = RwSignal::new(resolve_theme_mode(preference.get_untracked()));

        apply_theme_to_document(mode.get_untracked());
        SYSTEM_THEME_LISTENER.with(|listener| {
            *listener.borrow_mut() = setup_system_theme_listener(preference, mode);
        });

        Self { preference, mode }
    }

    pub fn preference(&self) -> RwSignal<ThemePreference> {
        self.preference
    }

    pub fn mode(&self) -> RwSignal<ThemeMode> {
        self.mode
    }

    pub fn set_preference(&self, preference: ThemePreference) {
        self.preference.set(preference);
        set_in_local_storage(THEME_STORAGE_KEY, preference.as_storage_value());
        let mode = resolve_theme_mode(preference);
        self.mode.set(mode);
        apply_theme_to_document(mode);
    }

    pub fn cycle_preference(&self) {
        self.set_preference(self.preference.get_untracked().next());
    }
}

pub fn create_thaw_theme(mode: ThemeMode) -> Theme {
    let mut theme = match mode {
        ThemeMode::Light => Theme::light(),
        ThemeMode::Dark => Theme::dark(),
    };

    let (brand_color, brand_hover, accessible_stroke, subtle_stroke) = match mode {
        ThemeMode::Light => (
            BRAND_COLOR_LIGHT,
            BRAND_COLOR_LIGHT_HOVER,
            STROKE_LIGHT,
            STROKE_LIGHT_SUBTLE,
        ),
        ThemeMode::Dark => (
            BRAND_COLOR_DARK,
            BRAND_COLOR_DARK_HOVER,
            STROKE_DARK,
            STROKE_DARK_SUBTLE,
        ),
    };

    theme
        .color
        .set_color_compound_brand_background(brand_color.to_string());
    theme
        .color
        .set_color_compound_brand_background_hover(brand_hover.to_string());
    theme
        .color
        .set_color_neutral_stroke_accessible(accessible_stroke.to_string());
    theme
        .color
        .set_color_neutral_stroke_accessible_pressed(brand_hover.to_string());
    theme
        .color
        .set_color_neutral_stroke_accessible_hover(brand_hover.to_string());
    theme
        .color
        .set_color_neutral_stroke_2(subtle_stroke.to_string());

    theme
}

fn resolve_theme_mode(preference: ThemePreference) -> ThemeMode {
    match preference {
        ThemePreference::Light => ThemeMode::Light,
        ThemePreference::Dark => ThemeMode::Dark,
        ThemePreference::System => {
            if window()
                .and_then(|win| {
                    win.match_media("(prefers-color-scheme: dark)")
                        .ok()
                        .flatten()
                })
                .map(|query| query.matches())
                .unwrap_or(false)
            {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            }
        }
    }
}

fn setup_system_theme_listener(
    preference: RwSignal<ThemePreference>,
    mode: RwSignal<ThemeMode>,
) -> Option<SystemThemeListener> {
    let media_query = window().and_then(|win| {
        win.match_media("(prefers-color-scheme: dark)")
            .ok()
            .flatten()
    })?;

    let closure = Closure::wrap(Box::new(move |event: MediaQueryListEvent| {
        if preference.get_untracked() != ThemePreference::System {
            return;
        }

        let next_mode = if event.matches() {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };

        mode.set(next_mode);
        apply_theme_to_document(next_mode);
    }) as Box<dyn FnMut(MediaQueryListEvent)>);

    let _ =
        media_query.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());

    Some(SystemThemeListener {
        media_query,
        closure,
    })
}

impl Drop for SystemThemeListener {
    fn drop(&mut self) {
        let _ = self
            .media_query
            .remove_event_listener_with_callback("change", self.closure.as_ref().unchecked_ref());
    }
}

fn apply_theme_to_document(mode: ThemeMode) {
    if let Some(document) = window().and_then(|win| win.document())
        && let Some(root) = document.document_element()
    {
        let _ = root.set_attribute("data-theme", mode.as_data_theme());
    }
}
