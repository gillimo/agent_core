"""Agent Core - Eyes, Brain, and Hands for AI agents

Rust core provides: capture, detection, input, OCR
Python layer provides: spotter (vision), executor (reasoning)
"""

# Import Rust bindings
from agent_core import agent_core as _core

# Re-export Rust functions
capture_screen = _core.capture_screen
capture_region = _core.capture_region
detect_color = _core.detect_color
detect_arrow = _core.detect_arrow
detect_highlight = _core.detect_highlight
move_mouse = _core.move_mouse
click = _core.click
press_key = _core.press_key
type_text = _core.type_text
get_observation = _core.get_observation
execute_action = _core.execute_action
validate_action_intent = _core.validate_action_intent
version = _core.version

# OCR functions
ocr_region = _core.ocr_region
ocr_regions = _core.ocr_regions
ocr_window_full = _core.ocr_window_full

# Import Python model layer
from .models import Spotter, Executor, Agent, TextReader
