use crate::{date::Date, location::Location, request_type::RequestType, service::Service};

// <https://cloud.google.com/storage/docs/authentication/signatures#credential-scope>
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CredentialScope {
    date: Date,
    location: Location,
    service: Service,
    request_type: RequestType,
}

impl CredentialScope {
    pub(crate) fn new(
        date: Date,
        location: Location,
        service: Service,
        request_type: RequestType,
    ) -> Self {
        Self {
            date,
            location,
            service,
            request_type,
        }
    }
}

impl std::fmt::Display for CredentialScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}",
            self.date,
            self.location,
            self.service.as_str(),
            self.request_type.as_str()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_impls<T: Clone + std::fmt::Debug + Eq + PartialEq>() {}
        assert_impls::<CredentialScope>();

        let chrono_date_time = chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(
            "2020-01-02T03:04:05+00:00",
        )?
        .naive_utc()
        .and_utc();
        assert_eq!(
            CredentialScope::new(
                Date::try_from(chrono_date_time)?,
                Location::try_from("us-east1")?,
                Service::Storage,
                RequestType::Goog4Request,
            )
            .to_string(),
            "20200102/us-east1/storage/goog4_request"
        );
        Ok(())
    }
}
