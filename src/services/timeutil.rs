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
