use std::ffi::OsStr;
use std::path::Path;

use crate::image::helpers;

pub fn convert(input: &str, target_format: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = Path::new(input);

    if !input_path.exists() {
        return Err(format!("Input file not found: {input}").into());
    }

    let input_ext = input_path
        .extension()
        .and_then(OsStr::to_str)
        .ok_or_else(|| format!("Could not determine extension for: {input}"))?
        .to_lowercase();

    // Validate input format
    if !helpers::is_supported_input(&input_ext) {
        let supported = helpers::INPUT_FORMATS.join(", ");
        return Err(format!(
            "Unsupported input format: '.{input_ext}'. Supported: {supported}"
        )
        .into());
    }

    // Validate and normalize output format
    let target_lower = target_format.to_lowercase();
    let output_ext = match target_lower.as_str() {
        "jpg" | "jpeg" => "jpg",
        "tif" => "tiff",
        other => {
            if helpers::is_supported_output(other) {
                other
            } else {
                let supported = helpers::OUTPUT_FORMATS.join(", ");
                return Err(format!(
                    "Unsupported output format: '{target_format}'. Supported: {supported}"
                )
                .into());
            }
        }
    };

    // Check for same-format conversion
    if helpers::same_format(&input_ext, output_ext) {
        return Err("Input and output formats are the same; nothing to do.".into());
    }

    let original_size = std::fs::metadata(input_path)?.len();
    let img = helpers::load_image(input_path)?;
    let output_path = input_path.with_extension(output_ext);

    // Save with format-appropriate encoding
    helpers::save_with_quality(&img, &output_path, None)?;

    let new_size = std::fs::metadata(&output_path)?.len();
    let pct = if original_size > 0 {
        (original_size as f64 - new_size as f64) / original_size as f64 * 100.0
    } else {
        0.0
    };

    let input_name = helpers::format_name(&input_ext);
    let output_name = helpers::format_name(output_ext);

    println!(
        "Converted {input_name} → {output_name} ({}B → {}B, {pct:.1}%): {}",
        original_size,
        new_size,
        output_path.display()
    );

    Ok(())
}
