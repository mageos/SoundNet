# Task 6.2: Implement the `GET /api/v1/status` endpoint

**Milestone:** 6 - Management API (REST)
**Depends On:** 6.1

## Description

Create a handler for the status endpoint. This handler should read the application's `SharedState` and return it as a JSON response.

## Acceptance Criteria

- `curl http://[device-ip]:[port]/api/v1/status` returns a JSON object representing the device's current state.