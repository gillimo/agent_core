# agent_core

Mission Learning Statement
- Mission: Build a high-performance capture + input core for agent systems.
- Learning focus: Rust/PyO3 FFI, low-latency screen capture, deterministic input control.
- Project start date: 2026-01-15 (inferred from earliest git commit)

High-performance screen capture, color detection, and input control for Python â€” powered by Rust.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

## Features

- **Fast Screen Capture** â€” Capture full screen or regions at 30-60 FPS
- **Parallel Color Detection** â€” Find pixels by RGB with tolerance (Rayon-powered)
- **Input Control** â€” Mouse movement, clicks, keyboard input via native APIs
- **Zero Python Dependencies** â€” Pure Rust compiled to a Python extension

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
| `detect_color(data, w, h, r, g, b, tol)` | Find pixels matching RGB Â± tolerance | `[(x, y), ...]` |
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

- **Game Automation** â€” Capture game screens, detect UI elements, send inputs
- **Testing** â€” Automated UI testing with visual verification
- **Accessibility** â€” Screen readers and interaction tools
- **Research** â€” Vision-based AI agent development

## Architecture

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚           Python Application            â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                  â”‚ import agent_core
                  â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚            agent_core (Rust)            â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  capture.rs â”‚  detection  â”‚   input.rs  â”‚
â”‚  (xcap)     â”‚  (rayon)    â”‚   (enigo)   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                  â”‚
                  â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚         Native OS APIs (Windows)        â”‚
â”‚      DXGI, SendInput, User32.dll        â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

## Roadmap

- [x] v0.1.0 â€” Screen capture, color detection, input control
- [ ] v0.2.0 â€” Humanization (timing profiles, Bezier mouse paths)
- [ ] v0.3.0 â€” OCR integration, template matching
- [ ] v0.4.0 â€” Hardware detection, SIMD optimization
- [ ] v1.0.0 â€” Multi-monitor, Linux/macOS support

## Project Structure

```
agent_core/
â”œâ”€â”€ src/
â”‚   â”œâ”€â”€ lib.rs        # PyO3 module definition
â”‚   â”œâ”€â”€ capture.rs    # Screen capture
â”‚   â”œâ”€â”€ detection.rs  # Color detection
â”‚   â”œâ”€â”€ input.rs      # Mouse/keyboard control
â”‚   â”œâ”€â”€ eye.rs        # Legacy Eye struct
â”‚   â”œâ”€â”€ brain.rs      # AI inference (CLI only)
â”‚   â””â”€â”€ main.rs       # CLI entry point
â”œâ”€â”€ docs/
â”‚   â”œâ”€â”€ ARCHITECTURE.md
â”‚   â”œâ”€â”€ TICKETS.md
â”‚   â””â”€â”€ LOGBOOK.md
â”œâ”€â”€ Cargo.toml
â”œâ”€â”€ pyproject.toml
â””â”€â”€ README.md
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

MIT License â€” see [LICENSE](LICENSE) for details.

## Acknowledgments

- [PyO3](https://pyo3.rs/) â€” Rust bindings for Python
- [xcap](https://github.com/aspect-rs/xcap) â€” Cross-platform screen capture
- [enigo](https://github.com/enigo-rs/enigo) â€” Cross-platform input simulation
- [Rayon](https://github.com/rayon-rs/rayon) â€” Data parallelism library
