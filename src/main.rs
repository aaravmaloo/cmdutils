use clap::{Parser, Subcommand};

use cmdutils::image;
use cmdutils::text;

#[derive(Parser)]
#[command(name = "cmdutils")]
#[command(about = "Cross-platform CLI utilities for everyday tasks")]
#[command(
    long_about = "cmdutils — a collection of cross-platform CLI utilities.\n\nExamples:\n  cmdutils image convert photo.png jpg       Convert PNG to JPEG\n  cmdutils image resize photo.png 800x600    Resize image to 800x600\n  cmdutils image compress photo.jpg 80       Compress JPEG with quality 80\n  cmdutils image crop photo.png 400x300+50+40   Crop a 400x300 region at +50+40\n  cmdutils image rotate photo.png 90         Rotate image 90° clockwise\n  cmdutils image grayscale '*.png'           Grayscale every PNG in the folder\n  cmdutils image watermark photo.png --text \"© 2026\"\n  cmdutils image metadata photo.png          Show image metadata\n  cmdutils text count file.txt               Count lines, words, chars, bytes\n  cmdutils text case --to snake file.txt     Convert text to snake_case\n  cmdutils text checksum file.iso --algo sha256"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Image operations (convert, compress, resize, crop, rotate, ...)
    Image {
        #[command(subcommand)]
        action: ImageAction,
    },
    /// Text operations (count, case, replace, base64, checksum)
    Text {
        #[command(subcommand)]
        action: TextAction,
    },
}

#[derive(Subcommand)]
enum ImageAction {
    /// Convert an image between formats (PNG, JPEG, WebP, BMP, GIF, TIFF, AVIF, and more)
    Convert {
        /// Path to the input image file (glob patterns supported)
        input: String,
        /// Target format (e.g., png, jpg, webp, bmp, gif, tiff, avif)
        format: String,
    },
    /// Resize an image to exact dimensions
    Resize {
        /// Path to the input image file (glob patterns supported)
        input: String,
        /// Target dimensions in WxH format (e.g., 800x600)
        dimensions: String,
    },
    /// Compress an image (PNG: lossless; JPEG/WebP: quality 1-100; others: re-encode)
    Compress {
        /// Path to the input image file (glob patterns supported)
        input: String,
        /// Quality level 1-100 for JPEG/WebP (not applicable to PNG, BMP, GIF, TIFF, etc.)
        quality: Option<String>,
    },
    /// Extract and display image metadata (EXIF, dimensions, color, etc.)
    Metadata {
        /// Path to the input image file (glob patterns supported)
        input: String,
        /// Optional path to generate a PDF metadata report (single input only)
        #[arg(short = 'r', long = "report")]
        report: Option<String>,
    },
    /// Crop an image to a region (WxH+X+Y)
    Crop {
        /// Path to the input image file (glob patterns supported)
        input: String,
        /// Crop region in WxH+X+Y format (e.g., 800x600+100+50)
        geometry: String,
    },
    /// Rotate an image clockwise (multiples of 90°)
    Rotate {
        /// Path to the input image file (glob patterns supported)
        input: String,
        /// Degrees to rotate clockwise (90, 180, 270, ...)
        degrees: i32,
    },
    /// Convert an image to grayscale
    Grayscale {
        /// Path to the input image file (glob patterns supported)
        input: String,
    },
    /// Overlay semi-transparent text on an image
    Watermark {
        /// Path to the input image file (glob patterns supported)
        input: String,
        /// Watermark text
        #[arg(long)]
        text: String,
        /// Text color as 6-digit hex (default ffffff)
        #[arg(long, default_value = "ffffff")]
        color: String,
        /// Opacity 0-100
        #[arg(long, default_value_t = 60)]
        opacity: u8,
        /// Position: top-left, top-right, bottom-left, bottom-right, center
        #[arg(long, default_value = "bottom-right")]
        position: String,
        /// Font size in pixels
        #[arg(long, default_value_t = 48)]
        size: u32,
        /// Path to a TTF/OTF font file (defaults to a system font)
        #[arg(long)]
        font: Option<String>,
    },
    /// Strip all metadata (EXIF, comments) from an image
    Strip {
        /// Path to the input image file (glob patterns supported)
        input: String,
    },
}

