use aes_gcm::KeyInit;
use argon2::{Argon2};
use rand::RngCore;
use chacha20poly1305::{
    XChaCha20Poly1305, Key, XNonce,
    aead::{Aead}
};

pub fn derive_key(master: &str, salt: &[u8]) -> [u8; 32] {
    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    argon2.hash_password_into(master.as_bytes(), salt, &mut key).expect("Key derivation failed");
    key
}

pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> (Vec<u8>, [u8; 24]) {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key));
    let mut nonce = [0u8; 24];
    rand::thread_rng().fill_bytes(&mut nonce);
    let ciphertext = cipher.encrypt(XNonce::from_slice(&nonce), plaintext).expect("Encryption failed");
    (ciphertext, nonce)
}

pub fn decrypt(key: &[u8; 32], ciphertext: &[u8], nonce: &[u8; 24]) -> anyhow::Result<Vec<u8>> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key));
    let plaintext = cipher
        .decrypt(XNonce::from_slice(nonce), ciphertext)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {:?}", e))?;
    Ok(plaintext)
}
