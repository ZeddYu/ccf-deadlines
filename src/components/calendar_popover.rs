use crate::components::dom::focus_element_by_id_after_render;
use leptos::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use thaw::*;
use web_sys::KeyboardEvent;

#[component]
pub fn CalendarPopover(
    google_calendar_url: Option<String>,
    icloud_calendar_url: Option<String>,
    is_mobile: RwSignal<bool>,
) -> impl IntoView {
    let has_calendar_options = google_calendar_url.is_some() || icloud_calendar_url.is_some();
    let popover_id = calendar_popover_id(&google_calendar_url, &icloud_calendar_url);
    let popover_title_id = format!("{popover_id}-title");
    let trigger_id = format!("{popover_id}-trigger");
    let close_button_id = format!("{popover_id}-close");
    let google_link_id = format!("{popover_id}-google");
    let icloud_link_id = format!("{popover_id}-icloud");

    if !has_calendar_options {
        return view! {
            <button
                type="button"
                class="calendar-popover-trigger"
                aria-label="Calendar options unavailable"
                disabled
            >
                <Icon icon=icondata::VsCalendar class="calendar-popover-trigger-icon" />
            </button>
        }
        .into_any();
    }

    let show_popover = RwSignal::new(false);
    let should_focus_popover = RwSignal::new(false);
    let focus_target_id = if google_calendar_url.is_some() {
        google_link_id.clone()
    } else {
        icloud_link_id.clone()
    };

    Effect::new(move |_| {
        if show_popover.get() && should_focus_popover.get() {
            should_focus_popover.set(false);
            focus_element_by_id_after_render(focus_target_id.clone());
        }
    });

    let trigger_id_for_mouseleave = trigger_id.clone();
    let close_button_id_for_mouseleave = close_button_id.clone();
    let google_link_id_for_mouseleave = google_link_id.clone();
    let icloud_link_id_for_mouseleave = icloud_link_id.clone();
    let trigger_id_for_keydown = trigger_id.clone();
    let trigger_id_for_close = trigger_id.clone();
    let popover_id_for_trigger = popover_id.clone();

    view! {
        <div
            class="calendar-popover-container"
            on:mouseenter=move |_| {
                if !is_mobile.get() {
                    show_popover.set(true);
                }
            }
            on:mouseleave=move |_| {
                if is_mobile.get() {
                    return;
                }

                if !is_active_element_id_one_of(&[
                    &trigger_id_for_mouseleave,
                    &close_button_id_for_mouseleave,
                    &google_link_id_for_mouseleave,
                    &icloud_link_id_for_mouseleave,
                ]) {
                    should_focus_popover.set(false);
                    show_popover.set(false);
                }
            }
            on:keydown=move |event: KeyboardEvent| {
                if event.key() == "Escape" && show_popover.get() {
                    event.stop_propagation();
                    should_focus_popover.set(false);
                    show_popover.set(false);
                    focus_element_by_id_after_render(trigger_id_for_keydown.clone());
                }
            }
        >
            <button
                id=trigger_id.clone()
                type="button"
                class="calendar-popover-trigger"
                aria-label="Add to calendar"
                aria-haspopup="dialog"
                aria-expanded=move || show_popover.get().to_string()
                aria-controls=popover_id_for_trigger
                on:click=move |_| {
                    if is_mobile.get() && show_popover.get_untracked() {
                        should_focus_popover.set(false);
                        show_popover.set(false);
                        return;
                    }

                    should_focus_popover.set(true);
                    show_popover.set(true);
                }
            >
                <Icon icon=icondata::VsCalendar class="calendar-popover-trigger-icon" />
            </button>

            {move || {
                let google_calendar_url_clone = google_calendar_url.clone();
                let icloud_calendar_url_clone = icloud_calendar_url.clone();
                let popover_id = popover_id.clone();
                let popover_title_id = popover_title_id.clone();
                let popover_title_labelledby = popover_title_id.clone();
                let close_button_id = close_button_id.clone();
                let google_link_id = google_link_id.clone();
                let icloud_link_id = icloud_link_id.clone();
                let trigger_id_for_close = trigger_id_for_close.clone();

                if show_popover.get() {
                    view! {
                        <div
                            id=popover_id
                            class="calendar-popover"
                            role="dialog"
                            aria-labelledby=popover_title_labelledby
                        >
                            <div class="calendar-popover-header">
                                <div id=popover_title_id class="calendar-popover-title">"Add to Calendar:"</div>
                                <button
                                    id=close_button_id
                                    type="button"
                                    class="calendar-popover-close"
                                    aria-label="Close calendar options"
                                    on:click=move |_| {
                                        should_focus_popover.set(false);
                                        show_popover.set(false);
                                        focus_element_by_id_after_render(trigger_id_for_close.clone());
                                    }
                                >
                                    "×"
                                </button>
                            </div>

                            <div>
                                {google_calendar_url_clone.map(|google_url| {
                                    view! {
                                        <div class="calendar-popover-item">
                                            <img
                                                src="//ssl.gstatic.com/calendar/images/dynamiclogo_2020q4/calendar_31_2x.png#"
                                                alt=""
                                                class="calendar-popover-icon"
                                            />
                                            <a
                                                id=google_link_id
                                                href=google_url
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="calendar-popover-link interactive-link"
                                            >
                                                "Google Calendar"
                                            </a>
                                        </div>
                                    }
                                })}

                                {icloud_calendar_url_clone.map(|icloud_url| {
                                    view! {
                                        <div class="calendar-popover-item">
                                            <img
                                                src="https://help.apple.com/assets/61526E8E1494760B754BD308/61526E8F1494760B754BD30F/zh_CN/2162f7d3de310d2b3503c0bbebdc3d56.png"
                                                alt=""
                                                class="calendar-popover-icon"
                                            />
                                            <a
                                                id=icloud_link_id
                                                href=icloud_url
                                                class="calendar-popover-link interactive-link"
                                            >
                                                "iCloud Calendar"
                                            </a>
                                        </div>
                                    }
                                })}
                            </div>

                            <div class="calendar-popover-arrow"></div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
    .into_any()
}

fn calendar_popover_id(
    google_calendar_url: &Option<String>,
    icloud_calendar_url: &Option<String>,
) -> String {
    let mut hasher = DefaultHasher::new();
    google_calendar_url.hash(&mut hasher);
    icloud_calendar_url.hash(&mut hasher);
    format!("calendar-popover-{:x}", hasher.finish())
}

fn is_active_element_id_one_of(ids: &[&str]) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.active_element())
            .map(|element| ids.iter().any(|id| element.id() == *id))
            .unwrap_or(false)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = ids;
        false
    }
}
