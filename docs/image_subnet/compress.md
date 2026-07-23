# `compress` — Image Compression

Compresses images to reduce file size. Behaviour differs by format.

## Usage

```
cmdutils image compress <input>          (PNG — max lossless compression)
cmdutils image compress <input> <quality> (JPEG — quality 1–100)
```

| Argument  | PNG | JPEG |
|-----------|-----|------|
| `input`   | Required | Required |
| `quality` | **Not accepted** (error) | Required (1–100) |

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
3. Quality is validated to be in the range 1–100 (inclusive). Values above
   `u8::MAX` (255) are caught at parse time.

```
Compressed JPEG (40677B → 8231B, saved 79.8%): photo_compressed.jpg
```

## Errors

| Condition | Behaviour |
|-----------|-----------|
| PNG with quality | Error: _quality setting is not applicable_ |
| JPEG without quality | Error: _requires a quality value 1–100_ |
| Quality not a number | Error: _Invalid quality value_ |
| Quality out of range | Error: _must be between 1 and 100_ |
| Unsupported format | Error: _Supported: png, jpg, jpeg_ |
| Missing file | Error: _Input file not found_ |
