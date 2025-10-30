# Task 2.1: Integrate audio I/O library

**Milestone:** 2 - Core Audio I/O
**Depends On:** 1.3

## Description

Add the `cpal` crate to the `soundnet` binary's dependencies. Write code to enumerate available audio host devices, and input/output devices, printing them to the console.

## Acceptance Criteria

- `cpal` is added as a dependency.
- The application, when run, can list available audio devices and their supported configurations.