import { asBytes32Hex } from '@fhevm/sdk/base';
import { appendTransientStoreInstructions, prepareTransientStore } from "@fhevm/sdk/solana";
import { encryptedStoreHandle } from "@fhevm/sdk/solana";
import { SOLANA_LEAF_PROOF_PORT, SOLANA_LEAF_PROOF_API_KEY } from "../../src/generate/solana";
// Live vault deposit: fund → wrap → join → dispatch → public decrypt → settle → claim → user decrypt.
// Run through `demo:smoke` against a seeded demo with its faucet running. On-chain assertions
// check each transition; the final KMS/WASM decrypt checks the encrypted payout amount.

import fs from "node:fs/promises";

import { describe, expect, test } from "bun:test";

import {
  appendTransactionMessageInstructions,
  assertIsTransactionWithBlockhashLifetime,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  getAddressEncoder,
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
import { withHostReachableFetch } from "../harness/solana/sdkEncrypt";
import { waitForSnsCommit } from "../../src/solana/sns";
import { depositRoots, type VaultDemoRoots } from "../../demo/config";
import { readCurrentDemoAuthorization } from "../../demo/lifecycle";
import { DEMO_KEYPAIRS, loadDemoEnv } from "../../demo/loadDemoEnv";

// A live batcher arc waits on slot age + SNS commit + settle certificate + the decrypt roundtrip.
// The bounded waits below sum to ~17.5 min worst case (health 3.5 + join visibility 1 + slot age 2
// + dispatch visibility 1 + SNS proof 5 + settle visibility 1 + claim visibility 1 + decrypt 3),
// so 30 min keeps ~12 min for the unwaited work. The HTTP probes fail on their own per-request +
// until() bounds; the RPC-backed waits carry no per-request bound (they rely on the transport), so
// a hung RPC read is ultimately caught by this scenario timeout.
const SCENARIO_TIMEOUT_MS = 30 * 60_000;

// The lifecycle-owned demo faucet binds loopback on 8090 and is health-gated before this runs.
// The endpoint remains overridable for a non-default run.
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
// dispatch measures ~304k CU under mollusk (batcher_mollusk.json `dispatch`); the same ~1.2x
// live/mollusk factor observed on the transfer CPI puts it near ~365k, so 600k is ample headroom.
const DISPATCH_COMPUTE_UNIT_LIMIT = 600_000;
// claim measures ~311k CU under mollusk (batcher_mollusk.json `claim`); the same ~1.2x factor puts
// it near ~373k, so 600k is ample headroom.
const CLAIM_COMPUTE_UNIT_LIMIT = 600_000;
// Bound for the user-decrypt relayer roundtrip: the SDK's default request timeout is one hour
// (RelayerAsyncRequest), which would let a stuck decrypt eat the whole scenario budget silently.
const DECRYPT_ROUNDTRIP_TIMEOUT_MS = 180_000;
// `BatchStatus` in the batcher's generated enum encoding (Pending=0, Dispatched=1, Settled=2).
const BATCH_STATUS_PENDING = 0;
const BATCH_STATUS_DISPATCHED = 1;
const BATCH_STATUS_SETTLED = 2;

import * as vault from "@demo-dapp/vault/index.js";
import type { FhevmSolanaChain } from "@fhevm/sdk/solana";
import type { Bytes32Hex } from "@fhevm/sdk/types";

const loadSolanaSdkModule = () => import('@fhevm/sdk/solana');

/** Loads a 64-byte Solana keypair file into a kit `TransactionSigner`. */
const loadSigner = async (keypairPath: string): Promise<TransactionSigner> => {
  const bytes = Uint8Array.from(JSON.parse(await fs.readFile(keypairPath, "utf8")) as number[]);
  return createKeyPairSignerFromBytes(bytes);
};

/** A base58 address as the bytes32 hex identity the RFC-021 proof binding uses. */
const addressToBytes32Hex = (value: Address): Bytes32Hex =>
  asBytes32Hex(`0x${Buffer.from(getAddressEncoder().encode(value)).toString("hex")}`);
const addressBytes = (value: Address): Uint8Array => new Uint8Array(getAddressEncoder().encode(value));

/** An unsigned decimal string as big-endian bytes32 — the shape the settle certificate's and the user-decrypt request's contextId take. */
const asBytes32BigEndian = (decimal: string): Uint8Array => {
  const bytes = new Uint8Array(32);
  let value = BigInt(decimal);
  for (let index = 31; index >= 0 && value > 0n; index -= 1) {
    bytes[index] = Number(value & 0xffn);
    value >>= 8n;
  }
  if (value > 0n) throw new Error(`${decimal} does not fit in 32 bytes`);
  return bytes;
};

// Demo-lane gate: `test:e2e` sweeps this directory on a stack that never ran `demo:seed`, so the
// seeded demo-config cannot exist there. The `demo:smoke` script sets RUN_DEMO_SCENARIOS=1; under
// it the test runs unconditionally, so a missing config still fails the acceptance gate loudly.
const runsDemoScenarios = process.env.RUN_DEMO_SCENARIOS === "1";

/** Written by the arc on success; `demo:smoke` requires it back so a skipped suite cannot pass. */
export const DEMO_SMOKE_MARKER = "/tmp/fhevm-demo-smoke-ran";

describe.skipIf(!runsDemoScenarios)("solana deposit-arc scenario", () => {
  test(
    "deposit arc (full arc): alice funds, initializes her confidential accounts, wraps mock USDC, and joins the pending deposit batch with a coprocessor-attested amount; the keeper dispatches the aged batch and settles it with the KMS burn certificate; alice claims her payout and user-decrypts the exact amount",
    async () => {
      const { env, config } = await loadDemoEnv();
      const authorization = await readCurrentDemoAuthorization();

      // Personas: the keeper is the operator that plays dispatch + settle; alice is the depositing
      // end-user. Both load from committed demo keypairs (pubkeys cross-checked against the config).
      const personas = await loadPersonas(env, {
        keeper: DEMO_KEYPAIRS.keeper,
        alice: DEMO_KEYPAIRS.alice,
      });
      const alicePersona = personas.roles.alice;
      if (!alicePersona) throw new Error("alice persona did not load");

      // Wrap + join + claim + decrypt are signed by alice, dispatch + settle by the keeper; load
      // both keypairs as signers and prove they are the pubkeys the seed published, so a
      // keypair/config drift fails here rather than on-chain. Alice's raw bytes are kept: the
      // decrypt phase signs the user-decrypt request with her 32-byte ed25519 seed (the first half of
      // the 64-byte keypair file) through the SDK's own signer wrapper.
      const aliceKeypairBytes = Uint8Array.from(
        JSON.parse(await fs.readFile(DEMO_KEYPAIRS.alice, "utf8")) as number[],
      );
      const alice = await createKeyPairSignerFromBytes(aliceKeypairBytes);
      if (alice.address !== config.personas.alice) {
        throw new Error(`alice keypair ${alice.address} does not match seeded persona ${config.personas.alice}`);
      }
      const keeper = await loadSigner(DEMO_KEYPAIRS.keeper);
      if (keeper.address !== config.personas.keeper) {
        throw new Error(`keeper keypair ${keeper.address} does not match seeded persona ${config.personas.keeper}`);
      }

      // Preconditions: the suite may run right after a relayer (re)start. Gate on its health
      // endpoint before submitting (same gate as the confidential-transfer scenario), plus the
      // faucet the persona funds through. Every probe carries a per-request abort timeout:
      // until() checks its deadline only between attempts, so a hanging TCP connect would otherwise
      // stall the whole test to the runner's limit.
      await until(
        async () => (await fetch(`${env.relayerUrl}/liveness`, { signal: AbortSignal.timeout(10_000) })).ok,
        { description: "relayer liveness", timeoutMs: 60_000 },
      );
      await until(
        async () => (await fetch(`${FAUCET_URL}/health`, { signal: AbortSignal.timeout(10_000) })).ok,
        { description: "demo faucet health", timeoutMs: 30_000 },
      );

      // Step 1: fund alice — SOL through the persona/faucet capability, mock USDC through the faucet's
      // mint-to-ATA endpoint (the ATA is created idempotently by the faucet).
      await personas.fund(alicePersona);
      const mintUsdc = await fetch(`${FAUCET_URL}/mint-usdc`, {
        method: "POST",
        headers: {
          authorization: `Bearer ${authorization.token}`,
          "content-type": "application/json",
          origin: "http://127.0.0.1:5173",
          "x-fhevm-demo-boot-id": authorization.bootId,
        },
        body: JSON.stringify({ address: alice.address, amount: DEPOSIT_USDC }),
      });
      if (!mintUsdc.ok) {
        throw new Error(`faucet /mint-usdc failed (${mintUsdc.status}): ${await mintUsdc.text()}`);
      }

      const rpc = createSolanaRpc(env.rpcUrl);
      const rpcSubscriptions = createSolanaRpcSubscriptions(env.wsUrl);
      const sendAndConfirm = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });

      /** Signs `instructions` with `payer` (fee payer) plus any account-embedded signers, then confirms. */
      const send = async (
        payer: TransactionSigner,
        instructions: readonly Instruction[],
        computeUnitLimit: number = WRAP_COMPUTE_UNIT_LIMIT,
      ): Promise<void> => {
        const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
        const base = setTransactionMessageFeePayerSigner(payer, createTransactionMessage({ version: 0 }));
        const withLifetime = setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, base);
        const withComputeLimit = setTransactionMessageComputeUnitLimit(computeUnitLimit, withLifetime);
        const message = appendTransactionMessageInstructions(instructions, withComputeLimit);
        const signedTransaction = await signTransactionMessageWithSigners(message);
        assertIsTransactionWithBlockhashLifetime(signedTransaction);
        await sendAndConfirm(signedTransaction, { commitment: "confirmed" });
      };


      const aliceTransientStore = await prepareTransientStore({ payer: alice, host: config.programs.host });
      // Step 2: create alice's confidential token accounts — cUSDC (join mint) for the wrap, and
      // cShares (payout mint) for the claim phase: claim.rs requires the user's payout account to
      // ALREADY exist (nothing creates it on the fly), so it is provisioned here with the same
      // one-time initialization the join mint gets, keeping the claim phase a pure claim. initialize
      // + wrap both revert on failure, so their confirmation IS the assertion for these phases.
      await send(alice, appendTransientStoreInstructions(aliceTransientStore, [
        await vault.buildInitializeTokenAccountInstruction({
          transientStore: aliceTransientStore,
          payer: alice,
          owner: alice.address,
          mint: config.mints.joinConfidential,
          hostConfig: config.hostConfig,
        }),
        await vault.buildInitializeTokenAccountInstruction({
          transientStore: aliceTransientStore,
          payer: alice,
          owner: alice.address,
          mint: config.mints.payoutConfidential,
          hostConfig: config.hostConfig,
        }),
      ]));

      // Step 3: wrap the funded mock USDC into alice's confidential cUSDC balance. wrap_usdc escrows a
      // PUBLIC amount and needs no input proof, which is why it wires cheaply here.
      const wrapBaseUnits = BigInt(Math.round(DEPOSIT_USDC * 10 ** USDC_DECIMALS));
      await send(alice, appendTransientStoreInstructions(aliceTransientStore, [
        await vault.buildWrapUsdcInstruction({
          transientStore: aliceTransientStore,
          owner: alice,
          mint: config.mints.joinConfidential,
          underlyingMint: config.mints.joinUnderlying,
          tokenProgram: vault.TOKEN_PROGRAM_ADDRESS,
          hostConfig: config.hostConfig,
          amount: wrapBaseUnits,
        }),
      ]));

      // Step 4: on-chain assertion for the wrap phase. Read alice's cUSDC confidential token account
      // back and assert it now exists and is owned by the confidential-token program — the concrete
      // state the join phase consumes next. This is the wrap phase's real state check, beyond "did not
      // revert".
      // Read at the same commitment `send` confirmed at: the RPC default is `finalized`, which lags
      // `confirmed` by ~31 slots on the test validator and would race a just-confirmed wrap.
      const aliceCusdc = await vault.tokenAccountAddress(config.mints.joinConfidential, alice.address);
      const account = await rpc.getAccountInfo(aliceCusdc, { encoding: "base64", commitment: "confirmed" }).send();
      expect(account.value).not.toBeNull();
      expect(account.value?.owner).toBe(vault.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);

      // Step 5: precondition for the join phase — join targets the batcher's current batch, which
      // must still be Pending (the seeder opens batch 0 that way). Fail here with a reason instead
      // of an opaque on-chain BatchNotPending revert. A rerun against a stack whose batch this test
      // already dispatched/settled fails here by design — the arc needs a fresh Pending batch, and
      // CI always seeds a fresh stack.
      const roots = depositRoots(config);
      const batchBeforeJoin = await vault.getCurrentBatch(rpc, roots);
      if (batchBeforeJoin.state.status !== BATCH_STATUS_PENDING) {
        throw new Error(
          `deposit batch ${batchBeforeJoin.index} (${batchBeforeJoin.addresses.batch}) is not joinable: ` +
            `status ${batchBeforeJoin.state.status} != Pending(${BATCH_STATUS_PENDING}); the stack has ` +
            "moved past the seeded pending batch — rerun demo:seed on a fresh stack.",
        );
      }

      // SDK client setup + the derivations the join, dispatch and settle phases share. All of this is
      // pure/local (no network), so it sits OUTSIDE the fetch patch below — dispatch and settle run
      // after the patch is restored and still need these bindings. The runtime auth config set here
      // is process-global and also serves settle's certificate phase (its runtime consumes only
      // `runtime.config.auth`), so it is not set a second time there.
      const solanaSdk = await loadSolanaSdkModule();
      solanaSdk.setFhevmRuntimeConfig({
        auth: { type: "ApiKeyHeader", value: process.env.ZAMA_FHEVM_API_KEY ?? "local" },
      });
      const chain = solanaSdk.defineFhevmSolanaChain({
        id: BigInt(config.chainId),
        fhevm: { relayerUrl: env.relayerUrl, programs: { host: { address: asBytes32Hex(config.aclProgram) } } },
      }) as FhevmSolanaChain;
      const encryptClient = solanaSdk.createFhevmEncryptClient({ chain, rpc });
      const { batch, batchAuthority, batchJoinTokenAccount } = batchBeforeJoin.addresses;
      const joinMint = config.mints.joinConfidential;

      // The MinIO fetch rewrite is scoped to the join phase only: just the input proof's
      // key-material fetch needs it — settle's certificate phase talks to the relayer's
      // /v2/public-decrypt endpoint only (verified against actions/publicDecryptCertificate.ts).
      await withHostReachableFetch(async () => {
        // Step 6: build + submit the coprocessor input proof for the join amount. Binding tuple per
        // joinBatch's own checks: contract identity = the confidential-token program (NOT the
        // batcher), user identity = alice, value = euint64 amount, chain id + ACL program from the
        // seeded config. Verification is purely cryptographic — no allowlist.
        console.log("deposit-arc join: building input proof (local TFHE prover)...");
        const { inputProof } = await encryptClient.encryptValues({
          contractAddress: addressToBytes32Hex(vault.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS),
          userAddress: addressToBytes32Hex(alice.address),
          values: [{ type: "uint64", value: wrapBaseUnits }],
        });

        // Step 7: join. joinBatch simulates, sends, and confirms; it derives every encrypted value account and
        // authority account internally from the semantic roots passed here — nothing comes from an
        // address dump. Alice pays her own join rent.
        console.log(`deposit-arc join: calling joinBatch on batch ${batchBeforeJoin.index} (${batch})...`);
        await vault.joinBatch(
          { solanaChain: chain, aclProgramAddress: asBytes32Hex(config.aclProgram) },
          {
            rpc,
            rpcSubscriptions,
            inputProof: inputProof as never,
            inputIndex: 0,
            user: alice,
            payer: alice,
            batcher: roots.batcher,
            batch,
            joinConfidentialMint: joinMint,
            joinUnderlyingMint: roots.joinUnderlyingMint,
            tokenProgram: vault.TOKEN_PROGRAM_ADDRESS,
            hostConfig: config.hostConfig,
            computeUnitLimit: JOIN_COMPUTE_UNIT_LIMIT,
          },
        );

        // Step 8: on-chain assertions for the join phase. The join handler `init`s the (batch, alice)
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
      });

      // Step 9: wait until the batch is old enough to dispatch. dispatch.rs rejects with
      // BatchTooYoung until the current slot reaches openedSlot + minBatchAgeSlots (the seeder sets
      // 25 slots, ~10s live), so wait for the slot age explicitly instead of catch-and-retrying an
      // on-chain revert. The dispatch transaction executes at a slot >= the confirmed slot observed
      // here, so this condition is sufficient, not merely close.
      const { minBatchAgeSlots } = await vault.getBatcher(rpc, roots.batcher);
      const dispatchableAtSlot = batchBeforeJoin.state.openedSlot + minBatchAgeSlots;
      console.log(`deposit-arc dispatch: waiting for batch to reach min dispatch age (slot ${dispatchableAtSlot})...`);
      await until(
        async () => (await rpc.getSlot({ commitment: "confirmed" }).send()) >= dispatchableAtSlot,
        { description: `batch reaches its minimum dispatch age (slot ${dispatchableAtSlot})`, timeoutMs: 120_000 },
      );

      // Step 10: dispatch. Permissionless on-chain; the demo has the keeper play it (and pay the
      // burn's output ACL rent). The SDK builder derives every validated account — authorities,
      // encrypted value accounts, event authorities — from these five roots (its unit test pins each derivation
      // against dispatch.rs), so nothing comes from an address dump.
      console.log(`deposit-arc dispatch: keeper dispatching batch ${batchBeforeJoin.index} (${batch})...`);
      const keeperTransientStore = await prepareTransientStore({ payer: keeper, host: config.programs.host });
      await send(
        keeper,
        appendTransientStoreInstructions(keeperTransientStore, [
          await vault.buildDispatchBatchInstruction({
            transientStore: keeperTransientStore,
            payer: keeper,
            batcher: roots.batcher,
            batch,
            joinConfidentialMint: joinMint,
            joinUnderlyingMint: roots.joinUnderlyingMint,
            tokenProgram: vault.TOKEN_PROGRAM_ADDRESS,
            hostConfig: config.hostConfig,
          }),
        ]),
        DISPATCH_COMPUTE_UNIT_LIMIT,
      );

      // Step 11: on-chain assertions for the dispatch phase. The burn records a created-public burned
      // total handle on the batch; settle refuses a zero handle, so assert both the status flip and
      // the nonzero handle here (getCurrentBatch reads at the RPC default `finalized`, hence until).
      console.log("deposit-arc dispatch: asserting batch status Dispatched + burned handle on-chain...");
      const batchAfterDispatch = await until(
        async () => {
          const snapshot = await vault.getCurrentBatch(rpc, roots);
          return snapshot.state.status === BATCH_STATUS_DISPATCHED &&
            snapshot.state.burnedTotalHandle.some((byte) => byte !== 0)
            ? snapshot
            : false;
        },
        { description: "batch status Dispatched with a nonzero burned total handle", timeoutMs: 60_000 },
      );
      expect(batchAfterDispatch.addresses.batch).toBe(batch);

      // Step 12: wait for the SNS commit of the burned total handle. The KMS certificate request
      // inside settleBatch needs the ciphertext materialized; the Connector reads the public leaf
      // itself, so nothing else has to catch up first.
      const burnedHandleHex = `0x${Buffer.from(batchAfterDispatch.state.burnedTotalHandle).toString("hex")}`;
      console.log("deposit-arc settle: waiting for the SNS commit of the burned total handle...");
      await waitForSnsCommit(burnedHandleHex, env.coprocessorDbPsql);

      // Step 13: settle. One SDK call runs both off-chain phases (the MMR inclusion proof rebuilt
      // from the burned value's account history, and the KMS burn certificate — its runtime
      // consumes the auth config already set before the join) and the on-chain settle as a v0
      // transaction against the seeded lookup table. The keeper signs;
      // authorityFundingLamports must suffice to cover the rent settle's CPIs charge to this
      // batch's authority — the seed recorded the open_batch value as a known-good amount.
      console.log("deposit-arc settle: calling settleBatch (MMR proof + KMS certificate + on-chain settle)...");
      const publicDecryptClient = solanaSdk.createFhevmPublicDecryptClient({ chain, rpc });
      await vault.settleBatch(publicDecryptClient, keeper, {
        rpc,
        rpcSubscriptions,
        proofService: { url: `http://127.0.0.1:${SOLANA_LEAF_PROOF_PORT}`, apiKey: SOLANA_LEAF_PROOF_API_KEY },
        roots,
        contextId: asBytes32BigEndian(config.userDecryptContextId),
        lookupTableAddress: config.batchers.deposit.lookupTable,
        authorityFundingLamports: BigInt(config.authorityFundingLamports),
      });

      // Step 14: on-chain assertions for the settle phase. A settled batch publishes its certified
      // totals: with a single join the batch total IS alice's deposit — inherent to a one-member
      // batch, not a leak this scenario introduces — so it can be asserted exactly. payoutReceived
      // is the vault's share payout for that total; payoutRate is informational and left alone.
      console.log("deposit-arc settle: asserting settled batch state on-chain...");
      const batchAfterSettle = await until(
        async () => {
          const snapshot = await vault.getCurrentBatch(rpc, roots);
          return snapshot.state.status === BATCH_STATUS_SETTLED ? snapshot : false;
        },
        { description: "batch status reflects the confirmed settle", timeoutMs: 60_000 },
      );
      expect(batchAfterSettle.addresses.batch).toBe(batch);
      expect(batchAfterSettle.state.totalJoined).toBe(wrapBaseUnits);
      expect(batchAfterSettle.state.payoutReceived > 0n).toBe(true);
      const settleAccounts = await vault.deriveSettleAccounts(roots, batchAfterSettle.addresses);
      const verifiedTotal = await publicDecryptClient.decryptPublicValue({
        handle: burnedHandleHex,
        encryptedStore: addressBytes(settleAccounts.batchBurnedAmountStore),
        options: { timeout: DECRYPT_ROUNDTRIP_TIMEOUT_MS },
      });
      expect(verifiedTotal.type).toBe("uint64");
      expect(verifiedTotal.value as bigint).toBe(wrapBaseUnits);


      // Claim into Alice's initially empty payout balance, then decrypt that balance to verify
      // the transfer credited the full proportional payout. No separate claim output is stored.
      const payoutMint = config.mints.payoutConfidential;
      const claimValueAccount = await vault.tokenStateAddress(payoutMint, await vault.tokenAccountAddress(payoutMint, alice.address));
      console.log(`deposit-arc claim: alice claiming her payout from batch ${batchBeforeJoin.index} (${batch})...`);
      await send(
        alice,
        appendTransientStoreInstructions(aliceTransientStore, [
          await vault.buildClaimInstruction({
            transientStore: aliceTransientStore,
            payer: alice,
            user: alice.address,
            batcher: roots.batcher,
            batch,
            payoutConfidentialMint: payoutMint,
            payoutUnderlyingMint: roots.payoutUnderlyingMint,
            tokenProgram: vault.TOKEN_PROGRAM_ADDRESS,
            hostConfig: config.hostConfig,
          }),
        ]),
        CLAIM_COMPUTE_UNIT_LIMIT,
      );

      // The claimed flag and credited payout balance must both be committed.
      console.log("deposit-arc claim: asserting claimed flag + claim encrypted value account on-chain...");
      // `send` confirmed at `confirmed`; read the record at the same commitment (the RPC default
      // `finalized` lags ~31 slots and would race the fresh claim).
      const joinRecordAfterClaim = await vault.getJoinRecord(
        rpc,
        await vault.deriveJoinRecordAddress(batch, alice.address),
        { commitment: "confirmed" },
      );
      expect(joinRecordAfterClaim.user).toBe(alice.address);
      expect(joinRecordAfterClaim.claimed).toBe(true);
      // getEncryptedStore throws while the account is missing and reads at the RPC default
      // `finalized`; until() swallows probe errors until its deadline, so poll it.
      const claimValueState = await until(
        async () => {
          const state = await vault.getEncryptedStore(publicDecryptClient, claimValueAccount);
          return encryptedStoreHandle(state, new TextEncoder().encode("balance_________________________")).some((byte) => byte !== 0) ? state : false;
        },
        { description: "claim-amount encrypted value account exists with a nonzero current handle", timeoutMs: 60_000 },
      );

      // Decrypt the credited balance through the signed permit and Connector proof path.
      console.log("deposit-arc decrypt: alice user-decrypting her claimed payout (KMS roundtrip)...");
      const aliceWallet = solanaSdk.solanaPermitWalletFromSecretKey(aliceKeypairBytes);
      const decryptChain = solanaSdk.defineFhevmSolanaChain({
        id: BigInt(config.chainId),
        fhevm: { relayerUrl: env.relayerUrl, programs: { host: { address: asBytes32Hex(config.aclProgram) } } },
      });
      const decryptClient = solanaSdk.createFhevmDecryptClient({
        chain: decryptChain,
        rpc,
        trust: {
          // Party ids follow the registry order — the same assumption the EVM SDK path makes.
          kmsSigners: config.kmsSigners.map((signer, index) => ({ partyId: index + 1, address: signer })),
          kmsContextId: asBytes32Hex(`0x${BigInt(config.userDecryptContextId).toString(16).padStart(64, "0")}`),
          kmsEpochId: asBytes32Hex(config.kmsEpochId),
          fheParameter: config.fheParameter,
          gatewayEip712Domain: {
            name: "Decryption",
            version: "1",
            chainId: BigInt(config.gatewayChainId),
            verifyingContract: config.gatewayDecryptionContract,
          },
        },
      });
      await decryptClient.ready;
      const permitSession = await decryptClient.signPermit({ wallet: aliceWallet, durationSeconds: 3_600n });
      const clearValues = await decryptClient.decryptValues({
        session: permitSession,
        entries: [{ handle: encryptedStoreHandle(claimValueState, new TextEncoder().encode("balance_________________________")), encryptedStore: addressBytes(claimValueAccount) }],
        options: { timeout: DECRYPT_ROUNDTRIP_TIMEOUT_MS },
      });

      // Alice is the batch's sole joiner, so totalJoined == her joined amount and the claim's
      // floor(joined x payoutReceived / totalJoined) is EXACTLY payoutReceived: assert equality,
      // not just "> 0" — this is the one place the arc proves the encrypted plumbing carried the
      // right number end to end.
      expect(clearValues.length).toBe(1);
      const payout = clearValues[0]!.value;
      expect(typeof payout).toBe("bigint");
      expect(payout === batchAfterSettle.state.payoutReceived).toBe(true);

      // Proof-of-run for the `demo:smoke` gate. `bun test` exits 0 when every matched test is
      // SKIPPED (measured: `0 pass, 1 skip`, exit 0), so renaming RUN_DEMO_SCENARIOS on either
      // side of the gate above would silently retire this whole arc with CI still green. The
      // script deletes this marker, runs the suite, and then requires it back; nothing but this
      // arc completing can produce it.
      await Bun.write(DEMO_SMOKE_MARKER, new Date().toISOString());
    },
    SCENARIO_TIMEOUT_MS,
  );
});
