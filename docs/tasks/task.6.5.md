# Task 6.5: Implement the `PUT /api/v1/stream/format` endpoint

**Milestone:** 6 - Management API (REST)
**Depends On:** 6.2
**Status:** Complete

## Description

Create a handler that accepts a JSON body to change the audio format for a server. This will require reconfiguring the audio capture stream and potentially an encoder.

## Acceptance Criteria

- Sending a PUT request to `/api/v1/stream/format` changes the codec/bitrate of the audio stream being broadcast by a server.