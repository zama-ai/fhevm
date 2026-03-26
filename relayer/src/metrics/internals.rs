// 1. TODO: (later) Measure the size of the queue (counter or gauge.)
// 2. Put latency metrics around readiness check.
// 3. readiness checked passed, readiness check timed out.
// 4. gauge on active readiness check (currently done, not queued status.)
// 4. later Latency after a receipt received + timed_out counter.
// Add a metrics + ALERT on the error selector for OFT token payment missed ! If the relayer address is not funded with oft token -> MUST RAISE HUGE ALARM.