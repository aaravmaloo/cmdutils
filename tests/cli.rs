use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

/// Sequence counter so concurrent test threads write distinct temp files.
static SEQ: AtomicU64 = AtomicU64::new(0);

/// Write a file atomically (write to a unique temp, then rename over the
/// target) so parallel tests never observe a partially-written shared file.
fn save_atomically(target: &Path, ext: &str, save: impl FnOnce(&Path)) {
    let n = SEQ.fetch_add(1, Ordering::Relaxed);
    let tmp = target.with_file_name(format!(".{}.{n}.{ext}", std::process::id()));
    save(&tmp);
    let _ = std::fs::rename(&tmp, target);
}

fn test_png_path() -> String {
    let dir = std::env::temp_dir().join("cmdutils_test_png.png");
    let path = dir.to_str().unwrap().to_string();
    if !dir.exists() {
        let img = image::RgbaImage::from_fn(4, 4, |x, y| {
            image::Rgba([(x * 64) as u8, (y * 64) as u8, 128, 255])
        });
        save_atomically(&dir, "png", |p| img.save(p).unwrap());
    }
    path
}

fn test_jpg_path() -> String {
    let dir = std::env::temp_dir().join("cmdutils_test_jpg.jpg");
    let path = dir.to_str().unwrap().to_string();
    if !dir.exists() {
        let img = image::RgbImage::from_fn(8, 6, |x, y| {
            image::Rgb([(x * 32) as u8, (y * 42) as u8, 200])
        });
        save_atomically(&dir, "jpg", |p| img.save(p).unwrap());
    }
    path
}

fn test_webp_path() -> String {
    let dir = std::env::temp_dir().join("cmdutils_test_webp.webp");
    let path = dir.to_str().unwrap().to_string();
    if !dir.exists() {
        let img = image::RgbaImage::from_fn(6, 4, |x, y| {
            image::Rgba([(x * 40) as u8, (y * 60) as u8, 180, 255])
        });
        save_atomically(&dir, "webp", |p| {
            // Use image::codecs::webp::WebPEncoder
            use image::codecs::webp::WebPEncoder;
            let file = std::fs::File::create(p).unwrap();
            let encoder = WebPEncoder::new_lossless(file);
            encoder
                .encode(
                    img.as_raw(),
                    img.width(),
                    img.height(),
                    image::ExtendedColorType::Rgba8,
                )
                .unwrap();
        });
    }
    path
}

fn test_bmp_path() -> String {
    let dir = std::env::temp_dir().join("cmdutils_test_bmp.bmp");
    let path = dir.to_str().unwrap().to_string();
    if !dir.exists() {
        let img = image::RgbImage::from_fn(5, 5, |x, y| {
            image::Rgb([(x * 50) as u8, (y * 50) as u8, 100])
        });
        save_atomically(&dir, "bmp", |p| img.save(p).unwrap());
    }
    path
}

fn binary_path() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    let binary = if cfg!(target_os = "windows") {
        "cmdutils.exe"
    } else {
        "cmdutils"
    };
    path.push(binary);
    path.to_str().unwrap().to_string()
}

// ── image::convert ──────────────────────────────────────────────────────────

#[test]
fn test_convert_png_to_jpg() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    cmdutils::image::convert(input.to_str().unwrap(), "jpg").unwrap();

    let output = input.with_extension("jpg");
    assert!(output.exists(), "JPEG output should exist");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 4);
    assert_eq!(img.height(), 4);
}

#[test]
fn test_convert_jpg_to_png() {
    let jpg = test_jpg_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.jpg");
    std::fs::copy(&jpg, &input).unwrap();

    cmdutils::image::convert(input.to_str().unwrap(), "png").unwrap();

    let output = input.with_extension("png");
    assert!(output.exists(), "PNG output should exist");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 8);
    assert_eq!(img.height(), 6);
}

#[test]
fn test_convert_png_to_webp() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    cmdutils::image::convert(input.to_str().unwrap(), "webp").unwrap();

    let output = input.with_extension("webp");
    assert!(output.exists(), "WebP output should exist");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 4);
    assert_eq!(img.height(), 4);
}

#[test]
fn test_convert_webp_to_png() {
    let webp = test_webp_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.webp");
    std::fs::copy(&webp, &input).unwrap();

    cmdutils::image::convert(input.to_str().unwrap(), "png").unwrap();

    let output = input.with_extension("png");
    assert!(output.exists(), "PNG output should exist");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 6);
    assert_eq!(img.height(), 4);
}

#[test]
fn test_convert_bmp_to_png() {
    let bmp = test_bmp_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.bmp");
    std::fs::copy(&bmp, &input).unwrap();

    cmdutils::image::convert(input.to_str().unwrap(), "png").unwrap();

    let output = input.with_extension("png");
    assert!(output.exists(), "PNG output should exist");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 5);
    assert_eq!(img.height(), 5);
}

#[test]
fn test_convert_same_format_error() {
    let png = test_png_path();
    let err = cmdutils::image::convert(&png, "png").unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("same"), "should error on same format: {msg}");
}

