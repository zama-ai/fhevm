'use client';

// CSR leg of the mixed cell: runs the SDK in the BROWSER after hydration. MT is
// effective only when cross-origin isolated (same rule as the /encrypt cell).

import { useEffect, useState } from 'react';
import { LIB, THREADS, USE_MT, runEncryptLeg } from './shared.js';
import { createTestLogger, logError, DiagnosticsView } from '../_diag/diagnostics.jsx';

export function CsrLeg() {
  const [diagnostics, setDiagnostics] = useState({ status: 'pending', logs: [] });

  useEffect(() => {
    let cancelled = false;
    const { logs, log, logger } = createTestLogger();

    async function run() {
      try {
        log(`[csr] lib=${LIB} threads=${THREADS}`);
        const crossOriginIsolated = globalThis.crossOriginIsolated ?? false;
        await runEncryptLeg(window.location.origin, USE_MT && crossOriginIsolated, log, logger);
        if (!cancelled) {
          setDiagnostics({ status: 'pass', logs });
        }
      } catch (err) {
        logError(log, err);
        if (!cancelled) {
          setDiagnostics({ status: 'fail', logs });
        }
      }
    }

    run();
    return () => {
      cancelled = true;
    };
  }, []);

  return <DiagnosticsView title="CSR leg" status={diagnostics.status} logs={diagnostics.logs} idPrefix="csr" />;
}
