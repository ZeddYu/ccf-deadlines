use leptos::prelude::*;
use thaw::*;

#[component]
pub fn CalendarPopover(
    google_calendar_url: Option<String>,
    icloud_calendar_url: Option<String>,
    is_mobile: RwSignal<bool>,
) -> impl IntoView {
    let show_popover = RwSignal::new(false);

    view! {
        <div
            class="calendar-popover-container"
            on:click=move |_| {
                if is_mobile.get() {
                    show_popover.update(|v| *v = !*v);
                } else {
                    show_popover.set(false);
                }
            }
            on:mouseenter=move |_| {
                if !is_mobile.get() {
                    show_popover.set(true);
                }
            }
            on:mouseleave=move |_| {
                if !is_mobile.get() {
                    show_popover.set(false);
                }
            }
        >
            <Icon icon=icondata::VsCalendar class="calendar-popover-trigger" />

            {move || {
                let google_calendar_url_clone = google_calendar_url.clone();
                let icloud_calendar_url_clone = icloud_calendar_url.clone();

                if show_popover.get() {
                    view! {
                        <div class="calendar-popover">
                            <div>
                                <div class="calendar-popover-title">"Add to Calendar:"</div>

                                <div class="calendar-popover-item">
                                    <img
                                        src="//ssl.gstatic.com/calendar/images/dynamiclogo_2020q4/calendar_31_2x.png#"
                                        alt="Google Calendar"
                                        class="calendar-popover-icon"
                                    />
                                    <a
                                        href=google_calendar_url_clone
                                        target="_blank"
                                        class="calendar-popover-link interactive-link"
                                    >
                                        "Google Calendar"
                                    </a>
                                </div>

                                <div class="calendar-popover-item">
                                    <img
                                        src="https://help.apple.com/assets/61526E8E1494760B754BD308/61526E8F1494760B754BD30F/zh_CN/2162f7d3de310d2b3503c0bbebdc3d56.png"
                                        alt="iCloud Calendar"
                                        class="calendar-popover-icon"
                                    />
                                    <a
                                        href=icloud_calendar_url_clone
                                        class="calendar-popover-link interactive-link"
                                    >
                                        "iCloud Calendar"
                                    </a>
                                </div>
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
}