#[test]
fn test_convert_invalid_input() {
    let err = cmdutils::image::convert("/nonexistent/image.png", "jpg").unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("not found"),
        "should report missing file: {msg}"
    );
}

#[test]
fn test_convert_unsupported_format() {
    let png = test_png_path();
    let err = cmdutils::image::convert(&png, "xxx").unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("Unsupported output"),
        "should reject unsupported format: {msg}"
    );
}

#[test]
fn test_convert_jpeg_extension_alias() {
    let jpg = test_jpg_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.jpeg");
    std::fs::copy(&jpg, &input).unwrap();

    // .jpeg → PNG (not jpg, which would be same format)
    cmdutils::image::convert(input.to_str().unwrap(), "png").unwrap();

    let output = input.with_extension("png");
    assert!(output.exists(), "PNG output should exist from .jpeg input");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 8);
    assert_eq!(img.height(), 6);
}

// ── image::resize ───────────────────────────────────────────────────────────

#[test]
fn test_resize() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    cmdutils::image::resize(input.to_str().unwrap(), "100x50").unwrap();

    let output = tmp.path().join("test_resized.png");
    assert!(output.exists(), "resized output should exist");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 100);
    assert_eq!(img.height(), 50);
}

#[test]
fn test_resize_webp() {
    let webp = test_webp_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.webp");
    std::fs::copy(&webp, &input).unwrap();

    cmdutils::image::resize(input.to_str().unwrap(), "30x20").unwrap();

    let output = tmp.path().join("test_resized.webp");
    assert!(output.exists(), "resized webp output should exist");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 30);
    assert_eq!(img.height(), 20);
}

#[test]
fn test_resize_with_uppercase_x() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    cmdutils::image::resize(input.to_str().unwrap(), "64X32").unwrap();

    let output = tmp.path().join("test_resized.png");
    assert!(output.exists());
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 64);
    assert_eq!(img.height(), 32);
}

#[test]
fn test_resize_invalid_dimensions_format() {
    let png = test_png_path();
    let err = cmdutils::image::resize(&png, "abc").unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("Invalid dimensions"),
        "should reject bad format: {msg}"
    );
}

#[test]
fn test_resize_invalid_width() {
    let png = test_png_path();
    let err = cmdutils::image::resize(&png, "abcx100").unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("Invalid width"),
        "should reject bad width: {msg}"
    );
}

#[test]
fn test_resize_invalid_height() {
    let png = test_png_path();
    let err = cmdutils::image::resize(&png, "100xabc").unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("Invalid height"),
        "should reject bad height: {msg}"
    );
}

#[test]
fn test_resize_nonexistent_file() {
    let err = cmdutils::image::resize("/nope/missing.png", "100x100").unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("not found"), "should report missing: {msg}");
}

// ── image::compress ─────────────────────────────────────────────────────────

#[test]
fn test_compress_jpeg() {
    let jpg = test_jpg_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.jpg");
    std::fs::copy(&jpg, &input).unwrap();

    cmdutils::image::compress(input.to_str().unwrap(), Some("80")).unwrap();

    let output = tmp.path().join("test_compressed.jpg");
    assert!(output.exists(), "compressed output should exist");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 8);
    assert_eq!(img.height(), 6);
}

#[test]
fn test_compress_png() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    // PNG: compress without quality (max compression)
    cmdutils::image::compress(input.to_str().unwrap(), None).unwrap();

    let output = tmp.path().join("test_compressed.png");
    assert!(output.exists(), "compressed output should exist");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 4);
    assert_eq!(img.height(), 4);
}

#[test]
fn test_compress_webp_with_quality() {
    let webp = test_webp_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.webp");
    std::fs::copy(&webp, &input).unwrap();

    // WebP: compress with quality
    cmdutils::image::compress(input.to_str().unwrap(), Some("50")).unwrap();

    let output = tmp.path().join("test_compressed.webp");
    assert!(output.exists(), "compressed webp output should exist");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 6);
    assert_eq!(img.height(), 4);
}

#[test]
fn test_compress_webp_without_quality() {
    let webp = test_webp_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.webp");
    std::fs::copy(&webp, &input).unwrap();

    // WebP: compress without quality (default lossy)
    cmdutils::image::compress(input.to_str().unwrap(), None).unwrap();

    let output = tmp.path().join("test_compressed.webp");
    assert!(output.exists(), "compressed webp output should exist");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 6);
    assert_eq!(img.height(), 4);
}

#[test]
fn test_compress_bmp() {
    let bmp = test_bmp_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.bmp");
    std::fs::copy(&bmp, &input).unwrap();

    // BMP: compress without quality
    cmdutils::image::compress(input.to_str().unwrap(), None).unwrap();

    let output = tmp.path().join("test_compressed.bmp");
    assert!(output.exists(), "compressed bmp output should exist");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 5);
    assert_eq!(img.height(), 5);
}

