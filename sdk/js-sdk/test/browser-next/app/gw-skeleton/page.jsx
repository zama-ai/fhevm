'use client';

// Step-1 skeleton: proves the same-origin gateway works from the browser under
// COOP/COEP — anvil RPC (proxied), the mini-relayer keyurl, and the key bytes —
// for every slot. No SDK logic yet; this only validates the infra plumbing.

import { useEffect, useState } from 'react';
import { createTestLogger, logError, DiagnosticsView } from '../_diag/diagnostics.jsx';
import { CURRENT_SLOT, LEGACY_SLOT } from '../_diag/slots.js';

const SLOTS = [LEGACY_SLOT, CURRENT_SLOT];

export default function Page() {
  const [diagnostics, setDiagnostics] = useState({ status: 'pending', logs: [] });

  useEffect(() => {
    let cancelled = false;
    const { logs, log } = createTestLogger();

    async function run() {
      try {
        for (const slot of SLOTS) {
          const keyUrlRes = await fetch(`/gw/${slot}/relayer/v2/keyurl`);
          if (!keyUrlRes.ok) {
            throw new Error(`${slot} keyurl HTTP ${keyUrlRes.status}`);
          }
          const json = await keyUrlRes.json();
          const pubUrl = json.response.fheKeyInfo[0].fhePublicKey.urls[0];
          const crsUrl = json.response.crs['2048'].urls[0];
          log(`${slot} keyurl ok`);

          const pubBytes = new Uint8Array(await (await fetch(pubUrl)).arrayBuffer());
          const crsBytes = new Uint8Array(await (await fetch(crsUrl)).arrayBuffer());
          if (pubBytes.length === 0 || crsBytes.length === 0) {
            throw new Error(`${slot} empty key bytes`);
          }
          log(`${slot} key bytes: pub=${pubBytes.length} crs=${crsBytes.length}`);

          const rpcRes = await fetch(`/gw/${slot}/rpc`, {
            method: 'POST',
            headers: { 'content-type': 'application/json' },
            body: JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'eth_chainId', params: [] }),
          });
          const rpcJson = await rpcRes.json();
          if (typeof rpcJson.result !== 'string') {
            throw new Error(`${slot} rpc returned no chainId`);
          }
          log(`${slot} rpc eth_chainId=${rpcJson.result}`);
        }

        log(`crossOriginIsolated=${String(globalThis.crossOriginIsolated)}`);
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

  return <DiagnosticsView title="Gateway skeleton" status={diagnostics.status} logs={diagnostics.logs} />;
}
