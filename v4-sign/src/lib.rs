mod date;
mod location;
mod request_type;
mod service;
mod signing_algorithm;

use std::{
    collections::{BTreeMap, BTreeSet},
    vec,
};

use location::Location;
use request_type::RequestType;
use signing_algorithm::SigningAlgorithm;

use crate::service::Service;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("FIXME: {0}")]
    Fixme(String),
}

// <https://cloud.google.com/storage/docs/authentication/signatures#string-to-sign>
fn construct_string_to_sign(
    request_timestamp: chrono::DateTime<chrono::Utc>,
    region: &str,
    canonical_request: &str,
) -> String {
    let signing_algorithm = SigningAlgorithm::Goog4RsaSha256;
    let active_datetime = request_timestamp.format("%Y%m%dT%H%M%SZ").to_string();
    let credential_scope = construct_credential_scope(
        date::Date::try_from(request_timestamp).expect("request_timestamp to be in a valid range"),
        Location::try_from(region).expect("region to be valid location"),
    );
    let hashed_canonical_request = sha256::digest(canonical_request);
    [
        signing_algorithm.as_str(),
        active_datetime.as_str(),
        credential_scope.as_str(),
        hashed_canonical_request.as_str(),
    ]
    .join("\n")
}

// <https://cloud.google.com/storage/docs/authentication/signatures#credential-scope>
fn construct_credential_scope(date: date::Date, location: Location) -> String {
    let service = Service::Storage.as_str();
    let request_type = RequestType::Goog4Request.as_str();
    format!("{date}/{location}/{service}/{request_type}")
}

