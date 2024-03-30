#[derive(Debug, thiserror::Error)]
#[error("error")]
pub struct Error;

pub(crate) struct ActiveDatetime(i64);

impl ActiveDatetime {
    pub(crate) fn now() -> Self {
        Self(chrono::Utc::now().timestamp())
    }
}

impl std::convert::TryFrom<chrono::DateTime<chrono::Utc>> for ActiveDatetime {
    type Error = Error;

    fn try_from(value: chrono::DateTime<chrono::Utc>) -> Result<Self, Self::Error> {
        let year = chrono::Datelike::year(&value);
        if !(0..=9999).contains(&year) {
            return Err(Error);
        }
        Ok(Self(value.timestamp()))
    }
}

impl std::fmt::Display for ActiveDatetime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let chrono_date_time =
            chrono::DateTime::from_timestamp(self.0, 0_u32).expect("self.0 to be valid timestamp");
        let s = chrono_date_time.format("%Y%m%dT%H%M%SZ");
        s.fmt(f)
    }
}
