use std::process::Command;

fn test_png_path() -> String {
    let dir = std::env::temp_dir().join("cmdutils_test_png.png");
    let path = dir.to_str().unwrap().to_string();
    if !dir.exists() {
        let img = image::RgbaImage::from_fn(4, 4, |x, y| {
            image::Rgba([(x * 64) as u8, (y * 64) as u8, 128, 255])
        });
        img.save(&dir).unwrap();
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
        img.save(&dir).unwrap();
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
        // Use image::codecs::webp::WebPEncoder
        use image::codecs::webp::WebPEncoder;
        let file = std::fs::File::create(&dir).unwrap();
        let encoder = WebPEncoder::new_lossless(file);
        encoder
            .encode(
                img.as_raw(),
                img.width(),
                img.height(),
                image::ExtendedColorType::Rgba8,
            )
            .unwrap();
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
        img.save(&dir).unwrap();
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

    assert!(output.status.success(), "CLI should succeed for WebP compress");

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
    cmdutils::image::metadata(
        input.to_str().unwrap(),
        Some(report.to_str().unwrap()),
    )
    .unwrap();

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
    assert!(stdout.contains("Metadata"), "output should contain Metadata");
    assert!(stdout.contains("Image"), "output should contain Image section");
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
}
