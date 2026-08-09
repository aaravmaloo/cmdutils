use std::ffi::OsStr;
use std::path::Path;

use crate::image::helpers;

pub fn resize(input: &str, dimensions: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = Path::new(input);

    if !input_path.exists() {
        return Err(format!("Input file not found: {input}").into());
    }

    let (width, height) = dimensions
        .split_once('x')
        .or_else(|| dimensions.split_once('X'))
        .ok_or_else(|| format!("Invalid dimensions '{dimensions}'. Use format like 800x600"))?;

    let width: u32 = width
        .parse()
        .map_err(|_| format!("Invalid width: '{width}'"))?;
    let height: u32 = height
        .parse()
        .map_err(|_| format!("Invalid height: '{height}'"))?;

    let input_ext = input_path
        .extension()
        .and_then(OsStr::to_str)
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    // Validate input format
    if !helpers::is_supported_input(&input_ext) {
        let supported = helpers::INPUT_FORMATS.join(", ");
        return Err(format!("Unsupported format: '.{input_ext}'. Supported: {supported}").into());
    }

    let original_size = std::fs::metadata(input_path)?.len();
    let img = helpers::load_image(input_path)?;
    let resized = img.resize_exact(width, height, image::imageops::Lanczos3);

    // Normalize extension for output (SVG gets rasterized → PNG by default)
    let ext = helpers::output_ext(&input_ext);
    let output_path = helpers::suffixed_path(input_path, &ext, "resized");

    // SVG input was already rasterized above; for other formats, save with optimization.
    helpers::save_with_quality(&resized, &output_path, None)?;

    let new_size = std::fs::metadata(&output_path)?.len();
    let pct = if original_size > 0 {
        (original_size as f64 - new_size as f64) / original_size as f64 * 100.0
    } else {
        0.0
    };

    let fmt_name = helpers::format_name(&ext);

    println!(
        "Resized {fmt_name} {}x{} → {width}x{height} ({}B → {}B, {pct:.1}%): {}",
        img.width(),
        img.height(),
        original_size,
        new_size,
        output_path.display()
    );

    Ok(())
}
