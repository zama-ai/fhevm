// Scenario: deposit arc — WRAP + JOIN LEGS (#1760), the live-cluster exercise of the confidential
// vault's forward path via `@fhevm/sdk/solana/vault`. Run as `demo:smoke` and hard-gated by the
// solana-demo-acceptance workflow: these legs are expected to pass live.
//
// This covers the part of the deposit arc that is wired end-to-end today: fund a persona,
// initialize her confidential cUSDC account, wrap mock USDC into a confidential cUSDC balance
// (a PUBLIC-amount escrow that needs no input proof), then JOIN the pending deposit batch with a
// coprocessor-attested amount — a real input proof built by the SDK's local TFHE prover, submitted
// to the relayer, and consumed by `joinBatch`. The dispatch -> settle -> claim -> decrypt
// continuation is NOT wired here — it still needs the settle `FhevmRuntime` plus the
// dispatch/claim/decrypt orchestration — so it lives in `deposit-arc-boundary.scenario.test.ts`
// (run as `demo:smoke-boundary`), which documents that remaining boundary explicitly instead of
// failing this gate.
//
// STATUS: live-only, UNVERIFIED here. It requires a running demo stack with the two demo programs
// deployed, `demo:seed` completed, and the `demo:faucet` running (all classifier-gated / blocked in
// this environment — see solana/scripts/demo/demo-keypairs/README and demo/seed.ts). The SDK is
// reached through the runtime dynamic-import seam (string module specifier), so the vault and
// solana modules are untyped here by construction (same reason as
// `src/solana/current-user-decrypt.ts`): the SDK's generated `_types` are not built at tsc time.
//
// Assertion map — wrap + join legs (deposit direction: join mint = cUSDC):
//   1. alice funded with SOL + mock USDC through the demo faucet         [live, wired below].
//   2. alice's cUSDC confidential token account initialized              [live, SDK, wired below].
//   3. wrap mock USDC → cUSDC confidential balance (public amount)       [live, SDK, wired below].
//   4. on-chain assertion: alice's cUSDC token account exists and is owned by confidential-token.
//   5. precondition: the seeded deposit batcher's current batch is still Pending (joinable).
//   6. input proof for the join amount built locally and verified by the relayer [live, SDK].
//   7. joinBatch: alice joins the pending batch with the attested amount [live, SDK, wired below].
//   8. on-chain assertions: the (batch, alice) join record exists under the batcher program, and
//      the batch's join count incremented by exactly one.

import fs from "node:fs/promises";

import { describe, expect, test } from "bun:test";

