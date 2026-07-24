// Scenario: deposit arc — WRAP PHASE (#1760), the FIRST live-cluster exercise of the confidential
// vault's forward path via `@fhevm/sdk/solana/vault`. Run as `demo:smoke` and hard-gated by the
// solana-demo-acceptance workflow: this phase is expected to pass live.
//
// This covers exactly the part of the deposit arc that is wired end-to-end today: fund a persona,
// initialize her confidential cUSDC account, and wrap mock USDC into a confidential cUSDC balance
// (a PUBLIC-amount escrow that needs no input proof). The join→dispatch→settle→claim→decrypt
// continuation is NOT wired here — it needs coprocessor input-proof + decrypt-runtime plumbing — so
// it lives in `deposit-arc-boundary.scenario.test.ts` (run as `demo:smoke-boundary`), which documents
// that boundary explicitly instead of failing this gate.
//
// STATUS: live-only, UNVERIFIED here. It requires a running demo stack with the two demo programs
// deployed, `demo:seed` completed, and the `demo:faucet` running (all classifier-gated / blocked in
// this environment — see solana/scripts/demo/demo-keypairs/README and demo/seed.ts). The SDK is
// reached through the runtime dynamic-import seam (string module specifier), so the vault module is
// untyped here by construction (same reason as `src/solana/current-user-decrypt.ts`): the SDK's
// generated `_types` are not built at tsc time.
//
// Assertion map — the wrap phase (join = mock USDC → cUSDC):
//   1. alice funded with SOL + mock USDC through the demo faucet         [live, wired below].
//   2. alice's cUSDC confidential token account initialized              [live, SDK, wired below].
//   3. wrap mock USDC → cUSDC confidential balance (public amount)       [live, SDK, wired below].
//   4. on-chain assertion: alice's cUSDC token account exists and is owned by confidential-token.

import fs from "node:fs/promises";

import { describe, expect, test } from "bun:test";

import {
  appendTransactionMessageInstructions,
  assertIsTransactionWithBlockhashLifetime,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  sendAndConfirmTransactionFactory,
  setTransactionMessageComputeUnitLimit,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type Address,
  type Instruction,
  type TransactionSigner,
} from "@solana/kit";

import { loadPersonas, until } from "../harness";
import { DEMO_KEYPAIRS, loadDemoEnv } from "../../demo/loadDemoEnv";

// A live batcher arc waits on slot age + SNS commit + settle certificate: allow well beyond the
// transfer scenario's budget.
const SCENARIO_TIMEOUT_MS = 20 * 60_000;

// The demo faucet binds loopback by default (same-machine demo boundary); the acceptance workflow
// starts it on 8090 and waits for /health before invoking this. Overridable for a non-default run.
const FAUCET_URL = process.env.DEMO_FAUCET_URL ?? "http://127.0.0.1:8090";
// Mock USDC decimals (matches the seeded SPL mint and the faucet).
const USDC_DECIMALS = 6;
// USDC the persona wraps. The workflow passes DEMO_DEPOSIT_AMOUNT (fresh per run avoids PDA reuse);
// default matches the faucet's default drip.
const DEPOSIT_USDC = Number(process.env.DEMO_DEPOSIT_AMOUNT ?? "1000");
// The confidential-token instructions emit FHE-handle CPIs; the default 200k CU ceiling is too low.
const WRAP_COMPUTE_UNIT_LIMIT = 600_000;

/** The vault provisioning/wrap surface the scenario drives (untyped: runtime dynamic-import seam). */
type VaultWrapSurface = {
  buildInitializeTokenAccountInstruction(parameters: {
    owner: TransactionSigner;
    mint: Address;
    hostConfig: Address;
    initialBalance?: number | bigint;
  }): Promise<Instruction>;
  buildWrapUsdcInstruction(parameters: {
    owner: TransactionSigner;
    mint: Address;
    underlyingMint: Address;
    hostConfig: Address;
    amount: number | bigint;
  }): Promise<Instruction>;
  tokenAccountAddress(mint: Address, owner: Address): Promise<Address>;
  CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS: Address;
};

const loadVaultModule = async (): Promise<VaultWrapSurface> => {
  const vaultModule = "@fhevm/sdk/solana/vault";
  return (await import(vaultModule)) as unknown as VaultWrapSurface;
};

/** Loads a 64-byte Solana keypair file into a kit `TransactionSigner`. */
const loadSigner = async (keypairPath: string): Promise<TransactionSigner> => {
  const bytes = Uint8Array.from(JSON.parse(await fs.readFile(keypairPath, "utf8")) as number[]);
  return createKeyPairSignerFromBytes(bytes);
};

// Demo-lane gate: `test:e2e` sweeps this directory on a stack that never ran `demo:seed`, so the
// seeded demo-config cannot exist there. The `demo:smoke` script sets RUN_DEMO_SCENARIOS=1; under
// it the test runs unconditionally, so a missing config still fails the acceptance gate loudly.
const runsDemoScenarios = process.env.RUN_DEMO_SCENARIOS === "1";

