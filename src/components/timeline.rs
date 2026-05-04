use crate::components::conf::TimePoint;
use crate::components::timezone::get_browser_time_and_timezone;
use chrono::{Duration, prelude::*};
use leptos::prelude::*;

#[derive(Clone, Copy)]
struct TimelineSignals {
    set_sel_time: WriteSignal<String>,
    set_start_date: WriteSignal<f64>,
    set_end_date: WriteSignal<f64>,
    set_incre_dates: WriteSignal<Vec<TimePoint>>,
    set_date_tips: WriteSignal<Vec<TimePoint>>,
    set_is_single: WriteSignal<bool>,
    set_expire_index: WriteSignal<i32>,
    set_can_line_style: WriteSignal<String>,
    set_sel_dot_style: WriteSignal<String>,
    set_sel_dot_class: WriteSignal<String>,
}

#[derive(Clone, Copy)]
struct SelectedDotSignals {
    set_sel_dot_style: WriteSignal<String>,
    set_sel_dot_class: WriteSignal<String>,
    set_sel_time: WriteSignal<String>,
}

#[derive(Clone, Copy)]
struct TimelineWindow {
    start_date: f64,
    end_date: f64,
    binlog_date: f64,
    full_date: f64,
}

#[component]
pub fn TimeLine(time_points: Vec<TimePoint>) -> impl IntoView {
    let (sel_time, set_sel_time) = signal(String::new());
    let (start_date, set_start_date) = signal(0.0);
    let (end_date, set_end_date) = signal(0.0);
    let (incre_dates, set_incre_dates) = signal(Vec::<TimePoint>::new());
    let (date_tips, set_date_tips) = signal(Vec::<TimePoint>::new());
    let (is_single, set_is_single) = signal(false);
    let (expire_index, set_expire_index) = signal(-1);
    let (sel_dot_style, set_sel_dot_style) = signal(String::new());
    let (sel_dot_class, set_sel_dot_class) = signal(String::new());
    let (can_line_style, set_can_line_style) = signal(String::new());

    let timeline_signals = TimelineSignals {
        set_sel_time,
        set_start_date,
        set_end_date,
        set_incre_dates,
        set_date_tips,
        set_is_single,
        set_expire_index,
        set_can_line_style,
        set_sel_dot_style,
        set_sel_dot_class,
    };

    Effect::new(move |_| {
        initialize_timeline(&time_points, timeline_signals);
    });

    let format_time_label =
        move |value: &DateTime<FixedOffset>, is_day: bool, index: usize| -> String {
            if !is_day {
                return format!("{}", value.format("%Y/%m/%d %H:%M:%S"));
            }

            let tips = date_tips.get();
            if tips.len() > 1 && index < tips.len() - 1 {
                let cur_percent = calculate_position_percent(
                    &tips[index].timepoint,
                    start_date.get(),
                    end_date.get(),
                );
                let next_percent = calculate_position_percent(
                    &tips[index + 1].timepoint,
                    start_date.get(),
                    end_date.get(),
                );
                if next_percent - cur_percent < 8.0 {
                    return String::new();
                }
            }
            format!("{}", value.format("%m/%d"))
        };

    let format_backup_type = |backup_type: i32| -> &'static str {
        match backup_type {
            0 => "Registration:",
            1 => "Submission:",
            _ => "",
        }
    };

    let format_backup_class = |backup_type: i32| -> &'static str {
        match backup_type {
            0 => "square square_all",
            1 => "dot dot_all",
            _ => "",
        }
    };

    let calculate_backup_position = move |time: &DateTime<FixedOffset>, index: usize| -> String {
        let left_percent = calculate_position_percent(time, start_date.get(), end_date.get());

        let clamped_percent = left_percent.clamp(0.5, 99.5);
        let base_style = format!("left:{}%;", clamped_percent);

        if index as i32 <= expire_index.get() {
            format!(
                "{}border: 2px solid var(--color-border-subtle);",
                base_style
            )
        } else {
            base_style
        }
    };

    let calculate_reference_position = move |time: &DateTime<FixedOffset>| -> String {
        let left_percent = calculate_position_percent(time, start_date.get(), end_date.get());

        let clamped_percent = left_percent.clamp(0.0, 100.0);
        format!("left:{}%;", clamped_percent)
    };

    let get_backup_text_style = move |index: usize| -> &'static str {
        if index as i32 <= expire_index.get() {
            "color: var(--color-border-subtle);"
        } else {
            ""
        }
    };

    view! {
        <div class="time_con">
            <style>
                r#"
                /* 时间轴容器 */
                .time_con {

                }

                /* 时间轴主容器 */
                .line_time {
                    position: relative;
                    -webkit-user-select: none;
                    -moz-user-select: none;
                    -ms-user-select: none;
                    user-select: none;
                }

                /* 时间轴外层容器 */
                .line_time .all_line {
                    width: 100%;
                    margin: 0;
                    padding: 16px 0 22px;
                    border: 0;
                    border-radius: 0;
                    background: transparent;
                    box-shadow: none;
                }

                /* 时间轴主线 */
                .line_time .line {
                    width: 100%;
                    height: 8px;
                    background: color-mix(in srgb, var(--color-border-light) 70%, transparent);
                    position: relative;
                    border-radius: 999px;
                }

                /* 可进度线 */
                .line_time .can_line {
                    background: color-mix(in srgb, var(--color-primary) 42%, transparent);
                    height: 8px;
                    width: 0%;
                    position: absolute;
                    left: 0;
                    border-radius: 999px;
                }

                .line_time .can_line span {
                    position: absolute;
                    right: 0;
                    margin-top: 20px;
                }

                /* 参考时间点 */
                .line_time .reference {
                    width: 1px;
                    height: 8px;
                    border: 0;
                    background: color-mix(in srgb, var(--color-border-strong) 75%, transparent);
                    position: absolute;
                    top: -1px;
                    white-space: nowrap;
                }

                .line_time .reference em {
                    color: var(--color-text-muted);
                    position: absolute;
                    transform: translateX(-50%);
                    margin-top: 15px;
                    font-size: 11px;
                }

                /* 圆形备份点 */
                .line_time .dot {
                    width: 10px;
                    height: 10px;
                    border-radius: 50%;
                    border: 2px solid var(--color-border-strong);
                    background: var(--surface-card, var(--color-bg-elevated));
                    position: absolute;
                    top: -1px;
                    white-space: nowrap;
                    margin-left: -5px;
                    box-sizing: border-box;
                }

                .line_time .dot_all em {
                    display: none;
                    color: var(--color-primary);
                    transform: translateX(-50%);
                    position: absolute;
                    top: -30px;
                    font-weight: 600;
                    padding: 4px 8px;
                    border-radius: 999px;
                    background: var(--color-bg-overlay);
                    box-shadow: var(--shadow-sm);
                    font-size: 11px;
                }

                .line_time .dot_all:hover,
                .line_time .dot_all:focus-visible {
                    width: 12px;
                    height: 12px;
                    border: 2px solid var(--color-primary);
                    top: -2px;
                    outline: none;
                }

                .line_time .dot_all:hover em,
                .line_time .dot_all:focus-visible em {
                    display: inline-block;
                }

                /* 方形备份点 */
                .line_time .square {
                    width: 10px;
                    height: 10px;
                    border-radius: 0;
                    border: 2px solid var(--color-border-strong);
                    background: var(--surface-card, var(--color-bg-elevated));
                    position: absolute;
                    top: -1px;
                    white-space: nowrap;
                    margin-left: -5px;
                    box-sizing: border-box;
                }

                .line_time .square_all em {
                    display: none;
                    color: var(--color-primary);
                    transform: translateX(-50%);
                    position: absolute;
                    top: -30px;
                    font-weight: 600;
                    padding: 4px 8px;
                    border-radius: 999px;
                    background: var(--color-bg-overlay);
                    box-shadow: var(--shadow-sm);
                    font-size: 11px;
                }

                .line_time .square_all:hover,
                .line_time .square_all:focus-visible {
                    width: 12px;
                    height: 12px;
                    border: 2px solid var(--color-primary);
                    top: -2px;
                    outline: none;
                }

                .line_time .square_all:hover em,
                .line_time .square_all:focus-visible em {
                    display: inline-block;
                }

                /* 当前选中点 */
                .line_time .sel_dot {
                    width: 12px;
                    height: 12px;
                    top: -2px;
                    border: 2px solid var(--color-warning);
                    background: var(--color-warning);
                    box-shadow: 0 0 0 4px color-mix(in srgb, var(--color-warning) 16%, transparent);
                    z-index: 5;
                    position: absolute;
                }

                .line_time .sel_dot em {
                    display: none;
                    color: var(--color-warning);
                    transform: translateX(-50%);
                    position: absolute;
                    top: -30px;
                    font-weight: 600;
                    padding: 4px 8px;
                    border-radius: 999px;
                    background: var(--color-bg-overlay);
                    box-shadow: var(--shadow-sm);
                    font-size: 11px;
                }

                .line_time .sel_dot:hover em {
                    display: inline-block;
                }

                /* 边界情况的特殊定位 */
                .line_time .sel_dot_left em {
                    transform: translateX(-20%);
                }

                .line_time .sel_dot_left i {
                    left: 20%;
                }

                .line_time .sel_dot_right em {
                    transform: translateX(-90%);
                }

                .line_time .sel_dot_left i {
                    left: 20%;
                }
                .line_time .sel_dot_right em {
                    transform: translateX(-80%);
                }
                .line_time .sel_dot_right i {
                    left: 80%;
                }

                "#
            </style>

            <div class="line_time">
                <div class="all_line">
                    <div class="line">
                        <div class="can_line" style=move || can_line_style.get()></div>

                        {move || {
                            let is_single_val = is_single.get();
                            let date_tips_val = date_tips.get();
                            date_tips_val
                                .into_iter()
                                .enumerate()
                                .map(|(index, date_tip)| {
                                    let style = calculate_reference_position(&date_tip.timepoint);
                                    let should_show = !(index == 0 && is_single_val);

                                    view! {
                                        <div class="reference" style=style>
                                            {move || {
                                                if !should_show {
                                                    return None;
                                                }

                                                let label = format_time_label(
                                                    &date_tip.timepoint,
                                                    true,
                                                    index,
                                                );
                                                if label.is_empty() {
                                                    None
                                                } else {
                                                    Some(view! { <em>{label}</em> })
                                                }
                                            }}
                                        </div>
                                    }
                                })
                                .collect_view()
                        }}

                        {move || {
                            incre_dates
                                .get()
                                .into_iter()
                                .enumerate()
                                .map(|(index, backup_point)| {
                                    let class_name = format_backup_class(backup_point.r#type);
                                    let style = calculate_backup_position(
                                        &backup_point.timepoint,
                                        index,
                                    );
                                    let text_style = get_backup_text_style(index);
                                    let type_label = format_backup_type(backup_point.r#type);
                                    let time_label = format_time_label(
                                        &backup_point.timepoint,
                                        false,
                                        0,
                                    );

                                    let milestone_label = format!("{type_label} {time_label}");
                                    let milestone_text = milestone_label.clone();

                                    view! {
                                        <div
                                            class=class_name
                                            style=style
                                            tabindex="0"
                                            role="note"
                                            aria-label=milestone_label
                                        >
                                            <em style=text_style>{milestone_text}</em>
                                        </div>
                                    }
                                })
                                .collect_view()
                        }}

                        <div
                            class=move || format!("dot sel_dot {}", sel_dot_class.get())
                            style=move || sel_dot_style.get()
                        >
                            <em>"Now: "{move || sel_time.get()}</em>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn calculate_position_percent(time: &DateTime<FixedOffset>, start: f64, end: f64) -> f64 {
    let timestamp = time.timestamp() as f64;
    (timestamp - start) / (end - start) * 100.0
}

fn timestamp_to_datetime(timestamp: f64, timezone: FixedOffset) -> DateTime<FixedOffset> {
    DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap_or_else(chrono::Utc::now)
        .with_timezone(&timezone)
}

fn initialize_timeline(time_points: &[TimePoint], signals: TimelineSignals) {
    let (now, now_timezone) = get_browser_time_and_timezone();
    let now_timestamp = now.timestamp() as f64;

    if time_points.is_empty() {
        signals
            .set_sel_time
            .set(format!("{}", now.format("%Y-%m-%d %H:%M:%S")));
        signals
            .set_start_date
            .set((now - Duration::days(7)).timestamp() as f64);
        signals
            .set_end_date
            .set((now + Duration::days(7)).timestamp() as f64);
        signals.set_incre_dates.set(Vec::new());
        signals.set_date_tips.set(Vec::new());
        signals.set_is_single.set(false);
        signals.set_expire_index.set(-1);
        signals.set_can_line_style.set("width:0%;".to_string());
        signals.set_sel_dot_style.set("left:50%;".to_string());
        signals.set_sel_dot_class.set(String::new());
        return;
    }

    let mut deadlines = Vec::new();
    let is_single_point = time_points.len() == 1;

    if is_single_point {
        deadlines.push(TimePoint {
            timepoint: now,
            r#type: 1,
        });
        signals.set_is_single.set(true);
    } else {
        signals.set_is_single.set(false);
    }

    deadlines.extend(time_points.iter().cloned());

    let expire_idx = deadlines
        .iter()
        .enumerate()
        .rev()
        .find(|(_, point)| now_timestamp >= point.timepoint.timestamp() as f64)
        .map(|(i, _)| i as i32)
        .unwrap_or(-1);
    signals.set_expire_index.set(expire_idx);

    let first_time = deadlines[0].timepoint.timestamp() as f64;
    let start_time = if now_timestamp < first_time {
        (now - Duration::days(7)).timestamp() as f64
    } else {
        (deadlines[0].timepoint - Duration::days(7)).timestamp() as f64
    };
    let end_time = (deadlines.last().unwrap().timepoint + Duration::days(7)).timestamp() as f64;

    signals.set_start_date.set(start_time);
    signals.set_end_date.set(end_time);

    let last_deadline_time = deadlines.last().unwrap().timepoint.timestamp() as f64;
    let progress_ratio = (last_deadline_time - now_timestamp) / (end_time - start_time);

    if progress_ratio > 0.0 {
        let width = progress_ratio * 100.0;
        let left = (now_timestamp - start_time) / (end_time - start_time) * 100.0;
        let max_width = 100.0 - left;
        signals.set_can_line_style.set(format!(
            "width:{}%;left:{}%;max-width:{}%;",
            width, left, max_width
        ));
    } else {
        signals.set_can_line_style.set("width:0%;".to_string());
    }

    signals.set_incre_dates.set(deadlines.clone());
    signals.set_date_tips.set(deadlines.clone());

    let mut all_incremental = deadlines.clone();
    all_incremental.push(TimePoint {
        timepoint: deadlines.last().unwrap().timepoint,
        r#type: 1,
    });
    all_incremental.push(TimePoint {
        timepoint: now,
        r#type: 1,
    });
    update_selected_dot(
        now_timestamp,
        now_timezone,
        SelectedDotSignals {
            set_sel_dot_style: signals.set_sel_dot_style,
            set_sel_dot_class: signals.set_sel_dot_class,
            set_sel_time: signals.set_sel_time,
        },
        TimelineWindow {
            start_date: start_time,
            end_date: end_time,
            binlog_date: last_deadline_time,
            full_date: now_timestamp,
        },
        all_incremental,
    );
}

fn update_selected_dot(
    target_time: f64,
    time_zone: FixedOffset,
    selected_dot_signals: SelectedDotSignals,
    window: TimelineWindow,
    all_incre: Vec<TimePoint>,
) {
    let mut actual_time = target_time;
    let mut position_ratio =
        (target_time - window.start_date) / (window.end_date - window.start_date);

    if (actual_time > window.binlog_date || actual_time < window.full_date)
        && let Some(nearest_point) = all_incre.iter().min_by(|a, b| {
            let a_diff = (a.timepoint.timestamp() as f64 - actual_time).abs();
            let b_diff = (b.timepoint.timestamp() as f64 - actual_time).abs();
            a_diff.total_cmp(&b_diff)
        })
    {
        actual_time = nearest_point.timepoint.timestamp() as f64;
        position_ratio = (actual_time - window.start_date) / (window.end_date - window.start_date);
    }

    let datetime = timestamp_to_datetime(actual_time, time_zone);
    selected_dot_signals
        .set_sel_time
        .set(format!("{}", datetime.format("%Y-%m-%d %H:%M:%S")));

    let left_percent = position_ratio * 100.0;
    let clamped_percent = left_percent.clamp(0.5, 99.5);
    selected_dot_signals
        .set_sel_dot_style
        .set(format!("left:{}%;", clamped_percent));

    let class_suffix = if clamped_percent < 10.0 {
        "sel_dot_left"
    } else if clamped_percent > 90.0 {
        "sel_dot_right"
    } else {
        ""
    };
    selected_dot_signals
        .set_sel_dot_class
        .set(class_suffix.to_string());
}
