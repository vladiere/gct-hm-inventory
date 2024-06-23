pub fn b32u_encode(content: impl AsRef<[u8]>) -> String {
    base32::encode(base32::Alphabet::Crockford, content.as_ref())
}

pub fn b32u_decode(content: &str) -> Result<Vec<u8>> {
    base32::decode(base32::Alphabet::Crockford, content).ok_or(Error::FailToDecodeB32)
}

pub fn b32uhex(content: impl AsRef<[u8]>) -> String {
    data_encoding::BASE32HEX_NOPAD.encode(content.as_ref())
}

pub fn b32dnssec(content: impl AsRef<[u8]>) -> String {
    data_encoding::BASE32_DNSSEC.encode(content.as_ref())
}

pub fn b32dncurve(content: impl AsRef<[u8]>) -> String {
    data_encoding::BASE32_DNSCURVE.encode(content.as_ref())
}

// region: ---- Error boilerplate.

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailToDecodeB32,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: ---- Error boilerplate.
