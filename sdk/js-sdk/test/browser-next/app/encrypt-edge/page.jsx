// SSR-edge cell (Chunk 2b — render: ssr-edge). A server component pinned to the
// EDGE runtime. Per the cube's pruning rules, edge has neither Web Workers nor
// node:worker_threads and no SharedArrayBuffer threading, so TFHE is ALWAYS
// single-threaded here — an MT request must degrade to ST, never crash. WASM still
// loads via embedded base64 (no fs/URL on edge). This cell also empirically probes
// whether the edge runtime can load + run the (multi-MB) TFHE module at all; if it
// can't, that's a documented edge limitation, surfaced in the logs.
//
// Selectors: NEXT_PUBLIC_FHEVM_TEST_LIB (viem|ethers), NEXT_PUBLIC_FHEVM_TEST_THREADS.

import { headers } from 'next/headers';
import { generateZkProof } from '@fhevm/sdk/actions/encrypt';
import { defineFhevmChain } from '@fhevm/sdk/chains';
import { createTestLogger, logError, DiagnosticsView } from '../_diag/diagnostics.jsx';
import { CURRENT_SLOT } from '../_diag/slots.js';

export const runtime = 'edge';
export const dynamic = 'force-dynamic';

const LIB = process.env.NEXT_PUBLIC_FHEVM_TEST_LIB ?? 'viem';
const THREADS = process.env.NEXT_PUBLIC_FHEVM_TEST_THREADS ?? 'st';
const USE_MT = THREADS === 'mt';
const MT_THREAD_COUNT = 2;

const CONTRACT_ADDRESS = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8';
const USER_ADDRESS = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';

async function fetchChain(origin, slot) {
  const res = await fetch(`${origin}/gw/${slot}/config`, { cache: 'no-store' });
  if (!res.ok) {
    throw new Error(`GET /gw/${slot}/config -> ${res.status}`);
  }
  const cfg = await res.json();
  return defineFhevmChain({
    id: cfg.chainId,
    fhevm: {
      contracts: {
        acl: { address: cfg.contracts.acl },
        inputVerifier: { address: cfg.contracts.inputVerifier },
        kmsVerifier: { address: cfg.contracts.kmsVerifier },
        protocolConfig: { address: cfg.contracts.protocolConfig },
      },
      relayerUrl: `${origin}/gw/${slot}/relayer`,
      gateway: {
        id: cfg.gateway.id,
        contracts: {
          decryption: { address: cfg.gateway.contracts.decryption },
          inputVerification: { address: cfg.gateway.contracts.inputVerification },
        },
      },
    },
  });
}

async function createClient(chain, rpcUrl, logger) {
  // Honor the requested mode so the MT cell genuinely exercises edge's graceful
  // degradation: edge has no Web Workers / worker_threads / SAB, so the SDK must
  // resolve this MT request down to ST (asserted below), never crash.
  const config = { singleThread: !USE_MT, numberOfThreads: USE_MT ? MT_THREAD_COUNT : 0, logger };
  if (LIB === 'viem') {
    const { createFhevmClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } = await import('@fhevm/sdk/viem');
    const { createPublicClient, http } = await import('viem');
    const { anvil } = await import('viem/chains');
    if (!hasFhevmRuntimeConfig()) {
      setFhevmRuntimeConfig(config);
    }
    return createFhevmClient({
      chain,
      publicClient: createPublicClient({ chain: { ...anvil, id: chain.id }, transport: http(rpcUrl) }),
    });
  }
  if (LIB === 'ethers') {
    const { createFhevmClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } = await import('@fhevm/sdk/ethers');
    const { ethers } = await import('ethers');
    if (!hasFhevmRuntimeConfig()) {
      setFhevmRuntimeConfig(config);
    }
    return createFhevmClient({ chain, provider: new ethers.JsonRpcProvider(rpcUrl) });
  }
  throw new Error(`Unknown NEXT_PUBLIC_FHEVM_TEST_LIB: ${LIB}`);
}

async function runScenario() {
  const { logs, log, logger } = createTestLogger();
  try {
    const h = await headers();
    const host = h.get('host');
    if (!host) {
      throw new Error('no Host header (cannot resolve gateway origin on edge)');
    }
    const origin = `http://${host}`;
    log(`[edge] lib=${LIB} threads=${THREADS} origin=${origin}`);
    if (USE_MT) {
      log('[note] edge has no Web Workers / worker_threads / SAB → always ST (MT request pruned)');
    }

    const chain = await fetchChain(origin, CURRENT_SLOT);
    const client = await createClient(chain, `${origin}/gw/${CURRENT_SLOT}/rpc`, logger);

    await client.init();
    const info = await client.runtime.encrypt.getTfheModuleInfo({ tfheVersion: client.tfheVersion });
    const numberOfThreads = info?.numberOfThreads ?? 0;
    log(`[PASS] init (tfheVersion=${String(client.tfheVersion)}, numberOfThreads=${numberOfThreads})`);

    if (numberOfThreads !== 0) {
      throw new Error(`ssr-edge must be ST, got numberOfThreads=${numberOfThreads}`);
    }

    const zkProof = await generateZkProof(client, {
      contractAddress: CONTRACT_ADDRESS,
      userAddress: USER_ADDRESS,
      values: [{ type: 'uint64', value: 42n }],
    });
    const handles = zkProof.getInputHandles();
    if (zkProof.ciphertextWithZkProof.length === 0 || handles.length !== 1) {
      throw new Error(`malformed proof: ciphertext=${zkProof.ciphertextWithZkProof.length} handles=${handles.length}`);
    }
    log(`[PASS] proof: ciphertext=${zkProof.ciphertextWithZkProof.length} bytes, handles=${handles.length}`);

    return { status: 'pass', logs };
  } catch (err) {
    logError(log, err);
    return { status: 'fail', logs };
  }
}

export default async function Page() {
  const { status, logs } = await runScenario();
  return (
    <DiagnosticsView title={`SDK encrypt EDGE (v13 · lib=${LIB} · threads=${THREADS})`} status={status} logs={logs} />
  );
}
