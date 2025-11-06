# Task 6.3: Implement the `POST /api/v1/mode` endpoint

**Milestone:** 6 - Management API (REST)
**Depends On:** 6.2
**Status:** Complete

## Description

Create a handler that accepts a JSON body to change the device's mode. This will involve starting/stopping the audio capture, playback, and network streaming tasks based on the new mode.

## Acceptance Criteria

- Sending a POST request to `/api/v1/mode` with `{"mode": "server"}` causes the device to start broadcasting audio.