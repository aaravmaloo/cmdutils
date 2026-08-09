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
| `--font`      | Path to a TTF/OTF font file (defaults to a system sans-serif font) |

## Behaviour

1. Loads the source image and rasterizes the text with anti-aliasing.
2. Applies the requested opacity and composites the text over the image.
3. Saves the result as `<stem>_watermarked.<ext>` in the same directory as
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
| No system font found | Error: _No system sans-serif font found. Use --font <path>_ |
