// The Solana host's leg of the `kms-context-switch` profile. The EVM ProtocolConfig drives every
// KMS context transition, and zama-host follows it through its admin, as a deployment operator
// does: `define_kms_context` after each EVM activation, `destroy_kms_context` after the EVM destroy.
// The leg checks Solana at the two transitions that change what a certificate proves: the first
// switch (values written before it still decrypt, and the new context certifies them) and the
// destroy (the destroyed context's certificate is refused, and the current context still serves).
import { address } from '@solana/kit';

import { TOTAL_SUPPLY_KEY } from '@fhevm/confidential-token';
import {
  ZAMA_HOST_ERROR__INVALID_KMS_CONTEXT,
  ZAMA_HOST_ERROR__NON_INCREASING_KMS_CONTEXT_ID,
  ZAMA_HOST_PROGRAM_ADDRESS,
  getDestroyKmsContextInstructionAsync,
} from '@fhevm/solana-zama-host';

import { defineKmsContextInstruction } from '../../../../solana/deploy/src/bootstrap';
import { SOLANA_ACL_PROGRAM } from '../layout';
import type { State } from '../types';
import { bytes32HexFromId, readGatewayKmsSignersForContext } from './addresses';
import { assertKmsThresholdsMatchEvmHost, solanaDeployerKeypairPath } from './deploy';
import { LOCAL_SOLANA_ENDPOINTS } from './endpoints';
import { type FheVerticalConfig, certifiedPublicDecrypt, currentHandle, userDecryptExpect } from './fhe-vertical';
import { sdkVerifyModule } from './lazy-modules';
import { expectProgramError } from './program-error';
import {
  createConfidentialMint,
  createProvisioningContext,
  createSplMint,
  generateSolanaKeypair,
  initializeConfidentialTokenAccount,
  loadKeypairSigner,
  mintSplTo,
  readHostChainId,
  readTokenBalanceStore,
  wrapUnderlying,
} from './provision';
import { waitForSnsCommit } from './sns';
import { readDecryptTrustInputs } from './target';
import { discloseCertifiedHandle, sealTotalSupplyHandle, totalSupplyStore } from './token-vertical';

const WRAP_AMOUNT = 1000n;
const HOLDER_SOL = 5;

const hex = (bytes: Uint8Array): `0x${string}` => `0x${Buffer.from(bytes).toString('hex')}`;
/** A KMS context id as zama-host stores it: the EVM uint256, 32 bytes big-endian. */
const contextIdBytes = (contextId: bigint): Uint8Array => Buffer.from(bytes32HexFromId(contextId).slice(2), 'hex');

export type SolanaKmsContextLeg = {
  /** Defines `contextId`, just activated on the EVM host, on zama-host with the same committee. */
  readonly mirrorContext: (contextId: bigint) => Promise<void>;
  /** Step 1: values written before the switch decrypt under `contextId`, which zama-host accepts. */
  readonly checkSwitch: (contextId: bigint) => Promise<void>;
  /**
   * Step 3: destroys `destroyedContextId` on zama-host. Its certificate from before the switch is
   * then refused, and a certificate of `currentContextId` is still accepted.
   */
  readonly destroyContext: (destroyedContextId: bigint, currentContextId: bigint) => Promise<void>;
};

/**
 * Writes the leg's values at the baseline context and certifies one of them under it. A wrap
 * writes two values without an input proof: the holder's balance, which the holder can
 * user-decrypt, and the mint's total supply, which the mint authority makes public.
 */
