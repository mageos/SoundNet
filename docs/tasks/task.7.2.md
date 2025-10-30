# Task 7.2: Implement `status` command

**Milestone:** 7 - Control Utility (`soundnet-ctl`)
**Depends On:** 7.1, 5.3

## Description

Create a `status` command in `soundnet-ctl` that takes a device identifier (IP address or friendly name). It will call the `GET /api/v1/status` endpoint on the specified device and print the result.

## Acceptance Criteria

- `soundnet-ctl status <device-ip>` prints the status of the device in a human-readable format.