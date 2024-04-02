#[test]
fn test_policy_document() -> anyhow::Result<()> {
    use std::str::FromStr as _;
    use v4_sign::policy_document::{Condition, Expiration, Field, PolicyDocument, Value};

    let json = r#"
{
  "conditions": [
    ["starts-with", "$key", ""],
    {"bucket": "travel-maps"},
    {"success_action_redirect": "http://www.example.com/success_notification.html"},
    ["eq", "$Content-Type", "image/jpeg"],
    ["content-length-range", 0, 1000000],
    {"x-goog-algorithm": "GOOG4-RSA-SHA256"},
    {"x-goog-credential": "example_account@example_project.iam.gserviceaccount.com/20191102/us-central1/storage/goog4_request"},
    {"x-goog-date": "20191102T043530Z"}
  ],
  "expiration": "2020-06-16T11:11:11Z"
}
"#.trim();
    let policy_document: PolicyDocument = serde_json::from_str(json)?;
    assert_eq!(
            policy_document,
            PolicyDocument {
                conditions: vec![
                    Condition::StartsWith(
                        Field::new("key")?,
                        Value::new("")
                    ),
                    Condition::ExactMatching(
                        Field::new("bucket")?,
                        Value::new("travel-maps")
                    ),
                    Condition::ExactMatching(
                        Field::new("success_action_redirect")?,
                        Value::new("http://www.example.com/success_notification.html")
                    ),
                    Condition::ExactMatching(
                        Field::new("Content-Type")?,
                        Value::new("image/jpeg")
                    ),
                    Condition::ContentLengthRange(0, 1000000),
                    Condition::ExactMatching(
                        Field::new("x-goog-algorithm")?,
                        Value::new("GOOG4-RSA-SHA256")
                    ),
                    Condition::ExactMatching(
                        Field::new("x-goog-credential")?,
                        Value::new("example_account@example_project.iam.gserviceaccount.com/20191102/us-central1/storage/goog4_request")
                    ),
                    Condition::ExactMatching(
                        Field::new("x-goog-date")?,
                        Value::new("20191102T043530Z")
                    )
                ],
                expiration: Expiration::from_str("2020-06-16T11:11:11Z")?,
            }
        );
    Ok(())
}

#[test]
fn test_impl_serialize_policy_document() -> anyhow::Result<()> {
    use std::str::FromStr as _;
    use v4_sign::policy_document::{Condition, Expiration, Field, PolicyDocument, Value};

    let policy_document = PolicyDocument {
                conditions: vec![
                    Condition::StartsWith(
                        Field::new("key")?,
                        Value::new("")
                    ),
                    Condition::ExactMatching(
                        Field::new("bucket")?,
                        Value::new("travel-maps")
                    ),
                    Condition::ExactMatching(
                        Field::new("success_action_redirect")?,
                        Value::new("http://www.example.com/success_notification.html")
                    ),
                    Condition::ExactMatching(
                        Field::new("Content-Type")?,
                        Value::new("image/jpeg")
                    ),
                    Condition::ContentLengthRange(0, 1000000),
                    Condition::ExactMatching(
                        Field::new("x-goog-algorithm")?,
                        Value::new("GOOG4-RSA-SHA256")
                    ),
                    Condition::ExactMatching(
                        Field::new("x-goog-credential")?,
                        Value::new("example_account@example_project.iam.gserviceaccount.com/20191102/us-central1/storage/goog4_request")
                    ),
                    Condition::ExactMatching(
                        Field::new("x-goog-date")?,
                        Value::new("20191102T043530Z")
                    )
                ],
                expiration: Expiration::from_str("2020-06-16T11:11:11Z")?,
            };
    assert_eq!(
            serde_json::to_string_pretty(&policy_document)?,
r#"
{
  "conditions": [
    [
      "starts-with",
      "$key",
      ""
    ],
    [
      "eq",
      "$bucket",
      "travel-maps"
    ],
    [
      "eq",
      "$success_action_redirect",
      "http://www.example.com/success_notification.html"
    ],
    [
      "eq",
      "$Content-Type",
      "image/jpeg"
    ],
    [
      "content-length-range",
      0,
      1000000
    ],
    [
      "eq",
      "$x-goog-algorithm",
      "GOOG4-RSA-SHA256"
    ],
    [
      "eq",
      "$x-goog-credential",
      "example_account@example_project.iam.gserviceaccount.com/20191102/us-central1/storage/goog4_request"
    ],
    [
      "eq",
      "$x-goog-date",
      "20191102T043530Z"
    ]
  ],
  "expiration": "2020-06-16T11:11:11Z"
}
"#.trim());
    Ok(())
}
