# Agent Core - Project Tickets

> **Base Path:** `C:/Users/gilli/OneDrive/Desktop/agent_core/`
> **Pokemon Yellow Agent Path:** `C:/Users/gilli/OneDrive/Desktop/projects/pokemon_yellow_agent/`

## Active Sprint - Pokemon Yellow Integration

### T-011: OCR Python API Exposure (IN PROGRESS)
**Priority:** HIGH
**Goal:** Expose existing Tesseract OCR to Python layer for text reading
**Status:** Not Started

**Tasks:**
- [ ] Add OCR re-exports to `python/agent_core/__init__.py`
  - `ocr_region`, `ocr_regions`, `ocr_window_full`
- [ ] Add `TextReader` class to `python/agent_core/models.py`
  - `read_region()` method
  - `read_text_box()` helper for common regions (top/bottom)
- [ ] Update `Spotter.see()` to accept `ocr_text` parameter for enhanced prompts

**Files:**
- `python/agent_core/__init__.py`
- `python/agent_core/models.py`

**Ref:** Rust OCR already implemented in `src/ocr.rs:1-100`, `src/lib.rs:138-206`

---

### T-012: Hierarchical Goal Manager (Pokemon Yellow)
**Priority:** HIGH
**Goal:** Add Phi3-powered goal tracking (short-term + medium-term)
**Status:** Not Started

**Tasks:**
- [ ] Create `pokemon_yellow_agent/src/goal_manager.py`
- [ ] Implement `GoalManager` class with:
  - `update_goals()` - Uses Phi3 to analyze progress and update goals
  - `get_goal_context()` - Format goals for Executor
  - `is_stuck()` - Detect when position/text unchanging
  - `get_unstuck_action()` - Suggest recovery action
- [ ] Track context history (last 5 steps)
- [ ] Parse Phi3 responses for SHORT_TERM/MEDIUM_TERM goals

**Files:**
- `pokemon_yellow_agent/src/goal_manager.py` (NEW)

**Why:** Agent currently has no goal tracking, defaults to random actions

---

### T-013: Pokemon Yellow Agent - OCR Integration
**Priority:** HIGH
**Goal:** Integrate OCR + Goals into main agent loop
**Status:** Not Started

**Tasks:**
- [ ] Import `agent_core` OCR functions + `GoalManager`
- [ ] Add `extract_game_text()` function for dialogue/menu OCR
- [ ] Update main loop to:
  1. Capture screen
  2. OCR text regions (top/bottom)
  3. Vision with OCR context
  4. Update goals based on progress
  5. Check stuck detection
  6. Executor decides with full context
- [ ] Enhanced logging (show OCR text, goals, stuck status)

**Files:**
- `pokemon_yellow_agent/run_agent.py`

**Expected Result:** Agent reads "Welcome to Pokemon!" instead of hallucinating, progresses past map 18

---

## Backlog

### Phase 0: Python Integration (COMPLETED)

- [x] **T-000A: PyO3 Setup & Module Skeleton**
    - ✅ Python bindings via PyO3/maturin
    - ✅ `Cargo.toml` configured with `crate-type = ["cdylib"]`
    - ✅ `pyproject.toml` for maturin builds
    - **Ref:** `src/lib.rs:334-360` (PyModule definition)

---

### Phase 1: Foundation & The Eye (COMPLETED)

- [x] **T-001: Project Skeleton & Dependencies**
    - ✅ `Cargo.toml` configured with all dependencies
    - ✅ Compiles successfully
    - ✅ Logging via `env_logger`
    - **Deps:** pyo3, rayon, xcap, image, enigo, candle, etc.
    - **Ref:** `Cargo.toml`, `Cargo.lock`

- [x] **T-002: Screen Capture Implementation (The Eye)**
    - ✅ `capture_region(x, y, width, height)` exposed
    - ✅ `capture_screen()` exposed
    - ✅ `capture_window_by_title()` exposed
    - ✅ Fast capture via xcap (~10-30ms)
    - **Ref:** `src/capture.rs:1-300` (full implementation)

---

### Phase 1.5: Visual Detection (COMPLETED)

