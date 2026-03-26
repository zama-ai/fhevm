# Request Status Transitions - Quick Reference

## Status Enum
`queued` → `processing` → `tx_in_flight` → `receipt_received` → `completed` | `timed_out` | `failure`

**Terminal states**: `completed`, `timed_out`, `failure` (no further transitions)

---

## 1. PublicDecrypt

```
                    [User Request]
                           │
                           ▼
                    ┌────────────┐
                    │  QUEUED    │ INSERT
                    └────────────┘
                           │
                    {Readiness Queue}
                           │
             ┌─────────────┼─────────────┐
             │             │             │
          Success       Timeout       Failure
             │             │             │
             ▼             ▼             ▼
      ┌────────────┐ ┌────────────┐ ┌────────────┐
      │ PROCESSING │ │ TIMED_OUT  │ │  FAILURE   │
      └────────────┘ └────────────┘ └────────────┘
             │             ✓             ✓
        {TX Queue}      TERMINAL      TERMINAL
             │
       ┌─────┴─────┐
       │           │
    Success     Failure
       │           │
       ▼           ▼
┌─────────────┐ ┌────────────┐
│TX_IN_FLIGHT │ │  FAILURE   │
└─────────────┘ └────────────┘
       │              ✓
       │          TERMINAL
       │
  ⟲ RECOVERY
  Reset to
  PROCESSING
       │
       │ TX Receipt
       │ (sendRawTransactionSync)
       ▼
┌──────────────────┐
│ RECEIPT_RECEIVED │
└──────────────────┘
       │
       │ [30min timeout]
       │
  ┌────┴────┐
  │         │
Response  Timeout
  │         │
  ▼         ▼
┌───────────┐ ┌────────────┐
│ COMPLETED │ │ TIMED_OUT  │
└───────────┘ └────────────┘
      ✓             ✓
  TERMINAL      TERMINAL
```

**WHERE Clauses**:
- `queued → processing`: `WHERE req_status = 'queued'`
- `processing → tx_in_flight`: `WHERE req_status = 'processing'`
- `tx_in_flight → receipt_received`: `WHERE req_status = 'tx_in_flight'`
- `receipt_received → completed`: `WHERE req_status = 'receipt_received'`
- `* → timed_out`: `WHERE req_status IN ('queued', 'receipt_received')`
- `* → failure` (TX): `WHERE req_status IN ('processing', 'tx_in_flight')`

---

## 2. UserDecrypt

```
                    [User Request]
                           │
                           ▼
                    ┌────────────┐
                    │  QUEUED    │ INSERT
                    └────────────┘
                           │
                  {Readiness Queue}
                  (Permission Check)
                           │
             ┌─────────────┼─────────────┐
             │             │             │
          Success       Timeout       Failure
             │             │             │
             ▼             ▼             ▼
      ┌────────────┐ ┌────────────┐ ┌────────────┐
      │ PROCESSING │ │ TIMED_OUT  │ │  FAILURE   │
      └────────────┘ └────────────┘ └────────────┘
             │             ✓             ✓
        {TX Queue}      TERMINAL      TERMINAL
             │
       ┌─────┴─────┐
       │           │
    Success     Failure
       │           │
       ▼           ▼
┌─────────────┐ ┌────────────┐
│TX_IN_FLIGHT │ │  FAILURE   │
└─────────────┘ └────────────┘
       │              ✓
       │          TERMINAL
       │
  ⟲ RECOVERY
  Reset to
  PROCESSING
       │
       │ TX Receipt
       │ (sendRawTransactionSync)
       ▼
┌──────────────────┐
│ RECEIPT_RECEIVED │
└──────────────────┘
       │
       │ Collect Shares
       │ (from N KMS nodes)
       │ [Share threshold]
       │
  ┌────┴────┐
  │         │
Threshold Timeout
 Reached    │
  │         │
  ▼         ▼
┌───────────┐ ┌────────────┐
│ COMPLETED │ │ TIMED_OUT  │
└───────────┘ └────────────┘
      ✓             ✓
  TERMINAL      TERMINAL
```

