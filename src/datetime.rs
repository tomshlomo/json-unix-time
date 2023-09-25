use chrono::{DateTime, TimeZone, Utc};

fn year_to_datetime(year: i32) -> Option<DateTime<Utc>> {
    Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).earliest()
}

pub fn year_to_ts(year: i32) -> Option<i64> {
    // Convert the DateTime<Utc> to Unix time in seconds
    let datetime = year_to_datetime(year)?;
    Some(datetime.timestamp())
}

pub fn ts_to_str(ts: i64, fmt: &str) -> Option<String> {
    let datetime_utc = Utc.timestamp_opt(ts, 0).earliest()?;
    // You can format the DateTime in a human-readable way
    Some(datetime_utc.format(fmt).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(year_to_ts(1970), Some(0))
    }
}
