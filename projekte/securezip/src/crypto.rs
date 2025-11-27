// TODO [Lukas]: Implement key derivation function (e.g., PBKDF2).
// Function signature: `derive_key(password: &str, salt: &[u8]) -> Key`

// TODO [Lukas]: Implement encryption logic.
// Function signature: `encrypt(data: &[u8], password: &str) -> Result<Vec<u8>>`
// - Generate a random salt.
// - Derive key.
// - Generate a random nonce.
// - Encrypt data using AES-GCM.
// - Prepend salt and nonce to the ciphertext.

// TODO [Lukas]: Implement decryption logic.
// Function signature: `decrypt(data: &[u8], password: &str) -> Result<Vec<u8>>`
// - Extract salt and nonce from the beginning of the data.
// - Derive key.
// - Decrypt data using AES-GCM.
