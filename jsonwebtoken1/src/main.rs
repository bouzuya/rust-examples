use serde::{Deserialize, Serialize};

// /// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//     sub: String,
//     company: String,
//     exp: usize,
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_claims = Claims {
        aud: "my_audience".into(),
        exp: 10000000000,
        iat: 1000000000,
        iss: "my_issuer".into(),
        nbf: 1000000000,
        sub: "my_subject".into(),
    };
    let encoded = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256),
        &my_claims,
        &jsonwebtoken::EncodingKey::from_rsa_pem(b"-----BEGIN PRIVATE KEY-----")?,
    )?;

    let _decoded = jsonwebtoken::decode::<Claims>(
        &encoded,
        &jsonwebtoken::DecodingKey::from_rsa_pem(b"-----BEGIN PUBLIC KEY-----")?,
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256),
    )?;
    Ok(())
}
