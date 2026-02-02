# agent_core

## What It Is

Start 2026-01-15 — Rust/PyO3 core for screen capture + deterministic input. DXGI capture, parallel pixel detection, and native input execution built as a reusable agent substrate. Purpose-built for high-FPS perception loops, low-latency action pipelines, and reliability under load.

## How It Works

Start 2026-01-15 — Rust/PyO3 core for screen capture + deterministic input. DXGI capture, parallel pixel detection, and native input execution built as a reusable agent substrate. Purpose-built for high-FPS perception loops, low-latency action pipelines, and reliability under load.


Mission Learning Statement
- Mission: Build a high-performance capture + input core for agent systems.
- Learning focus: Rust/PyO3 FFI, low-latency screen capture, deterministic input control.
- Project start date: 2026-01-15 (inferred from earliest git commit)

High-performance screen capture, color detection, and input control for Python — powered by Rust.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

## Features

- **Fast Screen Capture** — Capture full screen or regions at 30-60 FPS
- **Parallel Color Detection** — Find pixels by RGB with tolerance (Rayon-powered)
- **Input Control** — Mouse movement, clicks, keyboard input via native APIs
- **Zero Python Dependencies** — Pure Rust compiled to a Python extension

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/agent_core.git
cd agent_core

# Install with maturin (requires Rust toolchain)
pip install maturin
maturin develop --release
```

### Requirements

- Python 3.8+
- Rust 1.70+ (for building)
- Windows 10/11 (Linux/macOS support planned)

## Quick Start

```python
import agent_core

# Capture the screen
width, height, frame = agent_core.capture_screen()
print(f"Captured {width}x{height}")

# Find yellow pixels
yellows = agent_core.detect_color(frame, width, height, 255, 255, 0, 30)
print(f"Found {len(yellows)} yellow pixels")

# Move mouse and click
agent_core.move_mouse(500, 300)
agent_core.click("left")

# Type text
agent_core.type_text("Hello, World!")

# Press a key
agent_core.press_key("Return")
```

## API Reference

### Screen Capture (The Eye)

| Function | Description | Returns |
|----------|-------------|---------|
| `capture_screen()` | Capture full primary monitor | `(width, height, rgba_bytes)` |
| `capture_region(x, y, w, h)` | Capture screen region | `rgba_bytes` |

### Color Detection (The Eye)

| Function | Description | Returns |
|----------|-------------|---------|
| `detect_color(data, w, h, r, g, b, tol)` | Find pixels matching RGB ± tolerance | `[(x, y), ...]` |
| `detect_arrow(data, w, h)` | Find yellow arrow/marker | `(x, y, confidence)` or `None` |
| `detect_highlight(data, w, h)` | Find cyan highlight | `(x, y, confidence)` or `None` |

### Input Control (The Hand)

| Function | Description |
|----------|-------------|
| `move_mouse(x, y)` | Move cursor to absolute position |
| `click(button, x=None, y=None)` | Click "left", "right", or "middle" |
| `type_text(text)` | Type a string |
| `press_key(key)` | Press key by name |

**Supported Keys:** `return`, `escape`, `tab`, `space`, `backspace`, `delete`, `up`, `down`, `left`, `right`, `f1`-`f12`, `shift`, `control`, `alt`, or any single character.

### Utilities

| Function | Description |
|----------|-------------|
| `map_coordinates(ai_x, ai_y, ai_w, ai_h, screen_w, screen_h)` | Scale coordinates |
| `version()` | Get library version |

## Performance

| Operation | Latency |
|-----------|---------|
| `capture_screen()` | 10-30ms |
| `capture_region()` | 5-15ms |
| `detect_color()` | 5-15ms |
| `move_mouse()` | <1ms |
| `click()` | <5ms |
| `press_key()` | <5ms |

## Use Cases

- **Game Automation** — Capture game screens, detect UI elements, send inputs
- **Testing** — Automated UI testing with visual verification
- **Accessibility** — Screen readers and interaction tools
- **Research** — Vision-based AI agent development

## Architecture

```
┌─────────────────────────────────────────┐
│           Python Application            │
└─────────────────┬───────────────────────┘
                  │ import agent_core
                  ▼
┌─────────────────────────────────────────┐
│            agent_core (Rust)            │
├─────────────┬─────────────┬─────────────┤
│  capture.rs │  detection  │   input.rs  │
│  (xcap)     │  (rayon)    │   (enigo)   │
└─────────────┴─────────────┴─────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│         Native OS APIs (Windows)        │
│      DXGI, SendInput, User32.dll        │
└─────────────────────────────────────────┘
```

## Roadmap

- [x] v0.1.0 — Screen capture, color detection, input control
- [ ] v0.2.0 — Humanization (timing profiles, Bezier mouse paths)
- [ ] v0.3.0 — OCR integration, template matching
- [ ] v0.4.0 — Hardware detection, SIMD optimization
- [ ] v1.0.0 — Multi-monitor, Linux/macOS support

## Project Structure

```
agent_core/
├── src/
│   ├── lib.rs        # PyO3 module definition
│   ├── capture.rs    # Screen capture
│   ├── detection.rs  # Color detection
│   ├── input.rs      # Mouse/keyboard control
│   ├── eye.rs        # Legacy Eye struct
│   ├── brain.rs      # AI inference (CLI only)
│   └── main.rs       # CLI entry point
├── docs/
│   ├── ARCHITECTURE.md
│   ├── TICKETS.md
│   └── LOGBOOK.md
├── Cargo.toml
├── pyproject.toml
└── README.md
```

## Building

```bash
# Development build
maturin develop

# Release build (optimized)
maturin develop --release

# Build wheel for distribution
maturin build --release
```

## Contributing

Contributions welcome! Please read the [tickets](docs/TICKETS.md) for planned features.

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `cargo test` and `cargo clippy`
5. Submit a pull request

## License

MIT License — see [LICENSE](LICENSE) for details.

## Acknowledgments

- [PyO3](https://pyo3.rs/) — Rust bindings for Python
- [xcap](https://github.com/aspect-rs/xcap) — Cross-platform screen capture
- [enigo](https://github.com/enigo-rs/enigo) — Cross-platform input simulation
- [Rayon](https://github.com/rayon-rs/rayon) — Data parallelism library
