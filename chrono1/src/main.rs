fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::prelude::*;

    #[test]
    fn string_convesion_test() -> anyhow::Result<()> {
        let f1 = |s: &str| -> anyhow::Result<String> {
            Ok(DateTime::<FixedOffset>::from_str(s)?.to_rfc3339())
        };
        let f2 = |s: &str| -> anyhow::Result<String> {
            Ok(DateTime::<FixedOffset>::from_str(s)?.to_rfc3339_opts(SecondsFormat::Secs, true))
        };
        let s = "2014-11-28T21:00:09+09:00";
        assert_eq!(f1(s)?, s);
        assert_eq!(f2(s)?, s);
        let s = "2014-11-28T21:00:09+00:00";
        assert_eq!(f1(s)?, s);
        assert_eq!(f2(s)?, "2014-11-28T21:00:09Z"); // +00:00 -> Z
        let s = "2014-11-28T21:00:09Z";
        assert_eq!(f1(s)?, "2014-11-28T21:00:09+00:00"); // Z -> +00:00
        assert_eq!(f2(s)?, s);
        let s = "2014-11-28T21:00:09.123Z";
        assert_eq!(
            DateTime::<FixedOffset>::from_str(s)?.to_rfc3339_opts(SecondsFormat::Secs, false),
            "2014-11-28T21:00:09+00:00"
        );
        assert_eq!(
            DateTime::<FixedOffset>::from_str(s)?.to_rfc3339_opts(SecondsFormat::Millis, false),
            "2014-11-28T21:00:09.123+00:00"
        );
        {
            let dt1 = DateTime::<FixedOffset>::from_str("2014-11-28T21:00:09.123Z")?;
            let dt2 = DateTime::<FixedOffset>::from_str("2014-11-28T21:00:09Z")?;
            assert_ne!(dt1, dt2);
            assert_eq!(
                dt1.to_rfc3339_opts(SecondsFormat::Secs, true),
                dt2.to_rfc3339_opts(SecondsFormat::Secs, true),
            );
            assert_eq!(dt1.with_nanosecond(0).unwrap(), dt2);
        }
        Ok(())
    }

    #[test]
    fn timestamp_convesion_test() -> anyhow::Result<()> {
        let timestamp = 1612336638;
        let utc = NaiveDateTime::from_timestamp(timestamp, 0);
        let dt_utc: DateTime<Utc> = Utc.from_utc_datetime(&utc);
        assert_eq!(dt_utc.to_rfc3339(), "2021-02-03T07:17:18+00:00");
        assert_eq!(dt_utc.timestamp(), timestamp);
        let dt_fixed1: DateTime<FixedOffset> =
            dt_utc.with_timezone(&FixedOffset::east(9 * 60 * 60));
        assert_eq!(dt_fixed1.to_rfc3339(), "2021-02-03T16:17:18+09:00");
        assert_eq!(dt_fixed1.timestamp(), timestamp);
        assert_eq!(dt_fixed1.with_timezone(&Utc), dt_utc);
        let dt_fixed2 = FixedOffset::east(9 * 60 * 60).from_utc_datetime(&utc);
        assert_eq!(dt_fixed2.to_rfc3339(), "2021-02-03T16:17:18+09:00");
        assert_eq!(dt_fixed2.timestamp(), timestamp);
        assert_eq!(dt_fixed1, dt_fixed2);
        Ok(())
    }
}
