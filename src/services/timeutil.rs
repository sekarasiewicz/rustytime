use time::{OffsetDateTime, macros::format_description};
use time_tz::{OffsetDateTimeExt, timezones};

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn local_date_warsaw(at: OffsetDateTime) -> String {
    let tz = timezones::db::europe::WARSAW;
    let local = at.to_timezone(tz);
    let fmt = format_description!("[year]-[month]-[day]");
    local.format(&fmt).unwrap()
}

pub fn to_rfc3339(t: OffsetDateTime) -> String {
    t.format(&time::format_description::well_known::Rfc3339)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{Date, Month, Time, UtcOffset};

    #[test]
    fn test_now_utc_returns_valid_datetime() {
        let now = now_utc();

        // Check that the returned datetime is reasonable (within the last minute)
        let current_utc = OffsetDateTime::now_utc();
        let diff = (current_utc - now).whole_seconds().abs();

        // Should be within 1 second of current time
        assert!(
            diff <= 1,
            "now_utc() should return current time, got difference of {} seconds",
            diff
        );

        // Check that it's actually UTC
        assert_eq!(now.offset(), UtcOffset::UTC);
    }

    #[test]
    fn test_local_date_warsaw_summer_time() {
        // Test with a date during summer time (CEST = UTC+2)
        // July 15, 2024, 12:00 UTC
        let date = Date::from_calendar_date(2024, Month::July, 15).unwrap();
        let time = Time::from_hms(12, 0, 0).unwrap();
        let utc_datetime = date.with_time(time).assume_utc();

        let result = local_date_warsaw(utc_datetime);

        // In summer, Warsaw is UTC+2, so 12:00 UTC = 14:00 CEST, same date
        assert_eq!(result, "2024-07-15");
    }

    #[test]
    fn test_local_date_warsaw_winter_time() {
        // Test with a date during winter time (CET = UTC+1)
        // January 15, 2024, 12:00 UTC
        let date = Date::from_calendar_date(2024, Month::January, 15).unwrap();
        let time = Time::from_hms(12, 0, 0).unwrap();
        let utc_datetime = date.with_time(time).assume_utc();

        let result = local_date_warsaw(utc_datetime);

        // In winter, Warsaw is UTC+1, so 12:00 UTC = 13:00 CET, same date
        assert_eq!(result, "2024-01-15");
    }

    #[test]
    fn test_local_date_warsaw_date_boundary() {
        // Test date boundary crossing - late UTC time that becomes next day in Warsaw
        // December 31, 2023, 23:30 UTC
        let date = Date::from_calendar_date(2023, Month::December, 31).unwrap();
        let time = Time::from_hms(23, 30, 0).unwrap();
        let utc_datetime = date.with_time(time).assume_utc();

        let result = local_date_warsaw(utc_datetime);

        // In winter, Warsaw is UTC+1, so 23:30 UTC = 00:30 CET next day
        assert_eq!(result, "2024-01-01");
    }

    #[test]
    fn test_local_date_warsaw_early_morning() {
        // Test early morning UTC that's still previous day in Warsaw
        // January 1, 2024, 00:30 UTC
        let date = Date::from_calendar_date(2024, Month::January, 1).unwrap();
        let time = Time::from_hms(0, 30, 0).unwrap();
        let utc_datetime = date.with_time(time).assume_utc();

        let result = local_date_warsaw(utc_datetime);

        // In winter, Warsaw is UTC+1, so 00:30 UTC = 01:30 CET, same date
        assert_eq!(result, "2024-01-01");
    }

    #[test]
    fn test_to_rfc3339_formatting() {
        // Test with a known datetime
        let date = Date::from_calendar_date(2024, Month::March, 15).unwrap();
        let time = Time::from_hms(14, 30, 45).unwrap();
        let datetime = date.with_time(time).assume_utc();

        let result = to_rfc3339(datetime);

        // RFC3339 format should be: YYYY-MM-DDTHH:MM:SSZ for UTC
        assert_eq!(result, "2024-03-15T14:30:45Z");
    }

    #[test]
    fn test_to_rfc3339_with_offset() {
        // Test with a non-UTC timezone
        let date = Date::from_calendar_date(2024, Month::June, 10).unwrap();
        let time = Time::from_hms(9, 15, 30).unwrap();
        let offset = UtcOffset::from_hms(2, 0, 0).unwrap(); // +02:00
        let datetime = date.with_time(time).assume_offset(offset);

        let result = to_rfc3339(datetime);

        // RFC3339 format should include the offset
        assert_eq!(result, "2024-06-10T09:15:30+02:00");
    }

    #[test]
    fn test_to_rfc3339_with_negative_offset() {
        // Test with negative UTC offset
        let date = Date::from_calendar_date(2024, Month::December, 25).unwrap();
        let time = Time::from_hms(18, 45, 0).unwrap();
        let offset = UtcOffset::from_hms(-5, 0, 0).unwrap(); // -05:00
        let datetime = date.with_time(time).assume_offset(offset);

        let result = to_rfc3339(datetime);

        // RFC3339 format should include the negative offset
        assert_eq!(result, "2024-12-25T18:45:00-05:00");
    }

    #[test]
    fn test_to_rfc3339_midnight() {
        // Test edge case with midnight
        let date = Date::from_calendar_date(2024, Month::February, 29).unwrap(); // Leap year
        let time = Time::from_hms(0, 0, 0).unwrap();
        let datetime = date.with_time(time).assume_utc();

        let result = to_rfc3339(datetime);

        assert_eq!(result, "2024-02-29T00:00:00Z");
    }

    #[test]
    fn test_integration_now_utc_to_rfc3339() {
        // Integration test: get current time and format it
        let now = now_utc();
        let formatted = to_rfc3339(now);

        // Should be a valid RFC3339 string ending with 'Z' (UTC)
        assert!(
            formatted.ends_with('Z'),
            "UTC time should end with 'Z', got: {}",
            formatted
        );

        // Should contain the basic structure
        assert!(
            formatted.contains('T'),
            "RFC3339 should contain 'T' separator"
        );
        assert!(
            formatted.len() >= 19,
            "RFC3339 should be at least 19 characters long"
        ); // YYYY-MM-DDTHH:MM:SSZ
    }

    #[test]
    fn test_integration_now_utc_to_warsaw_date() {
        // Integration test: get current time and convert to Warsaw date
        let now = now_utc();
        let warsaw_date = local_date_warsaw(now);

        // Should be in YYYY-MM-DD format
        assert_eq!(
            warsaw_date.len(),
            10,
            "Warsaw date should be 10 characters long"
        );
        assert!(
            warsaw_date.matches('-').count() == 2,
            "Warsaw date should have exactly 2 dashes"
        );

        // Should be parseable as a date
        let parts: Vec<&str> = warsaw_date.split('-').collect();
        assert_eq!(
            parts.len(),
            3,
            "Warsaw date should have 3 parts separated by dashes"
        );

        // Year should be 4 digits
        assert_eq!(parts[0].len(), 4, "Year should be 4 digits");
        assert!(parts[0].parse::<u32>().is_ok(), "Year should be numeric");

        // Month should be 2 digits
        assert_eq!(parts[1].len(), 2, "Month should be 2 digits");
        let month: u32 = parts[1].parse().expect("Month should be numeric");
        assert!(
            month >= 1 && month <= 12,
            "Month should be between 1 and 12"
        );

        // Day should be 2 digits
        assert_eq!(parts[2].len(), 2, "Day should be 2 digits");
        let day: u32 = parts[2].parse().expect("Day should be numeric");
        assert!(day >= 1 && day <= 31, "Day should be between 1 and 31");
    }
}
