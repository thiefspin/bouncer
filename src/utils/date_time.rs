use chrono::{DateTime, FixedOffset, Utc};

pub struct DateUtils;

impl DateUtils {
    pub fn sast_now() -> DateTime<FixedOffset> {
        let utc: DateTime<Utc> = Utc::now();
        let offset = FixedOffset::east_opt(2 * 3600).unwrap();
        let sast: DateTime<FixedOffset> = DateTime::with_timezone(&utc, &offset);
        return sast;
    }
}