import {
  appendTransactionMessageInstructions,
  assertIsTransactionWithBlockhashLifetime,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  getProgramDerivedAddress,
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
import { depositRoots, type VaultDemoRoots } from "../../demo/config";
import {
  addressBytes,
  BALANCE_LABEL,
  encryptedValueAddress,
  TRANSFERRED_AMOUNT_LABEL,
} from "../../demo/encryptedValueLineage";
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
// join measures ~353k CU under mollusk (solana/runtime-tests/cost-snapshots/batcher_mollusk.json),
// but live runs of the confidential-transfer CPI alone were observed above 400k against a ~330k
// mollusk baseline (~1.2x live/mollusk — the reason the SDK's confidentialTransfer action uses
// 800k), and join is that CPI plus batcher evaluation. Match the SDK's 800k; headroom is free.
const JOIN_COMPUTE_UNIT_LIMIT = 800_000;
// `BatchStatus::Pending` in the batcher's generated enum encoding; join.rs rejects anything else.
const BATCH_STATUS_PENDING = 0;

/** Opaque coprocessor proof artifacts: built/submitted by the SDK encrypt client, consumed whole by `joinBatch`. */
type SolanaInputProof = unknown;
type SolanaInputProofSubmission = unknown;

/** The vault surface the scenario drives — wrap provisioning + batch join (untyped: runtime dynamic-import seam). */
type VaultDepositArcSurface = {
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
  getCurrentBatch(
    rpc: unknown,
    roots: VaultDemoRoots,
  ): Promise<{
    index: bigint;
    addresses: { batch: Address; batchAuthority: Address; batchJoinTokenAccount: Address };
    state: { status: number; joinCount: bigint };
  }>;
  pendingJoinLineage(
    batch: Address,
    batchAuthority: Address,
    user: Address,
  ): Promise<{ encryptedValueAddress: Address }>;
  deriveJoinRecordAddress(batch: Address, user: Address): Promise<Address>;
  joinBatch(
    fhevm: { solanaChain: unknown; aclProgramAddress: `0x${string}` },
    parameters: {
      rpc: unknown;
      rpcSubscriptions: unknown;
      inputProof: SolanaInputProof;
      inputProofResult: SolanaInputProofSubmission;
      inputIndex: number;
      user: TransactionSigner;
      payer: TransactionSigner;
      batcher: Address;
      batch: Address;
      joinConfidentialMint: Address;
      userBalanceValue: Address;
      batchBalanceValue: Address;
      userTransferredValue: Address;
      pendingJoinValue: Address;
      hostConfig: Address;
      computeUnitLimit?: number;
    },
  ): Promise<string>;
  CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS: Address;
  CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS: Address;
};

/** The SDK encrypt surface the join leg drives (untyped: runtime dynamic-import seam). */
type SolanaEncryptSurface = {
  setFhevmRuntimeConfig(config: { auth: { type: "ApiKeyHeader"; value: string } }): void;
  defineFhevmSolanaChain(definition: {
    id: bigint;
    fhevm: { relayerUrl: string; acl: { domainKeys: readonly `0x${string}`[] } };
  }): unknown;
  createFhevmEncryptClient(parameters: { chain: unknown; aclProgramAddress: `0x${string}` }): {
    buildInputProof(parameters: {
      contractAddress: `0x${string}`;
      userAddress: `0x${string}`;
      values: readonly { type: "uint64"; value: bigint }[];
    }): Promise<SolanaInputProof>;
    submitInputProof(parameters: { inputProof: SolanaInputProof }): Promise<SolanaInputProofSubmission>;
  };
};

const loadVaultModule = async (): Promise<VaultDepositArcSurface> => {
  const vaultModule = "@fhevm/sdk/solana/vault";
  return (await import(vaultModule)) as unknown as VaultDepositArcSurface;
};

const loadSolanaSdkModule = async (): Promise<SolanaEncryptSurface> => {
  const solanaModule = "@fhevm/sdk/solana";
  return (await import(solanaModule)) as unknown as SolanaEncryptSurface;
};

/** Loads a 64-byte Solana keypair file into a kit `TransactionSigner`. */
const loadSigner = async (keypairPath: string): Promise<TransactionSigner> => {
  const bytes = Uint8Array.from(JSON.parse(await fs.readFile(keypairPath, "utf8")) as number[]);
  return createKeyPairSignerFromBytes(bytes);
};

// EncryptedValue lineage derivation lives in demo/encryptedValueLineage.ts, pinned against the
// Rust golden vectors by its unit test so drift fails in the cheap sweep, not the live gate. Only
// the confidential-token `fhe-compute` compute-signer PDA seed remains restated here.
const COMPUTE_SIGNER_SEED = new TextEncoder().encode("fhe-compute");

/** A base58 address as the bytes32 hex identity the RFC-021 proof binding uses. */
const asBytes32Hex = (value: Address): `0x${string}` =>
  `0x${Buffer.from(addressBytes(value)).toString("hex")}` as `0x${string}`;

// Demo-lane gate: `test:e2e` sweeps this directory on a stack that never ran `demo:seed`, so the
// seeded demo-config cannot exist there. The `demo:smoke` script sets RUN_DEMO_SCENARIOS=1; under
// it the test runs unconditionally, so a missing config still fails the acceptance gate loudly.
const runsDemoScenarios = process.env.RUN_DEMO_SCENARIOS === "1";

describe.skipIf(!runsDemoScenarios)("solana deposit-arc scenario", () => {
  test(
    "deposit arc (wrap + join legs): alice funds, initializes cUSDC, wraps mock USDC, and joins the pending deposit batch with a coprocessor-attested amount",
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

      // Step 4: on-chain assertion for the wrap leg. Read alice's cUSDC confidential token account
      // back and assert it now exists and is owned by the confidential-token program — the concrete
      // state the join leg consumes next. This is the wrap leg's real state check, beyond "did not
      // revert".
      // Read at the same commitment `send` confirmed at: the RPC default is `finalized`, which lags
      // `confirmed` by ~31 slots on the test validator and would race a just-confirmed wrap.
      const aliceCusdc = await vault.tokenAccountAddress(config.mints.joinConfidential, alice.address);
      const account = await rpc.getAccountInfo(aliceCusdc, { encoding: "base64", commitment: "confirmed" }).send();
      expect(account.value).not.toBeNull();
      expect(account.value?.owner).toBe(vault.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);

      // Step 5: precondition for the join leg — join targets the batcher's current batch, which
      // must still be Pending (the seeder opens batch 0 that way). Fail here with a reason instead
      // of an opaque on-chain BatchNotPending revert; re-running against a stack whose batch already
      // dispatched/settled is a dispatch/settle-stage concern (CI always seeds a fresh stack).
      const roots = depositRoots(config);
      const batchBeforeJoin = await vault.getCurrentBatch(rpc, roots);
      if (batchBeforeJoin.state.status !== BATCH_STATUS_PENDING) {
        throw new Error(
          `deposit batch ${batchBeforeJoin.index} (${batchBeforeJoin.addresses.batch}) is not joinable: ` +
            `status ${batchBeforeJoin.state.status} != Pending(${BATCH_STATUS_PENDING}); the stack has ` +
            "moved past the seeded pending batch — rerun demo:seed on a fresh stack.",
        );
      }

      // The relayer's key-material URLs point at the docker-internal host `minio:9000`; rewrite to
      // the host-published endpoint (same same-machine boundary as solana-two-holder-transfer.ts).
      // Restored in the finally below so the patch cannot leak into other files of the same sweep.
      const originalFetch = globalThis.fetch;
      globalThis.fetch = ((url: string | URL | Request, options?: RequestInit) =>
        originalFetch(typeof url === "string" ? url.replace("//minio:9000", "//127.0.0.1:9000") : url, options)) as typeof fetch;

      try {
        // Step 6: build + submit the coprocessor input proof for the join amount. Binding tuple per
        // joinBatch's own checks: contract identity = the join mint's compute-signer PDA (NOT the
        // batcher), user identity = alice, value = euint64 amount, chain id + ACL program from the
        // seeded config. Verification is purely cryptographic — no allowlist.
        const solanaSdk = await loadSolanaSdkModule();
        solanaSdk.setFhevmRuntimeConfig({
          auth: { type: "ApiKeyHeader", value: process.env.ZAMA_FHEVM_API_KEY ?? "local" },
        });
        const chain = solanaSdk.defineFhevmSolanaChain({
          id: BigInt(config.chainId),
          fhevm: { relayerUrl: env.relayerUrl, acl: { domainKeys: [asBytes32Hex(config.mints.joinConfidential)] } },
        });
        const encryptClient = solanaSdk.createFhevmEncryptClient({ chain, aclProgramAddress: config.aclProgram });
        const [joinComputeSigner] = await getProgramDerivedAddress({
          programAddress: vault.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
          seeds: [COMPUTE_SIGNER_SEED, addressBytes(config.mints.joinConfidential)],
        });
        console.log("deposit-arc join: building input proof (local TFHE prover)...");
        const inputProof = await encryptClient.buildInputProof({
          contractAddress: asBytes32Hex(joinComputeSigner),
          userAddress: asBytes32Hex(alice.address),
          values: [{ type: "uint64", value: wrapBaseUnits }],
        });
        console.log("deposit-arc join: submitting input proof to the relayer...");
        const inputProofResult = await encryptClient.submitInputProof({ inputProof });

        // Step 7: join. joinBatch simulates, sends, and confirms; every account is derived from the
        // seeded roots (lineages via demo/encryptedValueLineage + pendingJoinLineage, records via
        // the SDK PDA helpers) — nothing comes from an address dump. Alice pays her own join rent.
        const { batch, batchAuthority, batchJoinTokenAccount } = batchBeforeJoin.addresses;
        const joinMint = config.mints.joinConfidential;
        const hostProgram = config.programs.host;
        console.log(`deposit-arc join: calling joinBatch on batch ${batchBeforeJoin.index} (${batch})...`);
        await vault.joinBatch(
          { solanaChain: chain, aclProgramAddress: config.aclProgram },
          {
            rpc,
            rpcSubscriptions,
            inputProof,
            inputProofResult,
            inputIndex: 0,
            user: alice,
            payer: alice,
            batcher: roots.batcher,
            batch,
            joinConfidentialMint: joinMint,
            userBalanceValue: await encryptedValueAddress(hostProgram, joinMint, aliceCusdc, BALANCE_LABEL),
            batchBalanceValue: await encryptedValueAddress(hostProgram, joinMint, batchJoinTokenAccount, BALANCE_LABEL),
            userTransferredValue: await encryptedValueAddress(hostProgram, joinMint, aliceCusdc, TRANSFERRED_AMOUNT_LABEL),
            pendingJoinValue: (await vault.pendingJoinLineage(batch, batchAuthority, alice.address)).encryptedValueAddress,
            hostConfig: config.hostConfig,
            computeUnitLimit: JOIN_COMPUTE_UNIT_LIMIT,
          },
        );

        // Step 8: on-chain assertions for the join leg. The join handler `init`s the (batch, alice)
        // join record, so its existence under the batcher program proves THIS join executed — not
        // merely that a transaction landed; the join-count increment pins it to the same batch.
        console.log("deposit-arc join: asserting join record + join count on-chain...");
        // joinBatch confirms at `confirmed`; read the record at that same commitment (the RPC
        // default `finalized` lags ~31 slots and would near-deterministically miss a fresh join).
        const joinRecord = await vault.deriveJoinRecordAddress(batch, alice.address);
        const joinRecordAccount = await rpc
          .getAccountInfo(joinRecord, { encoding: "base64", commitment: "confirmed" })
          .send();
        expect(joinRecordAccount.value).not.toBeNull();
        expect(joinRecordAccount.value?.owner).toBe(vault.CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS);
        // getCurrentBatch exposes no commitment parameter (it reads at the RPC default), so poll
        // until the finalized view catches up with the confirmed join instead of racing it.
        const batchAfterJoin = await until(
          async () => {
            const snapshot = await vault.getCurrentBatch(rpc, roots);
            return snapshot.state.joinCount === batchBeforeJoin.state.joinCount + 1n ? snapshot : false;
          },
          { description: "batch join count reflects the confirmed join", timeoutMs: 60_000 },
        );
        expect(batchAfterJoin.addresses.batch).toBe(batch);
        expect(batchAfterJoin.state.joinCount).toBe(batchBeforeJoin.state.joinCount + 1n);
      } finally {
        globalThis.fetch = originalFetch;
      }
    },
    SCENARIO_TIMEOUT_MS,
  );
});
