# RustyWatchdog Project Status & Context

> **Save File for Cross-Platform Continuation**
> **Current Phase:** Phase 2 (The Loop & Logic)
> **Last Action:** Explained why CPU usage is `0%` (needs sampling time).
> **Next Goal:** Implement `loop { ... thread::sleep }` to fix CPU monitoring.

## Completed Knowledge
- Rust Basics: `mut` vs immutable, Variable Ownership.
- Tooling: `cargo init`, `cargo run`, `cargo fmt`.
- Crate: `sysinfo` for cross-platform system data.
- System: Reading memory and CPU init.

## Active Tasks
- [x] Phase 1: The Foundation (Hello System)
    - [x] Initialize project
    - [x] Setup `sysinfo` crate
    - [x] Read memory and CPU usage
- [/] Phase 2: The Loop & Logic (Simulating a Daemon)
    - [ ] Implement main loop
    - [ ] Add logic check (>80% RAM)
    - [ ] Implement `thread::sleep`
- [ ] Phase 3: The Transformation (CLI to Web Server)
- [ ] Phase 4: Shared State & Safety (Concurrency)
