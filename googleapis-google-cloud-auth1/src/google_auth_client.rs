use http::StatusCode;

#[derive(Debug, thiserror::Error)]
#[error("GoogleAuthClient error: {kind}")]
pub struct Error {
    #[from]
    kind: ErrorKind,
}

#[derive(Debug, thiserror::Error)]
enum ErrorKind {
    #[error("auth: {0}")]
    Auth(#[from] google_cloud_auth::Error),
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),
    #[error("status code: {0} {1}")]
    Status(StatusCode, String),
}

#[derive(Clone)]
pub struct GoogleAuthClient {
    client: reqwest::Client,
    credential: Option<google_cloud_auth::Credential>,
}

impl GoogleAuthClient {
    pub fn new(credential: Option<google_cloud_auth::Credential>) -> Self {
        Self {
            client: reqwest::Client::new(),
            credential,
        }
    }

    pub async fn send<T: Into<reqwest::Body>>(
        &self,
        request: http::Request<T>,
    ) -> Result<reqwest::Response, Error> {
        Ok(self.send_inner(request).await?)
    }

    async fn send_inner<T: Into<reqwest::Body>>(
        &self,
        mut request: http::Request<T>,
    ) -> Result<reqwest::Response, ErrorKind> {
        match &self.credential {
            None => {
                // do nothing
            }
            Some(credential) => {
                let access_token = credential.access_token().await?;
                request.headers_mut().insert(
                    http::header::AUTHORIZATION,
                    http::HeaderValue::from_str(&format!("Bearer {}", access_token.value))
                        .expect("access_token value to be valid header value"),
                );
            }
        }
        let request = reqwest::Request::try_from(request)?;
        let response = self.client.execute(request).await?;
        if response.status().is_success() {
            Ok(response)
        } else {
            Err(ErrorKind::Status(response.status(), response.text().await?))
        }
    }
}
