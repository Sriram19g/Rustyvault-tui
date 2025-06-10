use base64::{Engine as _, engine::general_purpose};
use chacha20poly1305::{
    Key, XChaCha20Poly1305, XNonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};

pub fn encrypt(plaintext: &str, key_bytes: &[u8; 32]) -> String {
    let pass = plaintext.as_bytes();
    let key = Key::from_slice(key_bytes);
    let cipher = XChaCha20Poly1305::new(key);
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext = cipher.encrypt(&nonce, pass.as_ref()).unwrap();
    let mut combined = nonce.to_vec();
    combined.extend(ciphertext);

    let ciphertext = general_purpose::STANDARD.encode(&combined);
    ciphertext
}

pub fn decrypt(b64_ciphertext: &String, key_bytes: &[u8; 32]) -> String {
    let key = Key::from_slice(key_bytes);
    let cipher = XChaCha20Poly1305::new(key);
    let data = general_purpose::STANDARD.decode(&b64_ciphertext).unwrap();
    let (nonce_bytes, ciphertext) = data.split_at(24);
    let nonce = XNonce::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
    let plaintext = String::from_utf8(plaintext).unwrap();
    plaintext
}
