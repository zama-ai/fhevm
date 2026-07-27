'use client';

// module cell (Chunk 2e — module axis). Exercises the two modules no other cell runs:
//   kms       — a real decrypt client loads the TKMS wasm and RUNS it via
//               generateTransportKeyPair (TKMS keygen). Other cells only load TKMS.
//   cleartext — the mock cleartext runtime (the "universal" parallel mode) standalone:
//               a mock encrypt + a mock transport keypair, no real wasm.
//
// Selectors: NEXT_PUBLIC_FHEVM_TEST_LIB, NEXT_PUBLIC_FHEVM_TEST_MODULE (kms|cleartext).

import { useEffect, useState } from 'react';
import { generateTransportKeyPair } from '@fhevm/sdk/actions/decrypt';
import { defineFhevmChain } from '@fhevm/sdk/chains';
import { createTestLogger, logError, DiagnosticsView } from '../_diag/diagnostics.jsx';
import { CURRENT_SLOT } from '../_diag/slots.js';

const LIB = process.env.NEXT_PUBLIC_FHEVM_TEST_LIB ?? 'viem';
const MODULE = process.env.NEXT_PUBLIC_FHEVM_TEST_MODULE ?? 'kms';

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

// TKMS is single-threaded; cleartext is a mock — no threads in either.
function ensureConfig(has, set, logger) {
  if (!has()) {
    set({
      singleThread: true,
      numberOfThreads: 0,
      logger,
    });
  }
}

function assertTransportKeyPair(kp, label) {
  const pub = kp?.publicKey;
  if (typeof pub !== 'string' || !pub.startsWith('0x') || pub.length < 10) {
    throw new Error(`${label}: malformed transport public key: ${String(pub).slice(0, 24)}`);
  }
}

// kms: a real decrypt client loads + runs the TKMS wasm (keygen).
async function runKms(origin, rpcUrl, log, logger) {
  const chain = await fetchChain(origin, CURRENT_SLOT);
  let client;
  if (LIB === 'viem') {
    const { createFhevmDecryptClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } = await import('@fhevm/sdk/viem');
    const { createPublicClient, http } = await import('viem');
    const { anvil } = await import('viem/chains');
    ensureConfig(hasFhevmRuntimeConfig, setFhevmRuntimeConfig, logger);
    client = createFhevmDecryptClient({
      chain,
      publicClient: createPublicClient({ chain: { ...anvil, id: chain.id }, transport: http(rpcUrl) }),
    });
  } else {
    const { createFhevmDecryptClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } =
      await import('@fhevm/sdk/ethers');
    const { ethers } = await import('ethers');
    ensureConfig(hasFhevmRuntimeConfig, setFhevmRuntimeConfig, logger);
    client = createFhevmDecryptClient({ chain, provider: new ethers.JsonRpcProvider(rpcUrl) });
  }

  await client.init();
  log(`[PASS] decrypt init (tkmsVersion=${String(client.tkmsVersion)})`);

  const kp = await generateTransportKeyPair(client);
  assertTransportKeyPair(kp, 'kms');
  log(`[PASS] TKMS keygen: publicKey=${kp.publicKey.slice(0, 24)}… tkmsVersion=${String(kp.tkmsVersion)}`);
}

// cleartext: the mock runtime standalone — mock encrypt + mock transport keypair.
async function runCleartext(origin, rpcUrl, log, logger) {
  const chain = await fetchChain(origin, CURRENT_SLOT);
  let client;
  if (LIB === 'viem') {
    const { createFhevmCleartextClient } = await import('@fhevm/sdk/viem/cleartext');
    const { hasFhevmRuntimeConfig, setFhevmRuntimeConfig } = await import('@fhevm/sdk/viem');
    const { createPublicClient, http } = await import('viem');
    const { anvil } = await import('viem/chains');
    ensureConfig(hasFhevmRuntimeConfig, setFhevmRuntimeConfig, logger);
    client = createFhevmCleartextClient({
      chain,
      publicClient: createPublicClient({ chain: { ...anvil, id: chain.id }, transport: http(rpcUrl) }),
    });
  } else {
    const { createFhevmCleartextClient } = await import('@fhevm/sdk/ethers/cleartext');
    const { hasFhevmRuntimeConfig, setFhevmRuntimeConfig } = await import('@fhevm/sdk/ethers');
    const { ethers } = await import('ethers');
    ensureConfig(hasFhevmRuntimeConfig, setFhevmRuntimeConfig, logger);
    client = createFhevmCleartextClient({ chain, provider: new ethers.JsonRpcProvider(rpcUrl) });
  }

  await client.ready;
  log('[PASS] cleartext runtime ready');

  const enc = await client.encryptValues({
    contractAddress: CONTRACT_ADDRESS,
    userAddress: USER_ADDRESS,
    values: [{ type: 'uint64', value: 7n }],
  });
  if (enc.encryptedValues.length !== 1 || !enc.inputProof?.startsWith('0x')) {
    throw new Error(`cleartext encrypt malformed: values=${enc.encryptedValues.length}`);
  }
  log(`[PASS] cleartext encrypt: ${enc.encryptedValues.length} value(s)`);

  const kp = await generateTransportKeyPair(client);
  assertTransportKeyPair(kp, 'cleartext');
  log(`[PASS] cleartext transport keypair: publicKey=${kp.publicKey.slice(0, 24)}…`);
}

export default function Page() {
  const [diagnostics, setDiagnostics] = useState({ status: 'pending', logs: [] });

  useEffect(() => {
    let cancelled = false;
    const { logs, log, logger } = createTestLogger();

    async function run() {
      try {
        const origin = window.location.origin;
        const rpcUrl = `${origin}/gw/${CURRENT_SLOT}/rpc`;
        log(`lib=${LIB} module=${MODULE}`);
        if (MODULE === 'kms') {
          await runKms(origin, rpcUrl, log, logger);
        } else if (MODULE === 'cleartext') {
          await runCleartext(origin, rpcUrl, log, logger);
        } else {
          throw new Error(`Unknown NEXT_PUBLIC_FHEVM_TEST_MODULE: ${MODULE}`);
        }
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
      title={`SDK module (${MODULE} · lib=${LIB})`}
      status={diagnostics.status}
      logs={diagnostics.logs}
    />
  );
}
