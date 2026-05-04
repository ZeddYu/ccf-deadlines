use std::collections::HashMap;
use std::sync::OnceLock;

use urlencoding::encode;
use web_sys::window;

static UTC_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();

const MOBILE_KEYWORDS: &[&str] = &[
    "phone",
    "pad",
    "pod",
    "iphone",
    "ipod",
    "ios",
    "ipad",
    "android",
    "mobile",
    "blackberry",
    "iemobile",
    "mqqbrowser",
    "juc",
    "fennec",
    "wosbrowser",
    "browserng",
    "webos",
    "symbian",
    "windows phone",
];

pub(crate) fn normalize_timezone(tz: &str) -> String {
    match tz {
        "AoE" => "UTC-12".to_string(),
        "UTC" => "UTC+0".to_string(),
        _ => tz.to_string(),
    }
}

pub(crate) fn parse_deadline_to_rfc3339(deadline: &str, tz_offset: &str) -> String {
    if deadline.contains(' ') {
        format!(
            "{}T{}{}",
            deadline.split(' ').next().unwrap_or(""),
            deadline.split(' ').nth(1).unwrap_or("00:00:00"),
            tz_offset
        )
    } else {
        format!("{}T23:59:59{}", deadline, tz_offset)
    }
}

pub(crate) fn build_google_calendar_url(
    title: &str,
    year: i32,
    iso_string: &str,
    details: &str,
    time_zone: &str,
) -> String {
    format!(
        "https://www.google.com/calendar/render?action=TEMPLATE&text={}&dates={}/{}&details={}&location=Online&ctz={}&sf=true&output=xml",
        encode(&format!("{} {}", title, year)),
        iso_string,
        iso_string,
        encode(details),
        encode(time_zone),
    )
}

pub(crate) fn encode_ical_text(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            ';' => escaped.push_str("\\;"),
            ',' => escaped.push_str("\\,"),
            '\r' => {
                if matches!(chars.peek(), Some('\n')) {
                    chars.next();
                }
                escaped.push_str("\\n");
            }
            '\n' => escaped.push_str("\\n"),
            _ => escaped.push(ch),
        }
    }

    encode(&escaped).into_owned()
}

fn get_utc_map() -> &'static HashMap<String, String> {
    UTC_MAP.get_or_init(|| {
        let mut utc_map = HashMap::new();
        for i in -12..=12 {
            let offset_str = if i >= 0 {
                format!("+{:02}:00", i)
            } else {
                format!("-{:02}:00", -i)
            };
            let key = if i >= 0 {
                format!("UTC+{}", i)
            } else {
                format!("UTC{}", i)
            };
            utc_map.insert(key, offset_str);
        }
        utc_map.insert("AoE".to_string(), "-12:00".to_string());
        utc_map.insert("UTC".to_string(), "+00:00".to_string());
        utc_map
    })
}

#[allow(dead_code)]
pub(crate) fn load_utc_map() -> HashMap<String, String> {
    get_utc_map().clone()
}

pub(crate) fn is_mobile_device() -> bool {
    let Some(window) = window() else {
        return false;
    };

    let has_mobile_viewport = window
        .match_media("(max-width: 768px)")
        .ok()
        .flatten()
        .map(|query| query.matches())
        .unwrap_or(false);
    let has_mobile_user_agent = window
        .navigator()
        .user_agent()
        .ok()
        .map(|user_agent| {
            let user_agent = user_agent.to_lowercase();
            MOBILE_KEYWORDS
                .iter()
                .any(|&keyword| user_agent.contains(keyword))
        })
        .unwrap_or(false);

    has_mobile_viewport || has_mobile_user_agent
}

pub(crate) fn current_origin() -> Option<String> {
    window().and_then(|window| window.location().origin().ok())
}

#[cfg(test)]
mod tests {
    use super::{build_google_calendar_url, encode_ical_text, parse_deadline_to_rfc3339};

    #[test]
    fn parses_date_only_deadlines_as_end_of_day() {
        assert_eq!(
            parse_deadline_to_rfc3339("2026-05-04", "+00:00"),
            "2026-05-04T23:59:59+00:00"
        );
    }

    #[test]
    fn builds_google_calendar_details_without_debug_quotes() {
        let url = build_google_calendar_url(
            "TestConf",
            2026,
            "20260504T235959",
            "notes provided by @ccfddl",
            "UTC",
        );

        assert!(url.contains("details=notes%20provided%20by%20%40ccfddl"));
        assert!(!url.contains("details=%22notes%20provided%20by%20%40ccfddl%22"));
    }

    #[test]
    fn encodes_ical_text_without_literal_line_breaks() {
        assert_eq!(
            encode_ical_text("first\r\nsecond\rthird\nfourth"),
            "first%5Cnsecond%5Cnthird%5Cnfourth"
        );
    }

    #[test]
    fn encodes_ical_reserved_text_characters() {
        assert_eq!(
            encode_ical_text(r"path\to,topic;track"),
            "path%5C%5Cto%5C%2Ctopic%5C%3Btrack"
        );
    }
}
