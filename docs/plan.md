# SoundNet Development Plan

This document breaks down the development of the SoundNet project into logical milestones and tasks, based on the project specification.

## Milestone 1: Project Foundation

The goal of this milestone is to set up the project structure, dependencies, and core data types that will be used throughout the application.

*   **1.1:** Initialize Rust project and set up workspace.
*   **1.2:** Integrate core dependencies (async runtime, CLI parsing, logging, config).
*   **1.3:** Define core application state and data structures.

## Milestone 2: Core Audio I/O

This milestone focuses on proving out the fundamental audio capture and playback functionality.

*   **2.1:** Integrate audio I/O library (`cpal`).
*   **2.2:** Implement a basic audio loopback PoC (Proof of Concept).
*   **2.3:** Abstract audio capture and playback logic.

## Milestone 3: Network Streaming

This milestone implements the basic network transport for the audio data using IPv6 multicast.

*   **3.1:** Implement server-side multicast broadcasting.
*   **3.2:** Implement client-side multicast receiving.
*   **3.3:** Integrate audio I/O with network streaming.

## Milestone 4: Jitter Buffer & Reliability

This milestone adds the jitter buffer to the client to handle network imperfections and make playback reliable.

*   **4.1:** Implement server-side packet timestamping.
*   **4.2:** Design and implement the client-side jitter buffer.
*   **4.3:** Integrate jitter buffer into the client's audio playback loop.
*   **4.4:** Make jitter buffer size configurable.

## Milestone 5: Device Discovery (UDP)

This milestone implements the UDP-based discovery mechanism.

*   **5.1:** Define and implement the discovery packet format.
*   **5.2:** Implement the discovery listener and responder in the main application.
*   **5.3:** Implement the discovery broadcaster in the `soundnet-ctl` utility.

## Milestone 6: Management API (REST)

This milestone implements the reliable, TCP-based REST API for device control.

*   **6.1:** Integrate HTTP server framework (`axum`).
*   **6.2:** Implement the `GET /api/v1/status` endpoint.
*   **6.3:** Implement the `POST /api/v1/mode` endpoint to switch between idle, client, and server.
*   **6.4:** Implement the `PUT /api/v1/volume` endpoint.
*   **6.5:** Implement the `PUT /api/v1/stream/format` endpoint.

## Milestone 7: Control Utility (`soundnet-ctl`)

This milestone builds out the command-line control utility to interact with the management API.

*   **7.1:** Integrate an HTTP client into `soundnet-ctl`.
*   **7.2:** Implement `status` command to query a device.
*   **7.3:** Implement `set-mode` command.
*   **7.4:** Implement `set-volume` command.
*   **7.5:** Implement `set-format` command.

## Milestone 8: Finalization & Documentation

This milestone focuses on polishing the application, testing, and creating user-facing documentation.

*   **8.1:** Add comprehensive logging throughout the application.
*   **8.2:** Write user documentation and usage examples.
*   **8.3:** Create build scripts and packaging instructions.
*   **8.4:** End-to-end testing of all features.