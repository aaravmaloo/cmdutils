use crate::image::helpers;

/// Convert an image to grayscale.
pub fn grayscale(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (input_path, img, input_ext) = helpers::load_validated(input)?;
    let (img_w, img_h) = (img.width(), img.height());

    let gray = img.grayscale();
    let ext = helpers::output_ext(&input_ext);
    let output_path = helpers::suffixed_path(&input_path, &ext, "grayscale");

    helpers::save_with_quality(&gray, &output_path, None)?;

    let fmt_name = helpers::format_name(&ext);
    println!(
        "Grayscaled {fmt_name} {img_w}x{img_h}: {}",
        output_path.display()
    );

    Ok(())
}
