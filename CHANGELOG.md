<h1 align="center">cmdutils Changelog</h1>

<p align="center"><strong>Documenting every change</strong></p>

----

## v0.0.3 -> Text Subnet & Image Upgrades

### text subnet — new utils
- `count` counts lines, words, characters, and bytes of a file or stdin
  (wc-style, with `-w`/`-l`/`-m`/`-c` flags)
- `case` converts text between upper, lower, title, snake, kebab, camel,
  pascal, and constant letter cases
- `replace` replaces all occurrences of a string, with in-place (`-i`) and
  output-file (`-o`) modes
- `base64` encodes and decodes base64 data from a file or stdin
- `checksum` computes md5, sha256, or sha512 hashes of a file or stdin

### image subnet — new utils
- `crop` crops an image to a `WxH+X+Y` region with bounds validation
- `rotate` rotates an image clockwise in 90° steps
- `grayscale` converts an image to grayscale (Rec. 601 luma)
- `watermark` overlays semi-transparent text on an image using a system font,
  a bundled fallback font, or a custom `--font`, with `--color`, `--opacity`,
  `--position`, and `--size`
- `watermark` falls back to a bundled Roboto font when no system font is
  installed, so it works on font-less systems (e.g. minimal CI containers)
- `strip` removes all embedded metadata (EXIF, comments) from an image

### image subnet — batch & glob processing
- All image utils now accept glob patterns (e.g. `*.png`, `photos/*.jpg`)
- Multiple matches are processed in parallel with bounded concurrency and
  per-file error reporting

  TL;DR: v0.0.3 introduces the `text` subnet, adds five new `image` utils
  (`crop`, `rotate`, `grayscale`, `watermark`, `strip`), and brings glob
  patterns with parallel processing to the entire `image` subnet

---

## v0.0.2 -> Multi-format Support & Metadata

### image subnet — expanded format support
- All three image utils (`convert`, `compress`, `resize`) now support **all** image
  formats that the `image` crate can decode/encode: PNG, JPEG, WebP, BMP, GIF,
  ICO, TIFF, AVIF, PNM, QOI, TGA, OpenEXR, Farbfeld, DDS, HDR
- **SVG** input is now supported (rasterized via resvg) for `convert` and `resize`
- `convert` can now convert between any two supported formats (not just PNG ↔ JPEG)
- `compress` added WebP support (lossless in `image` 0.25) and format-agnostic
  re-encoding for all other formats
- `resize` now preserves the original format on output instead of always PNG/JPEG

### image subnet — new `metadata` util
- Extracts and displays image metadata: file info, dimensions, color type, EXIF
- Parses EXIF data from JPEG, TIFF, and WebP files using `kamadak-exif`
- Supports `--report <path>` flag to generate a formatted A4 PDF report via `printpdf`
- Works with all supported input formats including SVG

---

## v0.0.1 -> Initial Release
- Created CLI structure
- Added the `image` subnet
- Create `compress` util under `image` subnet
- Create `convert` util under `image` subnet
- Create `resize` util under `image` subnet

  TL;DR: This release is the first release of `cmdutils` and aims to introduce basic `image` utils
