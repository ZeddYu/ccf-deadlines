use leptos::prelude::*;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UrgencyLevel {
    Normal,
    Attention,
    Warning,
    Urgent,
}

fn get_urgency(remaining_secs: u64) -> UrgencyLevel {
    if remaining_secs < 3 * 86400 {
        UrgencyLevel::Urgent
    } else if remaining_secs < 7 * 86400 {
        UrgencyLevel::Warning
    } else if remaining_secs < 30 * 86400 {
        UrgencyLevel::Attention
    } else {
        UrgencyLevel::Normal
    }
}

fn format_countdown(remaining_secs: u64, compact: bool) -> String {
    let mut secs = remaining_secs;
    let days = secs / (24 * 3600);
    secs %= 24 * 3600;
    let hours = secs / 3600;
    secs %= 3600;
    let minutes = secs / 60;
    let seconds = secs % 60;

    if compact {
        if days > 0 {
            format!("{:02}d {:02}h", days, hours)
        } else if hours > 0 {
            format!("{:02}h {:02}m", hours, minutes)
        } else {
            format!("{:02}m {:02}s", minutes, seconds)
        }
    } else if days > 0 {
        format!("{:02}d {:02}h {:02}m {:02}s", days, hours, minutes, seconds)
    } else if hours > 0 {
        format!("{:02}h {:02}m {:02}s", hours, minutes, seconds)
    } else {
        format!("{:02}m {:02}s", minutes, seconds)
    }
}

pub fn use_interval<T, F>(interval_millis: T, f: F)
where
    F: Fn() + Clone + 'static,
    T: Into<Signal<u64>> + 'static,
{
    let interval_millis = interval_millis.into();
    Effect::new(move |prev_handle: Option<IntervalHandle>| {
        if let Some(prev_handle) = prev_handle {
            prev_handle.clear();
        }
        set_interval_with_handle(f.clone(), Duration::from_millis(interval_millis.get()))
            .expect("could not create interval")
    });
}

#[component]
pub fn CountDown(
    remain: u64,
    #[prop(optional, default = false)] compact: bool,
) -> impl IntoView {
    let remaining_time = RwSignal::new(remain / 1000);
    let urgency = Memo::new(move |_| get_urgency(remaining_time.get()));

    use_interval(1000, move || {
        remaining_time.update(|r| {
            if *r > 0 {
                *r -= 1;
            }
        });
    });

    let urgency_class = move || {
        match urgency.get() {
            UrgencyLevel::Normal => "countdown-normal",
            UrgencyLevel::Attention => "countdown-attention",
            UrgencyLevel::Warning => "countdown-warning",
            UrgencyLevel::Urgent => "countdown-urgent",
        }
    };

    view! {
        <span class=urgency_class>
            <span class="countdown-value" class:countdown-compact=compact>
                {move || format_countdown(remaining_time.get(), compact)}
            </span>
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_homepage_format_prioritizes_days_and_hours() {
        assert_eq!(format_countdown(2 * 86400 + 5 * 3600 + 18 * 60 + 42, true), "02d 05h");
    }

    #[test]
    fn compact_homepage_format_prioritizes_hours_and_minutes() {
        assert_eq!(format_countdown(7 * 3600 + 9 * 60 + 33, true), "07h 09m");
    }

    #[test]
    fn compact_homepage_format_falls_back_to_minutes_and_seconds_under_one_hour() {
        assert_eq!(format_countdown(14 * 60 + 8, true), "14m 08s");
    }

    #[test]
    fn default_format_keeps_existing_detailed_output() {
        assert_eq!(format_countdown(2 * 86400 + 5 * 3600 + 18 * 60 + 42, false), "02d 05h 18m 42s");
        assert_eq!(format_countdown(7 * 3600 + 9 * 60 + 33, false), "07h 09m 33s");
        assert_eq!(format_countdown(14 * 60 + 8, false), "14m 08s");
    }

    #[test]
    fn compact_homepage_mode_keeps_existing_urgency_thresholds() {
        assert_eq!(get_urgency(31 * 86400), UrgencyLevel::Normal);
        assert_eq!(get_urgency(29 * 86400), UrgencyLevel::Attention);
        assert_eq!(get_urgency(6 * 86400), UrgencyLevel::Warning);
        assert_eq!(get_urgency(2 * 86400), UrgencyLevel::Urgent);
    }
}
