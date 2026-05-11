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

## Variables

| Var | Description |
|-----|-------------|
| `p` | Request's position in queue (0-indexed) |
| `Q` | TX queue size (used when request will join at end) |
| `D` | TX drain rate (tps) |
| `C` | Readiness max concurrency |
| `R` | Nominal readiness check time (ms) |
| `P` | Nominal processing time (ms) - 2s for input proof, 4s for decrypt |
| `T` | Nominal TX confirmation time (ms) |
| `M` | Safety margin (e.g., 0.2) |
| `E` | Elapsed time in current state (ms) |
| `B(E)` | Backoff function based on elapsed time |

## ETA Computation Formulas

### Input Proof

| Status | Formula |
|--------|---------|
| **Queued** | `clamp(⌈(p/D + P + T) × (1+M) / 1000⌉, min, max)` |
| **Processing** | `clamp(⌈(p/D + P + T) × (1+M) / 1000⌉, min, max)` |
| **TxInFlight** | `clamp(⌈P × (1+M) / 1000⌉, min, max)` |
| **ReceiptReceived** | `B(E)` |
| **Completed/TimedOut/Failure** | `0` |

### Decrypt (User & Public)

| Status | Queue Location | Formula |
|--------|----------------|---------|
| **Queued** | In readiness queue | `clamp(⌈(p/C + Q/D + P + T) × (1+M) / 1000⌉, min, max)` |
| **Processing** | Out of readiness, not in TX | `clamp(⌈(R + Q/D + P + T) × (1+M) / 1000⌉, min, max)` |
| **Processing** | In TX queue | `clamp(⌈(p/D + P + T) × (1+M) / 1000⌉, min, max)` |
| **TxInFlight** | - | `clamp(⌈P × (1+M) / 1000⌉, min, max)` |
| **ReceiptReceived** | - | `B(E)` |
| **Completed/TimedOut/Failure** | - | `0` |

### Key Points

1. **Queued**: Uses request's actual position `p` in the queue (not total queue size)
2. **Processing for Decrypt**: Check which queue the request is in:
   - `readiness_throttler.get_position(id)` returns `None` → removed from readiness, check TX queue
   - `tx_throttler.get_position(id)` returns `Some(p)` → use TX queue formula
   - Both return `None` → out of readiness, not yet in TX queue
3. **TxInFlight**: Uses processing time `P` (time to get response after TX sent)
4. **ReceiptReceived**: Uses backoff `B(E)` since Copro/KMS response time is unpredictable

## Example Calculations

Using parameters: D=10, C=50, R=2s, P_input=2s, P_decrypt=4s, T=100ms, M=0.2, B(E)=3s

### Input Proof (P = 2000ms)

#### Queued / Processing: `⌈(p/D × 1000 + P + T) × (1+M) / 1000⌉`

| p | p/D (s) | + P + T (ms) | × 1.2 | Result |
|---|---------|--------------|-------|--------|
| 0 | 0 | 2100 | 2520 | **3s** |
| 1 | 0.1 | 2200 | 2640 | **3s** |
| 10 | 1 | 3100 | 3720 | **4s** |
| 100 | 10 | 12100 | 14520 | **15s** |
| 1000 | 100 | 102100 | 122520 | **123s** |

#### TxInFlight: `⌈P × 1.2 / 1000⌉`
= `⌈2000 × 1.2 / 1000⌉` = **3s** (constant)

#### ReceiptReceived: `B(E)` = **3s** (constant)

### Decrypt (P = 4000ms)

#### Queued (in readiness): `⌈(p/C × 1000 + Q/D × 1000 + P + T) × (1+M) / 1000⌉`

Assuming p = Q (same number of entries in both queues):

| p | p/C (s) | Q/D (s) | + P + T (ms) | × 1.2 | Result |
|---|---------|---------|--------------|-------|--------|
| 0 | 0 | 0 | 4100 | 4920 | **5s** |
| 1 | 0.02 | 0.1 | 4220 | 5064 | **6s** |
| 10 | 0.2 | 1 | 5300 | 6360 | **7s** |
| 100 | 2 | 10 | 16100 | 19320 | **20s** |
| 1000 | 20 | 100 | 124100 | 148920 | **149s** |

#### Processing (out of readiness, not in TX): `⌈(R + Q/D × 1000 + P + T) × (1+M) / 1000⌉`

| Q | R (ms) | Q/D (s) | + P + T (ms) | × 1.2 | Result |
|---|--------|---------|--------------|-------|--------|
| 0 | 2000 | 0 | 6100 | 7320 | **8s** |
| 1 | 2000 | 0.1 | 6200 | 7440 | **8s** |
| 10 | 2000 | 1 | 7100 | 8520 | **9s** |
| 100 | 2000 | 10 | 16100 | 19320 | **20s** |
| 1000 | 2000 | 100 | 106100 | 127320 | **128s** |

#### Processing (in TX queue): `⌈(p/D × 1000 + P + T) × (1+M) / 1000⌉`

| p | p/D (s) | + P + T (ms) | × 1.2 | Result |
|---|---------|--------------|-------|--------|
| 0 | 0 | 4100 | 4920 | **5s** |
| 1 | 0.1 | 4200 | 5040 | **6s** |
| 10 | 1 | 5100 | 6120 | **7s** |
| 100 | 10 | 14100 | 16920 | **17s** |
| 1000 | 100 | 104100 | 124920 | **125s** |

#### TxInFlight: `⌈P × 1.2 / 1000⌉`
= `⌈4000 × 1.2 / 1000⌉` = **5s** (constant)

#### ReceiptReceived: `B(E)` = **3s** (constant)

### Summary Table (p=100, Q=100)

| Status | Input Proof | Decrypt |
|--------|-------------|---------|
| Queued | 15s | 20s |
| Processing (in readiness) | - | 20s |
| Processing (in TX queue) | 15s | 17s |
| TxInFlight | 3s | 5s |
| ReceiptReceived | 3s | 3s |

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

**Why position-based instead of queue-size-based:**
- For GET requests polling an existing `Queued` request, using total queue size is incorrect
- A request that's been waiting and is now at position 5 should get a shorter ETA than a new request at position 100
- Using `get_position(id)` provides accurate estimates as the request advances through the queue
