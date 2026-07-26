# `resize` — Image Resizing

Resizes an image to exact pixel dimensions.

## Usage

```
cmdutils image resize <input> <dimensions>
```

| Argument     | Description |
|--------------|-------------|
| `input`      | Path to source image (any supported format — PNG, JPEG, WebP, BMP, GIF, TIFF, AVIF, SVG, etc.) |
| `dimensions` | Target size in `WxH` format (e.g. `800x600`, `1920X1080`) |

## Behaviour

1. Loads the source image using the `image` crate (SVG files are rendered to
   a raster at their native viewport size).
2. Resizes using **Lanczos3** filtering via
   `image::imageops::resize_exact()` — the highest-quality downsampling
   filter available in the crate.
3. The output is written in the same format as the input, with
   format-specific optimisations:
   - **PNG**: Saved then post-processed with **oxipng** (max compression,
     Zopfli + parallel).
   - **JPEG**: Encoded at **quality 90**.
   - **WebP**: Encoded with the built-in WebP encoder.
   - **SVG**: Rasterized to **PNG** (vector → raster). Since SVG can't
     meaningfully preserve vector data after pixel-level resize, the output
     becomes a `.png` file.
   - **All other formats**: Written via the `image` crate's default encoder.
4. The output is saved as `<stem>_resized.<ext>` in the same directory as
   the input.

## Output

A size and dimension summary is printed:

```
Resized PNG 850x566 → 400x300 (512596B → 182341B, -64.4%): photo_resized.png
```

## Errors

| Condition | Behaviour |
|-----------|-----------|
| Missing file | Error: _Input file not found_ |
| Bad dimension format | Error: _Invalid dimensions_ |
| Non-numeric width/height | Error: _Invalid width_ / _Invalid height_ |
| Unsupported format | Error: _Unsupported format_ |

## Technical Notes

- **Lanczos3** is a sinc-based windowed filter that preserves sharpness
  better than bilinear or bicubic when downscaling.
- The output always keeps the same format as the input (SVG → PNG) —
  conversion is handled by the `convert` command.
- oxipng post-processing for PNG outputs applies Zopfli DEFLATE, filter
  strategy trials, and colour-type reduction.
