# Application Startup

## Overview

This document describes how Oxide starts and initializes its main application.

The entry point is the `main()` function.

---

## Startup Flow

```mermaid
flowchart TD
    A[main()] --> B[Single Instance Check (Windows)]
    B --> C[Log: starting_Oxide]

    C --> D[Setup Window (Viewport)]
    D --> E[Load Icon & Splash Size]

    E --> F[Initialize eframe]

    F --> G[Load Fonts]
    G --> H[Load or Create Oxide State]

    H --> I[Reset Runtime State]
    I --> J[Apply Initial Path (if any)]

    J --> K[Start Application Loop]