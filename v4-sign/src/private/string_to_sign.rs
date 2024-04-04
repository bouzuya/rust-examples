use super::{
    active_datetime::ActiveDatetime, canonical_request::CanonicalRequest,
    credential_scope::CredentialScope, signing_algorithm::SigningAlgorithm,
};

/// <https://cloud.google.com/storage/docs/authentication/signatures#string-to-sign>
pub struct StringToSign(String);

impl StringToSign {
    pub fn new(
        signing_algorithm: SigningAlgorithm,
        active_datetime: ActiveDatetime,
        credential_scope: &CredentialScope,
        canonical_request: CanonicalRequest,
    ) -> Self {
        let hashed_canonical_request = sha256::digest(canonical_request.to_string());
        Self(
            [
                signing_algorithm.as_str(),
                active_datetime.to_string().as_str(),
                credential_scope.to_string().as_str(),
                hashed_canonical_request.as_str(),
            ]
            .join("\n"),
        )
    }
}

impl std::fmt::Display for StringToSign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
