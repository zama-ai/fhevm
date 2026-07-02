'use client';

// wasm-load cell (Chunk 2d): exercise the URL-based WASM load modes, not just the
// embedded-base64 default. `locateFile` points the SDK at the gateway's same-origin
// raw-asset route (/gw/asset/<filename>), and NEXT_PUBLIC_FHEVM_TEST_WASM_LOAD selects
// the mode. Encrypt-only (TFHE) so only TFHE assets are URL-loaded.
//
//   verified-blob       fetch URL -> SHA-256 verify -> blob/eval worker + compile
//   precheck-direct-url fetch once for an informational hash, then new Worker(url)
//   trusted-direct-url  new Worker(url) directly, no integrity check
//   auto                verified-blob when a worker URL is set, else embedded-base64
//
// Selectors: NEXT_PUBLIC_FHEVM_TEST_LIB, NEXT_PUBLIC_FHEVM_TEST_THREADS,
//            NEXT_PUBLIC_FHEVM_TEST_WASM_LOAD.

import { useEffect, useState } from 'react';
import { generateZkProof } from '@fhevm/sdk/actions/encrypt';
import { defineFhevmChain } from '@fhevm/sdk/chains';
import { createTestLogger, logError, DiagnosticsView } from '../_diag/diagnostics.jsx';
import { CURRENT_SLOT } from '../_diag/slots.js';

const LIB = process.env.NEXT_PUBLIC_FHEVM_TEST_LIB ?? 'viem';
const THREADS = process.env.NEXT_PUBLIC_FHEVM_TEST_THREADS ?? 'st';
const WASM_LOAD = process.env.NEXT_PUBLIC_FHEVM_TEST_WASM_LOAD ?? 'verified-blob';
const USE_MT = THREADS === 'mt';
const MT_THREAD_COUNT = 2;

const CONTRACT_ADDRESS = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8';
const USER_ADDRESS = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';

async function fetchChain(origin, slot) {
  const res = await fetch(`${origin}/gw/${slot}/config`);
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

// Route every TFHE asset (tfhe_bg.v*.wasm, tfhe-worker.v*.mjs) at the gateway's
// same-origin raw-asset route — this is what opts each asset into URL loading.
function makeLocateFile(origin, log) {
  return (filename) => {
    const url = new URL(`${origin}/gw/asset/${filename}`);
    log(`  [locateFile] ${filename} -> ${url.href}`);
    return url;
  };
}

function applyRuntimeConfig(has, set, origin, log, logger) {
  if (!has()) {
    set({
      singleThread: !USE_MT,
      numberOfThreads: USE_MT ? MT_THREAD_COUNT : 0,
      wasmAssetLoadMode: WASM_LOAD,
      locateFile: makeLocateFile(origin, log),
      logger,
    });
  }
}

async function createClient(chain, rpcUrl, origin, log, logger) {
  if (LIB === 'viem') {
    const { createFhevmEncryptClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } = await import('@fhevm/sdk/viem');
    const { createPublicClient, http } = await import('viem');
    const { anvil } = await import('viem/chains');
    applyRuntimeConfig(hasFhevmRuntimeConfig, setFhevmRuntimeConfig, origin, log, logger);
    const publicClient = createPublicClient({ chain: { ...anvil, id: chain.id }, transport: http(rpcUrl) });
    return createFhevmEncryptClient({ chain, publicClient });
  }
  if (LIB === 'ethers') {
    const { createFhevmEncryptClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } =
      await import('@fhevm/sdk/ethers');
    const { ethers } = await import('ethers');
    applyRuntimeConfig(hasFhevmRuntimeConfig, setFhevmRuntimeConfig, origin, log, logger);
    return createFhevmEncryptClient({ chain, provider: new ethers.JsonRpcProvider(rpcUrl) });
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
        log(`lib=${LIB} threads=${THREADS} wasmLoad=${WASM_LOAD}`);

        const chain = await fetchChain(origin, CURRENT_SLOT);
        const client = await createClient(chain, `${origin}/gw/${CURRENT_SLOT}/rpc`, origin, log, logger);

        await client.init();
        const info = await client.runtime.encrypt.getTfheModuleInfo({ tfheVersion: client.tfheVersion });
        const crossOriginIsolated = globalThis.crossOriginIsolated ?? false;
        const numberOfThreads = info?.numberOfThreads ?? 0;
        log(`[PASS] init numberOfThreads=${numberOfThreads}`);

        // Prove the SDK actually loaded from our gateway URLs (not embedded base64):
        // it emits this debug line only for 'user' (locateFile) asset resolution.
        if (!logs.some((l) => l.includes("using 'locateFile'"))) {
          throw new Error(
            `expected URL-based loading via locateFile (mode=${WASM_LOAD}); assets did not resolve to 'user'`,
          );
        }

        const expectMultiThread = USE_MT && crossOriginIsolated;
        if (numberOfThreads > 0 !== expectMultiThread) {
          throw new Error(
            `thread mode mismatch: threads=${THREADS}, coi=${crossOriginIsolated}, got ${numberOfThreads}`,
          );
        }

        const zkProof = await generateZkProof(client, {
          contractAddress: CONTRACT_ADDRESS,
          userAddress: USER_ADDRESS,
          values: [{ type: 'uint64', value: 42n }],
        });
        if (zkProof.ciphertextWithZkProof.length === 0 || zkProof.getInputHandles().length !== 1) {
          throw new Error(`malformed proof: ciphertext=${zkProof.ciphertextWithZkProof.length}`);
        }
        log(`[PASS] proof: ciphertext=${zkProof.ciphertextWithZkProof.length} bytes`);

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

  return (
    <DiagnosticsView
      title={`SDK wasm-load (v13 · lib=${LIB} · threads=${THREADS} · ${WASM_LOAD})`}
      status={diagnostics.status}
      logs={diagnostics.logs}
    />
  );
}
