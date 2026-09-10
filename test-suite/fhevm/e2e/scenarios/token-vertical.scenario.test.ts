// Live token consume arc: wrap -> burn -> publish -> certified decrypt -> redeem -> disclose.
// Also checks that changing the certificate's context id is rejected before signature verification.

import { describe, expect, test } from "bun:test";

import { getAddressEncoder, isSolanaError, SOLANA_ERROR__INSTRUCTION_ERROR__CUSTOM, type Address } from "@solana/kit";

import { livePublicLeafProof, certifiedPublicDecrypt, currentHandle } from "../../src/solana/fhe-vertical";
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
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from "@demo-dapp/vault/index.js";
import { submitUint64InputProof } from "../harness/solana/sdkEncrypt";
import { verticalSetup } from "../harness/solana/vertical";

// Provisioning (mint + wrap) + burn + SNS commit wait + KMS certificate + two consume sends.
const SCENARIO_TIMEOUT_MS = 20 * 60_000;

const WRAP_AMOUNT = 1000n;
const BURN_AMOUNT = 7n;
const hostIdl: { errors: readonly { name: string; code: number }[] } = await Bun.file(
  new URL("../../../../coprocessor/fhevm-engine/host-listener/idl/zama_host.json", import.meta.url),
).json();

const hex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString("hex")}`;
const hexToBytes = (value: string): Uint8Array => Uint8Array.from(Buffer.from(value.replace(/^0x/, ""), "hex"));
const asBytes32Hex = (value: Address): `0x${string}` =>
  `0x${Buffer.from(getAddressEncoder().encode(value)).toString("hex")}` as `0x${string}`;

/**
 * Walks a kit SolanaError cause chain (preflight failure -> transaction error -> instruction
 * error) to the custom program error code, if one is there.
 */
const customProgramErrorCode = (error: unknown): number | undefined => {
  for (let current = error, depth = 0; current && depth < 8; depth++) {
    if (isSolanaError(current, SOLANA_ERROR__INSTRUCTION_ERROR__CUSTOM)) return Number(current.context.code);
    current = (current as { cause?: unknown }).cause;
  }
  return undefined;
};

describe("solana confidential-token consume vertical", () => {
  test(
    "wrap 1000 -> burn attested 7 -> seal -> public-decrypt == 7 -> redeem releases 7 (leaf 1 of 3) -> disclose",
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
      const mint = await createConfidentialMint(context, { authority: wallet.signer, underlyingMint });
      await initializeConfidentialTokenAccount(context, {
        payer: wallet.signer,
        owner: wallet.signer.address,
        mint,
      });
      await wrapUnderlying(context, { owner: wallet.signer, mint, underlyingMint, amount: WRAP_AMOUNT });

      // The burn amount is a coprocessor-attested external input bound to (user = owner,
      // contract = the confidential-token program) — the contract identity the token requires
      // for transfer/burn amounts.
      const contractAddress = asBytes32Hex(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
      const submission = await submitUint64InputProof({
        chainId: config.chainId,
        relayerUrl: config.relayerUrl,
        aclProgramAddress: env.aclProgram,
        contractAddress,
        userAddress: walletHex,
        value: BURN_AMOUNT,
      });
      const amountHandle = hexToBytes(submission.handles[0].bytes32Hex);
      expect(amountHandle).toHaveLength(32);

      await confidentialBurn(context, {
        owner: wallet.signer,
        mint,
        underlyingMint,
        amountAttestation: {
          inputHandle: amountHandle,
          ctHandles: submission.handles.map((handle) => hexToBytes(handle.bytes32Hex)),
          handleIndex: 0,
          userAddress: hexToBytes(walletHex),
          contractAddress: hexToBytes(contractAddress),
          contractChainId: config.chainId,
          extraData: hexToBytes(submission.extraData),
          signatures: submission.signatures.map((signature) => hexToBytes(signature)),
        },
      });

      const target = await confidentialBurnTarget(mint, wallet.signer.address);
      const burnedHandle = await currentHandle(context, target.burnedAmountStore, new TextEncoder().encode("burned_amount___________________"));
      await stack.waitForSnsCommit(hex(burnedHandle));

      // Seal through the token wrapper (it signs the Host CPI as the State authority). The burn already appended [allowed(owner), markedPublic]; this explicit re-seal
      // appends a third leaf. The proof below is built for the burn's own public leaf (index 1)
      // against the 3-leaf history, which is what proves both the lifecycle leaves and the re-seal
      // reached the account in order.
      await sealBurnedAmountHandle(context, { owner: wallet.signer, mint, handle: burnedHandle });
      const inclusionProof = await livePublicLeafProof(
        context,
        target.burnedAmountStore,
        burnedHandle,
      );

      const { cleartext, certificate } = await certifiedPublicDecrypt(config, {
        encryptedStore: target.burnedAmountStore,
        handle: burnedHandle,
      });
      expect(cleartext).toBe(BURN_AMOUNT);

      // Redeem: the host verifier CPI checks the KMS certificate against the live context it
      // names plus the burned handle's MMR public-leaf proof, the PendingBurn closes, and the
      // certified amount of underlying releases to the owner.
      const balanceBefore = BigInt(
        (await context.rpc.getTokenAccountBalance(ownerUnderlying, { commitment: "confirmed" }).send()).value.amount,
      );
      await redeemBurnedAmount(context, { owner: wallet.signer, mint, underlyingMint, certificate, inclusionProof });
      const balanceAfter = BigInt(
        (await context.rpc.getTokenAccountBalance(ownerUnderlying, { commitment: "confirmed" }).send()).value.amount,
      );
      expect(balanceAfter - balanceBefore).toBe(BURN_AMOUNT);

      // Disclose: same verifier CPI, then the generic state/handle/cleartext event. Idempotent by design;
      // the burn already sealed the leaf, so the certificate's proof stays valid through both.
      await discloseBurnedAmount(context, { owner: wallet.signer, mint, certificate, inclusionProof });

      // Keep the v4 carrier and state binding intact; change only the committed context id.
      // The host must reject the context mismatch before checking the certificate signature.
      const wrongContextExtraData = hexToBytes(certificate.extraData);
      expect(wrongContextExtraData.length).toBe(65);
      expect(wrongContextExtraData[0]).toBe(4);
      wrongContextExtraData[32] = wrongContextExtraData[32]! ^ 1;
      const wrongContextCertificate = { ...certificate, extraData: hex(wrongContextExtraData) };
      const rejection = await discloseBurnedAmount(context, {
        owner: wallet.signer,
        mint,
        certificate: wrongContextCertificate,
        inclusionProof,
      }).then(
        () => undefined,
        (error: unknown) => error,
      );
      if (rejection === undefined) {
        throw new Error("SECURITY: context-mismatched certificate was disclosed on-chain");
      }
      // Pin the named rejection (zama_host IDL: InvalidKmsContext), not any transaction failure.
      const contextError = hostIdl.errors.find(({ name }) => name === "InvalidKmsContext");
      expect(contextError).toBeDefined();
      expect(customProgramErrorCode(rejection)).toBe(contextError!.code);
    },
    SCENARIO_TIMEOUT_MS,
  );
});
