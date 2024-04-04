// <https://cloud.google.com/storage/docs/authentication/signatures#credential-scope>

use crate::private::{Date, Location, RequestType, Service};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] ErrorKind);

#[derive(Debug, thiserror::Error)]
enum ErrorKind {
    #[error("SERVICE '{0}' and REQUEST_TYPE '{1}' is an invalid combination")]
    InvalidCombinationOfServiceAndRequestType(Service, RequestType),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CredentialScope {
    date: Date,
    location: Location,
    service: Service,
    request_type: RequestType,
}

impl CredentialScope {
    pub fn new(
        date: Date,
        location: Location,
        service: Service,
        request_type: RequestType,
    ) -> Result<Self, Error> {
        match (service, request_type) {
            (Service::Storage, RequestType::Aws4Request)
            | (Service::S3, RequestType::Goog4Request) => {
                return Err(Error::from(
                    ErrorKind::InvalidCombinationOfServiceAndRequestType(service, request_type),
                ));
            }
            (Service::Storage, RequestType::Goog4Request)
            | (Service::S3, RequestType::Aws4Request) => {
                // do nothing
            }
        }
        Ok(Self {
            date,
            location,
            service,
            request_type,
        })
    }
}

impl std::fmt::Display for CredentialScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}",
            self.date, self.location, self.service, self.request_type
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::private::utils::UnixTimestamp;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_impls<T: Clone + std::fmt::Debug + std::fmt::Display + Eq + PartialEq>() {}
        assert_impls::<CredentialScope>();

        let date1 = Date::from_unix_timestamp_obj(UnixTimestamp::from_rfc3339(
            "2020-01-02T03:04:05+00:00",
        )?);
        let location1 = Location::try_from("us-east1")?;
        assert_eq!(
            CredentialScope::new(
                date1,
                location1.clone(),
                Service::Storage,
                RequestType::Goog4Request,
            )?
            .to_string(),
            "20200102/us-east1/storage/goog4_request"
        );

        assert!(CredentialScope::new(
            date1,
            location1.clone(),
            Service::Storage,
            RequestType::Aws4Request
        )
        .is_err());
        assert!(CredentialScope::new(
            date1,
            location1.clone(),
            Service::S3,
            RequestType::Goog4Request
        )
        .is_err());
        assert!(CredentialScope::new(
            date1,
            location1.clone(),
            Service::S3,
            RequestType::Aws4Request
        )
        .is_ok());

        Ok(())
    }
}
