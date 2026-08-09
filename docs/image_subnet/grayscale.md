# `grayscale` — Grayscale Conversion

Converts an image to grayscale.

## Usage

```
cmdutils image grayscale <input>
```

| Argument | Description |
|----------|-------------|
| `input`  | Path to source image (any supported format) |

## Behaviour

1. Loads the source image.
2. Converts to grayscale using the standard Rec. 601 luma coefficients.
3. Saves the result as `<stem>_grayscale.<ext>` in the same directory as the
   input. The alpha channel is preserved.

## Output

```
Grayscaled PNG 1920x1080: photo_grayscale.png
```