describe.skipIf(!runsDemoScenarios)("solana deposit-arc scenario", () => {
  test(
    "deposit arc (wrap phase): alice funds, initializes cUSDC, and wraps mock USDC into a confidential cUSDC balance",
    async () => {
      const { env, config } = await loadDemoEnv();

      // Personas: the keeper is the operator that plays dispatch + settle; alice is the depositing
      // end-user. Both load from committed demo keypairs (pubkeys cross-checked against the config).
      const personas = await loadPersonas(env, {
        keeper: DEMO_KEYPAIRS.keeper,
        alice: DEMO_KEYPAIRS.alice,
      });
      const alicePersona = personas.roles.alice;
      if (!alicePersona) throw new Error("alice persona did not load");

      // The wrap is signed by alice; load her keypair as a signer and prove it is the pubkey the seed
      // published, so a keypair/config drift fails here rather than on-chain.
      const alice = await loadSigner(DEMO_KEYPAIRS.alice);
      if (alice.address !== config.personas.alice) {
        throw new Error(`alice keypair ${alice.address} does not match seeded persona ${config.personas.alice}`);
      }

      // Preconditions: the suite may run right after a relayer / proof-service (re)start. Gate on both
      // health endpoints before submitting (same gates as the confidential-transfer scenario), plus
      // the faucet the persona funds through.
      await until(
        async () => (await fetch(`${env.relayerUrl}/liveness`)).ok,
        { description: "relayer liveness", timeoutMs: 60_000 },
      );
      await until(
        async () => {
          const body = await (await fetch(`${env.proofServiceUrl}/health/readiness`)).text();
          return /"ready"\s*:\s*true/.test(body);
        },
        { description: "solana-proof-service readiness", timeoutMs: 120_000 },
      );
      await until(
        async () => (await fetch(`${FAUCET_URL}/health`)).ok,
        { description: "demo faucet health", timeoutMs: 30_000 },
      );

      // Step 1: fund alice — SOL through the persona/faucet capability, mock USDC through the faucet's
      // mint-to-ATA endpoint (the ATA is created idempotently by the faucet).
      await personas.fund(alicePersona);
      const mintUsdc = await fetch(`${FAUCET_URL}/mint-usdc`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ address: alice.address, amount: DEPOSIT_USDC }),
      });
      if (!mintUsdc.ok) {
        throw new Error(`faucet /mint-usdc failed (${mintUsdc.status}): ${await mintUsdc.text()}`);
      }

      const rpc = createSolanaRpc(env.rpcUrl);
      const rpcSubscriptions = createSolanaRpcSubscriptions(env.wsUrl);
      const sendAndConfirm = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });

      /** Signs `instructions` with `payer` (fee payer) plus any account-embedded signers, then confirms. */
      const send = async (payer: TransactionSigner, instructions: readonly Instruction[]): Promise<void> => {
        const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
        const base = setTransactionMessageFeePayerSigner(payer, createTransactionMessage({ version: 0 }));
        const withLifetime = setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, base);
        const withComputeLimit = setTransactionMessageComputeUnitLimit(WRAP_COMPUTE_UNIT_LIMIT, withLifetime);
        const message = appendTransactionMessageInstructions(instructions, withComputeLimit);
        const signedTransaction = await signTransactionMessageWithSigners(message);
        assertIsTransactionWithBlockhashLifetime(signedTransaction);
        await sendAndConfirm(signedTransaction, { commitment: "confirmed" });
      };

      const vault = await loadVaultModule();

      // Step 2: create alice's confidential token account on the join mint (cUSDC). initialize +
      // wrap both revert on failure, so their confirmation IS the assertion for these phases.
      await send(alice, [
        await vault.buildInitializeTokenAccountInstruction({
          owner: alice,
          mint: config.mints.joinConfidential,
          hostConfig: config.hostConfig,
        }),
      ]);

      // Step 3: wrap the funded mock USDC into alice's confidential cUSDC balance. wrap_usdc escrows a
      // PUBLIC amount and needs no input proof, which is why it wires cheaply here.
      const wrapBaseUnits = BigInt(Math.round(DEPOSIT_USDC * 10 ** USDC_DECIMALS));
      await send(alice, [
        await vault.buildWrapUsdcInstruction({
          owner: alice,
          mint: config.mints.joinConfidential,
          underlyingMint: config.mints.joinUnderlying,
          hostConfig: config.hostConfig,
          amount: wrapBaseUnits,
        }),
      ]);

      // Step 4: on-chain assertion. Read alice's cUSDC confidential token account back and assert it
      // now exists and is owned by the confidential-token program — the concrete state the (not-yet-
      // wired) join phase would consume. This is the wrap phase's real state check, beyond "did not revert".
      const aliceCusdc = await vault.tokenAccountAddress(config.mints.joinConfidential, alice.address);
      const account = await rpc.getAccountInfo(aliceCusdc, { encoding: "base64" }).send();
      expect(account.value).not.toBeNull();
      expect(account.value?.owner).toBe(vault.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
    },
    SCENARIO_TIMEOUT_MS,
  );
});
