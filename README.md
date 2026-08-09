<h1 align="center">cmdutils</h1>

<p align="center"> <strong> A fast, zero-bloat CLI utility toolbox, written in Rust.</p>

<p align="center">
<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/built%20with-Rust-orange?logo=rust&logoColor=white" alt="Rust"></a>
<a href="#license"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
<a href="#why-cmdutils"><img src="https://img.shields.io/badge/telemetry-none-brightgreen" alt="No Telemetry"></a>
<a href="#why-cmdutils"><img src="https://img.shields.io/badge/adware-none-brightgreen" alt="No Adware"></a>
<a href="#contributing"><img src="https://img.shields.io/badge/PRs-welcome-ff69b4.svg" alt="PRs Welcome"></a>
</p>

<p align="center">One binary. Every util you actually use. Nothing you don't.</p>

---

## Table of Contents

- [Why cmdutils](#why-cmdutils)
- [Architecture](#architecture)
- [Utils Supported](#utils-supported)
- [Installation](#installation)
- [Usage](#usage)
- [Roadmap](#roadmap)
- [Using cmdutils as a Dependency](#using-cmdutils-as-a-dependency)
- [Contributing](#contributing)
- [License](#license)

---

## Why cmdutils

The internet is full of "free" PDF, image, and file tools that come wrapped in pop up ads, tracking scripts, and a GUI you have to fight with just to compress one image.

`cmdutils` takes a different approach:

| | Web Tools | GUI Apps | cmdutils |
|---|:---:|:---:|:---:|
| Works offline | No | Yes | Yes |
| Zero telemetry | No | Sometimes | Yes |
| Zero adware | No | Sometimes | Yes |
| Scriptable / pipeable | No | No | Yes |
| Single static binary | No | No | Yes |
| Embeddable in your app | No | No | Yes |

---

## Architecture

`cmdutils` splits its core into two layers:

- `subnet`: a category of utilities for a given domain (for example `image`, `pdf`, `text`)
- `util`: a single command inside a subnet (for example `compress` inside `image`)

```
cmdutils
└── subnets
    ├── image                    (subnet)
    │   ├── resize                   (util)
    │   ├── compress                 (util)
    │   ├── convert                  (util)
    │   ├── metadata                 (util)
    │   ├── crop                     (util)
    │   ├── rotate                   (util)
    │   ├── grayscale                (util)
    │   ├── watermark                (util)
    │   └── strip                    (util)
    └── text                     (subnet)
        ├── count                    (util)
        ├── case                     (util)
        ├── replace                  (util)
        ├── base64                   (util)
        └── checksum                 (util)
```

This design keeps the core small while making it simple to add new subnets, such as `pdf` or `archive`, without touching existing code.

---

## Utils Supported

### `image` subnet

| Util | Description | Status |
|---|---|:---:|
| `resize` | Resize an image to exact dimensions (any format) | Stable |
| `compress` | Compress an image (PNG: lossless max; JPEG/WebP: quality 1–100; others: re-encode) | Stable |
| `convert` | Convert between any supported formats (PNG, JPEG, WebP, BMP, GIF, TIFF, AVIF, SVG, and more) | Stable |
| `metadata` | Extract and display image metadata (EXIF, dimensions, color); optional PDF report | Stable |
| `crop` | Crop an image to a `WxH+X+Y` region | Stable |
| `rotate` | Rotate clockwise in 90° steps | Stable |
| `grayscale` | Convert an image to grayscale | Stable |
| `watermark` | Overlay semi-transparent text on an image (bundled fallback font — works with no system fonts) | Stable |
| `strip` | Remove all embedded metadata (EXIF, comments) | Stable |

All image utils accept **glob patterns** (e.g. `*.png`) and process matches in **parallel**.

### `text` subnet

| Util | Description | Status |
|---|---|:---:|
| `count` | Count lines, words, characters, and bytes (file or stdin) | Stable |
| `case` | Convert between upper, lower, title, snake, kebab, camel, pascal, constant | Stable |
| `replace` | Find & replace (in-place, output file, or stdout) | Stable |
| `base64` | Encode / decode base64 (file or stdin) | Stable |
| `checksum` | Compute md5, sha256, or sha512 hashes | Stable |

More subnets (`pdf`, `archive`) are on the roadmap.

---

## Installation``` #clone and build from source ```


git clone https://github.com/aaravmaloo/cmdutils.git
cd cmdutils
cargo build --release
```
``` #install via winget  ```
winget install aaravmaloo.cmdutils
```
``` #install via AUR  ```
yay -S cmdutils-bin
```


---

## Usage

```bash
cmdutils <subnet> <util> [options]
```

Examples:

```bash
# Resize an image to 800x600
cmdutils image resize input.png 800x600

# Compress a JPEG to 70% quality
cmdutils image compress photo.jpg 70

# Convert PNG to JPEG
cmdutils image convert logo.png jpg

# Crop a 400x300 region from a photo
cmdutils image crop photo.png 400x300+50+40

# Watermark every PNG in a folder (parallel batch)
cmdutils image watermark '*.png' --text "© 2026"

# Count words in a file
cmdutils text count notes.txt

# Convert to snake_case from stdin
cmdutils text case --to snake --text "Hello World"

# Verify a download
cmdutils text checksum file.iso --algo sha256
```

---


## Contributing

Contributions, issues, and feature requests are welcome.

1. Fork the repo
2. Create your feature branch (`git checkout -b feature/new-subnet`)
3. Commit your changes
4. Open a PR

---

## License

Licensed under the MIT License. See [`LICENSE`](./LICENSE) for details.

The bundled `watermark` font (Roboto, latin subset) is © Google and licensed
under the [Apache License 2.0](./assets/fonts/LICENSE-Apache-2.0.txt).

---

<p align="center">No ads. No trackers. Just utils.</p>
