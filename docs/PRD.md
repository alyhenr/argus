# Product Requirements Document: Argus

## Overview
Argus is a high-throughput, low-latency telemetry and analytics ingestion engine built in Rust. It serves as the edge collection layer for web analytics, custom event tracking, and error monitoring, designed to process thousands of concurrent events per second with a minimal memory footprint.

## Target Audience
- Solo developers, Indie Hackers, and SaaS businesses needing cost-effective, high-performance analytics.
- Systems engineers requiring a lightweight, embeddable telemetry engine.

## Core Philosophies
1. **Zero-Blocking Ingestion:** The HTTP edge must never wait on database disk I/O. Requests are validated, queued, and acknowledged instantly.
2. **Predictable Memory:** Strict control over allocations on the hot path. Rust's ownership model ensures high throughput without garbage collection latency spikes.
3. **Privacy by Default:** IP addresses are one-way hashed at the edge. No raw PII is written to persistent storage unless explicitly designated by the client application.
4. **The "Hard Way" Architecture:** Core infrastructure components (like event buffering/queuing) are built in-house to maximize systems engineering learning and minimize operational costs.

## Phase 1: MVP Scope

### 1. Client SDK (JavaScript/TypeScript)
* **Description:** A lightweight, dependency-free script injected into client applications.
* **Features:**
  * Auto-track page views and unique sessions.
  * Capture unhandled JavaScript errors (`window.onerror`).
  * Provide a manual `Argus.track('eventName', { data })` function.
  * Network request batching to minimize browser impact (using `navigator.sendBeacon` or `fetch`).

### 2. Ingestion API (Rust Edge)
* **Description:** The high-concurrency front door receiving payloads from the SDK.
* **Features:**
  * Single `POST /collect` endpoint.
  * Sub-10ms response time target.
  * Payload validation and basic schema enforcement.
  * In-memory event buffering (custom-built WAL/Queue).
  * IP hashing for basic geo-location without storing raw IP data.

### 3. Storage & Processing Worker
* **Description:** The asynchronous system moving data from the edge to permanent storage.
* **Features:**
  * Background worker to flush the in-memory queue in optimized batches.
  * Integration with ClickHouse (Columnar OLAP) for fast time-series writes and aggregations.

## Out of Scope (Phase 1)
* Complex UI/Dashboard rendering (data verification will be via direct DB queries initially).
* Session replay (video recording of user screens).
* Distributed load balancing across multiple ingestion nodes.