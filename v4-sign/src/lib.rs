mod private;
mod service_account_credentials;

use std::str::FromStr;
use std::time::SystemTime;

use private::policy_document;

use self::private::utils::UnixTimestamp;
use self::private::ActiveDatetime;
use self::private::CredentialScope;
use self::private::Date;
use self::private::Expiration;
use self::private::HttpVerb;
use self::private::Location;
use self::private::RequestType;
use self::private::Service;
use self::private::SigningAlgorithm;
use self::private::{hex_encode, sign, SignedUrl};

pub use self::service_account_credentials::ServiceAccountCredentials;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] ErrorKind);

#[derive(Debug, thiserror::Error)]
enum ErrorKind {
    #[error(transparent)]
    CredentialScope(#[from] crate::private::credential_scope::Error),
    #[error(transparent)]
    Expiration(crate::private::expiration::Error),
    #[error("expiration out of range")]
    ExpirationOutOfRange,
    #[error(transparent)]
    File(std::io::Error),
    #[error(transparent)]
    Field(crate::private::policy_document::field::Error),
    #[error(transparent)]
    HttpMethod(crate::private::http_verb::Error),
    #[error(transparent)]
    HttpRequest(http::Error),
    #[error("invalid json")]
    InvalidServiceAccountJson(serde_json::Error),
    #[error(transparent)]
    Location(crate::private::location::Error),
    #[error("now out of range")]
    Now,
    #[error("pem")]
    Pem,
    #[error("policy document encoding")]
    PolicyDocumentEncoding,
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
    SignedUrl(crate::private::signed_url::Error),
}

pub struct BuildHtmlFormDataOptions {
    pub service_account_client_email: String,
    pub service_account_private_key: String,
    pub bucket_name: String,
    pub object_name: String,
    pub region: String,
    pub expiration: i64,
    pub now: SystemTime,
}

pub fn build_html_form_data(
    BuildHtmlFormDataOptions {
        service_account_client_email,
        service_account_private_key,
        bucket_name,
        object_name,
        region,
        expiration,
        now,
    }: BuildHtmlFormDataOptions,
) -> Result<Vec<(&'static str, String)>, Error> {
    let now = UnixTimestamp::from_system_time(now).map_err(|_| ErrorKind::Now)?;

    let credential_scope = CredentialScope::new(
        Date::from_unix_timestamp_obj(now),
        Location::try_from(region.as_str()).map_err(ErrorKind::Location)?,
        Service::Storage,
        RequestType::Goog4Request,
    )
    .map_err(ErrorKind::CredentialScope)?;
    let x_goog_algorithm = SigningAlgorithm::Goog4RsaSha256;
    let x_goog_credential = format!("{}/{}", service_account_client_email, credential_scope);
    let x_goog_date = ActiveDatetime::from_unix_timestamp_obj(now).to_string();
    let policy_document = policy_document::PolicyDocument {
        conditions: vec![
            policy_document::Condition::ExactMatching(
                policy_document::Field::new("bucket").map_err(ErrorKind::Field)?,
                policy_document::Value::new(bucket_name.clone()),
            ),
            policy_document::Condition::ExactMatching(
                policy_document::Field::new("key").map_err(ErrorKind::Field)?,
                policy_document::Value::new(object_name.clone()),
            ),
            // `policy` field is not included in the policy document
            policy_document::Condition::ExactMatching(
                policy_document::Field::new("x-goog-algorithm").map_err(ErrorKind::Field)?,
                policy_document::Value::new(x_goog_algorithm.as_ref()),
            ),
            policy_document::Condition::ExactMatching(
                policy_document::Field::new("x-goog-credential").map_err(ErrorKind::Field)?,
                policy_document::Value::new(x_goog_credential.as_str()),
            ),
            policy_document::Condition::ExactMatching(
                policy_document::Field::new("x-goog-date").map_err(ErrorKind::Field)?,
                policy_document::Value::new(x_goog_date.clone()),
            ),
            // `x-goog-signature` field is not included in the policy document
            // `file` field is not included in the policy document
        ],
        expiration: policy_document::Expiration::from_unix_timestamp_obj(
            UnixTimestamp::try_from(i64::from(now) + expiration)
                .map_err(|_| ErrorKind::ExpirationOutOfRange)?,
        ),
    };
    let policy =
        serde_json::to_string(&policy_document).map_err(|_| ErrorKind::PolicyDocumentEncoding)?;
    let encoded_policy = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        policy.as_bytes(),
    );

    let message = encoded_policy.as_str();
    let pkcs8 = pem::parse(service_account_private_key.as_bytes()).map_err(|_| ErrorKind::Pem)?;
    let signing_key = pkcs8.contents();
    let message_digest =
        sign(x_goog_algorithm, signing_key, message.as_bytes()).map_err(ErrorKind::SignedUrl)?;
    let request_signature = hex_encode(&message_digest);

    Ok(vec![
        ("bucket", bucket_name),
        ("key", object_name),
        ("policy", encoded_policy),
        ("x-goog-algorithm", x_goog_algorithm.as_ref().to_string()),
        ("x-goog-credential", x_goog_credential),
        ("x-goog-date", x_goog_date),
        ("x-goog-signature", request_signature),
    ])
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuildSignedUrlOptions {
    pub service_account_client_email: String,
    pub service_account_private_key: String,
    pub bucket_name: String,
    pub object_name: String,
    pub region: String,
    pub expiration: u32,
    pub http_method: String,
    pub now: SystemTime,
}

pub fn build_signed_url(
    BuildSignedUrlOptions {
        service_account_client_email,
        service_account_private_key,
        bucket_name,
        object_name,
        region,
        expiration,
        http_method,
        now,
    }: BuildSignedUrlOptions,
) -> Result<String, Error> {
    let now = UnixTimestamp::from_system_time(now).map_err(|_| ErrorKind::Now)?;

    let http_method = HttpVerb::from_str(http_method.as_str()).map_err(ErrorKind::HttpMethod)?;
    let request = http::Request::builder()
        .header("Host", "storage.googleapis.com")
        .method(http::Method::from(http_method))
        .uri(
            format!(
                "https://storage.googleapis.com/{}/{}",
                // TODO: escape bucket_name and object_name
                bucket_name,
                object_name
            )
            .as_str(),
        )
        .body(())
        .map_err(ErrorKind::HttpRequest)?;
    let credential_scope = CredentialScope::new(
        Date::from_unix_timestamp_obj(now),
        Location::try_from(region.as_str()).map_err(ErrorKind::Location)?,
        Service::Storage,
        RequestType::Goog4Request,
    )
    .map_err(ErrorKind::CredentialScope)?;
    let signed_url = SignedUrl::new(
        &credential_scope,
        ActiveDatetime::from_unix_timestamp_obj(now),
        Expiration::try_from(i64::from(expiration)).map_err(ErrorKind::Expiration)?,
        &service_account_client_email,
        &service_account_private_key,
        request,
    )
    .map_err(ErrorKind::SignedUrl)?;
    Ok(String::from(signed_url))
}
