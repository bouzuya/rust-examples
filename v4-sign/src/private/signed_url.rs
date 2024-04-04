use std::{collections::BTreeSet, vec};

use super::ActiveDatetime;
use super::CredentialScope;
use super::Expiration;
use super::SigningAlgorithm;
use super::StringToSign;
use super::{canonical_query_string, CanonicalRequest};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] ErrorKind);

#[derive(Debug, thiserror::Error)]
enum ErrorKind {
    #[error(transparent)]
    CanonicalRequest(crate::private::canonical_request::Error),
    #[error("host header not found")]
    HostHeaderNotFound,
    #[error("pem key rejected: {0}")]
    KeyRejected(ring::error::KeyRejected),
    #[error("pem: {0}")]
    Pem(pem::PemError),
    #[error("sign: {0}")]
    Sign(ring::error::Unspecified),
}

pub struct SignedUrl(String);

impl SignedUrl {
    pub(crate) fn new(
        credential_scope: &CredentialScope,
        active_datetime: ActiveDatetime,
        expiration: Expiration,
        service_account_client_email: &str,
        service_account_private_key: &str,
        mut request: http::Request<()>,
    ) -> Result<Self, Error> {
        let x_goog_algorithm = SigningAlgorithm::Goog4RsaSha256;
        add_signed_url_required_query_string_parameters(
            &mut request,
            service_account_client_email,
            x_goog_algorithm,
            active_datetime,
            credential_scope,
            expiration,
        )?;
        let canonical_query_string = canonical_query_string(&request);
        let string_to_sign = StringToSign::new(
            x_goog_algorithm,
            active_datetime,
            credential_scope,
            CanonicalRequest::new(&request).map_err(ErrorKind::CanonicalRequest)?,
        );

        let message = string_to_sign.to_string();
        let pkcs8 = pem::parse(service_account_private_key.as_bytes()).map_err(ErrorKind::Pem)?;
        let signing_key = pkcs8.contents();
        let message_digest = sign(x_goog_algorithm, signing_key, message.as_bytes())?;
        let request_signature = hex_encode(&message_digest);

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
    x_goog_algorithm: SigningAlgorithm,
    x_goog_date: ActiveDatetime,
    credential_scope: &CredentialScope,
    expiration: Expiration,
) -> Result<(), ErrorKind> {
    if !request.headers().contains_key(http::header::HOST) {
        return Err(ErrorKind::HostHeaderNotFound);
    }
    let authorizer = service_account_client_email;
    let mut url1 = url::Url::parse(request.uri().to_string().as_str()).expect("uri to be valid");
    url1.query_pairs_mut()
        .append_pair("X-Goog-Algorithm", x_goog_algorithm.as_str())
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

pub(crate) fn hex_encode(message_digest: &[u8]) -> String {
    use std::fmt::Write as _;
    message_digest.iter().fold(String::new(), |mut s, b| {
        let _ = write!(s, "{:02x}", b);
        s
    })
}

pub(crate) fn sign(
    algorithm: SigningAlgorithm,
    key: &[u8],
    message: &[u8],
) -> Result<Vec<u8>, Error> {
    match algorithm {
        SigningAlgorithm::Goog4RsaSha256 => {
            let key_pair =
                ring::signature::RsaKeyPair::from_pkcs8(key).map_err(ErrorKind::KeyRejected)?;
            let mut signature = vec![0; key_pair.public().modulus_len()];
            key_pair
                .sign(
                    &ring::signature::RSA_PKCS1_SHA256,
                    &ring::rand::SystemRandom::new(),
                    message,
                    &mut signature,
                )
                .map_err(ErrorKind::Sign)?;
            Ok(signature)
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::private::{utils::UnixTimestamp, Date, Location, RequestType, Service};

    use super::*;

    #[test]
    fn test_add_signed_url_required_query_string_parameters() -> anyhow::Result<()> {
        let unix_timestamp = i64::from(UnixTimestamp::from_rfc3339("2020-01-02T03:04:05Z")?);
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
            SigningAlgorithm::Goog4RsaSha256,
            ActiveDatetime::from_unix_timestamp(unix_timestamp)?,
            &CredentialScope::new(
                Date::from_unix_timestamp(unix_timestamp)?,
                Location::try_from("us-central1")?,
                Service::Storage,
                RequestType::Goog4Request,
            )?,
            expiration,
        )?;
        let s = CanonicalRequest::new(&request)?.to_string();
        assert!(s.contains("X-Goog-Algorithm=GOOG4-RSA-SHA256&X-Goog-Credential=service_account_name1%2F20200102%2Fus-central1%2Fstorage%2Fgoog4_request&X-Goog-Date=20200102T030405Z&X-Goog-Expires=604800&X-Goog-SignedHeaders=content-type%3Bhost%3Bx-goog-meta-reviewer&generation=1360887697105000&userProject=my-project"));
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
}
