# Task 5.3: Implement the discovery broadcaster in `soundnet-ctl`

**Milestone:** 5 - Device Discovery (UDP)
**Depends On:** 5.1
**Status:** Complete

## Description

Create a `discover` command in the `soundnet-ctl` utility. This command will broadcast a `DiscoveryRequest` packet and listen for `DiscoveryResponse` packets for a few seconds, printing the details of any discovered devices.

## Acceptance Criteria

- Running `soundnet-ctl discover` lists all active `soundnet` devices on the network with their name, mode, and API address.