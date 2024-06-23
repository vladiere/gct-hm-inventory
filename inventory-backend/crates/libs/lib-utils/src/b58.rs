// region: ---- Error boilerplate

use base58::{FromBase58, FromBase58Error, ToBase58};
use uuid::Uuid;

pub fn b58() -> String {
    let uuid = Uuid::now_v7();
    uuid.as_bytes().to_base58()
}

pub fn b58_encoding(content: &str) -> Result<Vec<u8>> {
    content
        .from_base58()
        .map_err(|ex| Error::FailToB58Encoding(ex))
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailToB58Encoding(FromBase58Error),
    FailToDecodeB58,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: ---- Error boilerplate
