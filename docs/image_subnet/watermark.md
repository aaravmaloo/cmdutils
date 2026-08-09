# `watermark` — Text Watermark

Overlays semi-transparent text on an image.

## Usage

```
cmdutils image watermark <input> --text <text> [options]
```

| Argument      | Description |
|---------------|-------------|
| `input`       | Path to source image (any supported format) |
| `--text`      | Watermark text (required) |
| `--color`     | Text color as 6-digit hex (default `ffffff`) |
| `--opacity`   | Opacity 0–100 (default `60`) |
| `--position`  | `top-left`, `top-right`, `bottom-left`, `bottom-right`, `center` (default `bottom-right`) |
| `--size`      | Font size in pixels (default `48`) |
| `--font`      | Path to a TTF/OTF font file (defaults to a system sans-serif font, then the bundled font) |

## Behaviour

1. Resolves the font: `--font` → a system sans-serif font → the font bundled
   with the binary (Roboto, latin subset).
2. Loads the source image and rasterizes the text with anti-aliasing.
3. Applies the requested opacity and composites the text over the image.
4. Saves the result as `<stem>_watermarked.<ext>` in the same directory as
   the input.

## Examples

```
cmdutils image watermark photo.png --text "© 2026"
cmdutils image watermark photo.png --text "CONFIDENTIAL" --position center --opacity 30
cmdutils image watermark photo.png --text "Draft" --color ff0000 --size 72 --font /path/to/font.ttf
```

## Errors

| Condition | Behaviour |
|-----------|-----------|
| Empty `--text` | Error: _Watermark text cannot be empty_ |
| Bad color | Error: _Invalid color_ |
| Unknown position | Error: _Unknown position_ |
| No system font found | Falls back to the bundled Roboto font — never an error |
| Bundled font cannot load | Error: _No system sans-serif font found and the bundled fallback failed to load_ |

> The bundled fallback font covers Latin scripts (Basic Latin, Latin-1,
> Latin Extended-A). For non-Latin watermark text, install fonts on your
> system or pass `--font` with a font that covers the glyphs you need.
