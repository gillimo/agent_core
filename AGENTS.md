# Mission Learning Statement
- Mission: Build a high-performance capture + input core for agent systems.
- Learning focus: Rust/PyO3 FFI, low-latency screen capture, deterministic input control.
- Project start date: 2026-01-15 (inferred from earliest git commit)

# AGENTS.md

Purpose: define how to work with the agent_core project and how to sign work.

- Project scope: `OneDrive/Desktop/agent_core`
- Sign-in: register in `docs/LOGBOOK.md` before edits; sign docs/logs with handle + date.
- Build: `maturin develop --release` (Python package) or `cargo build --release` (Rust only)
- Test: `cargo test` and `cargo clippy`
- Logs: check `docs/BUG_LOG.md` for known issues
- Tickets: see `docs/TICKETS.md` for planned work

Signed: Claude Opus 4.5 (2026-01-15)
