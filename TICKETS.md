# Release Ticket: agent_core “Eyes & Hands” (Windows)

Goal: ship a lean Windows-only “eyes + hands” PyPI package (fast screen capture, detection, input, optional OCR), with the heavy “brain” stack gated behind a feature flag.

## Scope
- Keep: capture (xcap), detection (color/arrow/highlight), input (mouse/keyboard), map_coordinates, optional OCR.
- Gate: candle/tokenizers/hf-hub/brain under feature flag; excluded from default wheel.
- Platform: Windows-only for first release.

## Tasks
1) Metadata & versioning
   - Align versions (Cargo + pyproject).
   - Add license field, homepage/repo URLs.
   - Note Windows-only in README and classifiers.

2) Feature gating
   - Default build = eyes/hands (no candle/tokenizers).
   - Add feature flag `brain` to include candle + tokenizers; off by default.
   - Ensure pyproject/maturin passes features correctly for wheels.

3) Build/test
   - Add small pytest suite (eyes/hands only): map_coordinates, detect_color on fixture image, import smoke.
   - GitHub Action (Windows) to build + pytest (skip capture/input live tests).
   - maturin build --release --strip for Windows wheel; twine check.

4) Docs
   - Clean README encoding; concise Quick Start and API table for eyes/hands functions.
   - Safety note: input uses SendInput; intended for local automation.
   - Performance note: capture ~10–30ms, detection ~5–15ms (existing benchmarks).
   - Example script (capture -> detect_color -> click).

5) Packaging
   - Ensure pyproject requires Python 3.9+ (match pyo3 0.22).
   - Add MANIFEST/license inclusion.
   - Upload to PyPI as v0.2.0 (eyes/hands default).

## Nice-to-have (cut if time)
- Async wrapper or batch capture API.
- macOS/Linux support.
- Prebuilt wheels for multiple Python versions via CI.

## Risks / constraints
- Windows-only dependency on xcap/enigo.
- Large wheels if brain feature is bundled; keep off by default.
- OCR backend performance/size; keep optional.
