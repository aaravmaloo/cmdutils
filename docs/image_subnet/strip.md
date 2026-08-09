# `strip` — Metadata Removal

Removes all embedded metadata (EXIF, comments, etc.) from an image.

## Usage

```
cmdutils image strip <input>
```

| Argument | Description |
|----------|-------------|
| `input`  | Path to source image (any supported format except SVG) |

## Behaviour

1. Loads the source image (metadata is discarded on decode).
2. Re-encodes the image without any metadata.
   - **JPEG** is re-encoded at quality 90.
   - **PNG** output is re-optimized with oxipng.
3. Saves the result as `<stem>_stripped.<ext>` in the same directory as the
   input.

This is useful for removing EXIF location data from photos before sharing
them.

## Output

```
Stripped metadata from JPEG (2.4MB → 523KB): photo_stripped.jpg
```

## Errors

| Condition | Behaviour |
|-----------|-----------|
| Missing file | Error: _Input file not found_ |
| SVG input | Error: _SVG files are vector-based and have no embedded metadata to strip_ |

> **Note:** stripping re-encodes the image, so it is lossy for JPEG
> (re-encoded at quality 90).
