mod active_datetime;
mod canonical_request;
mod credential_scope;
mod date;
mod expiration;
mod location;
pub mod policy_document;
mod private;
mod request_type;
mod service;
mod signed_url;
mod signing_algorithm;
mod string_to_sign;

use active_datetime::ActiveDatetime;
use expiration::Expiration;
use location::Location;
use signed_url::SignedUrl;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] ErrorKind);

#[derive(Debug, thiserror::Error)]
enum ErrorKind {
    #[error(transparent)]
    Expiration(crate::expiration::Error),
    #[error(transparent)]
    File(std::io::Error),
    #[error("env var GOOGLE_APPLICATION_CREDENTIALS is not found")]
    GoogleApplicationCredentialsNotFound,
    #[error(transparent)]
    HttpMethod(http::method::InvalidMethod),
    #[error(transparent)]
    HttpRequest(http::Error),
    #[error("invalid json")]
    InvalidServiceAccountJson(serde_json::Error),
    #[error(transparent)]
    Location(crate::location::Error),
    #[error("client_email is not found")]
    ServiceAccountJsonClientEmailIsNotFound,
    #[error("client_email is not string")]
    ServiceAccountJsonClientEmailIsNotString,
    #[error("json root is not object")]
    ServiceAccountJsonRootIsNotObject,
    #[error("private_key is not found")]
    ServiceAccountJsonPrivateKeyIsNotFound,
    #[error("private_key is not string")]
    ServiceAccountJsonPrivateKeyIsNotString,
    #[error(transparent)]
    SignedUrl(crate::signed_url::Error),
}

// FIXME: signature
pub fn signed_url(
    bucket_name: &str,
    object_name: &str,
    region: &str,
    expiration: i64,
    http_method: &str,
) -> Result<String, Error> {
    let google_application_credentials = std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
        .map_err(|_| ErrorKind::GoogleApplicationCredentialsNotFound)?;
    let (service_account_client_email, service_account_private_key) =
        load_service_account_credentials(google_application_credentials.as_str())?;
    let active_datetime = ActiveDatetime::now();
    let request = http::Request::builder()
        .header("Host", "storage.googleapis.com")
        .method(http::Method::try_from(http_method).map_err(ErrorKind::HttpMethod)?)
        .uri(
            format!(
                "https://storage.googleapis.com/{}{}",
                bucket_name, object_name
            )
            .as_str(),
        )
        .body(())
        .map_err(ErrorKind::HttpRequest)?;
    let signed_url = SignedUrl::new(
        active_datetime,
        Location::try_from(region).map_err(ErrorKind::Location)?,
        Expiration::try_from(expiration).map_err(ErrorKind::Expiration)?,
        &service_account_client_email,
        &service_account_private_key,
        request,
    )
    .map_err(ErrorKind::SignedUrl)?;
    Ok(String::from(signed_url))
}

fn load_service_account_credentials(
    google_application_credentials: &str,
) -> Result<(String, String), Error> {
    let path = google_application_credentials;
    let path = std::path::Path::new(path);
    let mut file = std::fs::File::open(path).map_err(ErrorKind::File)?;
    let mut s = String::new();
    std::io::Read::read_to_string(&mut file, &mut s).map_err(ErrorKind::File)?;
    let json_value: serde_json::Value =
        serde_json::from_str(s.as_str()).map_err(ErrorKind::InvalidServiceAccountJson)?;
    let json_object = json_value
        .as_object()
        .ok_or_else(|| ErrorKind::ServiceAccountJsonRootIsNotObject)?;
    let client_email = json_object
        .get("client_email")
        .ok_or_else(|| ErrorKind::ServiceAccountJsonClientEmailIsNotFound)?
        .as_str()
        .ok_or_else(|| ErrorKind::ServiceAccountJsonClientEmailIsNotString)?
        .to_string();
    let private_key = json_object
        .get("private_key")
        .ok_or_else(|| ErrorKind::ServiceAccountJsonPrivateKeyIsNotFound)?
        .as_str()
        .ok_or_else(|| ErrorKind::ServiceAccountJsonPrivateKeyIsNotString)?
        .to_string();
    Ok((client_email, private_key))
}
