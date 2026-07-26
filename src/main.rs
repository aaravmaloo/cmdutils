use clap::{Parser, Subcommand};

use cmdutils::image;

#[derive(Parser)]
#[command(name = "cmdutils")]
#[command(about = "Cross-platform CLI utilities for everyday tasks")]
#[command(
    long_about = "cmdutils — a collection of cross-platform CLI utilities.\n\nExamples:\n  cmdutils image convert photo.png jpg       Convert PNG to JPEG\n  cmdutils image resize photo.png 800x600    Resize image to 800x600\n  cmdutils image compress photo.jpg 80       Compress JPEG with quality 80\n  cmdutils image convert photo.png webp      Convert PNG to WebP\n  cmdutils image compress photo.webp 75      Compress WebP with quality 75
  cmdutils image metadata photo.png            Show image metadata
  cmdutils image metadata photo.jpg -r report.pdf  Save metadata as PDF report"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Image operations (convert, compress, resize)
    Image {
        #[command(subcommand)]
        action: ImageAction,
    },
}

#[derive(Subcommand)]
enum ImageAction {
    /// Convert an image between formats (PNG, JPEG, WebP, BMP, GIF, TIFF, AVIF, and more)
    Convert {
        /// Path to the input image file
        input: String,
        /// Target format (e.g., png, jpg, webp, bmp, gif, tiff, avif)
        format: String,
    },
    /// Resize an image to exact dimensions
    Resize {
        /// Path to the input image file
        input: String,
        /// Target dimensions in WxH format (e.g., 800x600)
        dimensions: String,
    },
    /// Compress an image (PNG: lossless; JPEG/WebP: quality 1-100; others: re-encode)
    Compress {
        /// Path to the input image file
        input: String,
        /// Quality level 1-100 for JPEG/WebP (not applicable to PNG, BMP, GIF, TIFF, etc.)
        quality: Option<String>,
    },
    /// Extract and display image metadata (EXIF, dimensions, color, etc.)
    Metadata {
        /// Path to the input image file
        input: String,
        /// Optional path to generate a PDF metadata report
        #[arg(short = 'r', long = "report")]
        report: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Image { action } => match action {
            ImageAction::Convert { input, format } => {
                if let Err(e) = image::convert::convert(&input, &format) {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
            ImageAction::Resize { input, dimensions } => {
                if let Err(e) = image::resize::resize(&input, &dimensions) {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
            ImageAction::Compress { input, quality } => {
                if let Err(e) = image::compress::compress(&input, quality.as_deref()) {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
            ImageAction::Metadata { input, report } => {
                if let Err(e) = image::metadata::metadata(&input, report.as_deref()) {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        },
    }
}
