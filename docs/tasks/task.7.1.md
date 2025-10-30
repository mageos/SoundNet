# Task 7.1: Integrate an HTTP client into `soundnet-ctl`

**Milestone:** 7 - Control Utility (`soundnet-ctl`)
**Depends On:** 1.3

## Description

Add the `reqwest` crate to the `soundnet-ctl` binary's dependencies. Create a helper module for making requests to the SoundNet REST API.

## Acceptance Criteria

- `reqwest` is added as a dependency. A client structure is created to simplify API calls.