export const prepareSolanaKmsContextLeg = async (
  state: State,
  baselineContextId: bigint,
): Promise<SolanaKmsContextLeg> => {
  const endpoints = LOCAL_SOLANA_ENDPOINTS;
  const context = createProvisioningContext(endpoints.validatorRpc, endpoints.validatorWs);
  const admin = await loadKeypairSigner(solanaDeployerKeypairPath());
  const holder = await generateSolanaKeypair();
  await context.fundSol(holder.signer.address, HOLDER_SOL);

  const underlyingMint = await createSplMint(context, { authority: holder.signer, decimals: 9 });
  await mintSplTo(context, {
    authority: holder.signer,
    mint: underlyingMint,
    recipient: holder.signer.address,
    baseUnits: WRAP_AMOUNT,
  });
  const mint = await createConfidentialMint(context, { authority: holder.signer, underlyingMint });
  await initializeConfidentialTokenAccount(context, { payer: holder.signer, owner: holder.signer.address, mint });
  await wrapUnderlying(context, { owner: holder.signer, mint, underlyingMint, amount: WRAP_AMOUNT });

  const supplyStore = await totalSupplyStore(mint);
  const supplyHandle = await currentHandle(context, supplyStore, TOTAL_SUPPLY_KEY);
  await waitForSnsCommit(hex(supplyHandle));
  await sealTotalSupplyHandle(context, { authority: holder.signer, mint, handle: supplyHandle });
  const balance = await readTokenBalanceStore(context, { mint, owner: holder.signer.address });
  await waitForSnsCommit(balance.currentHandle);

  const chainId = await readHostChainId(context);
  const decryptConfig = async (publicDecryptContextId: bigint): Promise<FheVerticalConfig> => {
    const trust = await readDecryptTrustInputs({ gatewayRpcUrl: endpoints.gatewayRpc, hostRpcUrl: endpoints.hostRpc });
    return {
      rpcUrl: endpoints.validatorRpc,
      relayerUrl: endpoints.relayer,
      chainId,
      publicDecryptContextId: bytes32HexFromId(publicDecryptContextId),
      userDecryptContextId: trust.kmsContextId.toString(),
      verifyingProgramId: SOLANA_ACL_PROGRAM,
      kmsSigners: trust.kmsSigners,
      kmsEpochId: bytes32HexFromId(trust.kmsEpochId),
      fheParameter: state.scenario.kms.fheParams.toLowerCase(),
      gatewayChainId: trust.gatewayChainId.toString(),
      gatewayDecryptionContract: trust.decryptionContract,
    };
  };

  /** Certifies the total supply under `contextId` and checks the certificate names that context. */
  const certifySupply = async (contextId: bigint) => {
    const { cleartext, certificate } = await certifiedPublicDecrypt(await decryptConfig(contextId), {
      encryptedStore: supplyStore,
      handle: supplyHandle,
    });
    if (cleartext !== WRAP_AMOUNT) throw new Error(`Solana total supply decrypted to ${cleartext}, expected ${WRAP_AMOUNT}`);
    const { solanaPublicDecryptContextId } = await sdkVerifyModule();
    const named = BigInt(hex(solanaPublicDecryptContextId(certificate)));
    if (named !== contextId) throw new Error(`the certificate names context ${named}, expected ${contextId}`);
    return certificate;
  };

  const gatewaySigners = (contextId: bigint) =>
    readGatewayKmsSignersForContext({ gatewayRpcUrl: endpoints.gatewayRpc }, contextId);
  const defineContext = async (contextId: bigint, signers: readonly Uint8Array[]) =>
    context.sendTransaction(admin, [
      await defineKmsContextInstruction({
        admin,
        contextId: contextIdBytes(contextId),
        signers,
        kmsCorruptionThreshold: state.scenario.kms.threshold,
      }),
    ]);

  const baselineCertificate = await certifySupply(baselineContextId);
  await discloseCertifiedHandle(context, { payer: holder.signer, certificate: baselineCertificate });
  console.log(`[kms-context-switch] solana: baseline context ${baselineContextId} certified and disclosed the total supply`);

  return {
    mirrorContext: async (contextId) => {
      const signers = await gatewaySigners(contextId);
      await defineContext(contextId, signers);
      await assertKmsThresholdsMatchEvmHost(ZAMA_HOST_PROGRAM_ADDRESS, contextIdBytes(contextId));
      console.log(`[kms-context-switch] solana: defined context ${contextId} (${signers.length} signers)`);
    },

    checkSwitch: async (contextId) => {
      const certificate = await certifySupply(contextId);
      await discloseCertifiedHandle(context, { payer: holder.signer, certificate });
      await userDecryptExpect(await decryptConfig(contextId), {
        encryptedStore: address(balance.encryptedStore),
        handle: Buffer.from(balance.currentHandle.slice(2), 'hex'),
        secretKey: hex(holder.bytes.subarray(0, 32)),
        expected: WRAP_AMOUNT,
      });
      // An id that was never defined, so the refusal comes from the ordering rule and not from
      // the existing context account.
      await expectProgramError(
        'define a KMS context id below the current one',
        ZAMA_HOST_ERROR__NON_INCREASING_KMS_CONTEXT_ID,
        async () => defineContext(baselineContextId - 1n, await gatewaySigners(contextId)),
      );
      console.log(
        `[kms-context-switch] solana: context ${contextId} certified the pre-switch supply, the holder ` +
          'user-decrypted the pre-switch balance, and a lower context id was refused',
      );
    },

    destroyContext: async (destroyedContextId, currentContextId) => {
      await context.sendTransaction(admin, [
        await getDestroyKmsContextInstructionAsync({ admin, contextId: contextIdBytes(destroyedContextId) }),
      ]);
      await expectProgramError(
        `disclose a certificate of destroyed context ${destroyedContextId}`,
        ZAMA_HOST_ERROR__INVALID_KMS_CONTEXT,
        () => discloseCertifiedHandle(context, { payer: holder.signer, certificate: baselineCertificate }),
      );
      await discloseCertifiedHandle(context, {
        payer: holder.signer,
        certificate: await certifySupply(currentContextId),
      });
      console.log(
        `[kms-context-switch] solana: destroyed context ${destroyedContextId}; its certificate was refused ` +
          `and context ${currentContextId} still certifies`,
      );
    },
  };
};
