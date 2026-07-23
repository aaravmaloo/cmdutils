# `convert` — Image Format Conversion

Converts images between PNG and JPEG formats.

## Usage

```
cmdutils image convert <input> <format>
```

| Argument | Description |
|----------|-------------|
| `input`  | Path to source image (`.png`, `.jpg`, `.jpeg`) |
| `format` | Target format: `jpg`, `jpeg`, or `png` |

## Behaviour

### PNG → JPEG

Opens the source PNG, converts to RGB (discarding alpha), and encodes as JPEG
at **quality 90** using `image::codecs::jpeg::JpegEncoder`. This produces a
visually lossless result with significant file-size savings for photographic
content.

### JPEG → PNG

Decodes the JPEG, saves an intermediate PNG, then runs **oxipng** at
maximum compression level (`Options::max_compression()`, Zopfli + parallel
enabled) to produce the smallest possible lossless output.

## Output

The output file is written alongside the input with the new extension.
A size comparison is printed:

```
Converted PNG → JPEG (512596B → 52341B, -89.8%): photo.jpg
```

## Errors

| Condition | Behaviour |
|-----------|-----------|
| Input file missing | Error: _Input file not found_ |
| Unsupported format | Error: _Unsupported output format_ |
| Source and target are the same | Error: _same format; nothing to do_ |

## Dependencies

- `image` — loading, pixel format conversion
- `image::codecs::jpeg::JpegEncoder` — JPEG encoding (quality 90)
- `oxipng` — PNG optimization with Zopfli + parallel