- [x] **T-002A: Color-Based Detection**
    - ✅ `detect_color(img_data, r, g, b, tolerance)` exposed
    - ✅ `detect_arrow()` for yellow quest markers
    - ✅ `detect_highlight()` for cyan interactive elements
    - ✅ Rayon parallel pixel scanning
    - **Ref:** `src/detection.rs:1-130`

- [x] **T-002B: OCR Integration (Rust only, not Python-exposed yet)**
    - ✅ Tesseract integration via subprocess
    - ✅ `ocr_region()`, `ocr_regions()` implemented in Rust
    - ✅ `ocr_window_full()`, `ocr_window_region()` implemented
    - ⚠️ **NOT exposed to Python layer yet** (T-011 will fix this)
    - **Ref:** `src/ocr.rs:1-100`, `src/lib.rs:138-206`

---

### Phase 2: The Brain (Local AI) (COMPLETED)

- [x] **T-003: Moondream Model Loader**
    - ✅ `Brain` struct loads `moondream2` via candle
    - ✅ Model weights cached/downloaded from HuggingFace
    - ✅ Load time acceptable, memory usage reasonable
    - **Ref:** `src/brain.rs:1-150`

- [x] **T-004: Inference Pipeline**
    - ✅ `see_and_think(image, prompt)` method implemented
    - ✅ Token generation loop with greedy search
    - ⚠️ **Performance Issue:** ~60-120s per inference (CPU-bound)
    - **Ref:** `src/brain.rs`, `python/agent_core/models.py:62-108`

---

### Phase 3: The Hand (Input Control) (COMPLETED)

- [x] **T-005: Input Control (The Hand)**
    - ✅ `move_mouse(x, y)` exposed
    - ✅ `click(button, x, y)` exposed
    - ✅ `press_key(key)` exposed
    - ✅ `type_text(text)` exposed
    - ✅ Using `enigo` crate for cross-platform input
    - **Ref:** `src/input.rs:1-150`, `src/lib.rs:303-321`

- [x] **T-006: Coordinate Mapping**
    - ✅ `map_coordinates()` exposed for AI->Screen mapping
    - ✅ Handles DPI scaling
    - **Ref:** `src/lib.rs:324-328`

---

### Phase 4: Integration (COMPLETED)

- [x] **T-007: Python Agent Classes**
    - ✅ `Spotter` class (Moondream vision via Ollama)
    - ✅ `Executor` class (Phi3 reasoning via Ollama)
    - ✅ `Agent` class (combined Spotter + Executor)
    - **Ref:** `python/agent_core/models.py:1-267`

---

## Future Enhancements

### Phase 5: Optimization

- [ ] **T-014: Performance Tuning - Moondream Inference**
    - **Current:** 60-120s per vision inference (CPU)
    - **Goal:** <10s per inference
    - **Options:**
      - GPU acceleration (CUDA/ROCm)
      - Quantization (f16, q8, q4)
      - Smaller vision model (LLaVA-tiny, Florence)
      - Cache identical frames
    - **Impact:** Pokemon Yellow agent currently takes ~1min per step

- [ ] **T-015: Humanization - Timing Profiles**
    - Natural timing variance for input actions
    - Gaussian sampling with min/max bounds
    - Profiles: NORMAL, FAST, SLOW, CAREFUL, AGGRESSIVE
    - **Use case:** Make agent look more human for anti-bot detection

- [ ] **T-016: Humanization - Mouse Pathing**
    - Bezier curve mouse movement
    - Easing functions, tremor/jitter
    - **Use case:** Natural cursor movement

---

## Completed Recently

- [x] **T-010: Agent Core Setup** (2025-01-14)
    - Created agent_core as standalone Python package
    - All I/O capabilities (capture, detection, input, OCR, vision)
    - Integrated into Pokemon Yellow agent

---

## Notes

### Runtime Performance Issue
**Problem:** Each agent step takes ~60-120 seconds
- Moondream inference: ~60-90s (CPU-bound, candle inference)
- Phi3 inference: ~20-40s (via Ollama)
- OCR: <1s (Tesseract)
- Capture: <0.1s (xcap)

**Mitigation (short-term):**
- Use OCR for text instead of relying on Moondream to read small text
- Cache Moondream observations if screen unchanged
- Run goal updates less frequently (every 3-5 steps)

**Solution (long-term):** GPU acceleration or smaller models (T-014)
