use md5::Md5;
use sha2::{Digest, Sha256, Sha512};

use crate::text::helpers;

/// Compute a checksum of a file or stdin.
///
/// * `algo` — one of `md5`, `sha256`, `sha512` (default `sha256`).
pub fn checksum(input: Option<&str>, algo: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = helpers::read_input(input)?;

    let (name, hex) = match algo.to_lowercase().as_str() {
        "md5" => {
            let mut h = Md5::new();
            h.update(&data);
            ("md5", format!("{:x}", h.finalize()))
        }
        "sha256" => {
            let mut h = Sha256::new();
            h.update(&data);
            ("sha256", format!("{:x}", h.finalize()))
        }
        "sha512" => {
            let mut h = Sha512::new();
            h.update(&data);
            ("sha512", format!("{:x}", h.finalize()))
        }
        other => {
            return Err(
                format!("Unsupported algorithm '{other}'. Valid: md5, sha256, sha512").into(),
            )
        }
    };

    println!("{hex}  {name}  {}", helpers::source_name(input));
    Ok(())
}
