use std::ffi::OsStr;
use std::io::BufReader;
use std::path::Path;

use image::GenericImageView; // needed for .dimensions(), .color()
use printpdf::{BuiltinFont, IndirectFontRef, Mm, PdfDocument, PdfLayerReference};

use crate::image::helpers;

// ── Public entry point ──────────────────────────────────────────────────────

pub fn metadata(input: &str, report: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = Path::new(input);

    if !input_path.exists() {
        return Err(format!("Input file not found: {input}").into());
    }

    let ext = input_path
        .extension()
        .and_then(OsStr::to_str)
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    if !helpers::is_supported_input(&ext) {
        return Err(format!("Unsupported format: '.{ext}'").into());
    }

    // ── Collect all metadata ─────────────────────────────────────────────
    let file_size = std::fs::metadata(input_path)?.len();
    let img = helpers::load_image(input_path)?;
    let (width, height) = img.dimensions();
    let color_type = img.color();
    let format_name = helpers::format_name(&ext);

    let exif_opt = parse_exif(input_path);
    let meta = ImageMetadata {
        path: input_path,
        file_size,
        format_name,
        width,
        height,
        color_type,
        exif: exif_opt.as_ref(),
    };

    // ── Display to stdout ────────────────────────────────────────────────
    print_metadata(&meta);

    // ── PDF Report ───────────────────────────────────────────────────────
    if let Some(report_path) = report {
        generate_pdf_report(&meta, report_path)?;
        println!("\n ✅ PDF report saved to: {report_path}");
    }

    Ok(())
}

/// All metadata collected for one image, shared between the terminal
/// output and the PDF report renderer.
struct ImageMetadata<'a> {
    path: &'a Path,
    file_size: u64,
    format_name: &'a str,
    width: u32,
    height: u32,
    color_type: image::ColorType,
    exif: Option<&'a exif::Exif>,
}

// ── EXIF parsing ────────────────────────────────────────────────────────────

/// Try to parse EXIF metadata from the raw file bytes.
/// Returns `None` if the file has no EXIF data or if parsing fails.
fn parse_exif(path: &Path) -> Option<exif::Exif> {
    let file = std::fs::File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    exif::Reader::new().read_from_container(&mut reader).ok()
}

// ── Terminal output ─────────────────────────────────────────────────────────

fn print_metadata(meta: &ImageMetadata<'_>) {
    println!();
    println!(" ╔══════════════════════════════════════════════╗");
    println!(" ║         Image Metadata Report                ║");
    println!(" ╚══════════════════════════════════════════════╝");
    println!();

    // ── File Info ────────────────────────────────────────────────────────
    println!(" 📁  File");
    println!("    Path:      {}", meta.path.display());
    if let Ok(canonical) = meta.path.canonicalize() {
        println!("    Resolved:  {}", canonical.display());
    }
    print_file_size(meta.file_size);
    println!("    Format:    {}", meta.format_name);
    println!();

    // ── Image Info ───────────────────────────────────────────────────────
    println!(" 🖼  Image");
    println!("    Dimensions: {} × {} px", meta.width, meta.height);
    let megapixels = (meta.width as f64 * meta.height as f64 / 1_000_000.0 * 10.0).round() / 10.0;
    if megapixels >= 0.1 {
        println!("    Megapixels: {megapixels} MP");
    }
    let bpp = meta.color_type.bits_per_pixel();
    println!("    Color:      {}", color_type_desc(meta.color_type));
    println!("    Bit depth:  {bpp} bpp");
    println!();

    // ── EXIF Data ────────────────────────────────────────────────────────
    if let Some(exif) = meta.exif {
        let fields: Vec<_> = exif.fields().collect();
        if !fields.is_empty() {
            println!(
                " 📷  EXIF Metadata ({nfields} fields)",
                nfields = fields.len()
            );
            for &field in &fields {
                let tag = format!("{:?}", field.tag);
                let value = field.display_value().with_unit(field).to_string();
                println!("    {tag}: {value}");
            }
            println!();
        }
    }
}

fn color_type_desc(ct: image::ColorType) -> &'static str {
    match ct {
        image::ColorType::L8 => "Grayscale (8-bit)",
        image::ColorType::La8 => "Grayscale + Alpha (8-bit)",
        image::ColorType::Rgb8 => "RGB (8-bit)",
        image::ColorType::Rgba8 => "RGBA (8-bit)",
        image::ColorType::L16 => "Grayscale (16-bit)",
        image::ColorType::La16 => "Grayscale + Alpha (16-bit)",
        image::ColorType::Rgb16 => "RGB (16-bit)",
        image::ColorType::Rgba16 => "RGBA (16-bit)",
        image::ColorType::Rgb32F => "RGB (32-bit float)",
        image::ColorType::Rgba32F => "RGBA (32-bit float)",
        _ => "Unknown",
    }
}

