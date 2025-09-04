use rustpass::crypto;

#[test]
fn encrypt_decrypt_roundtrip() {
    let master = "test-master-password";
    let salt = b"0123456789abcdef";
    let key = crypto::derive_key(master, salt);
    let plaintext = b"super secret data";
    let (ciphertext, nonce) = crypto::encrypt(&key, plaintext);
    let decrypted = crypto::decrypt(&key, &ciphertext, &nonce).expect("Decryption should succeed");
    assert_eq!(decrypted, plaintext);
}

#[test]
fn decrypt_with_wrong_password_fails() {
    let master = "test-master-password";
    let salt = b"0123456789abcdef";
    let key = crypto::derive_key(master, salt);
    let plaintext = b"super secret data";
    let (ciphertext, nonce) = crypto::encrypt(&key, plaintext);

    let wrong_key = crypto::derive_key("wrong-password", salt);
    let result = crypto::decrypt(&wrong_key, &ciphertext, &nonce);
    assert!(result.is_err(), "Decryption with wrong password should fail");
}
