# Task 5.1: Define and implement the discovery packet format

**Milestone:** 5 - Device Discovery (UDP)
**Depends On:** 1.3
**Status:** Complete

## Description

In the `soundnet-types` crate, define the Rust structs for the discovery request and response packets. Use `serde` and a binary format like `bincode` for serialization. The response packet must include the device's friendly name, mode, and REST API port, as specified in the project documentation.

## Acceptance Criteria

- `DiscoveryRequest` and `DiscoveryResponse` structs are defined and can be serialized/deserialized successfully.