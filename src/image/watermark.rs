use ab_glyph::{FontVec, PxScale};
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};

use crate::image::helpers;

/// Overlay semi-transparent text on an image.
///
/// * `text` — watermark text.
/// * `opacity` — 0–100 (text transparency).
/// * `position` — `top-left`, `top-right`, `bottom-left`, `bottom-right`, `center`.
/// * `size` — font size in pixels.
/// * `color_hex` — 6-digit hex color (e.g. `ffffff`).
/// * `font_path` — optional TTF/OTF file; falls back to a system font.
#[allow(clippy::too_many_arguments)]
pub fn watermark(
    input: &str,
    text: &str,
    opacity: u8,
    position: &str,
    size: u32,
    color_hex: &str,
    font_path: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if text.is_empty() {
        return Err("Watermark text cannot be empty".into());
    }

    let opacity = opacity.clamp(0, 100);
    let (r, g, b) = parse_hex_color(color_hex)?;

    let (input_path, img, input_ext) = helpers::load_validated(input)?;
    let (w, h) = img.dimensions();

    let font = load_font(font_path)?;
    // Keep a readable minimum size, but never exceed the image height.
    let font_size = size.clamp(8, h.max(8));
    let scale = PxScale::from(font_size as f32);

    let (tw, th) = imageproc::drawing::text_size(scale, &font, text);
    let margin = 20u32;

    let (tx, ty) = match position {
        "top-left" => (margin, margin),
        "top-right" => (w.saturating_sub(tw + margin), margin),
        "bottom-left" => (margin, h.saturating_sub(th + margin)),
        "bottom-right" => (w.saturating_sub(tw + margin), h.saturating_sub(th + margin)),
        "center" => (w.saturating_sub(tw) / 2, h.saturating_sub(th) / 2),
        other => {
            return Err(
                format!("Unknown position '{other}'. Valid: top-left, top-right, bottom-left, bottom-right, center")
                    .into(),
            )
        }
    };

    // Draw the text onto a transparent overlay, then apply global opacity
    // and alpha-composite it over the image.
    let mut overlay = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0]));
    imageproc::drawing::draw_text_mut(
        &mut overlay,
        Rgba([r, g, b, 255]),
        tx as i32,
        ty as i32,
        scale,
        &font,
        text,
    );

    for px in overlay.pixels_mut() {
        px[3] = ((px[3] as u32) * (opacity as u32) / 100) as u8;
    }

    let mut base = img.to_rgba8();
    image::imageops::overlay(&mut base, &overlay, 0, 0);

    let out = DynamicImage::ImageRgba8(base);
    let ext = helpers::output_ext(&input_ext);
    let output_path = helpers::suffixed_path(&input_path, &ext, "watermarked");

    helpers::save_with_quality(&out, &output_path, None)?;

    let fmt_name = helpers::format_name(&ext);
    println!(
        "Watermarked {fmt_name} with '{text}' (opacity {opacity}%, {position}): {}",
        output_path.display()
    );

    Ok(())
}

fn parse_hex_color(hex: &str) -> Result<(u8, u8, u8), Box<dyn std::error::Error>> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(
            format!("Invalid color '{hex}'. Use 6-digit hex, e.g. ffffff or #000000").into(),
        );
    }
    let byte = |i: usize| -> Result<u8, Box<dyn std::error::Error>> {
        u8::from_str_radix(&hex[i..i + 2], 16).map_err(|_| format!("Invalid color '{hex}'").into())
    };
    Ok((byte(0)?, byte(2)?, byte(4)?))
}

fn load_font(font_path: Option<&str>) -> Result<FontVec, Box<dyn std::error::Error>> {
    if let Some(path) = font_path {
        let data = std::fs::read(path).map_err(|e| format!("Could not read font file: {e}"))?;
        return FontVec::try_from_vec(data).map_err(|e| format!("Invalid font file: {e}").into());
    }

    // Fall back to a system sans-serif font.
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    let query = fontdb::Query {
        families: &[fontdb::Family::SansSerif],
        ..Default::default()
    };
    let id = db
        .query(&query)
        .ok_or("No system sans-serif font found. Use --font <path> to specify a font file.")?;
    // Note: `with_face_data` returns the source file's bytes; `FontVec` always
    // parses face 0, so fonts stored at face index > 0 in a collection (e.g.
    // Noto CJK) may render as a different face. Acceptable for the common case.
    let data = db
        .with_face_data(id, |data, _| data.to_vec())
        .ok_or("Could not load system font data")?;
    FontVec::try_from_vec(data).map_err(|e| format!("Invalid system font: {e}").into())
}
