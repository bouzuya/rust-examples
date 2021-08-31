fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::{DateTime, NaiveDateTime, Utc};
    use ulid::Ulid;

    #[test]
    fn new_test() {
        let ulid = Ulid::new();
        assert_ne!(ulid, Ulid::nil());
    }

    #[test]
    fn string_convertion_test() {
        let ulid = Ulid::new();
        let s = ulid.to_string();
        assert_eq!(Ulid::from_str(s.as_str()), Ok(ulid));
        assert_eq!(Ulid::from_string(s.as_str()), Ok(ulid));
        assert_eq!(ulid.to_string(), "");
    }

    #[test]
    fn datetime_convertion_test() {
        let ulid = Ulid::new();
        let datetime = ulid.datetime();
        assert_ne!(Ulid::from_datetime(datetime), ulid);

        let datetime = Utc::now();
        let ulid = Ulid::from_datetime(datetime);
        assert_ne!(ulid.datetime(), datetime); // ulid doesn't have nsecs, datetime has nsecs
        assert_ne!(
            ulid.datetime(),
            DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(
                    datetime.timestamp(),
                    datetime.timestamp_subsec_millis() * 1_000,
                ),
                Utc
            )
        );
        assert_eq!(ulid.timestamp_ms(), datetime.timestamp_millis() as u64);
    }

    #[test]
    fn nil_test() {
        let ulid = Ulid::nil();
        assert_eq!(ulid, Ulid::nil());
        assert_eq!(ulid.to_string(), "00000000000000000000000000");
        assert!(ulid.is_nil());
    }
}
