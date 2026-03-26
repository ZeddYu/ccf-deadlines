use crate::components::checkbox_button::*;
use leptos::either::Either;
use leptos::prelude::*;
use std::collections::HashSet;
use thaw::*;

#[component]
pub fn TopToolbar(
    input_value: RwSignal<String>,
    rank_list: RwSignal<HashSet<String>>,
    core_rank_list: RwSignal<HashSet<String>>,
    thcpl_rank_list: RwSignal<HashSet<String>>,
    open_dropdown: RwSignal<Option<String>>,
    show_filters: RwSignal<bool>,
    show_subscription_modal: RwSignal<bool>,
    is_mobile: RwSignal<bool>,
    use_english: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="primary-toolbar">
            <div class="primary-toolbar-main">
                <div class="timezone-search">
                    <Input
                        value=input_value
                        placeholder="Search conferences..."
                        size=InputSize::Small
                        class="custom-search-input"
                    >
                        <InputPrefix slot>
                            <Icon icon=icondata::FiSearch class="search-prefix-icon" />
                        </InputPrefix>
                    </Input>
                </div>
            </div>

            <div class="primary-toolbar-actions">
                <Button
                    class="toolbar-action-button subscribe-button"
                    size=ButtonSize::Small
                    appearance=ButtonAppearance::Subtle
                    on_click=move |_| show_subscription_modal.set(true)
                >
                    <Icon icon=icondata::AiCalendarOutlined class="calendar-button-icon" />
                    {move || if use_english.get() { "Subscribe" } else { "订阅" }}
                </Button>
                {move || {
                    if is_mobile.get() {
                        Either::Left(view! {
                            <div class="mobile-filter-menu">
                                <Button
                                    class="toolbar-action-button filter-toggle-button"
                                    size=ButtonSize::Small
                                    appearance=ButtonAppearance::Subtle
                                    on_click=move |_| show_filters.update(|v| *v = !*v)
                                >
                                    <Icon icon=icondata::FiFilter class="filter-button-icon" />
                                    {move || if use_english.get() { "Filters" } else { "筛选" }}
                                    <Icon
                                        icon=if show_filters.get() {
                                            icondata::BsChevronUp
                                        } else {
                                            icondata::BsChevronDown
                                        }
                                        class="filter-button-chevron"
                                    />
                                </Button>
                                {move || {
                                    if show_filters.get() {
                                        Either::Left(view! {
                                            <div class="mobile-filter-panel">
                                                <MultiSelectDropdown
                                                    dropdown_id="ccf".to_string()
                                                    title="CCF".to_string()
                                                    options=ccf_filter_options()
                                                    selected_values=rank_list
                                                    use_english=use_english
                                                    panel_width="180px".to_string()
                                                    open_dropdown=open_dropdown
                                                />
                                                <MultiSelectDropdown
                                                    dropdown_id="core".to_string()
                                                    title="CORE".to_string()
                                                    options=core_filter_options()
                                                    selected_values=core_rank_list
                                                    use_english=use_english
                                                    panel_width="188px".to_string()
                                                    open_dropdown=open_dropdown
                                                />
                                                <MultiSelectDropdown
                                                    dropdown_id="thcpl".to_string()
                                                    title="THCPL".to_string()
                                                    options=thcpl_filter_options()
                                                    selected_values=thcpl_rank_list
                                                    use_english=use_english
                                                    panel_width="196px".to_string()
                                                    open_dropdown=open_dropdown
                                                />
                                            </div>
                                        })
                                    } else {
                                        Either::Right(view! { <></> })
                                    }
                                }}
                            </div>
                        })
                    } else {
                        Either::Right(view! {
                            <div class="desktop-filter-actions">
                                <MultiSelectDropdown
                                    dropdown_id="ccf".to_string()
                                    title="CCF".to_string()
                                    options=ccf_filter_options()
                                    selected_values=rank_list
                                    use_english=use_english
                                    panel_width="180px".to_string()
                                    open_dropdown=open_dropdown
                                />
                                <MultiSelectDropdown
                                    dropdown_id="core".to_string()
                                    title="CORE".to_string()
                                    options=core_filter_options()
                                    selected_values=core_rank_list
                                    use_english=use_english
                                    panel_width="188px".to_string()
                                    open_dropdown=open_dropdown
                                />
                                <MultiSelectDropdown
                                    dropdown_id="thcpl".to_string()
                                    title="THCPL".to_string()
                                    options=thcpl_filter_options()
                                    selected_values=thcpl_rank_list
                                    use_english=use_english
                                    panel_width="196px".to_string()
                                    open_dropdown=open_dropdown
                                />
                            </div>
                        })
                    }
                }}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn top_toolbar_uses_expected_placeholder() {
        const SOURCE: &str = include_str!("top_toolbar.rs");

        assert!(SOURCE.contains("Search conferences..."));
    }

    #[test]
    fn top_toolbar_keeps_filter_and_subscribe_in_primary_actions() {
        const SOURCE: &str = include_str!("top_toolbar.rs");

        assert!(SOURCE.contains("class=\"primary-toolbar-actions\""));
        assert!(SOURCE.contains("class=\"toolbar-action-button subscribe-button\""));
        assert!(SOURCE.contains("class=\"toolbar-action-button filter-toggle-button\""));
        assert!(SOURCE.contains("on_click=move |_| show_subscription_modal.set(true)"));
    }
}
