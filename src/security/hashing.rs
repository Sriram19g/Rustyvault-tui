use argon2::Argon2;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use rand::rngs::OsRng;

pub fn verify_password(mk_hash: &String, ds: &String, pass: &String) -> bool {
    let mk = pass.to_owned() + ds;
    let mk = mk.into_bytes();

    let parsed_hash = PasswordHash::new(&mk_hash).unwrap();
    Argon2::default().verify_password(&mk, &parsed_hash).is_ok()
}

pub fn generate_key_derivation(pass: &String, ds: &String) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mk = pass.to_owned() + ds;
    let mk = mk.into_bytes();
    Argon2::default()
        .hash_password_into(&mk, ds.as_bytes(), &mut output)
        .unwrap();
    output
}
