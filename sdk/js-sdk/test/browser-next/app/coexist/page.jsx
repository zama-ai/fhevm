'use client';

// Coexistence cell (Chunk 2c — the per-platform definition of done).
// Proves multiple FHEVM runtimes coexist in ONE realm/process:
//   - v12 slot → TFHE 1.5.x (key.1.5.4), real proof via generateZkProof
//   - v13 slot → TFHE 1.6.1   (key.1.6.1), real proof via generateZkProof
//   - a cleartext runtime in parallel (mock encrypt) — the universal scenario
//   - the module×key forward-compat EXPECTED-FAIL: older module 1.5.x + newer
//     key 1.6.1 (v12 RPC/ACL but v13 relayer) must fail to deserialize.
// All legs run concurrently to battle-test concurrent multi-version init + shared
// per-version worker pools (in ST or MT, per NEXT_PUBLIC_FHEVM_TEST_THREADS).
//
// Selectors (inlined at `next dev` start):
//   NEXT_PUBLIC_FHEVM_TEST_LIB     viem | ethers   (default viem)
//   NEXT_PUBLIC_FHEVM_TEST_THREADS mt   | st       (default st)

import { useEffect, useState } from 'react';
import { generateZkProof } from '@fhevm/sdk/actions/encrypt';
import { defineFhevmChain } from '@fhevm/sdk/chains';
import { createTestLogger, logError, DiagnosticsView } from '../_diag/diagnostics.jsx';
import { CURRENT_SLOT, LEGACY_SLOT, OLD_MODULE_NEW_KEY_SLOT } from '../_diag/slots.js';

const LIB = process.env.NEXT_PUBLIC_FHEVM_TEST_LIB ?? 'viem';
const THREADS = process.env.NEXT_PUBLIC_FHEVM_TEST_THREADS ?? 'st';
const USE_MT = THREADS === 'mt';
const MT_THREAD_COUNT = 2;

const CONTRACT_ADDRESS = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8';
const USER_ADDRESS = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';

// Each anvil now deploys from a distinct deployer, so contract addresses differ per
// slot. The gateway is the single source: GET /gw/<slot>/config returns this slot's
// addresses + chain id. The page supplies only its origin + the slot id.
async function fetchSlotConfig(origin, slot) {
  const res = await fetch(`${origin}/gw/${slot}/config`);
  if (!res.ok) {
    throw new Error(`GET /gw/${slot}/config -> ${res.status}`);
  }
  return res.json();
}

