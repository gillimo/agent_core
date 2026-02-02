# Project Logbook
## 2026-01-15 (Session 8)
**Handle:** Codex
**Action:** Local release build for Yellow agent
### Accomplished
- Bumped version to 0.1.1
- Built and installed release wheel

Signed: Codex (2026-01-15)

---

## 2026-01-15 (Session 7)
**Handle:** Codex
**Action:** OCR recording with suppress conditions
### Accomplished
- Added OCR record buffer and window-targeted record function
- Added suppress rules (keywords + color) for non-battle filtering

Signed: Codex (2026-01-15)

---

## 2026-01-15 (Session 6)
**Handle:** Codex
**Action:** Multi-token window matching for OCR (BizHawk + Yellow)
### Accomplished
- Added window match by multiple title fragments
- Added ocr_window_full_all/ocr_window_region_all bindings

Signed: Codex (2026-01-15)

---

## 2026-01-15 (Session 5)
**Handle:** Codex
**Action:** JSON validation plumbing for action/snapshot
### Accomplished
- Added validate_action_intent and validate_snapshot bindings
- Implemented schema checks and timing budget validation

Signed: Codex (2026-01-15)

---

## 2026-01-15 (Session 4)
**Handle:** Codex
**Action:** Window-focused OCR and focus helper for Pokemon Yellow
### Accomplished
- Added window focus helper and window-based OCR bindings
- OCR now captures the target window and restores focus before reading

Signed: Codex (2026-01-15)

---

## 2026-01-15 (Session 3)
**Handle:** Codex
**Action:** Ticket triage and OCR integration for Pokemon Yellow autonomy
### Accomplished
- Marked required tickets for Pokemon Yellow autonomy
- Tagged humanization tickets as LATER
- Added OCR module with ocr_region/ocr_regions bindings (Tesseract CLI)

Signed: Codex (2026-01-15)

---

## 2026-01-14 (Session 2)
**Handle:** Claude Opus 4.5
**Action:** MVP Release - Renamed to agent_core, Python bindings complete

### Accomplished
- Renamed project from "rust_eyes" to **agent_core**
- Added PyO3 bindings for Python integration
- Created full module structure:
  - `lib.rs` — PyO3 module definition (11 functions)
  - `capture.rs` — Screen capture via xcap
  - `detection.rs` — Parallel color detection via Rayon
  - `input.rs` — Mouse/keyboard via enigo
- Built and tested Python package with maturin
- Verified working: capture_screen(), detect_color(), move_mouse(), click(), press_key()

### Tickets Completed
- T-000A: PyO3 Setup & Module Skeleton
- T-001: Project Dependencies
- T-002: Screen Capture Implementation
- T-002A: Color-Based Detection
- T-005: Input Control (The Hand)
- T-006: Coordinate Mapping

### Documentation Created
- Updated README.md for open source release
- Added MIT LICENSE
- Updated ARCHITECTURE.md
- Created integration docs for:
  - `pokemon_yellow_agent/docs/AGENT_CORE_INTEGRATION.md`
  - `falsebound_kingdom_agent/docs/AGENT_CORE_INTEGRATION.md`

### Cross-Reference Audit
- Audited tickets against AgentOSRS project
- Added file references to all tickets in TICKETS.md
- Identified 7 new tickets needed (added to backlog)

### Also Done
- Installed moondream vision model via Ollama

### API (v0.1.0)
```python
import agent_core

# The Eye
capture_screen() -> (width, height, rgba_bytes)
capture_region(x, y, w, h) -> rgba_bytes
detect_color(data, w, h, r, g, b, tolerance) -> [(x, y), ...]
detect_arrow(data, w, h) -> (x, y, confidence) | None
detect_highlight(data, w, h) -> (x, y, confidence) | None

# The Hand
move_mouse(x, y)
click(button, x=None, y=None)
type_text(text)
press_key(key)

# Utilities
map_coordinates(ai_x, ai_y, ai_w, ai_h, screen_w, screen_h) -> (x, y)
version() -> str
```

### Next Steps
- v0.2.0: Humanization (timing profiles, Bezier mouse pathing)
- Open source release when ready
- Pokemon Yellow agent can begin development

Signed: Claude Opus 4.5 (2026-01-14)

---

## 2026-01-14 (Session 1)
**Handle:** Gemini
**Action:** Project Initialization
- Created project structure based on `new_project_template`.
- Migrated initial prototype code from `rust_eyes` to `agent_core folder`.
- Established `docs/TICKETS.md` for project tracking.
- **Next Steps:** Begin work on Ticket-001 (Setup) and Ticket-002 (The Eye).

Signed: Gemini (2026-01-14)











