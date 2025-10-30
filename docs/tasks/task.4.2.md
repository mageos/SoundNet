# Task 4.2: Design and implement the client-side jitter buffer

**Milestone:** 4 - Jitter Buffer & Reliability
**Depends On:** 4.1

## Description

Create a data structure that can store incoming audio packets, ordered by their timestamp. It should be able to handle out-of-order packets and provide a method to pull the next audio chunk that is "due" for playback based on a configurable buffer size.

## Acceptance Criteria

- A `JitterBuffer` struct exists.
- It has methods to `add` a packet and `get_next_frame`.
- Unit tests verify its logic for ordering packets, handling gaps, and discarding old packets.