// Build a FhevmChain from a slot config. `relayerSlot` (defaults to the config's own
// slot) selects the relayer URL — and thus the global key-cache slot, which is keyed
// by relayer URL. The cleartext leg passes a UNIQUE relayerSlot so its mock (deadbeef)
// key never lands in a real slot's cache and poisons a real proof.
function buildChain(origin, cfg, relayerSlot) {
  return defineFhevmChain({
    id: cfg.chainId,
    fhevm: {
      contracts: {
        acl: { address: cfg.contracts.acl },
        inputVerifier: { address: cfg.contracts.inputVerifier },
        kmsVerifier: { address: cfg.contracts.kmsVerifier },
        protocolConfig: { address: cfg.contracts.protocolConfig },
      },
      relayerUrl: `${origin}/gw/${relayerSlot}/relayer`,
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

// Per-lib adapter: real + cleartext encrypt-client factories from the same lib, so
// the whole matrix runs under viem OR ethers. Each branch imports only its deps.
async function libAdapter(logger) {
  if (LIB === 'viem') {
    const real = await import('@fhevm/sdk/viem');
    const ct = await import('@fhevm/sdk/viem/cleartext');
    const { createPublicClient, http } = await import('viem');
    const { anvil } = await import('viem/chains');
    const mkPublic = (chain, rpcUrl) =>
      createPublicClient({ chain: { ...anvil, id: chain.id }, transport: http(rpcUrl) });
    applyRuntimeConfig(real.hasFhevmRuntimeConfig, real.setFhevmRuntimeConfig, logger);
    return {
      mkReal: (chain, rpcUrl) => real.createFhevmClient({ chain, publicClient: mkPublic(chain, rpcUrl) }),
      mkCleartext: (chain, rpcUrl) =>
        ct.createFhevmCleartextEncryptClient({ chain, publicClient: mkPublic(chain, rpcUrl) }),
    };
  }
  if (LIB === 'ethers') {
    const real = await import('@fhevm/sdk/ethers');
    const ct = await import('@fhevm/sdk/ethers/cleartext');
    const { ethers } = await import('ethers');
    const mkProvider = (rpcUrl) => new ethers.JsonRpcProvider(rpcUrl);
    applyRuntimeConfig(real.hasFhevmRuntimeConfig, real.setFhevmRuntimeConfig, logger);
    return {
      mkReal: (chain, rpcUrl) => real.createFhevmClient({ chain, provider: mkProvider(rpcUrl) }),
      mkCleartext: (chain, rpcUrl) => ct.createFhevmCleartextEncryptClient({ chain, provider: mkProvider(rpcUrl) }),
    };
  }
  throw new Error(`Unknown NEXT_PUBLIC_FHEVM_TEST_LIB: ${LIB}`);
}

// A real TFHE leg: init (resolves the on-chain version), assert the resolved TFHE
// version, the thread mode, and a well-formed proof built locally (generateZkProof).
async function realProofLeg(client, label, expectVersionPrefix, log) {
  await client.init();
  const info = await client.runtime.encrypt.getTfheModuleInfo({ tfheVersion: client.tfheVersion });
  const crossOriginIsolated = globalThis.crossOriginIsolated ?? false;
  const numberOfThreads = info?.numberOfThreads ?? 0;
  log(
    `[${label}] tfheVersion=${String(client.tfheVersion)} numberOfThreads=${numberOfThreads} coi=${crossOriginIsolated}`,
  );

  if (!String(client.tfheVersion).startsWith(expectVersionPrefix)) {
    throw new Error(`${label}: expected TFHE ${expectVersionPrefix}x, got ${String(client.tfheVersion)}`);
  }
  const expectMultiThread = USE_MT && crossOriginIsolated;
  if (numberOfThreads > 0 !== expectMultiThread) {
    throw new Error(
      `${label}: thread-mode mismatch (threads=${THREADS}, coi=${crossOriginIsolated}, got ${numberOfThreads})`,
    );
  }

  const zkProof = await generateZkProof(client, {
    contractAddress: CONTRACT_ADDRESS,
    userAddress: USER_ADDRESS,
    values: [{ type: 'uint64', value: 42n }],
  });
  const handles = zkProof.getInputHandles();
  if (zkProof.ciphertextWithZkProof.length === 0 || handles.length !== 1) {
    throw new Error(
      `${label}: malformed proof (ciphertext=${zkProof.ciphertextWithZkProof.length} handles=${handles.length})`,
    );
  }
  return `${label}: TFHE ${String(client.tfheVersion)}, proof ${zkProof.ciphertextWithZkProof.length} bytes`;
}

// The cleartext (mock) leg: a parallel runtime that encrypts without real WASM or a
// relayer — proves the universal cleartext mode coexists with the TFHE runtimes.
async function cleartextLeg(client, log) {
  await client.ready;
  const result = await client.encryptValues({
    contractAddress: CONTRACT_ADDRESS,
    userAddress: USER_ADDRESS,
    values: [{ type: 'uint64', value: 7n }],
  });
  if (result.encryptedValues.length !== 1 || !result.inputProof?.startsWith('0x')) {
    throw new Error(
      `cleartext: malformed result (values=${result.encryptedValues.length} inputProof=${String(result.inputProof).slice(0, 6)})`,
    );
  }
  log(`[cleartext] encrypted ${result.encryptedValues.length} value(s), inputProof ok`);
  return `cleartext: ${result.encryptedValues.length} value(s)`;
}

// Forward-compat EXPECTED-FAIL: an older module (1.5.x, from the v12 ACL) cannot
// deserialize a newer key (1.6.1, served by the v13 relayer). The whole leg MUST
// fail; a success is the real error.
async function expectedFailLeg(client, log) {
  try {
    await client.init();
    await generateZkProof(client, {
      contractAddress: CONTRACT_ADDRESS,
      userAddress: USER_ADDRESS,
      values: [{ type: 'uint64', value: 1n }],
    });
  } catch (err) {
    log(`[expected-fail] failed as expected: ${err instanceof Error ? err.message : String(err)}`);
    return 'expected-fail: failed as expected';
  }
  throw new Error('expected-fail: module 1.5.x + key 1.6.1 unexpectedly SUCCEEDED');
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
        const adapter = await libAdapter(logger);

        const [v12Cfg, v13Cfg, mismatchCfg] = await Promise.all([
          fetchSlotConfig(origin, LEGACY_SLOT),
          fetchSlotConfig(origin, CURRENT_SLOT),
          fetchSlotConfig(origin, OLD_MODULE_NEW_KEY_SLOT),
        ]);
        const v12 = adapter.mkReal(buildChain(origin, v12Cfg, LEGACY_SLOT), `${origin}/gw/${LEGACY_SLOT}/rpc`);
        const v13 = adapter.mkReal(buildChain(origin, v13Cfg, CURRENT_SLOT), `${origin}/gw/${CURRENT_SLOT}/rpc`);
        // Cleartext leg: reuse the current slot's addresses but a UNIQUE relayer URL
        // (its mock returns deadbeef bytes — under the real relayer it would poison
        // the real key).
        const clear = adapter.mkCleartext(buildChain(origin, v13Cfg, 'cleartext'), `${origin}/gw/${CURRENT_SLOT}/rpc`);
        // Dedicated alias slot: current key over the legacy anvil (older ACL → older
        // module), at its OWN relayer URL so the global key cache does not collide
        // with the real current leg → older module + newer key fails for real. Its
        // /config returns the legacy addresses (it proxies the legacy anvil).
        const mismatch = adapter.mkReal(
          buildChain(origin, mismatchCfg, OLD_MODULE_NEW_KEY_SLOT),
          `${origin}/gw/${OLD_MODULE_NEW_KEY_SLOT}/rpc`,
        );

        log('initializing legacy + current + cleartext + expected-fail concurrently...');
        const summaries = await Promise.all([
          realProofLeg(v12, LEGACY_SLOT, '1.5.', log),
          realProofLeg(v13, CURRENT_SLOT, '1.6.', log),
          cleartextLeg(clear, log),
          expectedFailLeg(mismatch, log),
        ]);

        // Coexistence proof: the two real legs loaded DIFFERENT TFHE versions.
        if (String(v12.tfheVersion) === String(v13.tfheVersion)) {
          throw new Error(`coexistence: both real legs resolved the same TFHE version ${String(v12.tfheVersion)}`);
        }

        for (const s of summaries) {
          log(`[PASS] ${s}`);
        }
        log('[PASS] multi-version + cleartext coexistence verified');

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
      title={`SDK coexistence (v12 + v13 + cleartext · lib=${LIB} · threads=${THREADS})`}
      status={diagnostics.status}
      logs={diagnostics.logs}
    />
  );
}
