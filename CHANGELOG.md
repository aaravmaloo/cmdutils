# cmdutils Changelog

## v0.0.3 - Text Subnet & Image Upgrades

The `text` subnet is here, along with a few new image tools and glob support
across the whole `image` subnet.

**New: the text subnet**

- `count` - wc-style line, word, character, and byte counting for a file or
  stdin. Use `-w`, `-l`, `-m`, or `-c` to narrow it down to just what you
  need.
- `case` - convert text between upper, lower, title, snake, kebab, camel,
  pascal, and constant case.
- `replace` - replace every occurrence of a string, either in place (`-i`)
  or written out to a new file (`-o`).
- `base64` - encode or decode base64 data, from a file or stdin.
- `checksum` - md5, sha256, or sha512 hash of a file or stdin.

**New: image utils**

- `crop` - cut an image down to a `WxH+X+Y` region, with bounds checking so
  you can't crop past the edge.
- `rotate` - rotate clockwise in 90 degree steps.
- `grayscale` - convert to grayscale using Rec. 601 luma weights.
- `watermark` - overlay semi-transparent text with configurable color,
  opacity, position, and font size. It uses a system font when one is
  available, a bundled Roboto otherwise, or any TTF/OTF you pass with
  `--font`.
- `strip` - remove all embedded metadata (EXIF, comments, etc.) from an
  image.

**Glob patterns and parallel processing**

All image utils accept glob patterns now (`*.png`, `photos/*.jpg`), process
the matching files in parallel, and report errors per file instead of
stopping at the first bad one.

## v0.0.2 - Multi-format Support & Metadata

- `convert`, `compress`, and `resize` now work with basically every format
  the `image` crate supports: PNG, JPEG, WebP, BMP, GIF, ICO, TIFF, AVIF,
  PNM, QOI, TGA, OpenEXR, Farbfeld, DDS, and HDR.
- SVG input is supported for `convert` and `resize` (rasterized with resvg).
- `convert` can now convert between any two formats, not just PNG and JPEG.
- `compress` gained WebP support (lossless, since `image` 0.25 doesn't do
  lossy WebP yet) and can re-encode any other format it's given.
- `resize` keeps the original format on output instead of always writing
  PNG or JPEG.
- New `metadata` util that prints file info, dimensions, color type, and
  EXIF data (parsed with `kamadak-exif`), and can write a formatted A4 PDF
  report via `--report`.

## v0.0.1 - Initial Release

First release. Sets up the CLI structure with the `image` subnet and three
utils: `convert`, `compress`, and `resize`.