#[test]
fn test_compress_bmp_with_quality_errors() {
    let bmp = test_bmp_path();
    let err = cmdutils::image::compress(&bmp, Some("80")).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("does not support a quality"),
        "should reject quality for BMP: {msg}"
    );
}

#[test]
fn test_compress_png_with_quality_errors() {
    let png = test_png_path();
    let err = cmdutils::image::compress(&png, Some("80")).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("quality setting is not applicable"),
        "should reject quality for PNG: {msg}"
    );
}

#[test]
fn test_compress_jpeg_without_quality_errors() {
    let jpg = test_jpg_path();
    let err = cmdutils::image::compress(&jpg, None).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("requires a quality value"),
        "should require quality for JPEG: {msg}"
    );
}

#[test]
fn test_compress_invalid_quality_not_a_number() {
    let jpg = test_jpg_path();
    let err = cmdutils::image::compress(&jpg, Some("abc")).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("Invalid quality"),
        "should reject non-numeric: {msg}"
    );
}

#[test]
fn test_compress_quality_too_low() {
    let jpg = test_jpg_path();
    let err = cmdutils::image::compress(&jpg, Some("0")).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("must be between"),
        "should reject out of range: {msg}"
    );
}

#[test]
fn test_compress_quality_too_high() {
    let jpg = test_jpg_path();
    let err = cmdutils::image::compress(&jpg, Some("101")).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("must be between"),
        "should reject out of range: {msg}"
    );
}

#[test]
fn test_compress_nonexistent_file() {
    let err = cmdutils::image::compress("/nope/missing.jpg", Some("50")).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("not found"), "should report missing: {msg}");
}

#[test]
fn test_compress_different_quality_levels_differ() {
    let jpg = test_jpg_path();

    // Use separate temp dirs so outputs don't collide
    let high_dir = tempfile::TempDir::new().unwrap();
    let high_input = high_dir.path().join("test.jpg");
    std::fs::copy(&jpg, &high_input).unwrap();

    let low_dir = tempfile::TempDir::new().unwrap();
    let low_input = low_dir.path().join("test.jpg");
    std::fs::copy(&jpg, &low_input).unwrap();

    cmdutils::image::compress(high_input.to_str().unwrap(), Some("95")).unwrap();
    let high_out = high_dir.path().join("test_compressed.jpg");
    assert!(high_out.exists());

    cmdutils::image::compress(low_input.to_str().unwrap(), Some("5")).unwrap();
    let low_out = low_dir.path().join("test_compressed.jpg");
    assert!(low_out.exists());

    let high_size = std::fs::metadata(&high_out).unwrap().len();
    let low_size = std::fs::metadata(&low_out).unwrap().len();
    assert!(
        high_size >= low_size,
        "higher quality ({high_size}B) should not be smaller than low quality ({low_size}B)"
    );
}

// ── CLI integration tests ───────────────────────────────────────────────────

#[test]
fn test_cli_image_convert() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["image", "convert", input.to_str().unwrap(), "jpg"])
        .output()
        .expect("failed to run cmdutils image convert");

    assert!(output.status.success(), "CLI should succeed");

    let result = tmp.path().join("test.jpg");
    assert!(result.exists(), "output should exist");
}

#[test]
fn test_cli_image_convert_webp() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["image", "convert", input.to_str().unwrap(), "webp"])
        .output()
        .expect("failed to run cmdutils image convert to webp");

    assert!(output.status.success(), "CLI should succeed");

    let result = tmp.path().join("test.webp");
    assert!(result.exists(), "webp output should exist");
}

#[test]
fn test_cli_image_resize() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["image", "resize", input.to_str().unwrap(), "50x30"])
        .output()
        .expect("failed to run cmdutils image resize");

    assert!(output.status.success(), "CLI should succeed");

    let result = tmp.path().join("test_resized.png");
    assert!(result.exists(), "output should exist");
    let img = image::open(&result).unwrap();
    assert_eq!(img.width(), 50);
    assert_eq!(img.height(), 30);
}

#[test]
fn test_cli_image_resize_webp() {
    let webp = test_webp_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.webp");
    std::fs::copy(&webp, &input).unwrap();

    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["image", "resize", input.to_str().unwrap(), "20x15"])
        .output()
        .expect("failed to run cmdutils image resize webp");

    assert!(output.status.success(), "CLI should succeed");

    let result = tmp.path().join("test_resized.webp");
    assert!(result.exists(), "resized webp output should exist");
    let img = image::open(&result).unwrap();
    assert_eq!(img.width(), 20);
    assert_eq!(img.height(), 15);
}

#[test]
fn test_cli_image_compress_jpeg() {
    let jpg = test_jpg_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.jpg");
    std::fs::copy(&jpg, &input).unwrap();

    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["image", "compress", input.to_str().unwrap(), "75"])
        .output()
        .expect("failed to run cmdutils image compress");

    assert!(output.status.success(), "CLI should succeed");

    let result = tmp.path().join("test_compressed.jpg");
    assert!(result.exists(), "output should exist");
}

