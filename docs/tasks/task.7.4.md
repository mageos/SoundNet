# Task 7.4: Implement `set-volume` command

**Milestone:** 7 - Control Utility (`soundnet-ctl`)
**Depends On:** 7.1, 5.3

## Description

Create a `set-volume` command that calls the `PUT /api/v1/volume` endpoint on a target device.

## Acceptance Criteria

- `soundnet-ctl set-volume <device-ip> 80` successfully changes the target device's volume.