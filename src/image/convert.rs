use std::path::Path;
use std::ffi::OsStr;
use std::fs::File;

use oxipng::{InFile, OutFile, Options as PngOptions};

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

    let output_ext = match target_format {
        "jpeg" | "jpg" => "jpg",
        "png" => "png",
        other => {
            return Err(
                format!("Unsupported output format: '{other}'. Supported: jpg, jpeg, png").into(),
            )
        }
    };

    let input_format_name = match input_ext.as_str() {
        "png" => "PNG",
        "jpg" | "jpeg" => "JPEG",
        _ => {
            return Err(
                format!("Unsupported input format: '.{input_ext}'. Supported: png, jpg, jpeg")
                    .into(),
            )
        }
    };

    // Check for same-format conversion
    if input_ext == output_ext
        || (matches!(input_ext.as_str(), "jpg" | "jpeg") && output_ext == "jpg")
    {
        return Err("Input and output formats are the same; nothing to do.".into());
    }

    let original_size = std::fs::metadata(input_path)?.len();
    let img = image::open(input_path)?;
    let output_path = input_path.with_extension(output_ext);
    let output_format_name = match output_ext {
        "jpg" => "JPEG",
        "png" => "PNG",
        _ => unreachable!(),
    };

    if output_ext == "jpg" {
        // PNG → JPEG: use high-quality JPEG encoding (quality 90)
        let rgb = img.to_rgb8();
        let mut file = File::create(&output_path)?;
        let mut encoder =
            image::codecs::jpeg::JpegEncoder::new_with_quality(&mut file, 90);
        encoder.encode(
            rgb.as_raw(),
            rgb.width(),
            rgb.height(),
            image::ExtendedColorType::Rgb8,
        )?;
    } else {
        // JPEG → PNG: save as PNG then optimize with oxipng
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
    }

    let new_size = std::fs::metadata(&output_path)?.len();
    let pct = if original_size > 0 {
        ((original_size as f64 - new_size as f64) / original_size as f64 * 100.0)
    } else {
        0.0
    };

    println!(
        "Converted {} → {} ({}B → {}B, {:.1}%): {}",
        input_format_name,
        output_format_name,
        original_size,
        new_size,
        pct,
        output_path.display()
    );

    Ok(())
}
