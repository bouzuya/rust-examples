fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use time::{format_description::well_known::Rfc3339, OffsetDateTime};

    #[test]
    fn test_from_unix_timestamp() -> anyhow::Result<()> {
        let offset_date_time = OffsetDateTime::from_unix_timestamp(1612325106_i64)?;
        assert_eq!(
            offset_date_time.to_string(),
            "2021-02-03 4:05:06.0 +00:00:00"
        );
        let offset_date_time = OffsetDateTime::from_unix_timestamp_nanos(1612325106789012345_i128)?;
        assert_eq!(
            offset_date_time.to_string(),
            "2021-02-03 4:05:06.789012345 +00:00:00"
        );
        Ok(())
    }

    #[test]
    fn test_parse_and_format() -> anyhow::Result<()> {
        // format requires *formatting* feature
        // parse requires *parsing* feature
        let s = "2021-02-03T04:05:06.789012345Z";
        let offset_date_time = OffsetDateTime::parse(s, &Rfc3339)?;
        assert_eq!(offset_date_time.format(&Rfc3339)?, s);

        let s = "2021-02-03T04:05:06.789012345+09:00";
        let offset_date_time = OffsetDateTime::parse(s, &Rfc3339)?;
        assert_eq!(offset_date_time.format(&Rfc3339)?, s);

        let s = "20210-02-03T04:05:06.789012345Z";
        assert!(OffsetDateTime::parse(s, &Rfc3339).is_err());

        let s = "1-02-03T04:05:06.789012345Z";
        assert!(OffsetDateTime::parse(s, &Rfc3339).is_err());

        let s = "0001-02-03T04:05:06.789012345Z";
        let offset_date_time = OffsetDateTime::parse(s, &Rfc3339)?;
        assert_eq!(offset_date_time.format(&Rfc3339)?, s);

        Ok(())
    }
}
