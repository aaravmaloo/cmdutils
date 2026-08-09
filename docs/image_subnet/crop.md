# `crop` — Image Cropping

Crops an image to a rectangular region.

## Usage

```
cmdutils image crop <input> <geometry>
```

| Argument   | Description |
|------------|-------------|
| `input`    | Path to source image (any supported format — PNG, JPEG, WebP, BMP, GIF, TIFF, AVIF, SVG, etc.) |
| `geometry` | Crop region in `WxH+X+Y` format (e.g. `800x600+100+50`) |

## Behaviour

1. Loads the source image.
2. Extracts the region `W`×`H` starting at offset `(+X, +Y)`.
3. Saves the result as `<stem>_cropped.<ext>` in the same directory as the
   input, with the same format-specific optimisations as other utils.

## Output

```
Cropped PNG 1920x1080 → 800x600 at +100+50: photo_cropped.png
```

## Errors

| Condition | Behaviour |
|-----------|-----------|
| Missing file | Error: _Input file not found_ |
| Bad geometry | Error: _Invalid crop geometry_ |
| Zero width/height | Error: _Crop width and height must be greater than 0_ |
| Region out of bounds | Error: _exceeds image bounds_ |
