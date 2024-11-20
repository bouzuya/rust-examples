use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::Engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let password = {
        let mut bytes = [0u8; 16];
        rand_core::RngCore::fill_bytes(&mut OsRng, &mut bytes);
        base64::engine::general_purpose::STANDARD.encode(bytes)
    };
    println!("password: {:?}", password);

    let salt = SaltString::generate(&mut OsRng);
    println!("salt: {:?}", salt);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    println!("password_hash: {:?}", password_hash);

    // verify
    let parsed_hash = PasswordHash::new(&password_hash)?;
    assert!(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok());

    Ok(())
}
