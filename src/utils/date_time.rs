use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};

pub struct DateUtils;

impl DateUtils {
    pub fn sast_now() -> DateTime<FixedOffset> {
        let utc: DateTime<Utc> = Utc::now();
        let offset = FixedOffset::east_opt(2 * 3600).unwrap();
        let sast: DateTime<FixedOffset> = DateTime::with_timezone(&utc, &offset);
        return sast;
    }

    pub fn utc_now() -> DateTime<Utc> {
        Utc::now()
    }

    pub fn utc_naive() -> NaiveDateTime {
        DateUtils::utc_now().naive_utc()
    }
}