#[test]
fn test_cli_image_compress_png() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    let bin = binary_path();
    // PNG compress without quality (max compression)
    let output = Command::new(&bin)
        .args(["image", "compress", input.to_str().unwrap()])
        .output()
        .expect("failed to run cmdutils image compress png");

    assert!(
        output.status.success(),
        "CLI should succeed for PNG compress"
    );

    let result = tmp.path().join("test_compressed.png");
    assert!(result.exists(), "compressed output should exist");
}

#[test]
fn test_cli_image_compress_webp() {
    let webp = test_webp_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.webp");
    std::fs::copy(&webp, &input).unwrap();

    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["image", "compress", input.to_str().unwrap(), "60"])
        .output()
        .expect("failed to run cmdutils image compress webp");

    assert!(
        output.status.success(),
        "CLI should succeed for WebP compress"
    );

    let result = tmp.path().join("test_compressed.webp");
    assert!(result.exists(), "compressed webp output should exist");
}

#[test]
fn test_cli_image_missing_args_errors() {
    let bin = binary_path();

    // `cmdutils image` with no subcommand should fail
    let output = Command::new(&bin)
        .arg("image")
        .output()
        .expect("failed to run cmdutils image with no args");

    assert!(
        !output.status.success(),
        "CLI should fail with missing subcommand"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("required") || stderr.contains("subcommand"),
        "should mention required args: {stderr}"
    );
}

#[test]
fn test_cli_image_bad_dimensions_errors() {
    let jpg = test_jpg_path();
    let bin = binary_path();

    let output = Command::new(&bin)
        .args(["image", "resize", &jpg, "abc"])
        .output()
        .expect("failed to run cmdutils image resize with bad dimensions");

    assert!(
        !output.status.success(),
        "CLI should fail with bad dimensions"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Invalid"),
        "should mention invalid: {stderr}"
    );
}

#[test]
fn test_cli_image_bad_quality_errors() {
    let jpg = test_jpg_path();
    let bin = binary_path();

    let output = Command::new(&bin)
        .args(["image", "compress", &jpg, "999"])
        .output()
        .expect("failed to run cmdutils image compress with bad quality");

    assert!(!output.status.success(), "CLI should fail with bad quality");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Invalid quality"),
        "should mention invalid quality: {stderr}"
    );
}

#[test]
fn test_cli_image_nonexistent_file_errors() {
    let bin = binary_path();

    let output = Command::new(&bin)
        .args(["image", "convert", "/nonexistent/photo.png", "jpg"])
        .output()
        .expect("failed to run cmdutils image convert with missing file");

    assert!(
        !output.status.success(),
        "CLI should fail with missing file"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("not found"),
        "should mention not found: {stderr}"
    );
}

// ── image::metadata ─────────────────────────────────────────────────────────

#[test]
fn test_metadata_png() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    cmdutils::image::metadata(input.to_str().unwrap(), None).unwrap();
}

#[test]
fn test_metadata_png_with_report() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    let report = tmp.path().join("report.pdf");
    cmdutils::image::metadata(input.to_str().unwrap(), Some(report.to_str().unwrap())).unwrap();

    assert!(report.exists(), "PDF report should exist");
    assert!(
        std::fs::metadata(&report).unwrap().len() > 100,
        "PDF should be non-trivial"
    );
}

#[test]
fn test_metadata_nonexistent_file() {
    let err = cmdutils::image::metadata("/nope/missing.png", None).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("not found"), "should report missing: {msg}");
}

#[test]
fn test_cli_image_metadata() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["image", "metadata", input.to_str().unwrap()])
        .output()
        .expect("failed to run cmdutils image metadata");

    assert!(output.status.success(), "CLI should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Metadata"),
        "output should contain Metadata"
    );
    assert!(
        stdout.contains("Image"),
        "output should contain Image section"
    );
}

