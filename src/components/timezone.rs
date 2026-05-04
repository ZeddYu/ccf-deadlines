use chrono::{DateTime, FixedOffset};
use chrono_tz::Tz;
use std::str::FromStr;

// Get browser timezone name (e.g., "Asia/Shanghai", "America/New_York")
#[cfg(target_arch = "wasm32")]
pub fn get_timezone_name() -> Option<String> {
    web_sys::js_sys::eval("Intl.DateTimeFormat().resolvedOptions().timeZone")
        .ok()
        .and_then(|v| v.as_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_timezone_name() -> Option<String> {
    std::env::var("TZ").ok()
}

#[cfg(target_arch = "wasm32")]
pub fn get_browser_time_and_timezone() -> (DateTime<FixedOffset>, FixedOffset) {
    let utc_now = chrono::Utc::now();
    let js_date = web_sys::js_sys::Date::new_0();
    let offset_minutes = -(js_date.get_timezone_offset() as i32);

    let timezone = FixedOffset::east_opt(offset_minutes * 60)
        .unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());
    let current_time = utc_now.with_timezone(&timezone);

    (current_time, timezone)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_browser_time_and_timezone() -> (DateTime<FixedOffset>, FixedOffset) {
    let local_time = chrono::Local::now();
    let timezone = *local_time.offset();
    (local_time.with_timezone(&timezone), timezone)
}

// Get timezone name with fallback to "UTC"
#[allow(dead_code)]
pub fn get_timezone_name_or_utc() -> String {
    get_timezone_name().unwrap_or_else(|| "UTC".to_string())
}

// Get timezone name and validate it's supported by chrono-tz
#[allow(dead_code)]
pub fn get_supported_timezone() -> Option<Tz> {
    get_timezone_name().and_then(|tz_name| Tz::from_str(&tz_name).ok())
}

// Get timezone name or return UTC timezone if not supported
#[allow(dead_code)]
pub fn get_timezone_or_utc() -> Tz {
    get_supported_timezone().unwrap_or(chrono_tz::UTC)
}
