<h1 align="center">cmdutils Changelog</h1>

<p align="center"><strong>Documenting every change</strong></p>

----

## v0.0.1 -> Initial Release
- Created CLI structure
- Added the `image` subnet
- Create `compress` util under `image` subnet
- Create `convert` util under `image` subnet
- Create `resize` util under `image` subnet

  TL;DR: This release is the first release of `cmdutils` and aims to introduce basic `image` utils

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
- Works with all supported input formats including SV
