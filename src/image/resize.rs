use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

use oxipng::{InFile, Options as PngOptions, OutFile};

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

    let original_size = std::fs::metadata(input_path)?.len();
    let img = image::open(input_path)?;
    let resized = img.resize_exact(width, height, image::imageops::Lanczos3);

    let stem = input_path
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("output");
    let ext = input_path
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("png");
    let is_jpeg = matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg");
    let output_path = input_path.with_file_name(format!("{}_resized.{ext}", stem));

    if is_jpeg {
        // JPEG output: high-quality encoding (quality 90)
        let rgb = resized.to_rgb8();
        let mut file = File::create(&output_path)?;
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut file, 90);
        encoder.encode(
            rgb.as_raw(),
            rgb.width(),
            rgb.height(),
            image::ExtendedColorType::Rgb8,
        )?;
    } else {
        // PNG output: save then optimize with oxipng
        resized.save(&output_path)?;
        let mut opts = PngOptions::max_compression();
        opts.force = true;
        oxipng::optimize(
            &InFile::Path(output_path.clone()),
            &OutFile::Path {
                path: Some(output_path.clone()),
                preserve_attrs: false,
            },
            &opts,
        )?;
    }

    let new_size = std::fs::metadata(&output_path)?.len();
    let pct = if original_size > 0 {
        (original_size as f64 - new_size as f64) / original_size as f64 * 100.0
    } else {
        0.0
    };

    println!(
        "Resized {}x{} → {}x{} ({}B → {}B, {:.1}%): {}",
        img.width(),
        img.height(),
        width,
        height,
        original_size,
        new_size,
        pct,
        output_path.display()
    );

    Ok(())
}
