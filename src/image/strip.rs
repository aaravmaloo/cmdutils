use crate::image::helpers;

/// Strip all metadata (EXIF, comments, etc.) by re-encoding the image
/// without any embedded metadata.
pub fn strip(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = std::path::Path::new(input);
    if !input_path.exists() {
        return Err(format!("Input file not found: {input}").into());
    }

    let original_size = std::fs::metadata(input)?.len();

    if std::path::Path::new(input)
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("svg"))
    {
        return Err(
            "SVG files are vector-based and have no embedded metadata to strip. \
             Use `convert` to rasterize SVG to a pixel format first."
                .into(),
        );
    }

    let (input_path, img, input_ext) = helpers::load_validated(input)?;

    let ext = helpers::output_ext(&input_ext);
    let output_path = helpers::suffixed_path(&input_path, &ext, "stripped");

    // Re-encode without metadata. JPEG output uses quality 90.
    helpers::save_with_quality(&img, &output_path, None)?;

    let new_size = std::fs::metadata(&output_path)?.len();
    let fmt_name = helpers::format_name(&ext);

    println!(
        "Stripped metadata from {fmt_name} ({}B → {}B): {}",
        original_size,
        new_size,
        output_path.display()
    );

    Ok(())
}