fn print_file_size(bytes: u64) {
    let human = if bytes >= 1_000_000_000 {
        format!("{:.2} GB", bytes as f64 / 1_000_000_000.0)
    } else if bytes >= 1_000_000 {
        format!("{:.2} MB", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1_000 {
        format!("{:.2} KB", bytes as f64 / 1_000.0)
    } else {
        format!("{bytes} B")
    };
    println!("    Size:      {human} ({bytes} bytes)");
}

// ── PDF Report ──────────────────────────────────────────────────────────────

fn generate_pdf_report(
    meta: &ImageMetadata<'_>,
    report_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let title = format!(
        "Image Metadata — {}",
        meta.path
            .file_name()
            .map(|s| s.to_string_lossy())
            .unwrap_or(std::borrow::Cow::Borrowed("unknown"))
    );

    // A4 page: 210 x 297 mm
    let (doc, page_idx, layer_idx) = PdfDocument::new(&title, Mm(210.0), Mm(297.0), "Content");
    let page = doc.get_page(page_idx);
    let layer = page.get_layer(layer_idx);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    // Bottom-left origin; A4 height = 297mm
    let mut y = Mm(275.0);

    // ── Title ────────────────────────────────────────────────────────────
    layer.use_text("Image Metadata Report", 20.0, Mm(20.0), y, &bold);
    y -= Mm(13.0);

    // ── File section ─────────────────────────────────────────────────────
    y = pdf_section_header(&layer, "File", y, &bold);
    y = pdf_field(&layer, "Path", &meta.path.display().to_string(), y, &font);
    if let Ok(canonical) = meta.path.canonicalize() {
        y = pdf_field(
            &layer,
            "Resolved",
            &canonical.display().to_string(),
            y,
            &font,
        );
    }
    let size_str = format!("{} bytes", meta.file_size);
    y = pdf_field(&layer, "Size", &size_str, y, &font);
    y = pdf_field(&layer, "Format", meta.format_name, y, &font);
    y -= Mm(5.0);

    // ── Image section ────────────────────────────────────────────────────
    y = pdf_section_header(&layer, "Image", y, &bold);
    let dims = format!("{} × {} px", meta.width, meta.height);
    y = pdf_field(&layer, "Dimensions", &dims, y, &font);
    y = pdf_field(&layer, "Color", color_type_desc(meta.color_type), y, &font);
    let bpp = format!("{} bpp", meta.color_type.bits_per_pixel());
    y = pdf_field(&layer, "Bit depth", &bpp, y, &font);
    y -= Mm(5.0);

    // ── EXIF section ─────────────────────────────────────────────────────
    if let Some(exif) = meta.exif {
        let fields: Vec<_> = exif.fields().collect();
        if !fields.is_empty() {
            let header = format!("EXIF Metadata ({} fields)", fields.len());
            y = pdf_section_header(&layer, &header, y, &bold);
            for &field in &fields {
                let tag = format!("{:?}", field.tag);
                let value = field.display_value().with_unit(field).to_string();
                y = pdf_field(&layer, &tag, &value, y, &font);

                // If we're running out of space, stop
                if y.0 < 30.0 {
                    break;
                }
            }
            y -= Mm(5.0);
        }
    }

    // ── Footer ───────────────────────────────────────────────────────────
    let footer = "Generated by cmdutils";
    y -= Mm(10.0);
    layer.use_text(footer, 8.0, Mm(20.0), y, &font);

    // Save
    let file = std::fs::File::create(report_path)?;
    let mut writer = std::io::BufWriter::new(file);
    doc.save(&mut writer)?;

    Ok(())
}

fn pdf_section_header(layer: &PdfLayerReference, text: &str, y: Mm, bold: &IndirectFontRef) -> Mm {
    let y = y - Mm(8.0);
    layer.use_text(text, 14.0, Mm(20.0), y, bold);
    y - Mm(6.0)
}

fn pdf_field(
    layer: &PdfLayerReference,
    label: &str,
    value: &str,
    y: Mm,
    font: &IndirectFontRef,
) -> Mm {
    let y = y - Mm(5.0);
    let line = format!("{label}: {value}");
    layer.use_text(&line, 9.0, Mm(25.0), y, font);
    y
}
