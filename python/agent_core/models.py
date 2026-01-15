"""Two-model architecture: Spotter (vision) + Executor (reasoning)

Spotter: Moondream - sees the screen, describes what's there
Executor: Phi3 - decides what action to take based on description
"""

import base64
import json
import time
import urllib.request
from io import BytesIO
from typing import Optional, Tuple, Dict, Any

try:
    from agent_core import agent_core as _core
except ImportError:
    _core = None

try:
    from PIL import Image
except ImportError:
    Image = None

OLLAMA_URL = "http://localhost:11434/api/generate"


class TextReader:
    """OCR text extraction using Tesseract (via agent_core Rust)."""

    def read_region(self, img_data: bytes, width: int, height: int,
                    x: int, y: int, w: int, h: int) -> str:
        """Extract text from image region.

        Args:
            img_data: Raw RGBA image bytes
            width: Image width
            height: Image height
            x, y: Top-left corner of region to OCR
            w, h: Width and height of region to OCR

        Returns:
            Extracted text string
        """
        if _core is None:
            return ""

        try:
            return _core.ocr_region(list(img_data), width, height, x, y, w, h)
        except Exception as e:
            return f"OCR Error: {e}"

    def read_text_box(self, img_data: bytes, width: int, height: int,
                     box_location: str = "bottom") -> str:
        """Read common text box locations (bottom/top of screen).

        Args:
            img_data: Raw RGBA image bytes
            width: Image width
            height: Image height
            box_location: "bottom", "top", or "full"

        Returns:
            Extracted text string
        """
        if box_location == "bottom":
            # Bottom 25% of screen (typical dialogue box)
            region_h = height // 4
            return self.read_region(img_data, width, height,
                                   0, height - region_h, width, region_h)
        elif box_location == "top":
            # Top 25% of screen (menus, battle text)
            region_h = height // 4
            return self.read_region(img_data, width, height, 0, 0, width, region_h)
        else:
            # Full screen
            return self.read_region(img_data, width, height, 0, 0, width, height)


def _image_to_base64(img_data: bytes, width: int, height: int) -> str:
    """Convert RGBA bytes to base64 PNG."""
    if Image is None:
        raise RuntimeError("PIL not installed")
    img = Image.frombytes("RGBA", (width, height), img_data)
    buffer = BytesIO()
    img.save(buffer, format="PNG")
    return base64.b64encode(buffer.getvalue()).decode("utf-8")


def _call_ollama(model: str, prompt: str, image_b64: str = None, timeout: float = 120.0) -> str:
    """Call Ollama API with optional image."""
    payload = {
        "model": model,
        "prompt": prompt,
        "stream": False
    }
    if image_b64:
        payload["images"] = [image_b64]

    req = urllib.request.Request(
        OLLAMA_URL,
        data=json.dumps(payload).encode("utf-8"),
        headers={"Content-Type": "application/json"},
        method="POST"
    )

    try:
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            result = json.loads(resp.read().decode("utf-8"))
            return result.get("response", "")
    except Exception as e:
        return f"Error: {e}"


class Spotter:
    """Vision model - sees the screen and describes it.

    Uses Moondream via Ollama for visual understanding.
    """

    def __init__(self, model: str = "moondream", timeout: float = 120.0):
        self.model = model
        self.timeout = timeout
        self._last_description = ""

    def see(self, prompt: str = None, bounds: Tuple[int, int, int, int] = None) -> str:
        """Capture screen and describe what's visible.

        Args:
            prompt: Custom prompt (default: describe the screen)
            bounds: Optional (left, top, right, bottom) region

        Returns:
            Description of what's on screen
        """
        if _core is None:
            return "Error: agent_core not available"

        # Capture screen
        if bounds:
            left, top, right, bottom = bounds
            width, height = right - left, bottom - top
            img_data = _core.capture_region(left, top, width, height)
        else:
            width, height, img_data = _core.capture_screen()

        # Convert to base64
        img_b64 = _image_to_base64(bytes(img_data), width, height)

        # Default prompt
        if prompt is None:
            prompt = "Describe what you see on this screen. Be brief and specific."

        # Call Moondream
        self._last_description = _call_ollama(self.model, prompt, img_b64, self.timeout)
        return self._last_description

    @property
    def last_description(self) -> str:
        """Get the last description without re-capturing."""
        return self._last_description


