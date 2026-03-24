use crate::components::conf::Category;
use leptos::prelude::*;

pub fn category_chip_label(category: &Category, is_mobile: bool, use_english: bool) -> String {
    if is_mobile {
        category.sub.clone()
    } else if use_english {
        category.name_en.clone()
    } else {
        category.name.clone()
    }
}

#[component]
pub fn CategoryChip(
    category: Category,
    selected: Signal<bool>,
    use_english: Signal<bool>,
    is_mobile: Signal<bool>,
    on_toggle: Callback<String>,
) -> impl IntoView {
    let sub = category.sub.clone();

    view! {
        <button
            type="button"
            class=move || {
                if selected.get() {
                    "category-chip category-chip-selected"
                } else {
                    "category-chip"
                }
            }
            aria-pressed=move || selected.get().to_string()
            on:click=move |_| on_toggle.run(sub.clone())
        >
            {move || category_chip_label(&category, is_mobile.get(), use_english.get())}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefers_sub_label_on_mobile() {
        let category = Category {
            name: "人工智能".to_string(),
            name_en: "Artificial Intelligence".to_string(),
            sub: "AI".to_string(),
        };

        assert_eq!(category_chip_label(&category, true, false), "AI");
        assert_eq!(category_chip_label(&category, true, true), "AI");
    }

    #[test]
    fn uses_language_specific_label_on_desktop() {
        let category = Category {
            name: "人工智能".to_string(),
            name_en: "Artificial Intelligence".to_string(),
            sub: "AI".to_string(),
        };

        assert_eq!(category_chip_label(&category, false, false), "人工智能");
        assert_eq!(category_chip_label(&category, false, true), "Artificial Intelligence");
    }
}
