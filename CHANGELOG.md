# Changelog

All notable changes to this project will be documented in this file.

---

## [0.91.1] - Stable

### Added
- Windows single-instance protection
- Debug terminal integration and settings access
- Recent ROM quick-load action
- Gamepad input support (gilrs)
- Audio volume slider (0–100)

### Fixed
- Language label encoding issues (UTF-8 / mojibake)

### Changed
- Updated and normalized multilingual translations (12 languages)
- Refactored modules and general cleanup

---

## [0.91.0]

### Added
- Save/load states (3 slots)
- Debug terminal with search and log export
- Configurable CPU quirks (5 flags, 3 presets)
- 6 CPU unit tests

### Changed
- Refactored codebase into multiple modules (`src/ui/`, `src/debug/`)

---

## [0.90.0]

### Added
- JSON-based i18n system (12 languages: FR, EN, ES, IT, DE, PT, RU, ZH, JA, KO, AR, HI)
- Automatic Windows system language detection

---

## [0.80.0]

### Added
- 880Hz buzzer audio via rodio
- Volume control (0–100)

---

## [0.70.0]

### Added
- Full CHIP-8 keyboard input (16-key mapping)
- Gamepad support via gilrs

---

## [0.60.0]

### Added
- ROM loading (.ch8) via file explorer
- Recent ROM memory and quick reload

---

## [0.50.0]

### Added
- CHIP-8 64×32 display rendering
- Bottom status bar with version number
- Automatic screen centering

---

## [0.40.0]

### Added
- Unified settings window with tabs
- Game menu (pause, reset, stop, save/load slots)
- Fullscreen toggle (F11 / Alt+Enter)
- Dynamic window scaling (1x–5x)
- Configurable keyboard shortcuts

---

## [0.30.0]

### Added
- Full CHIP-8 CPU (35 opcodes)
- `memory.rs` and `display.rs` modules
- Random opcode support (CXNN via rand)

---

## [0.20.0]

### Added
- Settings window with tabs (Emulator, Video, Controls)
- Ok / Apply / Cancel / Default buttons
- Snapshot system for proper cancel behavior

### Changed
- Migrated from eframe 0.27 to 0.33

---

## [0.10.0]

### Added
- Initial egui window
- Dark/light theme persistence
- Base Rust project structure

---

## Known Issues

- None reported
