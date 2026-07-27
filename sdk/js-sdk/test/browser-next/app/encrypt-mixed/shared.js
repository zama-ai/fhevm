// Isomorphic helper shared by the mixed cell's SSR leg (server component) and CSR
// leg (client component). No 'use client' and no server-only APIs (headers/fs), so
// Next can bundle it for BOTH the server and the client graph. Each leg supplies its
// own origin (server: request Host header; client: window.location) and its own
// expected thread mode, then calls runEncryptLeg.

import { generateZkProof } from '@fhevm/sdk/actions/encrypt';
import { defineFhevmChain } from '@fhevm/sdk/chains';
import { CURRENT_SLOT } from '../_diag/slots.js';

export const LIB = process.env.NEXT_PUBLIC_FHEVM_TEST_LIB ?? 'viem';
export const THREADS = process.env.NEXT_PUBLIC_FHEVM_TEST_THREADS ?? 'st';
export const USE_MT = THREADS === 'mt';
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
    return createFhevmClient({
      chain,
      publicClient: createPublicClient({ chain: { ...anvil, id: chain.id }, transport: http(rpcUrl) }),
    });
  }
  if (LIB === 'ethers') {
    const { createFhevmClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } = await import('@fhevm/sdk/ethers');
    const { ethers } = await import('ethers');
    applyRuntimeConfig(hasFhevmRuntimeConfig, setFhevmRuntimeConfig, logger);
    return createFhevmClient({ chain, provider: new ethers.JsonRpcProvider(rpcUrl) });
  }
  throw new Error(`Unknown NEXT_PUBLIC_FHEVM_TEST_LIB: ${LIB}`);
}

// Runs one encrypt leg against the v13 slot: init + thread-mode assertion + a real
// proof. `expectMultiThread` differs per leg (CSR needs crossOriginIsolated; SSR
// does not), so the caller computes it. `log`/`logger` come from createTestLogger.
// Returns the resolved thread count.
export async function runEncryptLeg(origin, expectMultiThread, log, logger) {
  const chain = await fetchChain(origin, CURRENT_SLOT);
  const client = await createClient(chain, `${origin}/gw/${CURRENT_SLOT}/rpc`, logger);
  await client.init();

  const info = await client.runtime.encrypt.getTfheModuleInfo({ tfheVersion: client.tfheVersion });
  const numberOfThreads = info?.numberOfThreads ?? 0;
  if (numberOfThreads > 0 !== expectMultiThread) {
    throw new Error(`thread mode mismatch: expected ${expectMultiThread ? 'MT' : 'ST'}, got ${numberOfThreads}`);
  }

  const zkProof = await generateZkProof(client, {
    contractAddress: CONTRACT_ADDRESS,
    userAddress: USER_ADDRESS,
    values: [{ type: 'uint64', value: 42n }],
  });
  if (zkProof.ciphertextWithZkProof.length === 0 || zkProof.getInputHandles().length !== 1) {
    throw new Error(`malformed proof: ciphertext=${zkProof.ciphertextWithZkProof.length}`);
  }
  log(
    `[PASS] tfheVersion=${String(client.tfheVersion)} numberOfThreads=${numberOfThreads} proof=${zkProof.ciphertextWithZkProof.length}b`,
  );
  return numberOfThreads;
}