fn add_signed_url_required_query_string_parameters(
    request: &mut http::Request<()>,
    service_account_client_email: &str,
    request_timestamp: chrono::DateTime<chrono::Utc>,
    credential_scope: &str,
    expiration: i64,
) -> Result<(), Error> {
    if !request.headers().contains_key(http::header::HOST) {
        return Err(Error::Fixme("Host header is required".to_string()));
    }
    let authorizer = service_account_client_email;
    let x_goog_date = request_timestamp.format("%Y%m%dT%H%M%SZ").to_string();
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
        .append_pair("X-Goog-Date", x_goog_date.as_str())
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

fn canonical_query_string(request: &http::Request<()>) -> String {
    let mut query_pairs = url::Url::parse(request.uri().to_string().as_str())
        .expect("uri to be valid")
        .query_pairs()
        .map(|(k, v)| format!("{}={}", percent_encode(&k), percent_encode(&v)))
        .collect::<Vec<String>>();
    query_pairs.sort();
    query_pairs.join("&")
}

// <https://cloud.google.com/storage/docs/authentication/canonical-requests>
fn canonical_request(request: &http::Request<()>) -> String {
    let http_verb = request.method().to_string();
    let path_to_resource = percent_encode(request.uri().path());
    let signed_headers = request
        .headers()
        .keys()
        .map(|k| k.to_string().to_ascii_lowercase())
        .collect::<BTreeSet<String>>()
        .into_iter()
        .collect::<Vec<String>>()
        .join(";");
    let canonical_query_string = canonical_query_string(&request);
    let canonical_headers = {
        let mut canonical_headers = BTreeMap::new();
        for (name, value) in request.headers() {
            canonical_headers
                .entry(name.to_string().to_ascii_lowercase())
                .or_insert_with(Vec::new)
                .push(value.to_str().unwrap());
        }
        canonical_headers
            .into_iter()
            .map(|(name, values)| format!("{name}:{}", values.join(",")))
            .collect::<Vec<String>>()
            .join("\n")
    };
    let payload = "UNSIGNED-PAYLOAD".to_string();
    // TODO: payload hash
    [
        http_verb,
        path_to_resource,
        canonical_query_string,
        canonical_headers,
        String::default(),
        signed_headers,
        payload,
    ]
    .join("\n")
}

fn sign(
    date_time: chrono::DateTime<chrono::Utc>,
    region: &str,
    expiration: i64,
    service_account_client_email: &str,
    service_account_private_key: &str,
    mut request: http::Request<()>,
) -> Result<String, Error> {
    let location = Location::try_from(region).expect("region to be valid location");
    let credential_scope = construct_credential_scope(
        date::Date::try_from(date_time).expect("date_time to be in a valid range"),
        location,
    );
    add_signed_url_required_query_string_parameters(
        &mut request,
        service_account_client_email,
        date_time,
        credential_scope.as_str(),
        expiration,
    )?;
    let canonical_query_string = canonical_query_string(&request);
    let canonical_request = canonical_request(&request);
    let string_to_sign = construct_string_to_sign(date_time, region, canonical_request.as_str());
    let request_signature = {
        let pkcs8 = pem::parse(service_account_private_key.as_bytes()).unwrap();
        let key_pair =
            ring::signature::RsaKeyPair::from_pkcs8(pkcs8.contents()).expect("key to be valid");
        let mut signature = vec![0; key_pair.public().modulus_len()];
        key_pair
            .sign(
                &ring::signature::RSA_PKCS1_SHA256,
                &ring::rand::SystemRandom::new(),
                string_to_sign.as_bytes(),
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
    Ok(signed_url)
}

// ?=!#$&'()*+,:;@[]"
fn percent_encode(s: &str) -> String {
    let mut encoded = String::new();
    for c in s.chars() {
        match c {
            '!' => encoded.push_str("%21"),
            '"' => encoded.push_str("%22"),
            '#' => encoded.push_str("%23"),
            '$' => encoded.push_str("%24"),
            '&' => encoded.push_str("%26"),
            '\'' => encoded.push_str("%27"),
            '(' => encoded.push_str("%28"),
            ')' => encoded.push_str("%29"),
            '*' => encoded.push_str("%2A"),
            '+' => encoded.push_str("%2B"),
            ',' => encoded.push_str("%2C"),
            ':' => encoded.push_str("%3A"),
            ';' => encoded.push_str("%3B"),
            '=' => encoded.push_str("%3D"),
            '?' => encoded.push_str("%3F"),
            '@' => encoded.push_str("%40"),
            '[' => encoded.push_str("%5B"),
            ']' => encoded.push_str("%5D"),
            _ => encoded.push(c),
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use super::*;

    #[test]
    fn test_string_to_sign() {
        // TODO:
    }

    #[test]
    fn test_add_signed_url_required_query_string_parameters() -> anyhow::Result<()> {
        // TODO: host header is required

        let date_time =
            chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339("2020-01-02T03:04:05Z")?
                .naive_utc()
                .and_utc();
        let date = date::Date::try_from(date_time)?;
        let expiration = 604800;
        let service_account_name = "service_account_name1";
        let mut request = http::Request::builder()
            .header("Host", "storage.googleapis.com")
            .header("Content-Type", "text/plain")
            .header("x-goog-meta-reviewer", "jane")
            .header("x-goog-meta-reviewer", "john")
            .method(http::Method::POST)
            .uri("https://storage.googleapis.com/example-bucket/cat-pics/tabby.jpeg?generation=1360887697105000&userProject=my-project")
            .body(())?;
        let location = Location::try_from("us-central1")?;
        let credential_scope = construct_credential_scope(date, location);
        add_signed_url_required_query_string_parameters(
            &mut request,
            service_account_name,
            date_time,
            credential_scope.as_str(),
            expiration,
        )?;
        let s = canonical_request(&request);
        assert!(s.contains("X-Goog-Algorithm=GOOG4-RSA-SHA256&X-Goog-Credential=service_account_name1/20200102/us-central1/storage/goog4_request&X-Goog-Date=20200102T030405Z&X-Goog-Expires=604800&X-Goog-SignedHeaders=content-type%3Bhost%3Bx-goog-meta-reviewer&generation=1360887697105000&userProject=my-project"));
        Ok(())
    }

    #[test]
    fn test_canonical_request() -> anyhow::Result<()> {
        let request = http::Request::builder()
            .header("Host", "storage.googleapis.com")
            .header("Content-Type", "text/plain")
            .header("x-goog-meta-reviewer", "jane")
            .header("x-goog-meta-reviewer", "john")
            .method(http::Method::POST)
            .uri("https://storage.googleapis.com/example-bucket/cat-pics/tabby.jpeg?generation=1360887697105000&userProject=my-project")
            .body(())?;
        assert_eq!(
            canonical_request(&request),
            r#"
POST
/example-bucket/cat-pics/tabby.jpeg
generation=1360887697105000&userProject=my-project
content-type:text/plain
host:storage.googleapis.com
x-goog-meta-reviewer:jane,john

content-type;host;x-goog-meta-reviewer
UNSIGNED-PAYLOAD
"#
            .trim()
        );
        Ok(())
    }

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let date_time =
            chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339("2020-01-02T03:04:05Z")?
                .naive_utc()
                .and_utc();
        let expiration = 604800;
        let service_account_name = "service_account_name1";
        let request = http::Request::builder()
            .header("Host", "storage.googleapis.com")
            .header("Content-Type", "text/plain")
            .header("x-goog-meta-reviewer", "jane")
            .header("x-goog-meta-reviewer", "john")
            .method(http::Method::POST)
            .uri("https://storage.googleapis.com/example-bucket/cat-pics/tabby.jpeg?generation=1360887697105000&userProject=my-project")
            .body("")?;

        let method = request.method().to_string();
        assert_eq!(method, "POST");

        let path_to_resource = percent_encode(request.uri().path());
        assert_eq!(path_to_resource, "/example-bucket/cat-pics/tabby.jpeg");

        let canonical_query_string = {
            let url1 = url::Url::parse(request.uri().to_string().as_str())?;
            let mut query_pairs = url1
                .query_pairs()
                .map(|(k, v)| format!("{}={}", percent_encode(&k), percent_encode(&v)))
                .collect::<Vec<String>>();
            query_pairs.sort();
            query_pairs.join("&")
        };
        assert_eq!(
            canonical_query_string,
            "generation=1360887697105000&userProject=my-project"
        );

        let canonical_query_string = {
            // required query string parameters
            let signed_headers = request
                .headers()
                .keys()
                .map(|k| k.to_string().to_ascii_lowercase())
                .collect::<BTreeSet<String>>()
                .into_iter()
                .collect::<Vec<String>>()
                .join(";");
            let authorizer = service_account_name;
            // <https://cloud.google.com/storage/docs/authentication/signatures#credential-scope>
            let credential_scope = {
                let date = date::Date::try_from(date_time)?.to_string();
                let location = Location::try_from("us-central1")?;
                let service = Service::Storage.as_str();
                let request_type = RequestType::Goog4Request.as_str();
                format!("{date}/{location}/{service}/{request_type}")
            };
            let x_goog_date = date_time.format("%Y%m%dT%H%M%SZ").to_string();
            let mut url1 = url::Url::parse(request.uri().to_string().as_str())?;
            let url_required_query_string_parameters_sadded = url1
                .query_pairs_mut()
                .append_pair(
                    "X-Goog-Algorithm",
                    SigningAlgorithm::Goog4RsaSha256.as_str(),
                )
                .append_pair(
                    "X-Goog-Credential",
                    format!("{authorizer}/{credential_scope}").as_str(),
                )
                .append_pair("X-Goog-Date", x_goog_date.as_str())
                .append_pair("X-Goog-Expires", expiration.to_string().as_str())
                .append_pair("X-Goog-SignedHeaders", signed_headers.as_str())
                .finish();
            let mut query_pairs = url_required_query_string_parameters_sadded
                .query_pairs()
                .map(|(k, v)| format!("{}={}", percent_encode(&k), percent_encode(&v)))
                .collect::<Vec<String>>();
            query_pairs.sort();
            query_pairs.join("&")
        };
        assert_eq!(
            canonical_query_string,
            "X-Goog-Algorithm=GOOG4-RSA-SHA256&X-Goog-Credential=service_account_name1/20200102/us-central1/storage/goog4_request&X-Goog-Date=20200102T030405Z&X-Goog-Expires=604800&X-Goog-SignedHeaders=content-type%3Bhost%3Bx-goog-meta-reviewer&generation=1360887697105000&userProject=my-project"
        );

        let canonical_headers = {
            let mut canonical_headers = BTreeMap::new();
            for (name, value) in request.headers() {
                canonical_headers
                    .entry(name.to_string().to_ascii_lowercase())
                    .or_insert_with(Vec::new)
                    .push(value.to_str()?);
            }
            canonical_headers
                .into_iter()
                .map(|(name, values)| format!("{name}:{}", values.join(",")))
                .collect::<Vec<String>>()
                .join("\n")
        };
        assert_eq!(
            canonical_headers,
            "content-type:text/plain\nhost:storage.googleapis.com\nx-goog-meta-reviewer:jane,john"
        );

        let payload = "UNSIGNED-PAYLOAD";
        assert_eq!(payload, "UNSIGNED-PAYLOAD");

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
    fn test_percent_encode() {
        assert_eq!(
            percent_encode(r#"?=!#$&'()*+,:;@[]""#),
            "%3F%3D%21%23%24%26%27%28%29%2A%2B%2C%3A%3B%40%5B%5D%22"
        );
    }

    #[test]
    fn test_chrono() -> anyhow::Result<()> {
        let date_time =
            chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339("2020-01-02T03:04:05Z")?
                .naive_utc()
                .and_utc();
        let date = date_time.format("%Y%m%d").to_string();
        let x_goog_date = date_time.format("%Y%m%dT%H%M%SZ").to_string();
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

        let date_time = chrono::Utc::now();
        let expiration = 604800;
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
        let signed_url = sign(
            date_time,
            &region,
            expiration,
            &service_account_client_email,
            &service_account_private_key,
            request,
        )?;

        let response = reqwest::get(signed_url).await?;
        assert_eq!(response.status().as_u16(), 200);
        assert_eq!(response.text().await?, "abc\n");

        Ok(())
    }
}
