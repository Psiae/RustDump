use std::time;
use std::time::Duration;
use time::Duration as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    return start + Duration::from_secs(1_000_000_000)
}
