# Dynamic Retry-After Computation Design

## Overview

This document describes the design for computing dynamic `Retry-After` values based on queue state, drain rate, and request processing stage. The goal is to provide clients with intelligent polling intervals that adapt to system load.

## Queue Architecture

The relayer has different queue structures per request type:

### Input Proof (Single Queue)
```
[HTTP] → [TX Throttler Queue] → [Gateway TX]
              ↑
         TPS-based drain (per_seconds)
```

### User Decrypt / Public Decrypt (Dual Queue)
```
[HTTP] → [Readiness Queue] → [Readiness Check] → [TX Throttler Queue] → [Gateway TX]
              ↑                                          ↑
    Concurrency-based drain                       TPS-based drain
       (max_concurrency)                          (per_seconds)
```

Key differences:
- **TX Throttler**: Rate-limited via TPS (tokens per second) using governor
- **Readiness Queue**: Concurrency-limited via semaphore (max_concurrency parallel tasks)

## Configuration

### Core Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `min_seconds` | Minimum retry interval (floor) | 1 |
| `max_seconds` | Maximum retry interval (ceiling) | 300 |
| `safety_margin` | Multiplier applied to computed ETA (0.0-1.0) | 0.2 |

### Nominal Processing Times

All nominal times are **required** in configuration (no code defaults):

| Request Type | Processing By | Config Field |
|--------------|---------------|--------------|
| Input Proof | Copro | `input_proof_processing_seconds` |
| User Decrypt | KMS | `user_decrypt_processing_seconds` |
| Public Decrypt | KMS | `public_decrypt_processing_seconds` |
| Readiness Check | (decrypts only) | `readiness_check_seconds` |
| TX Confirmation | Blockchain | `tx_confirmation_ms` |

### Copro/KMS Backoff Intervals

For `ReceiptReceived` state only (Copro/KMS response time is unpredictable):

| Elapsed Time | Retry-After | Reason |
|--------------|-------------|--------|
| 0-60s | 4s | Expect response soon |
| 60s-2m | 10s | Taking longer than usual |
| 2m-5m | 30s | Significant delay |
| 5m-15m | 60s | Major delay |
| 15m+ | 300s | Likely stuck, minimal polling |

## ETA Computation

### POST Request (New Request)

**Input Proof (Single Queue - Copro):**
```
ETA = tx_queue_wait_time + processing_time
    = (tx_queue_position / tx_throttler_tps) + nominal_input_proof_processing + nominal_tx_confirmation
```

**User/Public Decrypt (Dual Queue - KMS):**
```
ETA = readiness_queue_wait + readiness_processing + tx_queue_wait + tx_processing
    = (readiness_queue_size / max_concurrency * nominal_readiness)
    + nominal_readiness
    + (tx_queue_size / tx_tps)
    + nominal_decrypt_processing + nominal_tx_confirmation
```

Final value: `ETA * (1 + safety_margin)`, clamped to [min_seconds, max_seconds]

### Max Bounds Illustration (Full Queue Scenario)

Example with concrete numbers:
- **Queue sizes**: 10,000 for all queues
- **TX drain rate**: 5 TPS for all tx workers
- **Readiness concurrency**: 50 workers
- **Nominal readiness check time**: 1 second per batch

**Input Proof (Single Queue - worst case):**
```
TX queue wait = 10,000 / 5 TPS = 2,000 seconds = ~33 minutes
```

**Decrypt Operations (Dual Queue - worst case):**
```
Readiness queue wait = ceil(10,000 / 50) batches * 1 second
                     = 200 batches * 1 second = 200 seconds

TX queue wait        = 10,000 / 5 TPS = 2,000 seconds

Total                = 200 + 2,000 = 2,200 seconds = ~37 minutes
```

With 20% safety margin:
- Input Proof max: 2,000 * 1.2 = 2,400 seconds = **40 minutes**
- Decrypt max: 2,200 * 1.2 = 2,640 seconds = **44 minutes**

This demonstrates why `max_seconds` default (300s = 5 min) will clamp these extreme values, providing a reasonable upper bound for client polling while the queue naturally drains.

### GET Request (Polling)

| State | Retry-After Computation | Safety Margin? |
|-------|-------------------------|----------------|
| **Queued** | Dynamic: queue position / drain rate + remaining nominal times | Yes |
| **Processing** | Dynamic: remaining nominal time - elapsed | Yes |
| **TxInFlight** | Dynamic: tx_confirmation - elapsed | Yes |
| **ReceiptReceived** | Backoff intervals (unpredictable Copro/KMS wait) | **No** |
| **Completed/Failed/TimedOut** | 0 (no retry needed) | N/A |

## Response Format

### POST Response (202 Accepted)
```http
HTTP/1.1 202 Accepted
Retry-After: 27

{"status": "queued", "job_id": "...", "eta_seconds": 27}
```

### GET Response (202 In Progress)
```http
HTTP/1.1 202 Accepted
Retry-After: 10

{"status": "queued", "state": "tx_in_flight", "eta_seconds": 10, "elapsed_seconds": 15}
```

## Admin Configuration

All parameters are runtime-updatable via admin endpoints:
- Nominal processing times per request type
- TX throttler TPS (drain rate)
- Retry-after bounds (min/max)
- Safety margin
- Copro/KMS backoff intervals

## Design Rationale

**Why ReceiptReceived uses fixed backoff (no safety margin):**
- Copro/KMS response time is fundamentally unpredictable
- Backoff intervals are already conservative by design
- Adding margin would just increase polling delay unnecessarily

**Why milliseconds internally:**
- All internal calculations use milliseconds to avoid rounding errors
- Conversion to seconds only happens when setting the Retry-After header
