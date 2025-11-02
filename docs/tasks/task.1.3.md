# Task 1.3: Define core application state and data structures

**Milestone:** 1 - Project Foundation
**Depends On:** 1.2
**Status:** Complete

## Description

Create a shared library crate within the workspace (e.g., `soundnet-types`) to hold common data structures. Define the core state and data types that will be used across the application, protected by appropriate concurrency primitives.

- `DeviceMode` enum (`Idle`, `Server`, `Client`).
- `AudioFormat` struct/enum (`Codec`, `SampleRate`, `Bitrate`).
- `SharedState` struct, likely wrapped in `Arc<Mutex<...>>`, to hold the application's current state.

## Acceptance Criteria

- A new `soundnet-types` library crate exists and is included in the workspace.
- Core data structures are defined and serializable (using `serde`).
- The main `soundnet` application initializes and holds the `SharedState`.