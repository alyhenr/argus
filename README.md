# Argus

A high-throughput, low-latency telemetry and analytics ingestion engine built in Rust. 

Designed to process thousands of concurrent events per second with minimal memory footprint, Argus acts as the edge collection layer for web analytics, custom event tracking, and error monitoring.

## Architecture

* **Edge:** Rust (`axum`, `tokio`)
* **Transport:** Asynchronous batching via memory channels
* **Storage:** ClickHouse (Columnar OLAP)
* **Client:** Lightweight, dependency-free JavaScript SDK

## Core Philosophies

1. **Zero-Blocking Ingestion:** The HTTP edge must never wait on database disk I/O. Requests are validated, queued in memory, and acknowledged instantly.
2. **Predictable Memory:** Strict control over allocations on the hot path. Rust's ownership model ensures high throughput without garbage collection latency spikes.
3. **Privacy by Default:** IP addresses are one-way hashed at the edge. No raw PII is written to persistent storage unless explicitly designated by the client application.

## Getting Started (Development)

*(TODO: Instructions for setting up the local ClickHouse container and running the Rust server will go here).*