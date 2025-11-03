# Task 3.3: Integrate audio I/O with network streaming

**Milestone:** 3 - Network Streaming
**Depends On:** 2.3, 3.2
**Status:** Complete

## Description

Connect the full pipeline. The audio `capture` task should send its data to the multicast `broadcaster` task. The multicast `receiver` task should send its data to the audio `playback` task. This creates the first end-to-end streaming prototype.

## Acceptance Criteria

- Running one instance of the application in "server" mode and another in "client" mode results in audio being streamed between them.
- Audio quality will be poor due to jitter, which is expected at this stage.