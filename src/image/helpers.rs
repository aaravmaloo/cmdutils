use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

use image::codecs::jpeg::JpegEncoder;
use image::{DynamicImage, ExtendedColorType, RgbaImage};

/// Human-readable format name for display in output messages.
pub fn format_name(ext: &str) -> &'static str {
    match ext {
        "png" | "PNG" => "PNG",
        "jpg" | "JPG" | "jpeg" | "JPEG" => "JPEG",
        "webp" | "WEBP" => "WebP",
        "gif" | "GIF" => "GIF",
        "bmp" | "BMP" => "BMP",
        "ico" | "ICO" => "ICO",
        "tiff" | "TIFF" | "tif" | "TIF" => "TIFF",
        "avif" | "AVIF" => "AVIF",
        "pnm" | "PNM" => "PNM",
        "qoi" | "QOI" => "QOI",
        "tga" | "TGA" => "TGA",
        "exr" | "EXR" => "OpenEXR",
        "ff" | "FF" => "Farbfeld",
        "dds" | "DDS" => "DDS",
        "hdr" | "HDR" => "HDR",
        "svg" | "SVG" => "SVG",
        _ => "Unknown",
    }
}

/// All output formats we can encode to (lowercase).
pub const OUTPUT_FORMATS: &[&str] = &[
    "png", "jpg", "jpeg", "webp", "bmp", "gif", "ico", "tiff", "tif", "avif", "pnm", "qoi", "tga",
    "exr", "ff", "dds",
];

/// All input formats we can decode (lowercase). SVG is input-only.
pub const INPUT_FORMATS: &[&str] = &[
    "png", "jpg", "jpeg", "webp", "bmp", "gif", "ico", "tiff", "tif", "avif", "pnm", "qoi", "tga",
    "exr", "ff", "dds", "svg",
];

/// Check whether `ext` is a recognised output format.
pub fn is_supported_output(ext: &str) -> bool {
    let ext = ext.to_lowercase();
    OUTPUT_FORMATS.contains(&ext.as_str())
}

/// Check whether `ext` is a recognised input format.
pub fn is_supported_input(ext: &str) -> bool {
    let ext = ext.to_lowercase();
    INPUT_FORMATS.contains(&ext.as_str())
}

/// Normalise an extension to a canonical form for comparison.
fn normalize(s: &str) -> String {
    match s.to_lowercase().as_str() {
        "jpeg" => "jpg".to_string(),
        "tif" => "tiff".to_string(),
        other => other.to_string(),
    }
}

/// Check whether two extensions represent the same image format.
pub fn same_format(a: &str, b: &str) -> bool {
    normalize(a) == normalize(b)
}

/// Load an image from any supported format, including SVG (rendered via resvg).
pub fn load_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let path = path.as_ref();
    let ext = path
        .extension()
        .and_then(OsStr::to_str)
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    if ext == "svg" {
        return load_svg(path);
    }

    // All other formats are handled by the `image` crate.
    let img = image::open(path)?;
    Ok(img)
}

/// Render an SVG file to a raster DynamicImage.
fn load_svg(path: &Path) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let svg_data = std::fs::read(path)?;
    let opt = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_data(&svg_data, &opt)?;
    let size = tree.size();
    let pixmap_size = size.to_int_size();
    let width = pixmap_size.width();
    let height = pixmap_size.height();

    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| format!("Failed to allocate pixmap for SVG ({width}x{height})"))?;

    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::default(),
        &mut pixmap.as_mut(),
    );

    let img = DynamicImage::ImageRgba8(
        RgbaImage::from_raw(width, height, pixmap.data().to_vec()).ok_or(format!(
            "Failed to create image from rendered SVG ({width}x{height})"
        ))?,
    );

    Ok(img)
}

/// Save an image with the best-possible encoding for the target format.
///
/// * `img` — the image to save.
/// * `output_path` — destination path (extension determines format).
/// * `quality` — for lossy formats (JPEG). `None` means use a sensible default.
///   WebP in `image` 0.25 only supports lossless encoding; quality is ignored.
pub fn save_with_quality(
    img: &DynamicImage,
    output_path: &Path,
    quality: Option<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let ext = output_path
        .extension()
        .and_then(OsStr::to_str)
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "jpg" | "jpeg" => save_jpeg(img, output_path, quality.unwrap_or(90)),
        "png" => save_png_optimized(img, output_path),
        "webp" => save_webp(img, output_path),
        _ => {
            // All other formats: let `image` crate auto-detect from path.
            img.save(output_path)?;
            Ok(())
        }
    }
}

/// Encode as JPEG with the given quality (1–100).
fn save_jpeg(
    img: &DynamicImage,
    output_path: &Path,
    quality: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let quality = quality.clamp(1, 100);
    let rgb = img.to_rgb8();
    let file = File::create(output_path)?;
    let mut encoder = JpegEncoder::new_with_quality(file, quality);
    encoder.encode(
        rgb.as_raw(),
        rgb.width(),
        rgb.height(),
        ExtendedColorType::Rgb8,
    )?;
    Ok(())
}

/// Save as PNG then apply oxipng max compression.
fn save_png_optimized(
    img: &DynamicImage,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    img.save(output_path)?;
    let mut opts = oxipng::Options::max_compression();
    opts.force = true;
    oxipng::optimize(
        &oxipng::InFile::Path(output_path.to_path_buf()),
        &oxipng::OutFile::Path {
            path: Some(output_path.to_path_buf()),
            preserve_attrs: false,
        },
        &opts,
    )?;
    Ok(())
}

/// Encode as WebP (lossless only — `image` 0.25 does not support lossy WebP).
fn save_webp(img: &DynamicImage, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use image::codecs::webp::WebPEncoder;

    let has_alpha = img.color().has_alpha();
    let file = File::create(output_path)?;
    let encoder = WebPEncoder::new_lossless(file);

    if has_alpha {
        let rgba = img.to_rgba8();
        encoder.encode(
            rgba.as_raw(),
            rgba.width(),
            rgba.height(),
            ExtendedColorType::Rgba8,
        )?;
    } else {
        let rgb = img.to_rgb8();
        encoder.encode(
            rgb.as_raw(),
            rgb.width(),
            rgb.height(),
            ExtendedColorType::Rgb8,
        )?;
    }
    Ok(())
}
