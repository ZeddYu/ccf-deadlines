use leptos::prelude::*;

#[component]
pub fn ResultsMeta(
    #[prop(optional, into)] class: String,
    timezone_label: Signal<String>,
    result_count: Signal<usize>,
    has_active_filters: Signal<bool>,
    on_clear_filters: Callback<()>,
    use_english: RwSignal<bool>,
) -> impl IntoView {
    let summary_class = move || {
        if class.is_empty() {
            "results-meta-summary".to_string()
        } else {
            format!("{} results-meta-summary", class)
        }
    };

    view! {
        <div class=summary_class>
            <div class="results-meta-timezone">
                {move || format!("Deadlines are shown in {} time.", timezone_label.get())}
            </div>
            <div class="results-meta-actions results-meta-count-group">
                <div class="results-count-message">
                    {move || {
                        if use_english.get() {
                            format!("{} conferences found", result_count.get())
                        } else {
                            format!("共找到 {} 个会议", result_count.get())
                        }
                    }}
                </div>
                <Show when=move || has_active_filters.get()>
                    <button
                        type="button"
                        class="clear-filters-button"
                        on:click=move |_| on_clear_filters.run(())
                    >
                        {move || if use_english.get() { "Clear filters" } else { "清除筛选" }}
                    </button>
                </Show>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn results_meta_has_timezone_and_results_count_rows() {
        const SOURCE: &str = include_str!("results_meta.rs");
        let tests_start = SOURCE.find("#[cfg(test)]").expect("expected test module");
        let production_source = &SOURCE[..tests_start];

        assert!(production_source.contains("results-meta-summary"));
        assert!(production_source.contains("results-meta-timezone"));
        assert!(production_source.contains("results-meta-count-group"));
        assert!(production_source.contains("results-count-message"));
        assert!(production_source.contains("clear-filters-button"));
        assert!(production_source.contains("format!(\"{} conferences found\", result_count.get())"));
        assert!(production_source.contains("format!(\"共找到 {} 个会议\", result_count.get())"));
        assert!(!production_source.contains("<div class=class>"));
        assert!(!production_source.contains("results-meta-left"));
        assert!(!production_source.contains("results-meta-right"));
        assert!(!production_source.contains("timezone-message"));
    }
}