#[test]
fn test_cli_image_metadata_with_report() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    let report = tmp.path().join("report.pdf");
    let bin = binary_path();
    let output = Command::new(&bin)
        .args([
            "image",
            "metadata",
            input.to_str().unwrap(),
            "--report",
            report.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run cmdutils image metadata --report");

    assert!(output.status.success(), "CLI should succeed");
    assert!(report.exists(), "PDF report should exist");
}

#[test]
fn test_cli_image_help_succeeds() {
    let bin = binary_path();

    // Top-level help
    let output = Command::new(&bin)
        .arg("--help")
        .output()
        .expect("failed to run cmdutils --help");

    assert!(output.status.success(), "--help should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("image"), "help should list image command");
    assert!(stdout.contains("Examples"), "help should show examples");

    assert!(stdout.contains("text"), "help should list text command");

    // Image subcommand help
    let output = Command::new(&bin)
        .args(["image", "--help"])
        .output()
        .expect("failed to run cmdutils image --help");

    assert!(output.status.success(), "image --help should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("convert"), "image help should list convert");
    assert!(
        stdout.contains("compress"),
        "image help should list compress"
    );
    assert!(stdout.contains("resize"), "image help should list resize");
    assert!(stdout.contains("crop"), "image help should list crop");
    assert!(stdout.contains("rotate"), "image help should list rotate");
    assert!(
        stdout.contains("grayscale"),
        "image help should list grayscale"
    );
    assert!(
        stdout.contains("watermark"),
        "image help should list watermark"
    );
    assert!(stdout.contains("strip"), "image help should list strip");

    // Text subcommand help
    let output = Command::new(&bin)
        .args(["text", "--help"])
        .output()
        .expect("failed to run cmdutils text --help");

    assert!(output.status.success(), "text --help should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("count"), "text help should list count");
    assert!(stdout.contains("case"), "text help should list case");
    assert!(stdout.contains("replace"), "text help should list replace");
    assert!(stdout.contains("base64"), "text help should list base64");
    assert!(
        stdout.contains("checksum"),
        "text help should list checksum"
    );
}

// ── image::crop ─────────────────────────────────────────────────────────────

#[test]
fn test_crop() {
    let png = test_png_path(); // 4x4
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    cmdutils::image::crop(input.to_str().unwrap(), "2x2+1+1").unwrap();

    let output = tmp.path().join("test_cropped.png");
    assert!(output.exists(), "cropped output should exist");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 2);
    assert_eq!(img.height(), 2);
}

#[test]
fn test_crop_out_of_bounds_errors() {
    let png = test_png_path(); // 4x4
    let err = cmdutils::image::crop(&png, "100x100+0+0").unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("exceeds"), "should report bounds: {msg}");
}

#[test]
fn test_crop_invalid_geometry_errors() {
    let png = test_png_path();
    let err = cmdutils::image::crop(&png, "abc").unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("Invalid crop geometry"),
        "should reject bad geometry: {msg}"
    );
}

#[test]
fn test_crop_zero_size_errors() {
    let png = test_png_path();
    let err = cmdutils::image::crop(&png, "0x10+0+0").unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("greater than 0"),
        "should reject zero size: {msg}"
    );
}

// ── image::rotate ───────────────────────────────────────────────────────────

#[test]
fn test_rotate_90_swaps_dimensions() {
    let jpg = test_jpg_path(); // 8x6
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.jpg");
    std::fs::copy(&jpg, &input).unwrap();

    cmdutils::image::rotate(input.to_str().unwrap(), 90).unwrap();

    let output = tmp.path().join("test_rotated.jpg");
    assert!(output.exists(), "rotated output should exist");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 6);
    assert_eq!(img.height(), 8);
}

#[test]
fn test_rotate_180_keeps_dimensions() {
    let jpg = test_jpg_path(); // 8x6
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.jpg");
    std::fs::copy(&jpg, &input).unwrap();

    cmdutils::image::rotate(input.to_str().unwrap(), 180).unwrap();

    let output = tmp.path().join("test_rotated.jpg");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 8);
    assert_eq!(img.height(), 6);
}

#[test]
fn test_rotate_direction_clockwise() {
    // 2x1 image: left = red, right = blue. 90° clockwise puts blue on top.
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    let img = image::RgbImage::from_fn(2, 1, |x, _| {
        if x == 0 {
            image::Rgb([255, 0, 0])
        } else {
            image::Rgb([0, 0, 255])
        }
    });
    img.save(&input).unwrap();

    cmdutils::image::rotate(input.to_str().unwrap(), 90).unwrap();

    let output = tmp.path().join("test_rotated.png");
    let rotated = image::open(&output).unwrap().to_rgb8();
    assert_eq!(rotated.width(), 1);
    assert_eq!(rotated.height(), 2);
    let top = rotated.get_pixel(0, 0);
    let bottom = rotated.get_pixel(0, 1);
    assert_eq!(
        top.0,
        [0, 0, 255],
        "clockwise: blue (right) should be on top"
    );
    assert_eq!(bottom.0, [255, 0, 0], "red (left) should be at the bottom");
}

#[test]
fn test_rotate_invalid_degrees_errors() {
    let png = test_png_path();
    let err = cmdutils::image::rotate(&png, 45).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("multiple of 90"), "should reject 45°: {msg}");
}

#[test]
fn test_rotate_full_turn_errors() {
    let png = test_png_path();
    let err = cmdutils::image::rotate(&png, 360).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("nothing to do"), "should reject 360°: {msg}");
}

// ── image::grayscale ────────────────────────────────────────────────────────

#[test]
fn test_grayscale_pixels_are_gray() {
    let png = test_png_path(); // colorful RGBA
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    cmdutils::image::grayscale(input.to_str().unwrap()).unwrap();

    let output = tmp.path().join("test_grayscale.png");
    assert!(output.exists(), "grayscale output should exist");
    let img = image::open(&output).unwrap().to_rgb8();
    assert_eq!(img.width(), 4);
    assert_eq!(img.height(), 4);
    for y in 0..4 {
        for x in 0..4 {
            let p = img.get_pixel(x, y);
            assert_eq!(p.0[0], p.0[1], "r == g at ({x},{y})");
            assert_eq!(p.0[1], p.0[2], "g == b at ({x},{y})");
        }
    }
}

