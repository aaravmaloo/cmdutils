# `rotate` — Image Rotation

Rotates an image clockwise by multiples of 90 degrees.

## Usage

```
cmdutils image rotate <input> <degrees>
```

| Argument  | Description |
|-----------|-------------|
| `input`   | Path to source image (any supported format) |
| `degrees` | Rotation in degrees, clockwise. Must be a multiple of 90 (`90`, `180`, `270`, `-90`, ...) |

## Behaviour

1. Loads the source image.
2. Rotates clockwise by the requested angle (90° steps; negative values rotate
   counter-clockwise).
3. Saves the result as `<stem>_rotated.<ext>` in the same directory as the
   input.

## Output

```
Rotated JPEG 90° (1920x1080 → 1080x1920): photo_rotated.jpg
```

## Errors

| Condition | Behaviour |
|-----------|-----------|
| Missing file | Error: _Input file not found_ |
| Not a multiple of 90 | Error: _Rotation must be a multiple of 90 degrees_ |
| Full turn (360°) | Error: _nothing to do_ |
