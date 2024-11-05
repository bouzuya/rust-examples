use std::str::FromStr;

use secp256k1::{rand::rngs::OsRng, Keypair, Secp256k1, SecretKey};

fn main() {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    let (x_only_public_key, _) = public_key.x_only_public_key();
    let public_key_str = x_only_public_key.to_string();
    let secret_key_str = secret_key.display_secret().to_string();

    println!("public_key = {}", public_key_str);
    println!("secret_key = {}", secret_key_str);

    let m = "Hello, world!";

    let c_secp = Secp256k1::new();
    let c_secret_key = SecretKey::from_str(&secret_key_str).unwrap();
    let c_keypair = Keypair::from_secret_key(&c_secp, &c_secret_key);
    let sig = secp.sign_schnorr_with_rng(m.as_bytes(), &c_keypair, &mut OsRng);
    let sig_str = sig.to_string();

    println!("signature = {}", sig_str);

    // let x_only_public_key = XOnlyPublicKey::from_str(&public_key_str).unwrap();
    // println!("public_key = {}", x_only_public_key);

    x_only_public_key.verify(&secp, m.as_bytes(), &sig).unwrap();
    println!("verified");
}
