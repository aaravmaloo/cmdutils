# `compress` — Image Compression

Compresses images to reduce file size. Behaviour differs by format.

## Usage

```
cmdutils image compress <input>                         (PNG, BMP, GIF, TIFF, etc.)
cmdutils image compress <input> <quality>                (JPEG — quality 1–100)
cmdutils image compress <input> <quality>                (WebP — quality 1–100, optional)
```

| Argument  | PNG / lossless formats | JPEG | WebP |
|-----------|:----------------------:|:----:|:----:|
| `input`   | Required               | Required | Required |
| `quality` | **Not accepted** (error) | Required (1–100) | Accepted but ignored* |

*WebP encoding in `image` 0.25 is **lossless only**; the quality value is
accepted for forwards-compatibility but does not affect output.

## Supported Input Formats

All formats that can be decoded are accepted: PNG, JPEG, WebP, BMP, GIF, ICO,
TIFF, AVIF, PNM, QOI, TGA, OpenEXR, Farbfeld, DDS, HDR.

> **SVG** is not directly compressible — use `convert` to rasterize first.

## Behaviour

### PNG (lossless)

1. Opens the image and saves it as a PNG.
2. Runs **oxipng** in-place at maximum compression level
   (`Options::max_compression()` — level 6) with **Zopfli** DEFLATE and
   **parallel** processing enabled.
3. If a quality argument is provided, an error is returned — PNG is lossless
   and quality is not applicable.

Log output includes size deltas:

```
Compressed PNG (512596B → 482837B, saved 5.8%): photo_compressed.png
```

If the output grows (the source was already well-optimized), a warning is
printed:

```
Compressed PNG (512596B → 524581B, grew 2.3%): photo_compressed.png
  ⚠  Output is 2.3% larger — source may already be well-optimized
```

### JPEG (lossy)

1. Opens the image and re-encodes it as JPEG at the user-specified quality.
2. Uses `image::codecs::jpeg::JpegEncoder::new_with_quality()`.
3. Quality is validated to be in the range 1–100 (inclusive).

```
Compressed JPEG (40677B → 8231B, saved 79.8%): photo_compressed.jpg
```

### WebP (lossless)

1. Opens the image and re-encodes as **lossless WebP**.
2. The `image` crate in this version does not expose lossy WebP encoding;
   quality values are accepted but ignored.
3. Alpha channels are preserved.

```
Compressed WebP (512596B → 38921B, saved 24.1%): photo_compressed.webp
```

### Other formats (BMP, GIF, TIFF, etc.)

1. Opens the image and re-saves it in the same format.
2. Quality is not applicable — an error is returned if provided.

## Errors

| Condition | Behaviour |
|-----------|-----------|
| PNG with quality | Error: _quality setting is not applicable_ |
| JPEG without quality | Error: _requires a quality value 1–100_ |
| WebP with invalid quality | Error: _Invalid quality value_ |
| Quality not a number | Error: _Invalid quality value_ |
| Quality out of range | Error: _must be between 1 and 100_ |
| Unsupported format | Error: _Unsupported format_ |
| SVG input | Error: _cannot be lossily compressed_ |
| Missing file | Error: _Input file not found_ |
