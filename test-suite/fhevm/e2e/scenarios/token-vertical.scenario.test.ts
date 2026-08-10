// Scenario: the confidential-token consume arc — the [consume] phase ported from
// `solana/scripts/e2e/full-vertical.sh`: wrap -> burn (attested external amount) -> seal ->
// KMS-certified public decrypt -> redeem (SPL release) -> disclose (on-chain cleartext event).
//
// Assertion map — bash step -> this scenario:
//   "OK wrap_usdc" grep -> `wrapUnderlying` throws on failure (simulate+send+confirm).
//   burn input-proof handle scrape (`solana-input.ts` subprocess + python json) -> the in-process
//     SDK encrypt client returns the typed submission; the attestation binds
//     (user = owner, contract = the mint's compute-signer PDA) exactly as the token requires.
//   "burned handle"/"burned amount ACL" stdout scrapes -> typed reads: the burned-amount
//     encrypted value account derives from (mint, token account, burned_amount label) and its
//     current handle is read from chain, not from a log line.
//   burned-handle SNS poll (docker psql loop) -> `stack.waitForSnsCommit`.
//   "OK make_handle_public" grep -> `sealBurnedAmountHandle` throws on failure.
//   leaf_count=2 / leaf_index=0 proof assertions -> `publicDecryptExpect` expectedLeafCount /
//     expectedLeafIndex: the created-public lifecycle leaf (index 0) plus the explicit re-seal
//     (index 1) prove lifecycle-batch ingestion; the semantic endpoint resolves to the EARLIEST.
//   cleartext == burn amount -> `publicDecryptExpect(..., expect: BURN_AMOUNT)`.
//   "OK redeem_burned_amount" grep -> `redeemBurnedAmount` throws on failure, PLUS a stronger
//     assertion the bash never made: the owner's underlying token balance grows by exactly the
//     certified cleartext.
//   "OK disclose_secp" grep -> `discloseBurnedAmount` throws on failure.
//
// Adversarial tail — `adversarial-l4.sh` [L4-b] (context-mismatch cert reuse), ported here because
// it needs exactly this provisioning: a disclose presenting the GENUINE certificate but with
// extra_data naming KMS context 2 while the supplied on-chain kms_context account is the live
// context 1 must be rejected on-chain (the host verifier checks the context binding FIRST, so the
// still-valid MMR proof and signatures never rescue it) and no cleartext event is emitted.
// [L4-a] (publicKey-substitution user-decrypt) is NOT ported: the attack lives in the kms repo's
// live harness (`cargo test -p kms --test solana_user_decrypt_live` with
// SOLANA_UD_ATTACK=pubkey_substitution), which signs a valid v3 request and swaps the ML-KEM
// publicKey after signing — this repo only ever launched it, and that stays its vehicle.

import { describe, expect, test } from "bun:test";

import { getAddressEncoder, type Address } from "@solana/kit";

import { currentHandle, publicDecryptExpect } from "../../src/solana/fhe-vertical";
import {
  createConfidentialMint,
  createSplMint,
  initializeConfidentialTokenAccount,
  mintSplTo,
  wrapUnderlying,
} from "../../src/solana/provision";
import {
  confidentialBurn,
  confidentialBurnTarget,
  discloseBurnedAmount,
  redeemBurnedAmount,
  sealBurnedAmountHandle,
} from "../../src/solana/token-vertical";
import { submitUint64InputProof } from "../harness/solana/sdkEncrypt";
import { verticalSetup } from "../harness/solana/vertical";

// Provisioning (mint + wrap) + burn + SNS commit wait + KMS certificate + two consume sends.
const SCENARIO_TIMEOUT_MS = 20 * 60_000;

const WRAP_AMOUNT = 1000n;
const BURN_AMOUNT = 7n;

