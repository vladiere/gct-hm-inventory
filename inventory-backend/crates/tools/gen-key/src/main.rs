use lib_utils::b64::{self, b64u_encode};
use rand::RngCore;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<()> {
    let mut key = [0u8; 64];
    rand::thread_rng().fill_bytes(&mut key);
    println!("\nGenerated key from rand::thread_rng():\n{key:?}");

    let b64u = b64u_encode(key);
    println!("\nGenerated key from b64u_encode():\nkey: {b64u:?}");
    Ok(())
}
