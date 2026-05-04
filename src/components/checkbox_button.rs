use crate::components::dom::focus_element_by_id_after_render;
use leptos::prelude::*;
use std::collections::HashSet;
use web_sys::KeyboardEvent;

const NON_RANK_VALUE: &str = "N";

pub fn format_rank_label(system: &str, rank: &str) -> String {
    match (system, rank) {
        ("CCF", "N") => "Non-CCF".to_string(),
        ("CORE", "N") => "Non-CORE".to_string(),
        ("THCPL", "N") => "Non-THCPL".to_string(),
        _ => format!("{} {}", system, rank),
    }
}

pub fn format_rank_summary_value(system: &str, rank: &str) -> String {
    match (system, rank) {
        ("CCF", "N") => "Non-CCF".to_string(),
        ("CORE", "N") => "Non-CORE".to_string(),
        ("THCPL", "N") => "Non-THCPL".to_string(),
        _ => rank.to_string(),
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct FilterDropdownOption {
    pub value: &'static str,
    pub label: &'static str,
    pub summary_label: &'static str,
}

pub fn normalize_rank_filter_selection(selected_values: &mut HashSet<String>) {
    if selected_values.contains(NON_RANK_VALUE) && selected_values.len() > 1 {
        selected_values.remove(NON_RANK_VALUE);
    }
}

pub fn ccf_filter_options() -> Vec<FilterDropdownOption> {
    vec![
        FilterDropdownOption {
            value: "A",
            label: "CCF A",
            summary_label: "A",
        },
        FilterDropdownOption {
            value: "B",
            label: "CCF B",
            summary_label: "B",
        },
        FilterDropdownOption {
            value: "C",
            label: "CCF C",
            summary_label: "C",
        },
        FilterDropdownOption {
            value: "N",
            label: "Non-CCF",
            summary_label: "Non",
        },
    ]
}

pub fn core_filter_options() -> Vec<FilterDropdownOption> {
    vec![
        FilterDropdownOption {
            value: "A*",
            label: "CORE A*",
            summary_label: "A*",
        },
        FilterDropdownOption {
            value: "A",
            label: "CORE A",
            summary_label: "A",
        },
        FilterDropdownOption {
            value: "B",
            label: "CORE B",
            summary_label: "B",
        },
        FilterDropdownOption {
            value: "C",
            label: "CORE C",
            summary_label: "C",
        },
        FilterDropdownOption {
            value: "N",
            label: "Non-CORE",
            summary_label: "Non",
        },
    ]
}

pub fn thcpl_filter_options() -> Vec<FilterDropdownOption> {
    vec![
        FilterDropdownOption {
            value: "A",
            label: "THCPL A",
            summary_label: "A",
        },
        FilterDropdownOption {
            value: "B",
            label: "THCPL B",
            summary_label: "B",
        },
        FilterDropdownOption {
            value: "N",
            label: "Non-THCPL",
            summary_label: "Non",
        },
    ]
}

#[component]
pub fn MultiSelectDropdown(
    dropdown_id: String,
    title: String,
    options: Vec<FilterDropdownOption>,
    selected_values: RwSignal<HashSet<String>>,
    use_english: RwSignal<bool>,
    panel_width: String,
    open_dropdown: RwSignal<Option<String>>,
) -> impl IntoView {
    let dropdown_id_for_open = dropdown_id.clone();
    let is_open =
        Memo::new(move |_| open_dropdown.get().as_deref() == Some(dropdown_id_for_open.as_str()));
    let options_for_summary = options.clone();
    let title_for_summary = title.clone();
    let title_for_panel = title.clone();
    let options_for_render = StoredValue::new(options.clone());
    let summary = Memo::new(move |_| {
        let selected_labels: Vec<&str> = selected_values.with(|selected| {
            options_for_summary
                .iter()
                .filter(|option| selected.contains(option.value))
                .map(|option| option.summary_label)
                .collect()
        });

        match selected_labels.len() {
            0 => title_for_summary.clone(),
            1 => format!("{title_for_summary} {}", selected_labels[0]),
            2 => format!(
                "{title_for_summary} {},{}",
                selected_labels[0], selected_labels[1]
            ),
            _ => format!(
                "{title_for_summary} {},{}+{}",
                selected_labels[0],
                selected_labels[1],
                selected_labels.len() - 2
            ),
        }
    });

    let has_selection = Memo::new(move |_| selected_values.with(|selected| !selected.is_empty()));
    let clear_label = Memo::new(move |_| {
        if use_english.get() {
            "Clear".to_string()
        } else {
            "清空".to_string()
        }
    });
    let dropdown_id_for_toggle = dropdown_id.clone();
    let trigger_id = format!("{dropdown_id}-filter-trigger");
    let trigger_id_for_escape = StoredValue::new(trigger_id.clone());
    let panel_id = format!("{dropdown_id}-filter-panel");
    let panel_id_for_trigger = panel_id.clone();
    let panel_heading_id = format!("{dropdown_id}-filter-panel-heading");
    let panel_heading_id_for_label = StoredValue::new(panel_heading_id.clone());

    view! {
        <div class="filter-dropdown" style=format!("--filter-panel-width: {panel_width};")>
            <button
                id=trigger_id
                type="button"
                class=move || {
                if has_selection.get() {
                        if is_open.get() {
                            "filter-dropdown-trigger active open"
                        } else {
                            "filter-dropdown-trigger active"
                        }
                } else {
                        if is_open.get() {
                            "filter-dropdown-trigger open"
                        } else {
                            "filter-dropdown-trigger"
                        }
                    }
                }
                aria-haspopup="true"
                aria-expanded=move || is_open.get().to_string()
                aria-controls=panel_id_for_trigger
                on:click=move |_| {
                    if is_open.get_untracked() {
                        open_dropdown.set(None);
                    } else {
                        open_dropdown.set(Some(dropdown_id_for_toggle.clone()));
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    if event.key() == "Escape" && is_open.get_untracked() {
                        event.stop_propagation();
                        open_dropdown.set(None);
                    }
                }
            >
                <span class="filter-dropdown-trigger-text">{move || summary.get()}</span>
                <span class="filter-dropdown-trigger-icon" aria-hidden="true">"▾"</span>
            </button>

            <Show when=move || is_open.get()>
                <div
                    id=panel_id.clone()
                    class="filter-dropdown-panel"
                    role="group"
                    aria-labelledby=move || panel_heading_id_for_label.get_value()
                    on:keydown=move |event: KeyboardEvent| {
                        if event.key() == "Escape" {
                            event.stop_propagation();
                            open_dropdown.set(None);
                            focus_element_by_id_after_render(trigger_id_for_escape.get_value());
                        }
                    }
                >
                    <div class="filter-dropdown-panel-header">
                        <span id=panel_heading_id.clone()>{title_for_panel.clone()}</span>
                        <button
                            type="button"
                            class="filter-dropdown-clear"
                            on:click=move |_| {
                                selected_values.set(HashSet::new());
                            }
                            disabled=move || !has_selection.get()
                        >
                            {move || clear_label.get()}
                        </button>
                    </div>

                    <div class="filter-dropdown-options">
                        {move || {
                            options_for_render
                                .get_value()
                                .into_iter()
                                .map(|option| {
                                    let value = option.value.to_string();
                                    let label = option.label.to_string();
                                    let value_for_checked = value.clone();
                                    let value_for_update = value.clone();

                                    view! {
                                        <label class="filter-dropdown-option">
                                            <input
                                                type="checkbox"
                                                prop:checked=move || {
                                                    selected_values.with(|selected| {
                                                        selected.contains(&value_for_checked)
                                                    })
                                                }
                                                on:change=move |_| {
                                                    selected_values.update(|set| {
                                                        if set.contains(&value_for_update) {
                                                            set.remove(&value_for_update);
                                                        } else {
                                                            if value_for_update == NON_RANK_VALUE {
                                                                set.clear();
                                                            } else {
                                                                set.remove(NON_RANK_VALUE);
                                                            }
                                                            set.insert(value_for_update.clone());
                                                        }
                                                    });
                                                }
                                            />
                                            <span>{label}</span>
                                        </label>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                </div>
            </Show>
        </div>

        <style>
            {r#"
            .filter-dropdown {
              position: relative;
            }

            .filter-dropdown-trigger {
              display: inline-flex;
              align-items: center;
              justify-content: space-between;
              gap: 8px;
              min-width: 112px;
              max-width: 156px;
              min-height: 44px;
              padding: 0 14px;
              border: 1px solid var(--color-border);
              border-radius: 14px;
              background: color-mix(in srgb, var(--color-bg-elevated) 92%, transparent);
              color: var(--color-text-secondary);
              box-shadow: var(--shadow-sm);
              cursor: pointer;
              user-select: none;
              text-align: left;
              font-size: 13px;
              font-weight: 600;
              transition: border-color 0.2s, color 0.2s, box-shadow 0.2s, background-color 0.2s, transform 0.2s;
            }

            .filter-dropdown-trigger:hover,
            .filter-dropdown-trigger.open {
              border-color: var(--color-border-strong);
              color: var(--color-text-primary);
              background: var(--state-hover);
            }

            .filter-dropdown-trigger:focus-visible {
              outline: none;
              border-color: var(--color-primary);
              box-shadow: 0 0 0 3px var(--state-focus-ring);
            }

            .filter-dropdown-trigger.active {
              border-color: var(--color-accent);
              color: var(--color-text-accent);
              background: var(--state-selected);
            }

            .filter-dropdown-trigger-text {
              flex: 1;
              overflow: hidden;
              text-overflow: ellipsis;
              white-space: nowrap;
            }

            .filter-dropdown-trigger-icon {
              color: inherit;
              font-size: 11px;
              transition: transform 0.2s;
            }

            .filter-dropdown-trigger.open .filter-dropdown-trigger-icon {
              transform: rotate(180deg);
            }

            .filter-dropdown-panel {
              position: absolute;
              right: 0;
              top: calc(100% + 6px);
              box-sizing: border-box;
              width: var(--filter-panel-width, 220px);
              padding: 8px;
              border: 1px solid var(--color-border-light);
              border-radius: 12px;
              background: var(--color-bg-overlay);
              box-shadow: var(--shadow-overlay);
              backdrop-filter: blur(12px);
              z-index: 60;
            }

            .filter-dropdown-panel-header {
              display: flex;
              align-items: center;
              justify-content: space-between;
              gap: 6px;
              margin-bottom: 4px;
              padding: 0 2px;
              font-size: 11px;
              font-weight: 600;
              color: var(--color-text-tertiary);
            }

            .filter-dropdown-clear {
              border: none;
              border-radius: 999px;
              background: transparent;
              color: var(--color-primary);
              cursor: pointer;
              font-size: 11px;
              font-weight: 600;
              padding: 2px 6px;
              transition: background-color 0.2s, color 0.2s;
            }

            .filter-dropdown-clear:hover:not(:disabled),
            .filter-dropdown-clear:focus-visible:not(:disabled) {
              outline: none;
              background: var(--state-button-hover);
            }

            .filter-dropdown-clear:disabled {
              color: var(--color-text-muted);
              cursor: default;
            }

            .filter-dropdown-options {
              display: flex;
              flex-direction: column;
              gap: 2px;
            }

            .filter-dropdown-option {
              display: flex;
              align-items: center;
              box-sizing: border-box;
              min-height: 30px;
              gap: 7px;
              padding: 5px 8px;
              border-radius: 8px;
              color: var(--color-text-primary);
              font-size: 12px;
              cursor: pointer;
              transition: background-color 0.2s, color 0.2s;
            }

            .filter-dropdown-option:hover {
              background: var(--state-hover);
            }

            .filter-dropdown-option:focus-within {
              background: var(--state-selected);
            }

            .filter-dropdown-option input {
              margin: 0;
            }

            @media (max-width: 768px) {
              .filter-dropdown-trigger {
                width: 100%;
                max-width: none;
              }

              .filter-dropdown-panel {
                left: 0;
                right: auto;
                width: 100%;
                padding: 10px;
              }

              .filter-dropdown-clear {
                min-height: 44px;
                padding: 0 12px;
              }

              .filter-dropdown-option {
                min-height: 44px;
                padding: 7px 10px;
                font-size: 13px;
              }
            }
            "#}
        </style>
    }
}