// ── image::strip ────────────────────────────────────────────────────────────

#[test]
fn test_strip_jpg() {
    let jpg = test_jpg_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.jpg");
    std::fs::copy(&jpg, &input).unwrap();

    cmdutils::image::strip(input.to_str().unwrap()).unwrap();

    let output = tmp.path().join("test_stripped.jpg");
    assert!(output.exists(), "stripped output should exist");
    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 8);
    assert_eq!(img.height(), 6);
}

#[test]
fn test_strip_svg_errors() {
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.svg");
    std::fs::write(
        &input,
        "<svg xmlns='http://www.w3.org/2000/svg' width='10' height='10'><rect width='10' height='10' fill='red'/></svg>",
    )
    .unwrap();

    let err = cmdutils::image::strip(input.to_str().unwrap()).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("vector"),
        "SVG has no metadata to strip: {msg}"
    );
}

// ── image::watermark ────────────────────────────────────────────────────────

#[test]
fn test_watermark_changes_pixels() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    cmdutils::image::watermark(
        input.to_str().unwrap(),
        "TEST",
        50,
        "bottom-right",
        12,
        "ffffff",
        None,
    )
    .unwrap();

    let output = tmp.path().join("test_watermarked.png");
    assert!(output.exists(), "watermarked output should exist");
    let original = image::open(&png).unwrap().to_rgba8();
    let marked = image::open(&output).unwrap().to_rgba8();
    assert_eq!(original.dimensions(), marked.dimensions());
    assert_ne!(
        original.as_raw(),
        marked.as_raw(),
        "watermark should alter at least one pixel"
    );
}

#[test]
fn test_watermark_empty_text_errors() {
    let png = test_png_path();
    let err =
        cmdutils::image::watermark(&png, "", 50, "bottom-right", 12, "ffffff", None).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("cannot be empty"),
        "should reject empty text: {msg}"
    );
}

#[test]
fn test_watermark_bad_color_errors() {
    let png = test_png_path();
    let err =
        cmdutils::image::watermark(&png, "x", 50, "bottom-right", 12, "zzzz", None).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("Invalid color"),
        "should reject bad color: {msg}"
    );
}

#[test]
fn test_watermark_bad_position_errors() {
    let png = test_png_path();
    let err = cmdutils::image::watermark(&png, "x", 50, "middle", 12, "ffffff", None).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("Unknown position"),
        "should reject bad position: {msg}"
    );
}

// ── image::batch (glob expansion) ───────────────────────────────────────────

#[test]
fn test_expand_inputs_single_file() {
    let png = test_png_path();
    let files = cmdutils::image::batch::expand_inputs(&png).unwrap();
    assert_eq!(files.len(), 1);
}

#[test]
fn test_expand_inputs_glob() {
    let tmp = tempfile::TempDir::new().unwrap();
    for i in 0..3 {
        std::fs::write(tmp.path().join(format!("a{i}.png")), b"x").unwrap();
    }
    let pattern = tmp.path().join("*.png");
    let files = cmdutils::image::batch::expand_inputs(pattern.to_str().unwrap()).unwrap();
    assert_eq!(files.len(), 3);
}

#[test]
fn test_expand_inputs_no_match_errors() {
    let tmp = tempfile::TempDir::new().unwrap();
    let pattern = tmp.path().join("*.zzz");
    let err = cmdutils::image::batch::expand_inputs(pattern.to_str().unwrap()).unwrap_err();
    assert!(
        err.contains("No files matched"),
        "should report no match: {err}"
    );
}

#[test]
fn test_expand_inputs_missing_file_errors() {
    let err = cmdutils::image::batch::expand_inputs("/nope/missing.png").unwrap_err();
    assert!(err.contains("not found"), "should report missing: {err}");
}

// ── text::count ─────────────────────────────────────────────────────────────

#[test]
fn test_count_file() {
    let tmp = tempfile::TempDir::new().unwrap();
    let file = tmp.path().join("sample.txt");
    std::fs::write(&file, "hello world\nfoo bar baz\n").unwrap();

    let flags = cmdutils::text::count::CountFlags {
        words: false,
        lines: false,
        chars: false,
        bytes: false,
    };
    cmdutils::text::count(Some(file.to_str().unwrap()), flags).unwrap();
}

#[test]
fn test_count_missing_file_errors() {
    let flags = cmdutils::text::count::CountFlags {
        words: true,
        lines: false,
        chars: false,
        bytes: false,
    };
    let err = cmdutils::text::count(Some("/nope/missing.txt"), flags).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("No such file") || msg.contains("not found"),
        "should report missing file: {msg}"
    );
}

// ── text::case ──────────────────────────────────────────────────────────────

#[test]
fn test_case_snake() {
    cmdutils::text::case(None, Some("Hello World"), "snake").unwrap();
}

#[test]
fn test_case_camel() {
    cmdutils::text::case(None, Some("foo bar baz"), "camel").unwrap();
}

