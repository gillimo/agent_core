# agent_core Architecture

## Overview

**agent_core** is a high-performance Rust library that provides screen capture, color detection, and input control capabilities to Python applications via PyO3 bindings.

## Design Philosophy

1. **Speed First** — All performance-critical operations in Rust
2. **Simple API** — Minimal, intuitive Python interface
3. **Zero Dependencies** — No Python runtime dependencies (all compiled in)
4. **Parallel by Default** — Use Rayon for multi-core color detection

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Python Application                       │
│            (pokemon_yellow_agent, falsebound_kingdom, etc.)  │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              │ import agent_core
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      agent_core (Rust)                       │
│                                                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │   The Eye       │  │   The Hand      │  │  Utilities   │ │
│  │   (Vision)      │  │   (Input)       │  │              │ │
│  ├─────────────────┤  ├─────────────────┤  ├──────────────┤ │
│  │ capture.rs      │  │ input.rs        │  │ lib.rs       │ │
│  │ detection.rs    │  │                 │  │              │ │
│  └────────┬────────┘  └────────┬────────┘  └──────────────┘ │
│           │                    │                             │
│           ▼                    ▼                             │
│  ┌─────────────────┐  ┌─────────────────┐                   │
│  │ xcap            │  │ enigo           │                   │
│  │ (screen grab)   │  │ (input sim)     │                   │
│  └────────┬────────┘  └────────┬────────┘                   │
└───────────┼────────────────────┼────────────────────────────┘
            │                    │
            ▼                    ▼
┌─────────────────────────────────────────────────────────────┐
│                    Windows Native APIs                       │
│                                                              │
│   DXGI (Desktop Duplication)    SendInput / User32.dll      │
│   - GPU buffer capture          - Mouse movement            │
│   - High-speed screen grab      - Keyboard simulation       │
│                                 - Click events              │
└─────────────────────────────────────────────────────────────┘
```

## Module Breakdown

### lib.rs — PyO3 Module Definition

The main entry point that exposes Rust functions to Python:

```rust
#[pymodule]
fn agent_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Capture functions
    m.add_function(wrap_pyfunction!(capture_screen, m)?)?;
    m.add_function(wrap_pyfunction!(capture_region, m)?)?;
    // Detection functions
    m.add_function(wrap_pyfunction!(detect_color, m)?)?;
    // Input functions
    m.add_function(wrap_pyfunction!(move_mouse, m)?)?;
    // ...
}
```

### capture.rs — Screen Capture

Uses `xcap` crate for cross-platform screen capture:

- `capture_full_screen_impl()` — Returns (width, height, RGBA bytes)
- `capture_region_impl(x, y, w, h)` — Returns RGBA bytes for region

**Performance Target:** < 30ms for full screen capture

### detection.rs — Color Detection

Parallel pixel scanning using Rayon:

- `detect_color_impl()` — Find pixels matching RGB ± tolerance
- `find_yellow_arrow_impl()` — Specialized yellow detection with centroid
- `find_cyan_highlight_impl()` — Specialized cyan detection with centroid

**Performance Target:** < 15ms for 1920x1080 scan

### input.rs — Mouse & Keyboard Control

Uses `enigo` crate for input simulation:

- `move_mouse_impl(x, y)` — Absolute cursor positioning
- `click_impl(button, x, y)` — Mouse clicks
- `type_text_impl(text)` — Character-by-character typing
- `press_key_impl(key)` — Key press by name

**Performance Target:** < 5ms per input operation

## Data Flow

### Capture + Detect + Act Loop

```
1. Python calls capture_screen()
   └─> Rust: xcap grabs DXGI buffer
       └─> Returns (width, height, Vec<u8>) to Python

2. Python calls detect_color(frame, ...)
   └─> Rust: Rayon parallel scan
       └─> Returns Vec<(i32, i32)> coordinates

3. Python analyzes results, decides action

4. Python calls click("left", x, y)
   └─> Rust: enigo sends SendInput
       └─> Mouse moves and clicks
```

## Memory Layout

### Frame Data

Screen captures return raw RGBA bytes:

```
Offset:  0    1    2    3    4    5    6    7   ...
Data:   [R0] [G0] [B0] [A0] [R1] [G1] [B1] [A1] ...
         └─ pixel 0 ─┘     └─ pixel 1 ─┘
```

- 4 bytes per pixel (RGBA)
- Row-major order (left to right, top to bottom)
- Total size: `width * height * 4` bytes

### Coordinate System

```
(0,0) ─────────────────────────► X (width)
  │
  │
  │
  │
  ▼
  Y (height)
```

## Future Architecture (Planned)

### v0.2.0 — Humanization Layer

```
┌─────────────────────────────────────────┐
│           Humanization Layer            │
├─────────────────┬───────────────────────┤
│ timing.rs       │ mouse_pathing.rs      │
│ - Profiles      │ - Bezier curves       │
│ - Gaussian RNG  │ - Jitter/tremor       │
│ - Delays        │ - Speed ramps         │
└─────────────────┴───────────────────────┘
         │
         ▼
┌─────────────────────────────────────────┐
│              input.rs                    │
└─────────────────────────────────────────┘
```

### v0.3.0 — OCR Integration

```
┌─────────────────────────────────────────┐
│              ocr.rs                      │
│  - Tesseract FFI bindings               │
│  - Region-based text extraction         │
│  - Parallel multi-region OCR            │
└─────────────────────────────────────────┘
```

## Build Configuration

### Cargo.toml Key Settings

```toml
[lib]
name = "agent_core"
crate-type = ["cdylib", "rlib"]  # Python extension + Rust lib

[profile.release]
opt-level = 3   # Maximum optimization
lto = true      # Link-time optimization
```

### PyO3 Version

Using PyO3 0.22+ for Python 3.8-3.13 support.

## Testing Strategy

### Rust Unit Tests

```bash
cargo test
```

### Python Integration Tests

```python
import agent_core

# Test capture returns valid data
w, h, data = agent_core.capture_screen()
assert w > 0 and h > 0
assert len(data) == w * h * 4

# Test detection finds something
colors = agent_core.detect_color(data, w, h, 128, 128, 128, 50)
assert isinstance(colors, list)
```

## Performance Benchmarks

| Operation | Target | Achieved |
|-----------|--------|----------|
| Full screen capture | <30ms | ~15-25ms |
| Region capture | <15ms | ~5-10ms |
| Color detection (1080p) | <15ms | ~8-12ms |
| Mouse move | <5ms | <1ms |
| Key press | <5ms | <3ms |

## References

- [PyO3 User Guide](https://pyo3.rs/)
- [Rayon Documentation](https://docs.rs/rayon/)
- [Windows DXGI](https://docs.microsoft.com/en-us/windows/win32/direct3ddxgi/dx-graphics-dxgi)
