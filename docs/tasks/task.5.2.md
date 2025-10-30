# Task 5.2: Implement the discovery listener and responder

**Milestone:** 5 - Device Discovery (UDP)
**Depends On:** 5.1

## Description

In the `soundnet` binary, spawn an async task that listens on the well-known discovery multicast group. When a `DiscoveryRequest` is received, it should respond directly to the sender with a `DiscoveryResponse` containing its current state.

## Acceptance Criteria

- When the `soundnet` application is running, it correctly responds to discovery packets sent on the network.