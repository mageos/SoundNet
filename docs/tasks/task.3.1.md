# Task 3.1: Implement server-side multicast broadcasting

**Milestone:** 3 - Network Streaming
**Depends On:** 1.3

## Description

Create a networking module that can bind a UDP socket and configure it to send packets to a specified IPv6 multicast address and port.

## Acceptance Criteria

- An async task can be spawned that sends a continuous stream of dummy UDP packets to a multicast group.
- Tools like `tcpdump` or Wireshark can verify the packets are being sent correctly.