# `convert` — Image Format Conversion

Converts images between any two supported formats.

## Usage

```
cmdutils image convert <input> <format>
```

| Argument | Description |
|----------|-------------|
| `input`  | Path to source image (`.png`, `.jpg`, `.jpeg`, `.webp`, `.bmp`, `.gif`, `.tiff`, `.avif`, `.svg`, and more) |
| `format` | Target format: `png`, `jpg`, `jpeg`, `webp`, `bmp`, `gif`, `ico`, `tiff`, `avif`, `pnm`, `qoi`, `tga`, etc. |

## Supported Formats

| Format     | Input (read) | Output (write) |
|------------|:------------:|:---------------:|
| PNG        | ✅           | ✅ (oxipng-optimized) |
| JPEG       | ✅           | ✅ (quality 90) |
| WebP       | ✅           | ✅               |
| BMP        | ✅           | ✅               |
| GIF        | ✅           | ✅               |
| ICO        | ✅           | ✅               |
| TIFF       | ✅           | ✅               |
| AVIF       | ✅           | ✅               |
| PNM        | ✅           | ✅               |
| QOI        | ✅           | ✅               |
| TGA        | ✅           | ✅               |
| OpenEXR    | ✅           | ✅               |
| Farbfeld   | ✅           | ✅               |
| DDS        | ✅           | ✅               |
| HDR        | ✅           | ✅               |
| **SVG**    | ✅ (rasterized) | ❌ (vector → raster only) |

## Behaviour

1. The source image is decoded (SVG files are rendered to a raster at their
   native viewport size).
2. The result is encoded in the target format.
3. Format-specific optimisations are applied automatically:
   - **PNG** output is post-processed with **oxipng** (max compression,
     Zopfli + parallel).
   - **JPEG** output uses quality 90 (`JpegEncoder::new_with_quality`).
   - **WebP** output uses the built-in WebP encoder.
   - All other formats are written via the `image` crate's default encoder.
4. If the source and target format are the same, an error is returned.

## Output

The output file is written alongside the input with the new extension.
A size comparison is printed:

```
Converted PNG → JPEG (512596B → 52341B, -89.8%): photo.jpg
Converted PNG → WebP (512596B → 40123B, -21.7%): photo.webp
Converted SVG → PNG (2450B → 12400B, +406.1%): logo.png
```

## Errors

| Condition | Behaviour |
|-----------|-----------|
| Input file missing | Error: _Input file not found_ |
| Unsupported input format | Error: _Unsupported input format_ |
| Unsupported output format | Error: _Unsupported output format_ |
| Source and target are the same | Error: _same format; nothing to do_ |