#[derive(Subcommand)]
enum TextAction {
    /// Count lines, words, characters, and bytes (wc-style)
    Count {
        /// Input file (defaults to stdin)
        input: Option<String>,
        /// Count words only
        #[arg(short = 'w', long)]
        words: bool,
        /// Count lines only
        #[arg(short = 'l', long)]
        lines: bool,
        /// Count characters only
        #[arg(short = 'm', long)]
        chars: bool,
        /// Count bytes only
        #[arg(short = 'c', long)]
        bytes: bool,
    },
    /// Convert text between letter cases
    Case {
        /// Input file (defaults to stdin; ignored if --text is given)
        input: Option<String>,
        /// Literal text to convert
        #[arg(long)]
        text: Option<String>,
        /// Target case: upper, lower, title, snake, kebab, camel, pascal, constant
        #[arg(long)]
        to: String,
    },
    /// Replace all occurrences of a string
    Replace {
        /// Text to find
        find: String,
        /// Replacement text
        replace: String,
        /// Input file (defaults to stdin)
        input: Option<String>,
        /// Modify the input file in place (requires an input file)
        #[arg(short = 'i', long)]
        in_place: bool,
        /// Write output to a file instead of stdout
        #[arg(short = 'o', long)]
        output: Option<String>,
    },
    /// Base64 encode or decode
    Base64 {
        #[command(subcommand)]
        action: Base64Action,
    },
    /// Compute a checksum (md5, sha256, sha512)
    Checksum {
        /// Input file (defaults to stdin)
        input: Option<String>,
        /// Hash algorithm: md5, sha256, sha512
        #[arg(long, default_value = "sha256")]
        algo: String,
    },
}

#[derive(Subcommand)]
enum Base64Action {
    /// Encode to base64
    Encode {
        /// Input file (defaults to stdin)
        input: Option<String>,
        /// Write output to a file instead of stdout
        #[arg(short = 'o', long)]
        output: Option<String>,
    },
    /// Decode from base64
    Decode {
        /// Input file (defaults to stdin)
        input: Option<String>,
        /// Write output to a file instead of stdout
        #[arg(short = 'o', long)]
        output: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Image { action } => match action {
            ImageAction::Convert { input, format } => {
                run_image(&input, |p| image::convert::convert(p, &format));
            }
            ImageAction::Resize { input, dimensions } => {
                run_image(&input, |p| image::resize::resize(p, &dimensions));
            }
            ImageAction::Compress { input, quality } => {
                run_image(&input, |p| image::compress::compress(p, quality.as_deref()));
            }
            ImageAction::Metadata { input, report } => {
                if report.is_some() && input.contains(['*', '?', '[']) {
                    eprintln!(
                        "Error: --report is not supported with multiple inputs (glob patterns)"
                    );
                    std::process::exit(1);
                }
                run_image(&input, |p| image::metadata::metadata(p, report.as_deref()));
            }
            ImageAction::Crop { input, geometry } => {
                run_image(&input, |p| image::crop::crop(p, &geometry));
            }
            ImageAction::Rotate { input, degrees } => {
                run_image(&input, |p| image::rotate::rotate(p, degrees));
            }
            ImageAction::Grayscale { input } => {
                run_image(&input, image::grayscale::grayscale);
            }
            ImageAction::Watermark {
                input,
                text,
                color,
                opacity,
                position,
                size,
                font,
            } => {
                run_image(&input, |p| {
                    image::watermark::watermark(
                        p,
                        &text,
                        opacity,
                        &position,
                        size,
                        &color,
                        font.as_deref(),
                    )
                });
            }
            ImageAction::Strip { input } => {
                run_image(&input, image::strip::strip);
            }
        },
        Commands::Text { action } => match action {
            TextAction::Count {
                input,
                words,
                lines,
                chars,
                bytes,
            } => {
                run_text(|| {
                    text::count::count(
                        input.as_deref(),
                        text::count::CountFlags {
                            words,
                            lines,
                            chars,
                            bytes,
                        },
                    )
                });
            }
            TextAction::Case { input, text, to } => {
                run_text(|| text::case::case(input.as_deref(), text.as_deref(), &to));
            }
            TextAction::Replace {
                find,
                replace,
                input,
                in_place,
                output,
            } => {
                run_text(|| {
                    text::replace::replace(
                        &find,
                        &replace,
                        input.as_deref(),
                        in_place,
                        output.as_deref(),
                    )
                });
            }
            TextAction::Base64 { action } => match action {
                Base64Action::Encode { input, output } => {
                    run_text(|| text::base64::encode(input.as_deref(), output.as_deref()));
                }
                Base64Action::Decode { input, output } => {
                    run_text(|| text::base64::decode(input.as_deref(), output.as_deref()));
                }
            },
            TextAction::Checksum { input, algo } => {
                run_text(|| text::checksum::checksum(input.as_deref(), &algo));
            }
        },
    }
}

/// Run a single- or multi-file image operation, exiting with an error message
/// on failure (matching the existing CLI error convention).
fn run_image<F>(input: &str, worker: F)
where
    F: Fn(&str) -> Result<(), Box<dyn std::error::Error>> + Sync + Send,
{
    if let Err(e) = image::batch::run_batch(input, worker) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run_text<F>(f: F)
where
    F: FnOnce() -> Result<(), Box<dyn std::error::Error>>,
{
    if let Err(e) = f() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
