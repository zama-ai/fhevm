'use client';

// Real SDK cell: use the PUBLIC @fhevm/sdk API against the infra.
// Purpose: prove the TFHE wasm LOADS and RUNS by executing generateZkProof —
// in single-thread (st) or multi-thread (mt) mode.
//
// Selectors (env, inlined at `next dev` start):
//   NEXT_PUBLIC_FHEVM_TEST_LIB     viem | ethers   (default viem)
//   NEXT_PUBLIC_FHEVM_TEST_THREADS mt   | st       (default st)
// MT effectiveness also needs cross-origin isolation (COOP/COEP, toggled by
// FHEVM_TEST_COOP). Expectation: numberOfThreads>0 iff (mt && crossOriginIsolated);
// otherwise it degrades to ST — and must still run.

import { useEffect, useState } from 'react';
import { generateZkProof } from '@fhevm/sdk/actions/encrypt';
import { defineFhevmChain } from '@fhevm/sdk/chains';
import { createTestLogger, logError, DiagnosticsView } from '../_diag/diagnostics.jsx';
import { CURRENT_SLOT } from '../_diag/slots.js';

const LIB = process.env.NEXT_PUBLIC_FHEVM_TEST_LIB ?? 'viem';
const THREADS = process.env.NEXT_PUBLIC_FHEVM_TEST_THREADS ?? 'st';
const USE_MT = THREADS === 'mt';
const MT_THREAD_COUNT = 2;

const CONTRACT_ADDRESS = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8';
const USER_ADDRESS = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';

// v13 slot: chain id 31338; cleartext-deploy addresses are deterministic
// (chain-id independent), matching test/chains/localcleartext.ts.
function v13Chain(origin) {
  return defineFhevmChain({
    id: 31338,
    fhevm: {
      contracts: {
        acl: { address: '0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D' },
        inputVerifier: { address: '0x36772142b74871f255CbD7A3e89B401d3e45825f' },
        kmsVerifier: { address: '0x901F8942346f7AB3a01F6D7613119Bca447Bb030' },
        protocolConfig: { address: '0x44aA028fd264C76BF4A8f8B4d8A5272f6AE25CAc' },
      },
      relayerUrl: `${origin}/gw/${CURRENT_SLOT}/relayer`,
      gateway: {
        id: 654_321,
        contracts: {
          decryption: { address: '0xEaaA2FC6BC259dF015Aa7Dc8e59e0B67df622721' },
          inputVerification: { address: '0x6189F6c0c3E40B4a3c72ec86262295D78d845297' },
        },
      },
    },
  });
}

function applyRuntimeConfig(has, set, logger) {
  if (!has()) {
    set({
      singleThread: !USE_MT,
      numberOfThreads: USE_MT ? MT_THREAD_COUNT : 0,
      logger,
    });
  }
}

// Create a client with the selected lib. Each branch imports only its own deps.
async function createClient(chain, rpcUrl, logger) {
  if (LIB === 'viem') {
    const { createFhevmClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } = await import('@fhevm/sdk/viem');
    const { createPublicClient, http } = await import('viem');
    const { anvil } = await import('viem/chains');
    applyRuntimeConfig(hasFhevmRuntimeConfig, setFhevmRuntimeConfig, logger);
    const publicClient = createPublicClient({ chain: { ...anvil, id: chain.id }, transport: http(rpcUrl) });
    return createFhevmClient({ chain, publicClient });
  }
  if (LIB === 'ethers') {
    const { createFhevmClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } = await import('@fhevm/sdk/ethers');
    const { ethers } = await import('ethers');
    applyRuntimeConfig(hasFhevmRuntimeConfig, setFhevmRuntimeConfig, logger);
    const provider = new ethers.JsonRpcProvider(rpcUrl);
    return createFhevmClient({ chain, provider });
  }
  throw new Error(`Unknown NEXT_PUBLIC_FHEVM_TEST_LIB: ${LIB}`);
}

export default function Page() {
  const [diagnostics, setDiagnostics] = useState({ status: 'pending', logs: [] });

  useEffect(() => {
    let cancelled = false;
    const { logs, log, logger } = createTestLogger();

    async function run() {
      try {
        const origin = window.location.origin;
        log(`lib=${LIB} threads=${THREADS}`);

        const chain = v13Chain(origin);
        log('createFhevmClient...');
        const client = await createClient(chain, `${origin}/gw/${CURRENT_SLOT}/rpc`, logger);

        log('client.init()...');
        await client.init();

        // Thread-mode assertion: MT is effective only when cross-origin isolated.
        const info = await client.runtime.encrypt.getTfheModuleInfo({ tfheVersion: client.tfheVersion });
        const crossOriginIsolated = globalThis.crossOriginIsolated ?? false;
        const numberOfThreads = info?.numberOfThreads ?? 0;
        log(
          `[PASS] init ok (tfheVersion=${String(client.tfheVersion)}, numberOfThreads=${numberOfThreads}, ` +
            `threadsAvailable=${String(info?.threadsAvailable)}, crossOriginIsolated=${crossOriginIsolated})`,
        );

        const expectMultiThread = USE_MT && crossOriginIsolated;
        if (numberOfThreads > 0 !== expectMultiThread) {
          throw new Error(
            `thread mode mismatch: threads=${THREADS}, crossOriginIsolated=${crossOriginIsolated} ` +
              `→ expected ${expectMultiThread ? 'MT (numberOfThreads>0)' : 'ST (numberOfThreads===0)'}, ` +
              `got numberOfThreads=${numberOfThreads}`,
          );
        }

        // Run the TFHE wasm (in the resolved thread mode).
        log('generateZkProof uint64(42)...');
        const zkProof = await generateZkProof(client, {
          contractAddress: CONTRACT_ADDRESS,
          userAddress: USER_ADDRESS,
          values: [{ type: 'uint64', value: 42n }],
        });
        const handles = zkProof.getInputHandles();
        if (zkProof.ciphertextWithZkProof.length === 0 || handles.length !== 1) {
          throw new Error(
            `malformed proof: ciphertext=${zkProof.ciphertextWithZkProof.length} handles=${handles.length}`,
          );
        }
        log(`[PASS] proof: ciphertext=${zkProof.ciphertextWithZkProof.length} bytes, handles=${handles.length}`);

        if (!cancelled) {
          setDiagnostics({ status: 'pass', logs });
        }
      } catch (err) {
        // Surface the wrapped cause chain (e.g. the real worker-creation error).
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

  return (
    <DiagnosticsView
      title={`SDK encrypt (v13 · lib=${LIB} · threads=${THREADS})`}
      status={diagnostics.status}
      logs={diagnostics.logs}
    />
  );
}
