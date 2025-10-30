# Task 7.5: Implement `set-format` command

**Milestone:** 7 - Control Utility (`soundnet-ctl`)
**Depends On:** 7.1, 5.3

## Description

Create a `set-format` command that calls the `PUT /api/v1/stream/format` endpoint on a target device.

## Acceptance Criteria

- `soundnet-ctl set-format <device-ip> opus --bitrate 96` successfully changes the server's stream format.