**WHERE Clauses**:
- `queued → processing`: `WHERE req_status = 'queued'`
- `processing → tx_in_flight`: `WHERE req_status = 'processing'`
- `tx_in_flight → receipt_received`: `WHERE req_status = 'tx_in_flight'`
- `receipt_received → completed` (share threshold): `WHERE req_status = 'receipt_received'`
- `* → timed_out`: `WHERE req_status IN ('queued', 'receipt_received')`
- `* → failure` (TX): `WHERE req_status IN ('processing', 'tx_in_flight')`

---

## 3. InputProof

```
                    [User Request]
                           │
                           │
                           ▼
                    ┌────────────┐
                    │ PROCESSING │ INSERT ⚠️ (NOT QUEUED)
                    └────────────┘
                           │
                      {TX Queue}
                      (No Readiness)
                           │
                     ┌─────┴─────┐
                     │           │
                  Success     Failure
                     │           │
                     ▼           ▼
              ┌─────────────┐ ┌────────────┐
              │TX_IN_FLIGHT │ │  FAILURE   │
              └─────────────┘ └────────────┘
                     │              ✓
                     │          TERMINAL
                     │
                ⟲ RECOVERY
                Reset to
                PROCESSING
                     │
                     │ TX Receipt
                     │ (sendRawTransactionSync)
                     ▼
              ┌──────────────────┐
              │ RECEIPT_RECEIVED │
              └──────────────────┘
                     │
                     │ [ZK Proof Verify]
                     │
                ┌────┴────┐
                │         │
             Accept    Reject
                │         │
                ▼         ▼
           ┌───────────┐ ┌────────────┐
           │ COMPLETED │ │  FAILURE   │
           └───────────┘ └────────────┘
                 ✓             ✓
             TERMINAL      TERMINAL
```

**WHERE Clauses**:
- `processing → tx_in_flight`: `WHERE req_status = 'processing'` ⚠️
- `tx_in_flight → receipt_received`: `WHERE req_status = 'tx_in_flight'`
- `receipt_received → completed` (accept): `WHERE req_status = 'receipt_received'`
- `receipt_received → failure` (reject): `WHERE req_status = 'receipt_received'`
- `* → failure` (generic): `WHERE req_status IN ('processing', 'tx_in_flight', 'receipt_received')`

---

## Recovery (Startup)

```
┌──────────────────────────────────────────────┐
│ STEP 1: Reset Status                        │
│   UPDATE SET req_status = 'processing'       │
│   WHERE req_status = 'tx_in_flight'          │
│   (Updates metrics accordingly)              │
└──────────────────────────────────────────────┘
                     │
                     ▼
┌──────────────────────────────────────────────┐
│ STEP 2: Re-dispatch by Status               │
│                                              │
│   queued            → ReqRcvdFromUser        │
│   processing        → ReadinessCheckPassed   │
│   receipt_received  → NO ACTION              │
│   terminal          → NO ACTION              │
└──────────────────────────────────────────────┘
```

---

## Queues

```
┌─────────────────┬──────────────────────┬────────────────────┐
│ Queue           │ Used By              │ Control            │
├─────────────────┼──────────────────────┼────────────────────┤
│ Readiness       │ Public/User Decrypt  │ Semaphore          │
│ (Concurrency)   │                      │ (max parallel)     │
├─────────────────┼──────────────────────┼────────────────────┤
│ TX              │ All request types    │ Rate Limiter       │
│ (TPS)           │                      │ (transactions/sec) │
└─────────────────┴──────────────────────┴────────────────────┘
```

---

## Error Summary

```
┌────────────────────┬──────────────┬────────────────────────┐
│ From               │ To           │ Trigger                │
├────────────────────┼──────────────┼────────────────────────┤
│ queued             │ timed_out    │ Readiness timeout      │
│ queued             │ failure      │ Readiness failed       │
│ processing         │ failure      │ TX send failed         │
│ receipt_received   │ timed_out    │ Response timeout (30m) │
│ any non-terminal   │ failure      │ Internal error         │
└────────────────────┴──────────────┴────────────────────────┘
```
