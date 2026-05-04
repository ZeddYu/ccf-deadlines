use crate::components::conf::{Category, category_order};
use leptos::prelude::*;
use std::collections::HashSet;

#[component]
pub fn CategoryFilterChips(
    sub_list: RwSignal<Vec<Category>>,
    check_list: RwSignal<HashSet<String>>,
    use_english: RwSignal<bool>,
    clear_all_label: Memo<String>,
    input_value: RwSignal<String>,
    rank_list: RwSignal<HashSet<String>>,
    core_rank_list: RwSignal<HashSet<String>>,
    thcpl_rank_list: RwSignal<HashSet<String>>,
    open_dropdown: RwSignal<Option<String>>,
) -> impl IntoView {
    view! {
        <div class="home-chip-list">
            <For
                each=move || {
                    let mut categories = sub_list.get().into_iter().collect::<Vec<Category>>();
                    categories.sort_by_key(|item| category_order(item.sub.as_str()));
                    categories
                        .into_iter()
                        .enumerate()
                        .collect::<Vec<(usize, Category)>>()
                }
                key=|(_, item)| item.sub.clone()
                children=move |(_, item)| {
                    let sub_for_class = item.sub.clone();
                    let sub_for_pressed = item.sub.clone();
                    let sub_for_toggle = item.sub.clone();
                    let item_for_label = item.clone();

                    view! {
                        <button
                            type="button"
                            class=move || {
                                if check_list.with(|selected| selected.contains(&sub_for_class)) {
                                    "home-chip home-chip-active"
                                } else {
                                    "home-chip"
                                }
                            }
                            aria-pressed=move || {
                                check_list
                                    .with(|selected| selected.contains(&sub_for_pressed))
                                    .to_string()
                            }
                            on:click=move |_| {
                                check_list.update(|selected| {
                                    if selected.contains(&sub_for_toggle) {
                                        selected.remove(&sub_for_toggle);
                                    } else {
                                        selected.insert(sub_for_toggle.clone());
                                    }
                                });
                            }
                        >
                            {move || category_chip_label(&item_for_label, use_english.get())}
                        </button>
                    }
                }
            />
        </div>

        <div class="home-chip-actions">
            <button
                type="button"
                class="home-chip-action"
                on:click=move |_| {
                    if input_value.with(|value| !value.is_empty()) {
                        input_value.set(String::new());
                    }
                    if check_list.with(|selected| !selected.is_empty()) {
                        check_list.set(HashSet::new());
                    }
                    if rank_list.with(|selected| !selected.is_empty()) {
                        rank_list.set(HashSet::new());
                    }
                    if core_rank_list.with(|selected| !selected.is_empty()) {
                        core_rank_list.set(HashSet::new());
                    }
                    if thcpl_rank_list.with(|selected| !selected.is_empty()) {
                        thcpl_rank_list.set(HashSet::new());
                    }
                    if open_dropdown.with(Option::is_some) {
                        open_dropdown.set(None);
                    }
                }
            >
                {move || clear_all_label.get()}
            </button>
        </div>
    }
}

fn category_chip_label(item: &Category, use_english: bool) -> String {
    if use_english {
        item.name_en.clone()
    } else {
        item.name.clone()
    }
}