#[test]
fn test_case_unknown_style_errors() {
    let err = cmdutils::text::case(None, Some("x"), "banana").unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("Unknown case"),
        "should reject unknown style: {msg}"
    );
}

// ── text::replace ───────────────────────────────────────────────────────────

#[test]
fn test_replace_with_output() {
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("in.txt");
    let output = tmp.path().join("out.txt");
    std::fs::write(&input, "foo bar foo").unwrap();

    cmdutils::text::replace(
        "foo",
        "baz",
        Some(input.to_str().unwrap()),
        false,
        Some(output.to_str().unwrap()),
    )
    .unwrap();

    assert_eq!(std::fs::read_to_string(&output).unwrap(), "baz bar baz");
    assert_eq!(std::fs::read_to_string(&input).unwrap(), "foo bar foo");
}

#[test]
fn test_replace_in_place() {
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("in.txt");
    std::fs::write(&input, "foo foo foo").unwrap();

    cmdutils::text::replace("foo", "bar", Some(input.to_str().unwrap()), true, None).unwrap();

    assert_eq!(std::fs::read_to_string(&input).unwrap(), "bar bar bar");
}

#[test]
fn test_replace_in_place_requires_file() {
    let err = cmdutils::text::replace("a", "b", None, true, None).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("input file"), "--in-place needs a file: {msg}");
}

#[test]
fn test_replace_in_place_and_output_conflict() {
    let err = cmdutils::text::replace("a", "b", None, true, Some("x.txt")).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("both"),
        "--in-place + --output conflict: {msg}"
    );
}

#[test]
fn test_replace_empty_find_errors() {
    let err = cmdutils::text::replace("", "b", None, false, None).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("cannot be empty"),
        "empty find rejected: {msg}"
    );
}

// ── text::base64 ────────────────────────────────────────────────────────────

#[test]
fn test_base64_roundtrip() {
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("data.bin");
    std::fs::write(&input, b"hello world").unwrap();
    let enc = tmp.path().join("enc.txt");
    let dec = tmp.path().join("dec.bin");

    cmdutils::text::base64::encode(Some(input.to_str().unwrap()), Some(enc.to_str().unwrap()))
        .unwrap();
    cmdutils::text::base64::decode(Some(enc.to_str().unwrap()), Some(dec.to_str().unwrap()))
        .unwrap();

    assert_eq!(std::fs::read(&dec).unwrap(), b"hello world");
}

#[test]
fn test_base64_decode_invalid_errors() {
    let tmp = tempfile::TempDir::new().unwrap();
    let file = tmp.path().join("bad.txt");
    std::fs::write(&file, "!!!not base64!!!").unwrap();

    let err = cmdutils::text::base64::decode(Some(file.to_str().unwrap()), None).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("Invalid base64"),
        "should reject bad input: {msg}"
    );
}

// ── text::checksum ──────────────────────────────────────────────────────────

#[test]
fn test_checksum_supported_algos() {
    let tmp = tempfile::TempDir::new().unwrap();
    let file = tmp.path().join("f.txt");
    std::fs::write(&file, "hello").unwrap();

    for algo in ["md5", "sha256", "sha512"] {
        cmdutils::text::checksum(Some(file.to_str().unwrap()), algo).unwrap();
    }
}

#[test]
fn test_checksum_unknown_algo_errors() {
    let tmp = tempfile::TempDir::new().unwrap();
    let file = tmp.path().join("f.txt");
    std::fs::write(&file, "hello").unwrap();

    let err = cmdutils::text::checksum(Some(file.to_str().unwrap()), "crc32").unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("Unsupported algorithm"),
        "reject unknown algo: {msg}"
    );
}

// ── CLI: image upgrades ─────────────────────────────────────────────────────

#[test]
fn test_cli_image_crop() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["image", "crop", input.to_str().unwrap(), "2x2+1+1"])
        .output()
        .expect("failed to run cmdutils image crop");

    assert!(output.status.success(), "CLI crop should succeed");
    let result = tmp.path().join("test_cropped.png");
    assert!(result.exists(), "cropped output should exist");
    let img = image::open(&result).unwrap();
    assert_eq!(img.width(), 2);
    assert_eq!(img.height(), 2);
}

#[test]
fn test_cli_image_rotate() {
    let jpg = test_jpg_path(); // 8x6
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.jpg");
    std::fs::copy(&jpg, &input).unwrap();

    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["image", "rotate", input.to_str().unwrap(), "90"])
        .output()
        .expect("failed to run cmdutils image rotate");

    assert!(output.status.success(), "CLI rotate should succeed");
    let result = tmp.path().join("test_rotated.jpg");
    assert!(result.exists(), "rotated output should exist");
    let img = image::open(&result).unwrap();
    assert_eq!(img.width(), 6);
    assert_eq!(img.height(), 8);
}

#[test]
fn test_cli_image_grayscale() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["image", "grayscale", input.to_str().unwrap()])
        .output()
        .expect("failed to run cmdutils image grayscale");

    assert!(output.status.success(), "CLI grayscale should succeed");
    let result = tmp.path().join("test_grayscale.png");
    assert!(result.exists(), "grayscale output should exist");
}

