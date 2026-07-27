// SSR cell (Chunk 2b — render: ssr-node). NO 'use client': this is an async SERVER
// component. The SDK runs server-side during render (Node runtime, worker_threads),
// and the result is baked into the HTML — so it passes even with JavaScript disabled
// in the browser (the spec asserts that, proving the work happened on the server).
//
// Server-side differences vs the CSR cell:
//   - no window.location → the gateway origin comes from the request `headers()`.
//   - threading uses worker_threads; MT is possible regardless of COOP/COEP
//     (SharedArrayBuffer is always available in Node), so MT ⟺ threads==mt.
//
// Selectors (env, available server-side too):
//   NEXT_PUBLIC_FHEVM_TEST_LIB     viem | ethers   (default viem)
//   NEXT_PUBLIC_FHEVM_TEST_THREADS mt   | st       (default st)

import { headers } from 'next/headers';
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

// Same gateway contract as the CSR cells: GET /gw/<slot>/config → addresses.
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

function applyRuntimeConfig(has, set, logger) {
  if (!has()) {
    set({
      singleThread: !USE_MT,
      numberOfThreads: USE_MT ? MT_THREAD_COUNT : 0,
      logger,
    });
  }
}

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

async function runScenario() {
  const { logs, log, logger } = createTestLogger();
  try {
    const h = await headers();
    const host = h.get('host');
    if (!host) {
      throw new Error('no Host header (cannot resolve gateway origin server-side)');
    }
    const origin = `http://${host}`;
    log(`[ssr] lib=${LIB} threads=${THREADS} origin=${origin}`);

    const chain = await fetchChain(origin, CURRENT_SLOT);
    const client = await createClient(chain, `${origin}/gw/${CURRENT_SLOT}/rpc`, logger);

    await client.init();
    const info = await client.runtime.encrypt.getTfheModuleInfo({ tfheVersion: client.tfheVersion });
    const numberOfThreads = info?.numberOfThreads ?? 0;
    log(`[PASS] init (tfheVersion=${String(client.tfheVersion)}, numberOfThreads=${numberOfThreads})`);

    // SSR-node: COOP/COEP do not apply (no browser); MT is effective iff requested.
    if (numberOfThreads > 0 !== USE_MT) {
      throw new Error(
        `thread mode mismatch (ssr-node): threads=${THREADS} → expected ${USE_MT ? 'MT' : 'ST'}, got ${numberOfThreads}`,
      );
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

// Force dynamic (per-request) SSR — `headers()` already opts out of static, but be
// explicit: the SDK work must run on every request, not at build time.
export const dynamic = 'force-dynamic';

export default async function Page() {
  const { status, logs } = await runScenario();
  return (
    <DiagnosticsView title={`SDK encrypt SSR (v13 · lib=${LIB} · threads=${THREADS})`} status={status} logs={logs} />
  );
}
