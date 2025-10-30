# Task 7.3: Implement `set-mode` command

**Milestone:** 7 - Control Utility (`soundnet-ctl`)
**Depends On:** 7.1, 5.3

## Description

Create a `set-mode` command that calls the `POST /api/v1/mode` endpoint on a target device.

## Acceptance Criteria

- `soundnet-ctl set-mode <device-ip> server` successfully changes the target device's mode.