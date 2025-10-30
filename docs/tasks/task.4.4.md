# Task 4.4: Make jitter buffer size configurable

**Milestone:** 4 - Jitter Buffer & Reliability
**Depends On:** 4.3

## Description

Expose the size (in milliseconds) of the jitter buffer as a configuration option for the `soundnet` application. This can be set via a command-line argument or a configuration file.

## Acceptance Criteria

- The client's jitter buffer size can be changed at startup.
- A larger buffer results in more latency but greater resilience to network issues, and this behavior can be verified.