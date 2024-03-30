use std::{collections::BTreeSet, vec};

use crate::active_datetime::ActiveDatetime;
use crate::canonical_request::{canonical_query_string, CanonicalRequest};
use crate::credential_scope::CredentialScope;
use crate::date::Date;
use crate::expiration::Expiration;
use crate::location::Location;
use crate::request_type::RequestType;
use crate::signing_algorithm::SigningAlgorithm;
use crate::string_to_sign::StringToSign;

use crate::service::Service;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] ErrorKind);

#[derive(Debug, thiserror::Error)]
enum ErrorKind {
    #[error("FIXME: {0}")]
    Fixme(String),
    #[error(transparent)]
    CredentialScope(#[from] crate::credential_scope::Error),
}

pub struct SignedUrl(String);

impl SignedUrl {
    pub(crate) fn new(
        active_datetime: ActiveDatetime,
        location: Location,
        expiration: Expiration,
        service_account_client_email: &str,
        service_account_private_key: &str,
        mut request: http::Request<()>,
    ) -> Result<Self, Error> {
        let credential_scope = CredentialScope::new(
            Date::from_unix_timestamp(active_datetime.unix_timestamp())
                .expect("active_datetime.unix_timestamp to be valid date"),
            location,
            Service::Storage,
            RequestType::Goog4Request,
        )
        .map_err(ErrorKind::CredentialScope)?;
        add_signed_url_required_query_string_parameters(
            &mut request,
            service_account_client_email,
            active_datetime,
            &credential_scope,
            expiration,
        )?;
        let canonical_query_string = canonical_query_string(&request);
        let string_to_sign = StringToSign::new(
            SigningAlgorithm::Goog4RsaSha256,
            active_datetime,
            credential_scope,
            CanonicalRequest::new(&request),
        );
        let request_signature = {
            let pkcs8 = pem::parse(service_account_private_key.as_bytes()).unwrap();
            let key_pair =
                ring::signature::RsaKeyPair::from_pkcs8(pkcs8.contents()).expect("key to be valid");
            let mut signature = vec![0; key_pair.public().modulus_len()];
            key_pair
                .sign(
                    &ring::signature::RSA_PKCS1_SHA256,
                    &ring::rand::SystemRandom::new(),
                    string_to_sign.to_string().as_bytes(),
                    &mut signature,
                )
                .unwrap();
            use std::fmt::Write as _;
            signature.into_iter().fold(String::new(), |mut s, b| {
                let _ = write!(s, "{:02x}", b);
                s
            })
        };

        let hostname = "https://storage.googleapis.com";
        let path_to_resource = request.uri().path();
        let signed_url = [
            hostname,
            path_to_resource,
            "?",
            canonical_query_string.as_str(),
            "&X-Goog-Signature=",
            request_signature.as_str(),
        ]
        .join("");
        Ok(Self(signed_url))
    }
}

impl std::convert::From<SignedUrl> for String {
    fn from(value: SignedUrl) -> Self {
        value.0
    }
}

fn add_signed_url_required_query_string_parameters(
    request: &mut http::Request<()>,
    service_account_client_email: &str,
    x_goog_date: ActiveDatetime,
    credential_scope: &CredentialScope,
    expiration: Expiration,
) -> Result<(), ErrorKind> {
    if !request.headers().contains_key(http::header::HOST) {
        return Err(ErrorKind::Fixme("Host header is required".to_string()));
    }
    let authorizer = service_account_client_email;
    let mut url1 = url::Url::parse(request.uri().to_string().as_str()).expect("uri to be valid");
    url1.query_pairs_mut()
        .append_pair(
            "X-Goog-Algorithm",
            SigningAlgorithm::Goog4RsaSha256.as_str(),
        )
        .append_pair(
            "X-Goog-Credential",
            format!("{authorizer}/{credential_scope}")
                .replace('/', "%2F")
                .as_str(),
        )
        .append_pair("X-Goog-Date", x_goog_date.to_string().as_str())
        .append_pair("X-Goog-Expires", expiration.to_string().as_str())
        .append_pair(
            "X-Goog-SignedHeaders",
            request
                .headers()
                .keys()
                .map(|k| k.to_string().to_ascii_lowercase())
                .collect::<BTreeSet<String>>()
                .into_iter()
                .collect::<Vec<String>>()
                .join(";")
                .as_str(),
        )
        .finish();
    *request.uri_mut() = http::Uri::try_from(url1.to_string()).expect("url to be valid");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::date::Date;

    use super::*;

    #[test]
    fn test_add_signed_url_required_query_string_parameters() -> anyhow::Result<()> {
        // TODO: host header is required

        let chrono_date_time =
            chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339("2020-01-02T03:04:05Z")?
                .naive_utc()
                .and_utc();
        let expiration = Expiration::try_from(604800)?;
        let service_account_client_email = "service_account_name1";
        let mut request = http::Request::builder()
            .header("Host", "storage.googleapis.com")
            .header("Content-Type", "text/plain")
            .header("x-goog-meta-reviewer", "jane")
            .header("x-goog-meta-reviewer", "john")
            .method(http::Method::POST)
            .uri("https://storage.googleapis.com/example-bucket/cat-pics/tabby.jpeg?generation=1360887697105000&userProject=my-project")
            .body(())?;
        add_signed_url_required_query_string_parameters(
            &mut request,
            service_account_client_email,
            ActiveDatetime::try_from(chrono_date_time)?,
            &CredentialScope::new(
                Date::try_from(chrono_date_time)?,
                Location::try_from("us-central1")?,
                Service::Storage,
                RequestType::Goog4Request,
            )?,
            expiration,
        )?;
        let s = CanonicalRequest::new(&request).to_string();
        assert!(s.contains("X-Goog-Algorithm=GOOG4-RSA-SHA256&X-Goog-Credential=service_account_name1/20200102/us-central1/storage/goog4_request&X-Goog-Date=20200102T030405Z&X-Goog-Expires=604800&X-Goog-SignedHeaders=content-type%3Bhost%3Bx-goog-meta-reviewer&generation=1360887697105000&userProject=my-project"));
        Ok(())
    }

    #[test]
    fn test_request_header() -> anyhow::Result<()> {
        let request = http::Request::builder()
            .header("Content-Type", "text/plain")
            .body(())?;
        assert!(request.headers().contains_key("Content-Type"));
        assert!(request.headers().contains_key("content-type"));
        assert!(request.headers().contains_key(http::header::CONTENT_TYPE));
        Ok(())
    }

    #[test]
    fn test_chrono() -> anyhow::Result<()> {
        let chrono_date_time =
            chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339("2020-01-02T03:04:05Z")?
                .naive_utc()
                .and_utc();
        let date = chrono_date_time.format("%Y%m%d").to_string();
        let x_goog_date = chrono_date_time.format("%Y%m%dT%H%M%SZ").to_string();
        assert_eq!(date, "20200102");
        assert_eq!(x_goog_date, "20200102T030405Z");
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_sign() -> anyhow::Result<()> {
        use anyhow::Context as _;
        let bucket_name = std::env::var("BUCKET_NAME")?;
        let google_application_credentials = std::env::var("GOOGLE_APPLICATION_CREDENTIALS")?;
        let object_name = std::env::var("OBJECT_NAME")?;
        let region = std::env::var("REGION")?;

        let active_datetime = ActiveDatetime::now();
        let location = Location::try_from(region.as_str())?;
        let expiration = Expiration::try_from(604800)?;
        let (service_account_client_email, service_account_private_key) = {
            let path = google_application_credentials;
            let path = std::path::Path::new(path.as_str());
            let mut file = std::fs::File::open(path)?;
            let mut s = String::new();
            std::io::Read::read_to_string(&mut file, &mut s)?;
            let json_value: serde_json::Value = serde_json::from_str(s.as_str())?;
            let json_object = json_value.as_object().context("json root is not object")?;
            let client_email = json_object
                .get("client_email")
                .context("client_email is not found")?
                .as_str()
                .context("client_email is not string")?
                .to_string();
            let private_key = json_object
                .get("private_key")
                .context("private_key is not found")?
                .as_str()
                .context("private_key is not string")?
                .to_string();
            Result::<_, anyhow::Error>::Ok((client_email, private_key))
        }?;
        let request = http::Request::builder()
            .header("Host", "storage.googleapis.com")
            .method(http::Method::GET)
            .uri(
                format!(
                    "https://storage.googleapis.com/{}{}",
                    bucket_name, object_name
                )
                .as_str(),
            )
            .body(())?;
        let signed_url = SignedUrl::new(
            active_datetime,
            location,
            expiration,
            &service_account_client_email,
            &service_account_private_key,
            request,
        )?;

        let response = reqwest::get(String::from(signed_url)).await?;
        assert_eq!(response.status().as_u16(), 200);
        assert_eq!(response.text().await?, "abc\n");

        Ok(())
    }
}
