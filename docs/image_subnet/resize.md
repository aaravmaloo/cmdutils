# `resize` — Image Resizing

Resizes an image to exact pixel dimensions.

## Usage

```
cmdutils image resize <input> <dimensions>
```

| Argument     | Description |
|--------------|-------------|
| `input`      | Path to source image |
| `dimensions` | Target size in `WxH` format (e.g. `800x600`, `1920X1080`) |

## Behaviour

1. Opens the source image using the `image` crate.
2. Resizes using **Lanczos3** filtering via
   `image::imageops::resize_exact()` — the highest-quality downsampling
   filter available in the crate.
3. Output encoding depends on the input format:

   **PNG input** — saves as PNG then runs **oxipng** at max compression
   (`Options::max_compression()`, Zopfli + parallel) to minimise file size.

   **JPEG input** — encodes directly with
   `image::codecs::jpeg::JpegEncoder::new_with_quality()` at **quality 90**.

4. The output is saved as `<stem>_resized.<ext>` in the same directory as
   the input.

## Output

A size and dimension summary is printed:

```
Resized 850x566 → 400x300 (512596B → 182341B, -64.4%): photo_resized.png
```

## Errors

| Condition | Behaviour |
|-----------|-----------|
| Missing file | Error: _Input file not found_ |
| Bad dimension format | Error: _Invalid dimensions_ |
| Non-numeric width/height | Error: _Invalid width_ / _Invalid height_ |

## Technical Notes

- **Lanczos3** is a sinc-based windowed filter that preserves sharpness
  better than bilinear or bicubic when downscaling.
- The output always keeps the same format as the input — conversion is
  handled by the `convert` command.
- oxipng post-processing for PNG outputs applies Zopfli DEFLATE, filter
  strategy trials, and colour-type reduction.
