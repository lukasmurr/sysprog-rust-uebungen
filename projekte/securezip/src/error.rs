use thiserror::Error;

#[derive(Error, Debug)]
pub enum SecureZipError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Compression error: {0}")]
    Compression(String),

    #[error("Cryptographic error: {0}")]
    Crypto(String),

    #[error("Archive error: {0}")]
    Archive(String),

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Unknown error: {0}")]
    Unknown(String),
}
