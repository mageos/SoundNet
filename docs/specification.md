# Project Specification: SoundNet

**Version:** 1.0
**Last Updated:** 2023-10-27

## 1. Executive Summary

SoundNet is a low-latency audio streaming system written in Rust, designed specifically for Single Board Computers (SBCs) running Linux. It captures audio from an analog source on a "server" device and broadcasts it efficiently to multiple "client" devices on the same local network using IPv6 multicast. The system includes a management protocol for device discovery, status monitoring, remote control, and latency calculation, making it ideal for creating synchronized, multi-room audio setups.

---

## 2. Project Goals and Objectives

### 2.1. Project Goals
*   Create a high-fidelity, low-latency, open-source solution for multi-room audio streaming.
*   Provide a cost-effective alternative to proprietary systems by leveraging commodity SBCs and standard network infrastructure.
*   Build a robust and reliable system suitable for both hobbyist and small-scale professional use.

### 2.2. Project Objectives
*   Develop a single application that can operate in either "server" mode (capture and broadcast) or "client" mode (receive and play).
*   Implement a management protocol for network-wide device discovery, status queries, and remote control.
*   Ensure CPU and memory usage is low enough to run smoothly on common SBCs (e.g., Raspberry Pi 3/4).

### 2.3. Key Success Metrics (KPIs)
*   **Latency:** Glass-to-glass (analog-in on server to analog-out on client) latency should be under 50ms.
*   **Reliability:** The audio stream should remain stable with no dropouts or glitches under normal network conditions.
*   **Resource Usage:** CPU usage on a Raspberry Pi 4 should remain below 50% on a single core during streaming.

---

## 3. Target Audience & User Personas

### 3.1. Persona 1: The DIY Audio Enthusiast
*   **Description:** A hobbyist who enjoys tinkering with SBCs, Linux, and home automation.
*   **Needs/Goals:** Wants to build a custom, synchronized whole-home audio system without being locked into an expensive, proprietary ecosystem like Sonos. Values control, flexibility, and open-source technology.
*   **Pain Points:** Finds existing solutions either too expensive, too complex to set up, or too unreliable.

---

## 4. Functional Requirements & Features

### 4.1. Epic: Core Audio Streaming
*   **Server: Audio Capture:** As a server device, I must be able to capture audio from a connected sound card's line-in or microphone input.
*   **Server: Broadcasting:** As a server device, I must broadcast the captured audio stream to a configurable IPv6 multicast address and port.
*   **Client: Receiving:** As a client device, I must be able to subscribe to the IPv6 multicast group to receive the audio stream.
*   **Client: Playback:** As a client device, I must play the received audio stream through the system's sound card output.

### 4.2. Epic: Device Management & Control
*   **Discovery:** As a control utility, I must be able to broadcast a discovery request and receive responses from all active SoundNet devices on the network.
*   **Status Query:** As a control utility, I must be able to query any device for its current status (e.g., mode: server/client, streaming/idle, volume level, current audio format).
*   **Remote Control (Server):** As a control utility, I must be able to remotely command a server to start or stop its broadcast.
*   **Remote Control (Client):** As a control utility, I must be able to remotely set the volume level of a client device.
*   **Remote Control (Mode):** As a control utility, I must be able to remotely command any device to switch its operational mode between "client" and "server".
*   **Remote Control (Audio Format):** As a control utility, I must be able to remotely command a server to change its audio stream format (e.g., codec, bitrate, sample rate).
*   **Latency Calculation:** As a control utility, I must be able to initiate a process to calculate the network latency between any two devices on the network.

---

## 5. Non-Functional Requirements

*   **Performance:**
    *   The system must prioritize low latency for audio transport.
    *   The system must support configurable audio stream formats, including uncompressed PCM (for lowest latency and highest quality) and compressed formats like FLAC (lossless) or Opus (lossy) to allow users to balance audio quality, network bandwidth, and CPU load.
*   **Scalability:** A single server should be able to stream to at least 10 clients simultaneously without performance degradation.
*   **Reliability:**
    *   Clients must be able to gracefully handle stream interruptions and attempt to reconnect automatically.
    *   A client must implement a jitter buffer to handle network packet timing variations. The size of this buffer should be configurable to allow users to trade off latency for reliability.
