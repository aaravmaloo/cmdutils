use std::path::Path;
use std::ffi::OsStr;
use std::fs::File;

use oxipng::{InFile, OutFile, Options as PngOptions};

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

    let is_png = input_ext == "png";
    if !is_png && !matches!(input_ext.as_str(), "jpg" | "jpeg") {
        return Err(format!("Unsupported format: '.{input_ext}'. Supported: png, jpg, jpeg").into());
    }

    let original_size = std::fs::metadata(input_path)?.len();
    let img = image::open(input_path)?;

    let stem = input_path
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("output");
    let ext = if input_ext == "jpeg" { "jpg" } else { &input_ext };
    let output_path = input_path.with_file_name(format!("{}_compressed.{ext}", stem));

    if is_png {
        // PNG: don't allow quality percentage (lossless, always max compression)
        if quality_str.is_some() {
            return Err(
                "PNG compression is lossless — quality setting is not applicable.\n\
                 Use: cmdutils image compress <input.png>\n\
                 (JPEG compression accepts a quality value 1-100)"
                    .into(),
            );
        }
        // Save the PNG first, then optimize with oxipng (zopfli + max level)
        img.save(&output_path)?;
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
    } else {
        // JPEG: quality is required
        let quality: u8 = match quality_str {
            Some(q) => q.parse().map_err(|_| {
                format!("Invalid quality value: '{q}'. Must be a number 1-100")
            })?,
            None => {
                return Err(
                    "JPEG compression requires a quality value 1-100.\n\
                     Use: cmdutils image compress <input.jpg> <quality>"
                        .into(),
                );
            }
        };

        if !(1..=100).contains(&quality) {
            return Err(format!("Quality must be between 1 and 100, got {quality}").into());
        }

        let rgb = img.to_rgb8();
        let mut file = File::create(&output_path)?;
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut file, quality);
        encoder.encode(
            rgb.as_raw(),
            rgb.width(),
            rgb.height(),
            image::ExtendedColorType::Rgb8,
        )?;
    }

    let new_size = std::fs::metadata(&output_path)?.len();
    // Show percentage if known
    if original_size > 0 {
        let diff = original_size.abs_diff(new_size);
        let pct = diff as f64 / original_size as f64 * 100.0;
        let direction = if new_size < original_size { "saved" } else { "grew" };
        println!(
            "Compressed {} ({}B → {}B, {} {:.1}%): {}",
            if is_png { "PNG" } else { "JPEG" },
            original_size,
            new_size,
            direction,
            pct,
            output_path.display()
        );
        if new_size > original_size {
            println!(
                "  ⚠  Output is {:.1}% larger — source may already be well-optimized",
                pct
            );
        }
    } else {
        println!(
            "Compressed {} ({}B → {}B): {}",
            if is_png { "PNG" } else { "JPEG" },
            original_size,
            new_size,
            output_path.display()
        );
    }

    Ok(())
}
