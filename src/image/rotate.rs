use crate::image::helpers;

/// Rotate an image by a multiple of 90 degrees (clockwise).
pub fn rotate(input: &str, degrees: i32) -> Result<(), Box<dyn std::error::Error>> {
    if degrees % 90 != 0 {
        return Err(format!("Rotation must be a multiple of 90 degrees, got {degrees}").into());
    }

    let normalized = degrees.rem_euclid(360);
    if normalized == 0 {
        return Err(format!("Rotation of {degrees}° is a full turn; nothing to do.").into());
    }

    let (input_path, img, input_ext) = helpers::load_validated(input)?;
    let (img_w, img_h) = (img.width(), img.height());

    // Note: `image` crate's rotate90 is counter-clockwise in practice, so we
    // map 90° clockwise → rotate270 and vice versa.
    let rotated = match normalized {
        90 => img.rotate270(), // clockwise 90°
        180 => img.rotate180(),
        270 => img.rotate90(), // clockwise 270° = counter-clockwise 90°
        _ => unreachable!(),
    };

    let ext = helpers::output_ext(&input_ext);
    let output_path = helpers::suffixed_path(&input_path, &ext, "rotated");

    helpers::save_with_quality(&rotated, &output_path, None)?;

    let fmt_name = helpers::format_name(&ext);
    println!(
        "Rotated {fmt_name} {degrees}° ({img_w}x{img_h} → {}x{}): {}",
        rotated.width(),
        rotated.height(),
        output_path.display()
    );

    Ok(())
}
