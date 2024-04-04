mod private;
mod service_account_credentials;

use std::str::FromStr;

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

pub fn html_form_params(
    service_account_client_email: &str,
    service_account_private_key: &str,
    bucket_name: &str,
    object_name: &str,
    region: &str,
    expiration: i64,
) -> Result<Vec<(&'static str, String)>, Error> {
    let active_datetime = ActiveDatetime::now();

    let credential_scope = CredentialScope::new(
        Date::from_unix_timestamp_obj(active_datetime.unix_timestamp_obj()),
        Location::try_from(region).map_err(ErrorKind::Location)?,
        Service::Storage,
        RequestType::Goog4Request,
    )
    .map_err(ErrorKind::CredentialScope)?;
    let x_goog_algorithm = SigningAlgorithm::Goog4RsaSha256;
    let x_goog_credential = format!("{}/{}", service_account_client_email, credential_scope);
    let x_goog_date = active_datetime.to_string();
    let policy_document = policy_document::PolicyDocument {
        conditions: vec![
            policy_document::Condition::ExactMatching(
                policy_document::Field::new("bucket").map_err(ErrorKind::Field)?,
                policy_document::Value::new(bucket_name),
            ),
            policy_document::Condition::ExactMatching(
                policy_document::Field::new("key").map_err(ErrorKind::Field)?,
                policy_document::Value::new(object_name),
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
            UnixTimestamp::try_from(active_datetime.unix_timestamp() + expiration)
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
        ("bucket", bucket_name.to_string()),
        ("key", object_name.to_string()),
        ("policy", encoded_policy),
        ("x-goog-algorithm", x_goog_algorithm.as_ref().to_string()),
        ("x-goog-credential", x_goog_credential),
        ("x-goog-date", x_goog_date),
        ("x-goog-signature", request_signature),
    ])
}

// FIXME: signature
pub struct BuildSignedUrlOptions {
    pub service_account_client_email: String,
    pub service_account_private_key: String,
    pub bucket_name: String,
    pub object_name: String,
    pub region: String,
    pub expiration: i64,
    pub http_method: String,
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
    }: BuildSignedUrlOptions,
) -> Result<String, Error> {
    let active_datetime = ActiveDatetime::now();

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
        Date::from_unix_timestamp_obj(active_datetime.unix_timestamp_obj()),
        Location::try_from(region.as_str()).map_err(ErrorKind::Location)?,
        Service::Storage,
        RequestType::Goog4Request,
    )
    .map_err(ErrorKind::CredentialScope)?;
    let signed_url = SignedUrl::new(
        &credential_scope,
        active_datetime,
        Expiration::try_from(expiration).map_err(ErrorKind::Expiration)?,
        &service_account_client_email,
        &service_account_private_key,
        request,
    )
    .map_err(ErrorKind::SignedUrl)?;
    Ok(String::from(signed_url))
}
