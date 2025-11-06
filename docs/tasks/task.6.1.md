# Task 6.1: Integrate HTTP server framework

**Milestone:** 6 - Management API (REST)
**Depends On:** 1.3
**Status:** Complete

## Description

Add `axum` and `tower-http` to the `soundnet` binary's dependencies. Create a new module for the REST API and spawn an async task that starts the HTTP server on a configurable port.

## Acceptance Criteria

- The `soundnet` application starts an HTTP server on startup.
- `curl`ing the server's root URL returns a `200 OK` or `404 Not Found`.