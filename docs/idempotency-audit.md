# Idempotency Audit - 3-Pass Review

## Pass 1: State Transition Diagrams ✅

All three request types (PublicDecrypt, UserDecrypt, InputProof) have complete ASCII diagrams showing:
- Normal flow transitions
- Error paths (timeout, failure)
- Queue interactions (Readiness Queue, TX Queue)
- Recovery behavior (tx_in_flight → processing reset)
- Terminal states clearly marked

## Pass 2: Repository WHERE Clause Audit ✅

### PublicDecrypt Repository (6 methods)

| Method | Transition | WHERE Clause | Status |
|--------|------------|--------------|--------|
| `update_status_to_processing` | queued → processing | `WHERE req_status = 'queued'` | ✅ CORRECT |
| `update_status_to_timed_out` | queued/receipt_received → timed_out | `WHERE req_status IN ('queued', 'receipt_received')` | ✅ CORRECT |
| `update_status_to_tx_in_flight` | processing → tx_in_flight | `WHERE req_status = 'processing'` | ✅ CORRECT |
| `update_status_to_receipt_received_on_tx_success` | tx_in_flight → receipt_received | `WHERE req_status = 'tx_in_flight'` | ✅ CORRECT |
| `update_status_to_failure_on_tx_failed` | processing/tx_in_flight → failure | `WHERE req_status IN ('processing', 'tx_in_flight')` | ✅ CORRECT (TX failures only) |
| `complete_req_with_res` | receipt_received → completed | `WHERE req_status = 'receipt_received'` | ✅ CORRECT |

### UserDecrypt Repository (6 methods)

| Method | Transition | WHERE Clause | Status |
|--------|------------|--------------|--------|
| `update_status_to_processing` | queued → processing | `WHERE req_status = 'queued'` | ✅ CORRECT |
| `update_status_to_timed_out` | queued/receipt_received → timed_out | `WHERE req_status IN ('queued', 'receipt_received')` | ✅ CORRECT |
| `update_status_to_tx_in_flight` | processing → tx_in_flight | `WHERE req_status = 'processing'` | ✅ CORRECT |
| `update_status_to_receipt_received_on_tx_success` | tx_in_flight → receipt_received | `WHERE req_status = 'tx_in_flight'` | ✅ CORRECT |
| `update_status_to_failure_on_tx_failed` | processing/tx_in_flight → failure | `WHERE req_status IN ('processing', 'tx_in_flight')` | ✅ CORRECT (TX failures only) |
| `insert_share_and_complete_if_threshold_reached` | receipt_received → completed | `WHERE req_status = 'receipt_received'` | ✅ CORRECT (requires gw_reference_id) |

### InputProof Repository (5 methods)

| Method | Transition | WHERE Clause | Status |
|--------|------------|--------------|--------|
| `update_status_to_tx_in_flight` | processing → tx_in_flight | `WHERE req_status = 'processing'` | ✅ CORRECT |
| `update_input_proof_status_to_receipt_received` | tx_in_flight → receipt_received | `WHERE req_status = 'tx_in_flight'` | ✅ CORRECT |
| `update_status_to_failure` | processing/tx_in_flight/receipt_received → failure | `WHERE req_status IN ('processing', 'tx_in_flight', 'receipt_received')` | ✅ CORRECT (generic failure) |
| `accept_and_complete_input_proof_req` | receipt_received → completed | `WHERE req_status = 'receipt_received'` | ✅ CORRECT |
| `reject_and_complete_input_proof_req` | receipt_received → failure | `WHERE req_status = 'receipt_received'` | ✅ CORRECT |

## Pass 3: Cross-Verification ✅

### PublicDecrypt Flow
**Diagram matches code**: ✅
- Normal flow: queued → processing → tx_in_flight → receipt_received → completed
- Timeout paths: queued → timed_out, receipt_received → timed_out
- Failure path: processing/tx_in_flight → failure
- Recovery: tx_in_flight → processing (on startup)
- All WHERE clauses enforce proper state transitions

### UserDecrypt Flow
**Diagram matches code**: ✅
- Normal flow: queued → processing → tx_in_flight → receipt_received → (share collection) → completed
- Timeout paths: queued → timed_out, receipt_received → timed_out
- Failure path: processing/tx_in_flight → failure
- Recovery: tx_in_flight → processing (on startup)
- Share insertion: ONLY from receipt_received (requires gw_reference_id from receipt)
- All WHERE clauses enforce proper state transitions

### InputProof Flow
**Diagram matches code**: ✅
- **Key difference**: Starts in `processing` (NOT queued) - no Readiness Queue
- Normal flow: processing → tx_in_flight → receipt_received → completed/failure (ZK verify)
- Failure paths:
  - Generic failure: from processing/tx_in_flight/receipt_received
  - Reject (after ZK verify): from receipt_received
- Recovery: tx_in_flight → processing (on startup)
- All WHERE clauses enforce proper state transitions

## Key Idempotency Principles Applied

### 1. **STRICT State Checks**
Single-state transitions use exact equality:
```sql
WHERE req_status = 'queued'        -- queued → processing
WHERE req_status = 'processing'    -- processing → tx_in_flight
WHERE req_status = 'tx_in_flight'  -- tx_in_flight → receipt_received
WHERE req_status = 'receipt_received'  -- receipt_received → completed
```

**Important**: `receipt_received` ONLY from `tx_in_flight` because:
- `sendRawTransactionSync` sets receipt_received after getting the TX receipt
- Even after recovery (tx_in_flight → processing), the request is re-dispatched and status is set back to `tx_in_flight` BEFORE `sendRawTransactionSync` runs
- Therefore, when the receipt arrives, status is ALWAYS `tx_in_flight`

### 2. **MULTI-STATE Checks** (for specific scenarios)
Two-state transitions for timeout scenarios:
```sql
WHERE req_status IN ('queued', 'receipt_received')  -- → timed_out
-- Reason: Timeout from readiness queue (queued) OR response timeout (receipt_received)
```

Two-state transitions for TX failure scenarios:
```sql
WHERE req_status IN ('processing', 'tx_in_flight')  -- → failure (TX)
-- Reason: TX failures only during TX lifecycle, not before/after
```

### 3. **NO PERMISSIVE Patterns**
❌ Eliminated all `NOT IN (terminal_states)` patterns
✅ Replaced with explicit state checks

This ensures:
- No accidental double-processing
- Clear state machine enforcement
- Idempotent operations (safe to retry)
- Metrics accuracy (proper transition tracking)

## Documentation Fixes Applied

1. ✅ Fixed InputProof `receipt_received` transition: Changed from `('queued', 'tx_in_flight')` to `('processing', 'tx_in_flight')`
2. ✅ Added complete WHERE clause documentation for all three request types
3. ✅ Clarified share collection happens AFTER receipt_received for UserDecrypt
4. ✅ Corrected diagram annotations: "TX Receipt (sendRawTransactionSync)" not "Gateway Listener"

## Build Verification

✅ All changes compiled successfully: `cargo build --bin fhevm-relayer`
✅ Query cache updated: `cargo sqlx prepare`

## Summary

**All idempotency checks are correctly implemented across all three request types.**

- Total methods audited: 17
- Correctness rate: 17/17 (100%)
- Documentation errors fixed: 4
- Build status: ✅ Passing

The hybrid approach (explicit state checks instead of NOT IN) has been successfully applied throughout the codebase, ensuring robust idempotency and preventing race conditions.
