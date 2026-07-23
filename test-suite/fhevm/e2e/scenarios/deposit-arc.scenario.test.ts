// Scenario: deposit arc — the confidential-vault demo's forward leg (#1760), the FIRST live-cluster
// exercise of the batcher + vault via `@fhevm/sdk/solana/vault`. Run as `demo:smoke`.
//
// STATUS: live-only, UNVERIFIED here. It requires a running demo stack with the two demo programs
// deployed, `demo:seed` completed, and the `demo:faucet` running (all classifier-gated / blocked in
// this environment — see solana/scripts/demo/demo-keypairs/README and demo/seed.ts). It is exercised
// only by the `solana-demo-acceptance` workflow (manual dispatch), which runs demo-up.sh (deploy +
// seed) and starts the faucet before invoking this. The SDK is reached through the runtime
// dynamic-import seam (string module specifier), so the vault module is untyped here by construction
// (same reason as `src/solana/current-user-decrypt.ts`): the SDK's generated `_types` are not built
// at tsc time.
//
// Assertion map — the deposit arc (join = mock USDC → cUSDC, payout = cShares):
//   1. alice funded with SOL + mock USDC through the demo faucet         [live, wired below].
//   2. alice's cUSDC confidential token account initialized              [live, SDK, wired below].
//   3. wrap mock USDC → cUSDC confidential balance (public amount)       [live, SDK, wired below].
//   4. joinBatch(deposit batcher current batch) with an SDK input proof for the wrapped amount.
//   5. dispatchBatch once the batch is old enough (min_batch_age_slots).
//   6. settleBatch (keeper/operator action) against the batch's settle lookup table.
//   7. claim alice's proportional cShares payout.
//   8. decryptPosition asserts alice's claimed cShares > 0 (live user-decrypt of on-chain state).
//
// JOIN BOUNDARY (steps 4–8): the arc is wired live through the confidential wrap (step 3). Wrapping
// is now a first-class `@fhevm/sdk/solana/vault` action (`buildWrapUsdcInstruction`) — a PUBLIC-amount
// escrow that needs no input proof — so it no longer goes through the Rust live-client the two-holder
// transfer scenario used. The remaining arc is deliberately NOT wired here: `joinBatch` needs a real
// coprocessor input proof (encrypt → submitInputProof via the relayer) bound to the join mint's
// compute signer, and `settleBatch` needs a decrypt `FhevmRuntime`, the KMS burn certificate, and the
// proof-service MMR proof over the settle ALT. That is new proof/runtime plumbing; until it is wired
// this fails loudly at the join boundary rather than yielding a false green. Cost + plan are in the
// PR report; the solana-demo-acceptance workflow is the end-to-end verification vehicle.

import fs from "node:fs/promises";

import { describe, test } from "bun:test";

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
// USDC the persona wraps + (later) joins with. The workflow passes DEMO_DEPOSIT_AMOUNT (fresh per run
// avoids PDA reuse); default matches the faucet's default drip.
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

describe("solana deposit-arc scenario", () => {
  test(
    "deposit: alice wraps mock USDC -> joins -> dispatch -> settle (keeper) -> claim -> decrypt cShares > 0",
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

      // The wrap (and the later join) are signed by alice; load her keypair as a signer and prove it
      // is the pubkey the seed published, so a keypair/config drift fails here rather than on-chain.
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
      // wrap both revert on failure, so their confirmation IS the assertion for these legs.
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

      // Steps 4–8: the join boundary. joinBatch needs a real coprocessor input proof bound to the join
      // mint's compute signer, and settle needs a decrypt runtime + KMS certificate + proof-service MMR
      // proof over the settle ALT — proof/runtime plumbing not yet wired here. Fail loudly rather than
      // yield a false green; the solana-demo-acceptance workflow is the verification vehicle.
      throw new Error(
        "deposit-arc smoke is wired through the confidential wrap (mock USDC -> cUSDC); the remaining " +
          "arc (joinBatch with a coprocessor input proof, dispatchBatch, settleBatch with the KMS burn " +
          "certificate + proof-service MMR proof over the settle ALT, claim, decryptPosition) needs the " +
          "input-proof + FhevmRuntime plumbing that is not yet wired. Tracked in the PR report.",
      );
    },
    SCENARIO_TIMEOUT_MS,
  );
});
