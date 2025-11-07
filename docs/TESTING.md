# End-to-End Testing Plan

This document outlines the steps for manually testing the SoundNet application.

## Prerequisites

- At least two devices with microphones and speakers, preferably single-board computers (e.g., Raspberry Pi).
- The `soundnet` and `soundnet-ctl` applications built for the target hardware.

## Test Cases

### 1. Basic Streaming

1.  **Device 1 (Server):** Run `./soundnet server`.
2.  **Device 2 (Client):** Run `./soundnet client`.
3.  **Verification:** Speak into the microphone of Device 1. The audio should be audible on the speakers of Device 2 with minimal latency.

### 2. Device Discovery

1.  **Device 1 (Server):** Run `./soundnet server`.
2.  **Device 2 (Client):** Run `./soundnet client`.
3.  **Device 3:** Run `./soundnet-ctl discover`.
4.  **Verification:** The output should list both Device 1 and Device 2 with their respective modes and API addresses.

### 3. Mode Switching

1.  **Device 1:** Run `./soundnet server`.
2.  **Device 2:** Run `./soundnet-ctl discover` to find the IP address of Device 1.
3.  **Device 2:** Send a request to switch Device 1 to client mode:
    ```
    curl -X POST -H "Content-Type: application/json" -d '{"mode": "client"}' http://[device-1-ip]:8080/api/v1/mode
    ```
4.  **Verification:** Device 1 should stop broadcasting and start listening for audio. You can verify this by running another client on the network.

### 4. Volume Control

1.  **Device 1 (Server):** Run `./soundnet server`.
2.  **Device 2 (Client):** Run `./soundnet client`.
3.  **Device 3:** Send a request to change the volume of Device 2:
    ```
    curl -X PUT -H "Content-Type: application/json" -d '{"volume": 0.5}' http://[device-2-ip]:8080/api/v1/volume
    ```
4.  **Verification:** The audio playback volume on Device 2 should be noticeably quieter.

### 5. Stream Format

1.  **Device 1 (Server):** Run `./soundnet server`.
2.  **Device 2:** Send a request to change the stream format of Device 1:
    ```
    curl -X PUT -H "Content-Type: application/json" -d '{"format": {"codec": "opus", "sample_rate": 24000, "bitrate": 32000, "volume": 1.0}}' http://[device-1-ip]:8080/api/v1/stream/format
    ```
4.  **Verification:** The server should now be broadcasting with the new audio format. This can be verified by inspecting the logs on the server.

### 6. Error Conditions

- **Network Disconnect:** While streaming, disconnect the network cable from the client device. The audio should stop. Reconnect the cable. The audio should resume automatically.
- **Server Restart:** While a client is playing audio, restart the server application. The client should eventually reconnect and resume playback.
