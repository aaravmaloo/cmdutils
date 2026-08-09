use std::ffi::OsStr;
use std::path::Path;

use crate::image::helpers;

pub fn compress(input: &str, quality_str: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
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
        return Err(format!("Unsupported format: '.{input_ext}'. Supported: {supported}").into());
    }

    // SVG is not directly compressible (it's vector); we could rasterize then compress,
    // but that changes the nature of the file. Reject with a clear message.
    if input_ext == "svg" {
        return Err(
            "SVG files are vector-based and cannot be lossily compressed. \
             Use `convert` to rasterize SVG to a pixel format first."
                .into(),
        );
    }

    let original_size = std::fs::metadata(input_path)?.len();
    let img = helpers::load_image(input_path)?;

    // Determine output extension (normalize jpeg → jpg, tif → tiff)
    let ext = helpers::output_ext(&input_ext);
    let output_path = helpers::suffixed_path(input_path, &ext, "compressed");

    // Format-specific compression logic
    match ext.as_str() {
        "png" => {
            // PNG: lossless, max compression — no quality accepted
            if quality_str.is_some() {
                return Err(
                    "PNG compression is lossless — quality setting is not applicable.\n\
                     Use: cmdutils image compress <input.png>\n\
                     (JPEG accepts a quality value 1-100)"
                        .into(),
                );
            }
            helpers::save_with_quality(&img, &output_path, None)?;
        }
        "jpg" => {
            // JPEG: quality is required
            let quality: u8 = match quality_str {
                Some(q) => q
                    .parse()
                    .map_err(|_| format!("Invalid quality value: '{q}'. Must be a number 1-100"))?,
                None => {
                    return Err("JPEG compression requires a quality value 1-100.\n\
                         Use: cmdutils image compress <input.jpg> <quality>"
                        .into());
                }
            };
            if !(1..=100).contains(&quality) {
                return Err(format!("Quality must be between 1 and 100, got {quality}").into());
            }
            helpers::save_with_quality(&img, &output_path, Some(quality))?;
        }
        "webp" => {
            // WebP: only lossless encoding is available via `image` 0.25.
            // Quality argument is accepted for compatibility but ignored.
            if let Some(q) = quality_str {
                // Validate it's a number even though we don't use it yet.
                let _: u8 = q
                    .parse()
                    .map_err(|_| format!("Invalid quality value: '{q}'. Must be a number 1-100"))?;
            }
            helpers::save_with_quality(&img, &output_path, None)?;
        }
        _ => {
            // All other formats: re-encode as-is. Quality is not applicable.
            if quality_str.is_some() {
                let fmt = helpers::format_name(&ext);
                return Err(format!(
                    "{fmt} compression does not support a quality setting.\n\
                     Use: cmdutils image compress <input.{ext}>"
                )
                .into());
            }
            helpers::save_with_quality(&img, &output_path, None)?;
        }
    }

    let new_size = std::fs::metadata(&output_path)?.len();
    let fmt_name = helpers::format_name(&ext);

    if original_size > 0 {
        let diff = original_size.abs_diff(new_size);
        let pct = diff as f64 / original_size as f64 * 100.0;
        let direction = if new_size < original_size {
            "saved"
        } else {
            "grew"
        };
        println!(
            "Compressed {fmt_name} ({}B → {}B, {direction} {pct:.1}%): {}",
            original_size,
            new_size,
            output_path.display()
        );
        if new_size > original_size {
            println!("  ⚠  Output is {pct:.1}% larger — source may already be well-optimized");
        }
    } else {
        println!(
            "Compressed {fmt_name} ({}B → {}B): {}",
            original_size,
            new_size,
            output_path.display()
        );
    }

    Ok(())
}