*   **Resource Efficiency:** The applications must be lightweight and have a small memory footprint, suitable for continuous operation on SBCs.
*   **Compatibility:** The system must be compatible with major Linux distributions for ARM-based SBCs (e.g., Raspberry Pi OS, Armbian).

---

## 6. Scope Definition

### 6.1. In Scope (for Version 1.0)
*   A single application, configurable at startup or runtime to act as a "server" or "client" daemon.
*   A separate command-line utility for discovery and control.
*   Streaming from a single analog source.
*   Support for multiple, independent audio streams on the network, each on a unique port.
*   Operation over a wired IPv6-enabled LAN.

### 6.2. Out of Scope (for Version 1.0)
*   A graphical user interface (GUI).
*   Support for IPv4 networks. (IPv6 multicast is a core design choice).
*   Streaming from digital sources (e.g., MP3, FLAC files).
*   Authentication and encryption of streams or control messages.
*   Operation over Wi-Fi (due to potential for higher latency and jitter).
*   Advanced clock drift correction for long-term client synchronization (a v2 feature).

---

## 7. Technical Considerations

*   **Language:** Rust (as defined).
*   **Audio I/O:** A cross-platform audio library like `cpal` should be used for accessing sound card devices.
*   **Networking:**
    *   Standard library `std::net` for UDP sockets with multicast options enabled.
    *   An asynchronous HTTP server library (e.g., `axum`, `actix-web`) for the REST API.
    *   An asynchronous runtime like `tokio` for handling concurrent network and audio tasks efficiently.
*   **Audio Format:** The stream format must be configurable.
    *   **Default:** Raw LPCM (e.g., 16-bit, 44.1kHz, stereo) for maximum quality and minimal CPU overhead.
    *   **Compressed Options:** The system should also support compressed formats to reduce bandwidth. Good candidates are FLAC (lossless compression, higher CPU) and Opus (lossy compression, low latency, very efficient).
    *   The active format (codec, sample rate, bitrate) must be communicated via the management protocol so clients know how to decode the stream.
*   **Management Protocol:** The management system is split into two parts: UDP for discovery and a REST API for control.
    *   **Device Discovery (UDP):**
        *   A control utility broadcasts a discovery message to a well-known IPv6 multicast address and port.
        *   All SoundNet devices listen on this address and respond directly to the sender's address.
        *   The response packet contains essential information like the device's friendly name, current mode, and the port for its REST API.
    *   **Device Control (REST API):**
        *   Each SoundNet device runs a lightweight HTTP server to expose a RESTful API for management. This provides a reliable, TCP-based mechanism for control.
        *   Example Endpoints:
            *   `GET /api/v1/status`: Returns the full device status (mode, volume, audio format, etc.).
            *   `POST /api/v1/mode`: Switches the device mode (e.g., `{"mode": "server"}`).
            *   `PUT /api/v1/volume`: Sets the client volume (e.g., `{"volume": 85}`).
            *   `PUT /api/v1/stream/format`: Sets the server's audio format.

---

## 8. Assumptions, Constraints, and Risks

### 8.1. Assumptions
*   Users have a stable, wired LAN with IPv6 enabled and properly configured for multicast.
*   SBCs are equipped with compatible sound cards/DACs for audio input and output.

### 8.2. Risks
*   **Risk:** Audio drift between clients over long playback periods.
    *   **Mitigation (v2):** This will be addressed in a future version by implementing a clock synchronization mechanism. For v1, we accept the risk of minor drift over multi-hour playback sessions. Relying on NTP on all devices is recommended to minimize this effect.
*   **Risk:** Network jitter causing audio dropouts.
    *   **Mitigation:** Implement a configurable jitter buffer on each client to smooth out packet arrival times. An adaptive buffer can be considered for a future version.

---

## 9. Defined Device Behaviors

*   **Initial State:** On startup, a SoundNet device enters an "idle" mode. It will listen for management commands but will not be acting as a client or server.

*   **Server Activation & Conflict:**
    *   When a device is commanded to switch to "server" mode, it will scan for an open multicast port within a predefined range and begin broadcasting on it.
    *   This allows for multiple, independent server streams to exist on the same network simultaneously.
    *   The discovery protocol will announce all active streams, allowing a control utility to differentiate between them.

*   **Client Activation:**
    *   When a device is commanded to switch to "client" mode, the command must specify which server stream (multicast address and port) to subscribe to.
    *   If a client loses connection to the server stream, it will return to an idle state, awaiting further commands.
