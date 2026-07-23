use clap::{Parser, Subcommand};

use cmdutils::image;

#[derive(Parser)]
#[command(name = "cmdutils")]
#[command(about = "Cross-platform CLI utilities for everyday tasks")]
#[command(long_about = "cmdutils — a collection of cross-platform CLI utilities.\n\nExamples:\n  cmdutils image convert photo.png jpg       Convert PNG to JPEG\n  cmdutils image resize photo.png 800x600    Resize image to 800x600\n  cmdutils image compress photo.jpg 80       Compress JPEG with quality 80")]
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
    /// Convert an image between formats (PNG ↔ JPEG)
    Convert {
        /// Path to the input image file
        input: String,
        /// Target format: jpg, jpeg, or png
        format: String,
    },
    /// Resize an image to exact dimensions
    Resize {
        /// Path to the input image file
        input: String,
        /// Target dimensions in WxH format (e.g., 800x600)
        dimensions: String,
    },
    /// Compress an image (PNG: max compression; JPEG: use a quality 1-100)
    Compress {
        /// Path to the input image file
        input: String,
        /// Quality level 1-100 for JPEG (not applicable to PNG)
        quality: Option<String>,
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
        },
    }
}
