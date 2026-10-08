// Live token consume arc: wrap -> burn -> publish -> certified decrypt -> redeem -> disclose.
// Also checks that a certificate whose context id was changed is refused: with the live context
// account, by the host's InvalidKmsContext before signature verification; with the account of the
// context it names, which does not exist, by AccountNotInitialized.

import { describe, expect, test } from "bun:test";

import { getAddressEncoder, type Address } from "@solana/kit";
import { ZAMA_HOST_ERROR__INVALID_KMS_CONTEXT } from "@fhevm/solana-zama-host";

import { certifiedPublicDecrypt, currentHandle, publicDecryptValues } from "../../src/solana/fhe-vertical";
import {
  createConfidentialMint,
  createSplMint,
  initializeConfidentialTokenAccount,
  mintSplTo,
  wrapUnderlying,
} from "../../src/solana/provision";
import {
  certificateKmsContext,
  confidentialBurn,
  confidentialBurnTarget,
  discloseCertifiedHandle,
  redeemBurnedAmount,
  sealBurnedAmountHandle,
  sealTotalSupplyHandle,
  totalSupplyStore,
} from "../../src/solana/token-vertical";
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from "@demo-dapp/vault/index.js";
import { vaultModule } from "../../src/solana/lazy-modules";
import { ANCHOR_ACCOUNT_NOT_INITIALIZED, expectProgramError } from "../../src/solana/program-error";
import { timed } from "../../src/utils/timing";
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

describe("solana confidential-token consume vertical", () => {
  test(
    "wrap 1000 -> burn attested 7 -> seal -> public-decrypt == 7, batch with supply == [7, 993] -> redeem releases 7 (leaf 1 of 3) -> disclose; a context-tampered certificate is refused (InvalidKmsContext, AccountNotInitialized)",
    async () => {
      const { env, stack, context, wallets, wallet, config, walletHex } = await verticalSetup();

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
      const submission = await timed("encrypt + input proof (burn amount)", () =>
        submitUint64InputProof({
          rpcUrl: env.rpcUrl,
          chainId: config.chainId,
          relayerUrl: config.relayerUrl,
          aclProgramAddress: env.aclProgram,
          contractAddress,
          userAddress: walletHex,
          value: BURN_AMOUNT,
        }),
      );
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

      // The burn already sealed the handle public. Sealing it again through the token wrapper (it
      // signs the Host CPI as the Store authority) exercises the wrapper's make-public path live.
      await sealBurnedAmountHandle(context, { owner: wallet.signer, mint, handle: burnedHandle });

      const { cleartext, certificate } = await timed("certified public decrypt (KMS)", () =>
        certifiedPublicDecrypt(config, {
          encryptedStore: target.burnedAmountStore,
          handle: burnedHandle,
        }),
      );
      expect(cleartext).toBe(BURN_AMOUNT);

      // The burn also wrote the total supply, in the mint's own store. Sealed public, it is
      // decrypted together with the burned amount from one certificate: the KMS returns one word
      // per handle in request order, and each handle is proven against its own store.
      const supplyStore = await totalSupplyStore(mint);
      const supplyHandle = await currentHandle(context, supplyStore, new TextEncoder().encode("total_supply____________________"));
      await stack.waitForSnsCommit(hex(supplyHandle));
      await sealTotalSupplyHandle(context, { authority: wallet.signer, mint, handle: supplyHandle });
      const batch = await timed("batch public decrypt of two stores (KMS)", () =>
        publicDecryptValues(config, [
          { encryptedStore: target.burnedAmountStore, handle: burnedHandle },
          { encryptedStore: supplyStore, handle: supplyHandle },
        ]),
      );
      expect(batch).toEqual([BURN_AMOUNT, WRAP_AMOUNT - BURN_AMOUNT]);

      // Redeem: the host verifier CPI checks the KMS certificate against the live context it
      // names, the token program requires the burned handle pinned in PendingBurn, the PendingBurn
      // closes, and the certified amount of underlying releases to the owner.
      const balanceBefore = BigInt(
        (await context.rpc.getTokenAccountBalance(ownerUnderlying).send()).value.amount,
      );
      await timed("redeem with certificate (host verifier CPI)", () =>
        redeemBurnedAmount(context, { owner: wallet.signer, mint, underlyingMint, certificate }),
      );
      const balanceAfter = BigInt(
        (await context.rpc.getTokenAccountBalance(ownerUnderlying).send()).value.amount,
      );
      expect(balanceAfter - balanceBefore).toBe(BURN_AMOUNT);

      // Disclose: same verifier CPI, then the handle/cleartext event. Idempotent by design.
      await discloseCertifiedHandle(context, { payer: wallet.signer, certificate });

      // Keep the v2 KMS routing intact; change only the committed context id (bytes 1..33).
      const wrongContextExtraData = hexToBytes(certificate.extraData);
      expect(wrongContextExtraData.length).toBe(65);
      expect(wrongContextExtraData[0]).toBe(2);
      wrongContextExtraData[32] = wrongContextExtraData[32]! ^ 1;
      const wrongContextCertificate = { ...certificate, extraData: hex(wrongContextExtraData) };
      // The attack: the live context account beside a certificate that names another context. The
      // host must reject the mismatch before checking the certificate signature.
      const vault = await vaultModule();
      const liveContextInstruction = await vault.buildDiscloseSecpInstruction(
        { kmsContext: await certificateKmsContext(certificate) },
        wrongContextCertificate,
      );
      await expectProgramError(
        "SECURITY: the live context account with a certificate naming another context",
        ZAMA_HOST_ERROR__INVALID_KMS_CONTEXT,
        () => context.sendTransaction(wallet.signer, [liveContextInstruction]),
      );
      // The account of the context the certificate names does not exist, so the token program
      // refuses it before any host CPI.
      await expectProgramError(
        "SECURITY: the account of a context that was never defined",
        ANCHOR_ACCOUNT_NOT_INITIALIZED,
        () => discloseCertifiedHandle(context, { payer: wallet.signer, certificate: wrongContextCertificate }),
      );
      await wallets.sweep();
    },
    SCENARIO_TIMEOUT_MS,
  );
});
