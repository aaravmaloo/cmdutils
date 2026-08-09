use base64::Engine;

use crate::text::helpers;

/// Base64-encode a file or stdin.
pub fn encode(input: Option<&str>, output: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let data = helpers::read_input(input)?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(&data);
    helpers::write_output(output, encoded.as_bytes(), helpers::source_name(input))
}

/// Base64-decode a file or stdin.
pub fn decode(input: Option<&str>, output: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let data = helpers::read_input(input)?;
    let text =
        String::from_utf8(data).map_err(|_| "Input is not valid UTF-8 base64 text".to_string())?;
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(text.trim())
        .map_err(|e| format!("Invalid base64 input: {e}"))?;
    helpers::write_output(output, &decoded, helpers::source_name(input))
}
