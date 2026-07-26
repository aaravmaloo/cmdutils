# `metadata` — Image Metadata Extraction

Extracts and displays all available metadata from an image file, with an
optional PDF report.

## Usage

```
cmdutils image metadata <input>
cmdutils image metadata <input> --report <path>
cmdutils image metadata <input> -r <path>
```

| Argument     | Description |
|--------------|-------------|
| `input`      | Path to source image (any supported format — PNG, JPEG, WebP, BMP, GIF, TIFF, AVIF, SVG, etc.) |
| `-r, --report` | Optional path to generate a PDF metadata report |

## Behaviour

1. Loads the source image and extracts metadata across multiple categories:

### 📁 File Info
- Full and resolved (canonical) file path
- File size (human-readable + bytes)
- Detected image format

### 🖼 Image Info
- Pixel dimensions (width × height)
- Megapixel count (when ≥ 0.1 MP)
- Color type and bit depth

### 📷 EXIF Metadata
When present (common in JPEG, TIFF, and WebP files), the following EXIF
fields are extracted:
- Camera make and model
- Date and time taken
- Exposure settings (shutter speed, aperture, ISO)
- Focal length, flash, metering mode
- GPS coordinates (if available)
- Software, orientation, and more

PNG and SVG metadata (text chunks, XML metadata) are currently not extracted.

## Output

### Terminal

```
 ╔══════════════════════════════════════════════╗
 ║         Image Metadata Report                ║
 ╚══════════════════════════════════════════════╝

 📁  File
    Path:      /Users/me/photo.jpg
    Resolved:  /Users/me/photo.jpg
    Size:      2.45 MB (2,567,890 bytes)
    Format:    JPEG

 🖼  Image
    Dimensions: 4000 × 3000 px
    Megapixels: 12.0 MP
    Color:      RGB (8-bit)
    Bit depth:  24 bpp

 📷  EXIF Metadata (12 fields)
    Make: Canon
    Model: Canon EOS R5
    DateTimeOriginal: 2024-06-15 14:30:00
    ExposureTime: 1/250 sec
    FNumber: 2.8
    ISOSpeedRatings: 400
```

### PDF Report

When `--report <path>` is provided, a nicely formatted A4 PDF is generated
containing the same metadata. The PDF uses the Helvetica font family and is
designed for easy printing or sharing.

```
 ✅ PDF report saved to: /Users/me/photo_metadata.pdf
```

## Errors

| Condition | Behaviour |
|-----------|-----------|
| Input file missing | Error: _Input file not found_ |
| Unsupported format | Error: _Unsupported format_ |
