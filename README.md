# bindu
> বিন্দু — Bengali for "dot" or "point"

A CLI tool that converts SVG files into halftone dot-pattern images. Dot size scales with pixel brightness — bright areas get big dots, transparent areas get none.

---

## Usage

```bash
bindu input.svg output.png
```

---

## Stack

- Rust
- `resvg` — SVG parsing and rendering
- `tiny-skia` — pixel canvas
- `image` — PNG output

---

## How It Works

1. SVG is read from disk and rendered into a pixel buffer
2. A dot grid walks the buffer at fixed spacing
3. Brightness is sampled at each grid point using the standard luminance formula
4. A filled circle is drawn at each point, radius proportional to brightness
5. Result is saved as a PNG

---

## Structure

```
src/
└── main.rs       # CLI args, SVG render, grid loop, dot draw, PNG save
Cargo.toml        # Dependencies
test.svg          # Test input
```

---

## Versions

| Version | Status |
|---|---|

---
