use crate::image::helpers;

/// Crop an image to a region given as `WxH+X+Y` (e.g. `800x600+100+50`).
pub fn crop(input: &str, geometry: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (dims, offset) = geometry.split_once('+').ok_or_else(|| {
        format!("Invalid crop geometry '{geometry}'. Use WxH+X+Y, e.g. 800x600+100+50")
    })?;
    let (w_s, h_s) = dims
        .split_once('x')
        .or_else(|| dims.split_once('X'))
        .ok_or_else(|| {
            format!("Invalid crop geometry '{geometry}'. Use WxH+X+Y, e.g. 800x600+100+50")
        })?;
    let (x_s, y_s) = offset.split_once('+').ok_or_else(|| {
        format!("Invalid crop geometry '{geometry}'. Use WxH+X+Y, e.g. 800x600+100+50")
    })?;

    let width: u32 = w_s.parse().map_err(|_| format!("Invalid width: '{w_s}'"))?;
    let height: u32 = h_s
        .parse()
        .map_err(|_| format!("Invalid height: '{h_s}'"))?;
    let x: u32 = x_s
        .parse()
        .map_err(|_| format!("Invalid x offset: '{x_s}'"))?;
    let y: u32 = y_s
        .parse()
        .map_err(|_| format!("Invalid y offset: '{y_s}'"))?;

    if width == 0 || height == 0 {
        return Err("Crop width and height must be greater than 0".into());
    }

    let (input_path, mut img, input_ext) = helpers::load_validated(input)?;
    let (img_w, img_h) = (img.width(), img.height());

    if x + width > img_w || y + height > img_h {
        return Err(format!(
            "Crop region {width}x{height}+{x}+{y} exceeds image bounds ({img_w}x{img_h})"
        )
        .into());
    }

    let cropped = img.crop(x, y, width, height);
    let ext = helpers::output_ext(&input_ext);
    let output_path = helpers::suffixed_path(&input_path, &ext, "cropped");

    helpers::save_with_quality(&cropped, &output_path, None)?;

    let fmt_name = helpers::format_name(&ext);
    println!(
        "Cropped {fmt_name} {img_w}x{img_h} → {width}x{height} at +{x}+{y}: {}",
        output_path.display()
    );

    Ok(())
}
