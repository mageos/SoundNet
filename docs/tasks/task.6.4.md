# Task 6.4: Implement the `PUT /api/v1/volume` endpoint

**Milestone:** 6 - Management API (REST)
**Depends On:** 6.2
**Status:** Complete

## Description

Create a handler that accepts a JSON body to change the device's playback volume. This should only be effective when the device is in client mode.

## Acceptance Criteria

- Sending a PUT request to `/api/v1/volume` with `{"volume": 85}` changes the output volume of a client device.