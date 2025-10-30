# Task 4.3: Integrate jitter buffer into the client's audio playback loop

**Milestone:** 4 - Jitter Buffer & Reliability
**Depends On:** 4.2

## Description

Modify the client's audio pipeline. The network receiver task now adds packets to the `JitterBuffer`. The audio playback task now pulls data from the `JitterBuffer` instead of directly from the network channel.

## Acceptance Criteria

- End-to-end audio streaming is significantly more reliable, with fewer audible glitches due to network jitter.