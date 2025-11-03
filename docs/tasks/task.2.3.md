# Task 2.3: Abstract audio capture and playback logic

**Milestone:** 2 - Core Audio I/O
**Depends On:** 2.2
**Status:** Complete

## Description

Refactor the loopback PoC into reusable, asynchronous components. Create a `capture` module/task that reads from a sound device and sends audio data through a channel. Create a `playback` module/task that receives audio data from a channel and writes it to a sound device.

## Acceptance Criteria

- The audio capture and playback logic runs in separate async tasks.
- The loopback functionality still works, but data now flows through a `tokio::sync::mpsc` channel.