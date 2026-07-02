// Mixed cell (Chunk 2b — render: mixed). One route, two legs:
//   - SSR leg: this async SERVER component runs the SDK during render (Node), baking
//     #ssr-result into the HTML (works with JS disabled).
//   - CSR leg: a 'use client' child runs the SDK in the BROWSER after hydration,
//     flipping #csr-result.
// The spec runs with JS ENABLED and asserts BOTH legs pass — proving the SDK works in
// the server render AND the client render of the same page. SSR MT uses worker_threads
// (COOP-independent); CSR MT needs cross-origin isolation.

import { headers } from 'next/headers';
import { LIB, THREADS, USE_MT, runEncryptLeg } from './shared.js';
import { CsrLeg } from './CsrLeg.jsx';
import { createTestLogger, logError, DiagnosticsView } from '../_diag/diagnostics.jsx';

export const dynamic = 'force-dynamic';

async function runSsrLeg() {
  const { logs, log, logger } = createTestLogger();
  try {
    const h = await headers();
    const host = h.get('host');
    if (!host) {
      throw new Error('no Host header (cannot resolve gateway origin server-side)');
    }
    log(`[ssr] lib=${LIB} threads=${THREADS} origin=http://${host}`);
    await runEncryptLeg(`http://${host}`, USE_MT, log, logger); // COOP irrelevant server-side
    return { status: 'pass', logs };
  } catch (err) {
    logError(log, err);
    return { status: 'fail', logs };
  }
}

export default async function Page() {
  const ssr = await runSsrLeg();
  return (
    <main>
      <h1>
        SDK mixed (v13 · lib={LIB} · threads={THREADS})
      </h1>
      <DiagnosticsView title="SSR leg" status={ssr.status} logs={ssr.logs} idPrefix="ssr" />
      <CsrLeg />
    </main>
  );
}