const hex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString("hex")}`;
const hexToBytes = (value: string): Uint8Array => Uint8Array.from(Buffer.from(value.replace(/^0x/, ""), "hex"));
const asBytes32Hex = (value: Address): `0x${string}` =>
  `0x${Buffer.from(getAddressEncoder().encode(value)).toString("hex")}` as `0x${string}`;

/**
 * Flattens a kit SolanaError chain into searchable evidence: messages, preflight simulation logs
 * (where anchor prints "Error Code: InvalidKmsContext"), and nested causes.
 */
const errorEvidence = (error: unknown): string => {
  const parts: string[] = [];
  for (let current = error, depth = 0; current && depth < 5; depth++) {
    parts.push(String(current));
    const context = (current as { context?: { logs?: readonly string[] } }).context;
    if (context?.logs) parts.push(...context.logs);
    parts.push(
      JSON.stringify((current as { context?: unknown }).context ?? null, (_key, value: unknown) =>
        typeof value === "bigint" ? value.toString() : value,
      ),
    );
    current = (current as { cause?: unknown }).cause;
  }
  return parts.join("\n");
};

describe("solana confidential-token consume vertical", () => {
  test(
    "wrap 1000 -> burn attested 7 -> seal -> public-decrypt == 7 (leaves 2/0) -> redeem releases 7 -> disclose",
    async () => {
      const { env, stack, context, wallet, config, walletHex } = await verticalSetup();

      // Provision the token pair: a fresh 9-decimals underlying with the wallet as mint authority
      // funded well past the wrap, the confidential wrapper mint with its escrow, the wallet's
      // confidential token account, and 1000 base units wrapped into the confidential balance.
      const underlyingMint = await createSplMint(context, { authority: wallet.signer, decimals: 9 });
      const ownerUnderlying = await mintSplTo(context, {
        authority: wallet.signer,
        mint: underlyingMint,
        recipient: wallet.signer.address,
        baseUnits: 1_000_000n,
      });
      const { mint, computeSigner } = await createConfidentialMint(context, {
        authority: wallet.signer,
        underlyingMint,
      });
      await initializeConfidentialTokenAccount(context, {
        payer: wallet.signer,
        owner: wallet.signer.address,
        mint,
      });
      await wrapUnderlying(context, { owner: wallet.signer, mint, underlyingMint, amount: WRAP_AMOUNT });

      // The burn amount is a coprocessor-attested external input bound to (user = owner,
      // contract = the mint's compute-signer PDA) — the token + host require
      // contract == compute_signer for transfer/burn amounts.
      const submission = await submitUint64InputProof({
        chainId: config.chainId,
        relayerUrl: config.relayerUrl,
        domainKey: asBytes32Hex(mint),
        aclProgramAddress: env.aclProgram,
        contractAddress: asBytes32Hex(computeSigner),
        userAddress: walletHex,
        value: BURN_AMOUNT,
      });
      const amountHandle = hexToBytes(submission.handles[0].bytes32Hex);
      expect(amountHandle).toHaveLength(32);

      await confidentialBurn(context, {
        owner: wallet.signer,
        mint,
        amountAttestation: {
          inputHandle: amountHandle,
          ctHandles: submission.handles.map((handle) => hexToBytes(handle.bytes32Hex)),
          handleIndex: 0,
          userAddress: hexToBytes(walletHex),
          contractAddress: hexToBytes(asBytes32Hex(computeSigner)),
          contractChainId: config.chainId,
          extraData: hexToBytes(submission.extraData),
          signatures: submission.signatures.map((signature) => hexToBytes(signature)),
        },
      });

      const target = await confidentialBurnTarget(mint, wallet.signer.address);
      const burnedHandle = await currentHandle(context, target.burnedAmount.encryptedValueAddress);
      await stack.waitForSnsCommit(hex(burnedHandle));

      // Seal through the token wrapper (it signs the Host CPI as the encrypted value account
      // authority). The burn's created-public lifecycle leaf is index 0; this explicit re-seal
      // appends index 1 — the leaf_count=2 assertion below is what proves lifecycle-batch
      // ingestion, and the semantic endpoint resolves the decrypt to the EARLIEST leaf.
      await sealBurnedAmountHandle(context, { owner: wallet.signer, mint, handle: burnedHandle });

      const { cleartext, certificate } = await publicDecryptExpect(context, config, {
        target: {
          encryptedValue: target.burnedAmount.encryptedValueAddress,
          encryptedValueId: target.burnedAmount.aclValueKey,
        },
        handle: burnedHandle,
        expect: BURN_AMOUNT,
        expectedLeafCount: 2n,
        expectedLeafIndex: 0n,
      });
      expect(cleartext).toBe(BURN_AMOUNT);

      // Redeem: the host verifier CPI checks the KMS certificate against the live context it
      // names plus the burned handle's MMR public-leaf proof, the PendingBurn closes, and the
      // certified amount of underlying releases to the owner.
      const balanceBefore = BigInt(
        (await context.rpc.getTokenAccountBalance(ownerUnderlying, { commitment: "confirmed" }).send()).value.amount,
      );
      await redeemBurnedAmount(context, { owner: wallet.signer, mint, underlyingMint, certificate });
      const balanceAfter = BigInt(
        (await context.rpc.getTokenAccountBalance(ownerUnderlying, { commitment: "confirmed" }).send()).value.amount,
      );
      expect(balanceAfter - balanceBefore).toBe(BURN_AMOUNT);

      // Disclose: same verifier CPI, then the token-scoped cleartext event. Idempotent by design;
      // the burn already sealed the leaf, so the certificate's proof stays valid through both.
      await discloseBurnedAmount(context, { owner: wallet.signer, mint, certificate });

      // Adversarial (adversarial-l4.sh [L4-b]): the same genuine certificate, but its extra_data
      // rewritten to name KMS context 2 (version byte 0x01 + 32-byte BE id) while the supplied
      // kms_context account is the live context 1. extract_kms_context_id(extra) = 2 does not
      // match the supplied account, so verify_public_decrypt fails closed on the context binding
      // before ever weighing the (still valid) signatures and MMR proof. Not just any rejection
      // counts: the evidence must name InvalidKmsContext (zama-host anchor error 6064 = 0x17b0),
      // or a blockhash expiry / client encode error would read as the attack being repelled.
      const wrongContextCertificate = { ...certificate, extraData: `0x01${(2n).toString(16).padStart(64, "0")}` };
      const rejection = await discloseBurnedAmount(context, {
        owner: wallet.signer,
        mint,
        certificate: wrongContextCertificate,
      }).then(
        () => undefined,
        (error: unknown) => error,
      );
      if (rejection === undefined) {
        throw new Error("SECURITY: context-mismatched certificate was disclosed on-chain");
      }
      expect(errorEvidence(rejection)).toMatch(/InvalidKmsContext|0x17b0|Custom":\s*6064/);
    },
    SCENARIO_TIMEOUT_MS,
  );
});
