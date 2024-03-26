use std::collections::{BTreeMap, BTreeSet};

// <https://cloud.google.com/storage/docs/authentication/canonical-requests>
fn canonical_request(request: http::Request<()>) -> String {
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
    let canonical_query_string = {
        let mut query_pairs = url::Url::parse(request.uri().to_string().as_str())
            .expect("uri to be valid")
            .query_pairs()
            .map(|(k, v)| format!("{}={}", percent_encode(&k), percent_encode(&v)))
            .collect::<Vec<String>>();
        query_pairs.sort();
        query_pairs.join("&")
    };
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
            canonical_request(request),
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
                let date = date_time.format("%Y%m%d").to_string();
                let location = "us-central1";
                let service = "storage";
                let request_type = "goog4_request";
                format!("{date}/{location}/{service}/{request_type}")
            };
            let x_goog_date = date_time.format("%Y%m%dT%H%M%SZ").to_string();
            let mut url1 = url::Url::parse(request.uri().to_string().as_str())?;
            let url_required_query_string_parameters_sadded = url1
                .query_pairs_mut()
                .append_pair("X-Goog-Algorithm", "GOOG4-RSA-SHA256")
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
}
