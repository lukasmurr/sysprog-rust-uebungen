use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use pbkdf2::pbkdf2;
use rand::{RngCore, rngs::OsRng};
use sha2::Sha256;

use crate::error::SecureZipError;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;
const ITERATIONS: u32 = 600_000;

// [Lukas]
fn derive_key(password: &str, salt: &[u8]) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    let _ = pbkdf2::<hmac::Hmac<Sha256>>(password.as_bytes(), salt, ITERATIONS, &mut key);
    key
}

pub fn encrypt(data: &[u8], password: &str) -> Result<Vec<u8>, SecureZipError> {
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);

    let key = derive_key(password, &salt);
    let cipher = Aes256Gcm::new(&key.into());

    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|e| SecureZipError::Crypto(format!("Encryption failed: {}", e)))?;

    let mut result = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

// [Lukas]
pub fn decrypt(data: &[u8], password: &str) -> Result<Vec<u8>, SecureZipError> {
    if data.len() < SALT_LEN + NONCE_LEN {
        return Err(SecureZipError::Crypto(
            "Data too short to contain salt and nonce".to_string(),
        ));
    }

    let (salt, rest) = data.split_at(SALT_LEN);
    let (nonce_bytes, ciphertext) = rest.split_at(NONCE_LEN);

    let key = derive_key(password, salt);
    let cipher = Aes256Gcm::new(&key.into());
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| SecureZipError::InvalidPassword)?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let data = b"Hello, world!";
        let password = "secure_password";

        let encrypted = encrypt(data, password).expect("Encryption failed");
        let decrypted = decrypt(&encrypted, password).expect("Decryption failed");

        assert_eq!(data, &decrypted[..]);
    }

    #[test]
    fn test_decrypt_wrong_password() {
        let data = b"Secret data";
        let password = "password123";
        let wrong_password = "password321";

        let encrypted = encrypt(data, password).expect("Encryption failed");
        let result = decrypt(&encrypted, wrong_password);

        assert!(matches!(result, Err(SecureZipError::InvalidPassword)));
    }

    #[test]
    fn test_decrypt_corrupted_data() {
        let data = b"Important info";
        let password = "pass";

        let mut encrypted = encrypt(data, password).expect("Encryption failed");
        // Corrupt the last byte
        let len = encrypted.len();
        encrypted[len - 1] ^= 0xFF;

        let result = decrypt(&encrypted, password);
        assert!(matches!(result, Err(SecureZipError::InvalidPassword)));
    }

    #[test]
    fn test_short_data() {
        let data = vec![0u8; 10]; // Too short for salt + nonce
        let password = "pass";
        let result = decrypt(&data, password);
        assert!(matches!(result, Err(SecureZipError::Crypto(_))));
    }
}
