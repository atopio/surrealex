use std::time::Duration;

use crate::types::select::SelectField;

pub trait ToSelectField {
    fn to_select_field(self) -> SelectField;
}

impl ToSelectField for &str {
    fn to_select_field(self) -> SelectField {
        SelectField {
            name: self.to_string(),
            alias: None,
        }
    }
}

impl ToSelectField for (&str, &str) {
    fn to_select_field(self) -> SelectField {
        SelectField {
            name: self.0.to_string(),
            alias: Some(self.1.to_string()),
        }
    }
}

/// Trait for values that can be used as a SurrealQL `TIMEOUT` duration.
///
/// Implemented for:
/// - `&str` — passed through as a raw SurrealQL duration string (e.g. `"500ms"`, `"2s"`, `"1m"`).
/// - `String` — same as `&str` but owned.
/// - [`std::time::Duration`] — automatically converted to a compound SurrealQL duration string.
///
/// # SurrealQL duration units
///
/// | Suffix | Unit         |
/// |--------|--------------|
/// | `ns`   | Nanoseconds  |
/// | `us`   | Microseconds |
/// | `ms`   | Milliseconds |
/// | `s`    | Seconds      |
/// | `m`    | Minutes      |
/// | `h`    | Hours        |
/// | `d`    | Days         |
/// | `w`    | Weeks        |
/// | `y`    | Years        |
///
/// # Examples
///
/// ```
/// # use surrealex::QueryBuilder;
/// use std::time::Duration;
///
/// // Using a raw string
/// let sql = QueryBuilder::create("person")
///     .content("{ name: 'Tobie' }")
///     .timeout("2s")
///     .build();
/// assert_eq!(sql, "CREATE person CONTENT { name: 'Tobie' } TIMEOUT 2s");
///
/// // Using std::time::Duration
/// let sql = QueryBuilder::create("person")
///     .content("{ name: 'Tobie' }")
///     .timeout(Duration::from_secs(2))
///     .build();
/// assert_eq!(sql, "CREATE person CONTENT { name: 'Tobie' } TIMEOUT 2s");
/// ```
pub trait IntoTimeout {
    /// Convert this value into a SurrealQL duration string.
    fn into_timeout(self) -> String;
}

impl IntoTimeout for &str {
    fn into_timeout(self) -> String {
        self.to_string()
    }
}

impl IntoTimeout for String {
    fn into_timeout(self) -> String {
        self
    }
}

impl IntoTimeout for Duration {
    fn into_timeout(self) -> String {
        duration_to_string(self)
    }
}

const UNITS: [(u128, &str); 9] = [
    (365 * 86_400 * 1_000_000_000, "y"),
    (7 * 86_400 * 1_000_000_000, "w"),
    (86_400 * 1_000_000_000, "d"),
    (3_600 * 1_000_000_000, "h"),
    (60 * 1_000_000_000, "m"),
    (1_000_000_000, "s"),
    (1_000_000, "ms"),
    (1_000, "us"),
    (1, "ns"),
];

/// Converts a [`std::time::Duration`] into a compound SurrealQL duration string.
///
/// Decomposes from largest to smallest unit, emitting only non-zero components.
/// For example, `Duration::from_secs(90)` becomes `"1m30s"`.
fn duration_to_string(duration: Duration) -> String {
    let mut nanos = duration.as_nanos();

    if nanos == 0 {
        return "0ns".to_string();
    }

    let mut result = String::new();

    for (unit_nanos, suffix) in UNITS {
        if nanos >= unit_nanos {
            result.push_str(&(nanos / unit_nanos).to_string());
            result.push_str(suffix);
            nanos %= unit_nanos;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_duration() {
        assert_eq!(duration_to_string(Duration::ZERO), "0ns");
    }

    #[test]
    fn pure_nanoseconds() {
        assert_eq!(duration_to_string(Duration::from_nanos(42)), "42ns");
    }

    #[test]
    fn pure_microseconds() {
        assert_eq!(duration_to_string(Duration::from_micros(100)), "100us");
    }

    #[test]
    fn pure_milliseconds() {
        assert_eq!(duration_to_string(Duration::from_millis(500)), "500ms");
    }

    #[test]
    fn pure_seconds() {
        assert_eq!(duration_to_string(Duration::from_secs(2)), "2s");
    }

    #[test]
    fn pure_minutes() {
        assert_eq!(duration_to_string(Duration::from_secs(60)), "1m");
    }

    #[test]
    fn pure_hours() {
        assert_eq!(duration_to_string(Duration::from_secs(3600)), "1h");
    }

    #[test]
    fn pure_days() {
        assert_eq!(duration_to_string(Duration::from_secs(86_400)), "1d");
    }

    #[test]
    fn pure_weeks() {
        assert_eq!(duration_to_string(Duration::from_secs(604_800)), "1w");
    }

    #[test]
    fn pure_years() {
        assert_eq!(duration_to_string(Duration::from_secs(365 * 86_400)), "1y");
    }

    #[test]
    fn compound_minutes_and_seconds() {
        // 90 seconds = 1m30s
        assert_eq!(duration_to_string(Duration::from_secs(90)), "1m30s");
    }

    #[test]
    fn compound_hours_minutes_seconds() {
        // 3661 seconds = 1h1m1s
        assert_eq!(duration_to_string(Duration::from_secs(3661)), "1h1m1s");
    }

    #[test]
    fn compound_seconds_and_milliseconds() {
        // 1500ms = 1s500ms
        assert_eq!(duration_to_string(Duration::from_millis(1500)), "1s500ms");
    }

    #[test]
    fn compound_milliseconds_and_microseconds() {
        // 1_001 microseconds = 1ms1us
        assert_eq!(duration_to_string(Duration::from_micros(1_001)), "1ms1us");
    }

    #[test]
    fn compound_microseconds_and_nanoseconds() {
        // 1_001 nanoseconds = 1us1ns
        assert_eq!(duration_to_string(Duration::from_nanos(1_001)), "1us1ns");
    }

    #[test]
    fn compound_days_hours_minutes() {
        // 1 day + 2 hours + 30 minutes
        let secs = 86_400 + 7_200 + 1_800;
        assert_eq!(duration_to_string(Duration::from_secs(secs)), "1d2h30m");
    }

    #[test]
    fn compound_weeks_and_days() {
        // 10 days = 1w3d
        let secs = 10 * 86_400;
        assert_eq!(duration_to_string(Duration::from_secs(secs)), "1w3d");
    }

    #[test]
    fn compound_years_weeks_days() {
        // 1 year + 2 weeks + 3 days
        let secs = 365 * 86_400 + 2 * 604_800 + 3 * 86_400;
        assert_eq!(duration_to_string(Duration::from_secs(secs)), "1y2w3d");
    }

    #[test]
    fn str_into_timeout() {
        assert_eq!("5s".into_timeout(), "5s");
    }

    #[test]
    fn string_into_timeout() {
        assert_eq!(String::from("10m").into_timeout(), "10m");
    }

    #[test]
    fn duration_into_timeout() {
        assert_eq!(Duration::from_secs(120).into_timeout(), "2m");
    }
}