class Executor:
    """Reasoning model - decides what action to take.

    Uses Phi3 via Ollama for decision making.
    """

    def __init__(self, model: str = "phi3", timeout: float = 60.0):
        self.model = model
        self.timeout = timeout
        self._last_decision = ""

    def decide(self, context: str, options: list = None, goal: str = None) -> str:
        """Decide what action to take based on context.

        Args:
            context: Description of current situation (from Spotter)
            options: Available actions (default: UP/DOWN/LEFT/RIGHT/A/B/START)
            goal: What we're trying to achieve

        Returns:
            Chosen action
        """
        if options is None:
            options = ["UP", "DOWN", "LEFT", "RIGHT", "A", "B", "START"]

        if goal is None:
            goal = "Make progress in the game."

        prompt = f"""Based on this situation, decide what to do.

SITUATION: {context}

GOAL: {goal}

AVAILABLE ACTIONS: {', '.join(options)}

What single action should be taken? Reply with just the action name."""

        self._last_decision = _call_ollama(self.model, prompt, timeout=self.timeout)
        return self._last_decision

    def parse_action(self, response: str, options: list = None) -> str:
        """Parse an action from the model's response."""
        if options is None:
            options = ["START", "DOWN", "UP", "LEFT", "RIGHT", "B", "A"]

        if not response:
            return options[-1]  # Default to last option (usually A)

        r = response.upper()
        for opt in options:
            if opt in r:
                return opt

        return options[-1]

    @property
    def last_decision(self) -> str:
        """Get the last decision without re-querying."""
        return self._last_decision


class Agent:
    """Combined Spotter + Executor for autonomous operation.

    Example:
        agent = Agent()
        action = agent.step(goal="Become Pokemon Champion")
        agent.execute(action)
    """

    def __init__(
        self,
        spotter_model: str = "moondream",
        executor_model: str = "phi3",
        spotter_timeout: float = 120.0,
        executor_timeout: float = 60.0
    ):
        self.spotter = Spotter(model=spotter_model, timeout=spotter_timeout)
        self.executor = Executor(model=executor_model, timeout=executor_timeout)
        self._last_observation = ""
        self._last_action = ""

    def step(
        self,
        goal: str = None,
        context: Dict[str, Any] = None,
        options: list = None,
        see_prompt: str = None
    ) -> str:
        """One complete perception-decision cycle.

        Args:
            goal: What we're trying to achieve
            context: Additional context (game state, etc.)
            options: Available actions
            see_prompt: Custom prompt for spotter

        Returns:
            Chosen action
        """
        # 1. Spotter sees
        description = self.spotter.see(prompt=see_prompt)
        self._last_observation = description

        # 2. Build full context
        full_context = description
        if context:
            context_str = ", ".join(f"{k}={v}" for k, v in context.items())
            full_context = f"{description}\nGame state: {context_str}"

        # 3. Executor decides
        decision = self.executor.decide(full_context, options=options, goal=goal)
        self._last_action = self.executor.parse_action(decision, options)

        return self._last_action

    def execute(self, action: str) -> bool:
        """Execute an action using agent_core input functions.

        Args:
            action: Button name (UP, DOWN, A, etc.)

        Returns:
            True if successful
        """
        if _core is None:
            return False

        # Map game buttons to keyboard keys
        key_map = {
            "UP": "up",
            "DOWN": "down",
            "LEFT": "left",
            "RIGHT": "right",
            "A": "z",
            "B": "x",
            "START": "return",
            "SELECT": "space"
        }

        key = key_map.get(action.upper(), "z")

        try:
            _core.press_key(key)
            return True
        except Exception:
            return False

    @property
    def last_observation(self) -> str:
        return self._last_observation

    @property
    def last_action(self) -> str:
        return self._last_action
