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
    view! {
        <div class=class>
            <div class="results-meta-summary">
                <div class="results-meta-left">
                    <div class="timezone-message results-meta-timezone">
                        {move || format!("Deadlines are shown in {} time.", timezone_label.get())}
                    </div>
                </div>
                <div class="results-meta-right">
                    <div class="results-meta-actions results-meta-count-group">
                        <div class="results-count-message">
                            {move || {
                                if use_english.get() {
                                    format!("{} results", result_count.get())
                                } else {
                                    format!("共 {} 条结果", result_count.get())
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
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn results_meta_has_timezone_and_results_count_rows() {
        const SOURCE: &str = include_str!("results_meta.rs");

        assert!(SOURCE.contains("results-meta-summary"));
        assert!(SOURCE.contains("results-meta-left"));
        assert!(SOURCE.contains("results-meta-right"));
        assert!(SOURCE.contains("results-meta-count-group"));
        assert!(SOURCE.contains("clear-filters-button"));
    }
}
