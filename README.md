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
    └── image                    (subnet)
        ├── resize                   (util)
        ├── compress                 (util)
        └── convert                  (util)
```

This design keeps the core small while making it simple to add new subnets, such as `pdf`, `text`, or `archive`, without touching existing code.

---

## Utils Supported

### `image` subnet

| Util | Description | Status |
|---|---|:---:|
| `resize` | Resize an image to exact dimensions | Stable |
| `compress` | Compress an image with a quality setting (1 to 100) | Stable |
| `convert` | Convert an image between formats (PNG to JPEG) | Stable |

More subnets (`pdf`, `text`, `archive`) are on the roadmap below.

---

## Installation

``` #clone and build from source ```


git clone https://github.com/yourname/cmdutils.git
cd cmdutils
cargo build --release
```

---

## Usage

```bash
cmdutils <subnet> <util> [options]
```

Examples:

```bash
# Resize an image to 800x600
cmdutils image resize input.png --width 800 --height 600 -o output.png

# Compress a JPEG to 70% quality
cmdutils image compress photo.jpg --quality 70 -o photo_compressed.jpg

# Convert PNG to JPEG
cmdutils image convert logo.png --to jpeg -o logo.jpg
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

---

<p align="center">No ads. No trackers. Just utils.</p>