#[test]
fn test_cli_image_grayscale_batch_glob() {
    let tmp = tempfile::TempDir::new().unwrap();
    for i in 0..3 {
        let img = image::RgbaImage::from_fn(4, 4, |x, y| {
            image::Rgba([(x * 60) as u8, (y * 60) as u8, i as u8, 255])
        });
        img.save(tmp.path().join(format!("photo{i}.png"))).unwrap();
    }

    let pattern = tmp.path().join("*.png");
    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["image", "grayscale", pattern.to_str().unwrap()])
        .output()
        .expect("failed to run cmdutils image grayscale batch");

    assert!(
        output.status.success(),
        "batch grayscale should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    for i in 0..3 {
        assert!(
            tmp.path().join(format!("photo{i}_grayscale.png")).exists(),
            "batch output photo{i}_grayscale.png should exist"
        );
    }
}

#[test]
fn test_cli_image_watermark() {
    let png = test_png_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.png");
    std::fs::copy(&png, &input).unwrap();

    let bin = binary_path();
    let output = Command::new(&bin)
        .args([
            "image",
            "watermark",
            input.to_str().unwrap(),
            "--text",
            "TEST",
            "--opacity",
            "50",
        ])
        .output()
        .expect("failed to run cmdutils image watermark");

    assert!(output.status.success(), "CLI watermark should succeed");
    let result = tmp.path().join("test_watermarked.png");
    assert!(result.exists(), "watermarked output should exist");
}

#[test]
fn test_cli_image_strip() {
    let jpg = test_jpg_path();
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("test.jpg");
    std::fs::copy(&jpg, &input).unwrap();

    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["image", "strip", input.to_str().unwrap()])
        .output()
        .expect("failed to run cmdutils image strip");

    assert!(output.status.success(), "CLI strip should succeed");
    let result = tmp.path().join("test_stripped.jpg");
    assert!(result.exists(), "stripped output should exist");
}

// ── CLI: text ───────────────────────────────────────────────────────────────

#[test]
fn test_cli_text_count() {
    let tmp = tempfile::TempDir::new().unwrap();
    let file = tmp.path().join("sample.txt");
    std::fs::write(&file, "hello world\nfoo bar\nbaz\n").unwrap();

    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["text", "count", file.to_str().unwrap()])
        .output()
        .expect("failed to run cmdutils text count");

    assert!(output.status.success(), "CLI count should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("3 lines"), "count output: {stdout}");
    assert!(stdout.contains("5 words"), "count output: {stdout}");
}

#[test]
fn test_cli_text_count_stdin() {
    let bin = binary_path();
    let mut child = Command::new(&bin)
        .args(["text", "count"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run cmdutils text count on stdin");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"hello world\nfoo bar\nbaz\n")
        .unwrap();
    let out = child.wait_with_output().unwrap();

    assert!(out.status.success(), "CLI count stdin should succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("3 lines"), "count stdin: {stdout}");
    assert!(stdout.contains("5 words"), "count stdin: {stdout}");
    assert!(stdout.contains("stdin"), "should label stdin: {stdout}");
}

#[test]
fn test_cli_text_case() {
    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["text", "case", "--to", "snake", "--text", "Hello World"])
        .output()
        .expect("failed to run cmdutils text case");

    assert!(output.status.success(), "CLI case should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout, "hello_world\n", "case output: {stdout}");
}

#[test]
fn test_cli_text_replace() {
    let tmp = tempfile::TempDir::new().unwrap();
    let input = tmp.path().join("in.txt");
    let output = tmp.path().join("out.txt");
    std::fs::write(&input, "foo bar foo").unwrap();

    let bin = binary_path();
    let result = Command::new(&bin)
        .args([
            "text",
            "replace",
            "foo",
            "baz",
            input.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run cmdutils text replace");

    assert!(result.status.success(), "CLI replace should succeed");
    assert_eq!(std::fs::read_to_string(&output).unwrap(), "baz bar baz");
}

#[test]
fn test_cli_text_base64() {
    let tmp = tempfile::TempDir::new().unwrap();
    let file = tmp.path().join("data.txt");
    std::fs::write(&file, "hello world").unwrap();

    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["text", "base64", "encode", file.to_str().unwrap()])
        .output()
        .expect("failed to run cmdutils text base64 encode");

    assert!(output.status.success(), "CLI base64 encode should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout, "aGVsbG8gd29ybGQ=", "base64 output: {stdout}");
}

#[test]
fn test_cli_text_checksum() {
    let tmp = tempfile::TempDir::new().unwrap();
    let file = tmp.path().join("hello.txt");
    std::fs::write(&file, "hello").unwrap();

    let bin = binary_path();
    let output = Command::new(&bin)
        .args(["text", "checksum", file.to_str().unwrap()])
        .output()
        .expect("failed to run cmdutils text checksum");

    assert!(output.status.success(), "CLI checksum should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"),
        "sha256 of 'hello': {stdout}"
    );
}
