# Task 4.1: Implement server-side packet timestamping

**Milestone:** 4 - Jitter Buffer & Reliability
**Depends On:** 3.3
**Status:** Complete

## Description

Modify the server's broadcasting logic to include a high-resolution timestamp with each audio packet. This timestamp should represent the capture time of the audio data. The packet structure will need a header to accommodate this.

## Acceptance Criteria

- Audio packets sent by the server now have a header containing a timestamp.
- Tools like Wireshark can be used to inspect the packet structure and verify the presence of the timestamp.