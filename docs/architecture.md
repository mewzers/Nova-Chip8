# Architecture

## Overview

Oxide is built around a main core (`Oxide`) that orchestrates several modules:

- CPU (CHIP-8 emulation)
- Memory
- Display
- Input
- Audio
- UI

## Architecture Diagram

```mermaid
flowchart TD
    Oxide --> CPU
    Oxide --> Memory
    Oxide --> Display
    Oxide --> Input
    Oxide --> Audio
    Oxide --> UI

    CPU --> Memory
    CPU --> Display
    CPU --> Input

    Display --> UI
    Input --> UI