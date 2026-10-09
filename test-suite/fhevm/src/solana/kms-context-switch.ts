// The Solana host's leg of the `kms-context-switch` profile. The EVM ProtocolConfig drives every
// KMS context transition, and zama-host follows it through its admin, as an operator mirrors a
// replica (KMS_CONTEXT_SWITCH_RUNBOOK.md §5): `define_kms_context` with the new pair after each EVM
// context activation, `define_kms_epoch` after each EVM epoch activation, and `destroy_kms_context`
// after the EVM destroy. After each `define_kms_context`, zama-host's signers and thresholds must
// match the EVM context's. The leg checks Solana decrypts at the transitions that change what a
// certificate proves: the first switch and the node swap (values written at the baseline still
// decrypt, and the new context certifies them), and the destroy (the destroyed context's
// certificate is refused, and the current context still serves).
import { address } from '@solana/kit';

import { hexToBytes } from '@fhevm/sdk/base';
import { TOTAL_SUPPLY_KEY } from '@fhevm/confidential-token';
import {
  ZAMA_HOST_ERROR__INVALID_KMS_CONTEXT,
  ZAMA_HOST_ERROR__NON_INCREASING_KMS_CONTEXT_ID,
  ZAMA_HOST_PROGRAM_ADDRESS,
  getDefineKmsEpochInstructionAsync,
  getDestroyKmsContextInstructionAsync,
} from '@fhevm/solana-zama-host';

import { defineKmsContextInstruction } from '../../../../solana/deploy/src/bootstrap';
import type { ContextAndEpoch } from '../commands/kms-context-switch';
import { SOLANA_ACL_PROGRAM } from '../layout';
import type { State } from '../types';
import { bytes32HexFromId, readEvmKmsSignersForContext } from './addresses';
import { assertKmsContextMatchesEvmHost, bootstrapThresholdsForState, solanaDeployerKeypairPath } from './deploy';
import { LOCAL_SOLANA_ENDPOINTS } from './endpoints';
import { type FheVerticalConfig, certifiedPublicDecrypt, currentHandle, userDecryptExpect } from './fhe-vertical';
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
import { sdkVerifyModule } from './lazy-modules';
import { readDecryptTrustInputs } from './target';
import { discloseCertifiedHandle, sealTotalSupplyHandle, totalSupplyStore } from './token-vertical';

const WRAP_AMOUNT = 1000n;
const HOLDER_SOL = 5;

const hex = (bytes: Uint8Array): `0x${string}` => `0x${Buffer.from(bytes).toString('hex')}`;
/** A KMS context or epoch id as zama-host stores it: the EVM uint256, 32 bytes big-endian. */
const idBytes = (id: bigint): Uint8Array => hexToBytes(bytes32HexFromId(id));

export type SolanaKmsContextLeg = {
  /**
   * Defines the context of `pair`, just activated on the EVM host, on zama-host with `pair`'s epoch
   * and the EVM context's signers. Then checks that zama-host holds those signers in the same order
   * and the EVM thresholds.
   */
  readonly mirrorContext: (pair: ContextAndEpoch) => Promise<void>;
  /** Makes the epoch of `pair`, just activated on the EVM host, the active epoch on zama-host. */
  readonly mirrorEpoch: (pair: ContextAndEpoch) => Promise<void>;
  /**
   * Steps 1 and 5: values written at the baseline decrypt under `contextId`. zama-host accepts the
   * certificate its committee signs, and the holder user-decrypts the balance.
   */
  readonly checkDecrypts: (contextId: bigint) => Promise<void>;
  /**
   * Step 3: destroys the baseline context on zama-host. Its certificate from before the switch is
   * then refused, and a certificate of `currentContextId` is still accepted.
   */
  readonly destroyBaseline: (currentContextId: bigint) => Promise<void>;
};

/**
 * Writes the leg's values at the baseline context and certifies one of them under it. A wrap
 * writes two values without an input proof: the holder's balance, which the holder can
 * user-decrypt, and the mint's total supply, which the mint authority makes public.
 */
export const prepareSolanaKmsContextLeg = async (
  state: State,
  baseline: ContextAndEpoch,
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
  const decryptConfig = async (): Promise<FheVerticalConfig> => {
    const trust = await readDecryptTrustInputs({ gatewayRpcUrl: endpoints.gatewayRpc, hostRpcUrl: endpoints.hostRpc });
    return {
      rpcUrl: endpoints.validatorRpc,
      relayerUrl: endpoints.relayer,
      chainId,
      userDecryptContextId: trust.kmsContextId.toString(),
      verifyingProgramId: SOLANA_ACL_PROGRAM,
      kmsSigners: trust.kmsSigners,
      kmsEpochId: bytes32HexFromId(trust.kmsEpochId),
      fheParameter: state.scenario.kms.fheParams.toLowerCase(),
      gatewayChainId: trust.gatewayChainId.toString(),
      gatewayDecryptionContract: trust.decryptionContract,
    };
  };

  /**
   * Certifies the total supply. The SDK names zama-host's active context in the certificate, which
   * must be `contextId`.
   */
  const certifySupply = async (contextId: bigint) => {
    const { cleartext, certificate } = await certifiedPublicDecrypt(await decryptConfig(), {
      encryptedStore: supplyStore,
      handle: supplyHandle,
    });
    if (cleartext !== WRAP_AMOUNT) throw new Error(`Solana total supply decrypted to ${cleartext}, expected ${WRAP_AMOUNT}`);
    const { solanaPublicDecryptContextId } = await sdkVerifyModule();
    const namedContext = hex(solanaPublicDecryptContextId(certificate));
    if (namedContext !== bytes32HexFromId(contextId)) {
      throw new Error(`the certificate names KMS context ${namedContext}, expected ${bytes32HexFromId(contextId)}`);
    }
    return certificate;
  };

  const evmSigners = (contextId: bigint) => readEvmKmsSignersForContext({ hostRpcUrl: endpoints.hostRpc, contextId });
  const adminClient = await context.client(admin);
  const defineContext = async ({ contextId, epochId }: ContextAndEpoch, signers: readonly Uint8Array[]) =>
    adminClient.sendTransaction([
      await defineKmsContextInstruction({
        admin,
        contextId: idBytes(contextId),
        epochId: idBytes(epochId),
        signers,
        kmsCorruptionThreshold: bootstrapThresholdsForState(state).kmsCorruptionThreshold,
      }),
    ]);

  const baselineCertificate = await certifySupply(baseline.contextId);
  await discloseCertifiedHandle(context, { payer: holder.signer, certificate: baselineCertificate });
  // A context id that was never defined, with a higher epoch, so the refusal comes from the context
  // ordering rule and not from the existing context account or the epoch rule.
  await expectProgramError(
    'define a KMS context id below the current one',
    ZAMA_HOST_ERROR__NON_INCREASING_KMS_CONTEXT_ID,
    async () =>
      defineContext(
        { contextId: baseline.contextId - 1n, epochId: baseline.epochId + 1n },
        await evmSigners(baseline.contextId),
      ),
  );
  console.log(
    `[kms-context-switch] solana: baseline context ${baseline.contextId} certified and disclosed the total supply, ` +
      'and a lower context id was refused',
  );

  return {
    mirrorContext: async (pair) => {
      const signers = await evmSigners(pair.contextId);
      await defineContext(pair, signers);
      await assertKmsContextMatchesEvmHost(ZAMA_HOST_PROGRAM_ADDRESS, idBytes(pair.contextId));
      console.log(
        `[kms-context-switch] solana: defined context ${pair.contextId} with epoch ${pair.epochId}; its ` +
          `${signers.length} signers, in order, and its thresholds match the EVM context`,
      );
    },

    mirrorEpoch: async ({ contextId, epochId }) => {
      await adminClient.sendTransaction([
        await getDefineKmsEpochInstructionAsync({ admin, contextId: idBytes(contextId), epochId: idBytes(epochId) }),
      ]);
      console.log(`[kms-context-switch] solana: epoch ${epochId} is active under context ${contextId}`);
    },

    checkDecrypts: async (contextId) => {
      const certificate = await certifySupply(contextId);
      await discloseCertifiedHandle(context, { payer: holder.signer, certificate });
      await userDecryptExpect(await decryptConfig(), {
        encryptedStore: address(balance.encryptedStore),
        handle: Buffer.from(balance.currentHandle.slice(2), 'hex'),
        secretKey: hex(holder.bytes.subarray(0, 32)),
        expected: WRAP_AMOUNT,
      });
      console.log(
        `[kms-context-switch] solana: context ${contextId} certified the baseline supply and the holder ` +
          'user-decrypted the baseline balance',
      );
    },

    destroyBaseline: async (currentContextId) => {
      await adminClient.sendTransaction([
        await getDestroyKmsContextInstructionAsync({ admin, contextId: idBytes(baseline.contextId) }),
      ]);
      await expectProgramError(
        `disclose a certificate of destroyed context ${baseline.contextId}`,
        ZAMA_HOST_ERROR__INVALID_KMS_CONTEXT,
        () => discloseCertifiedHandle(context, { payer: holder.signer, certificate: baselineCertificate }),
      );
      await discloseCertifiedHandle(context, {
        payer: holder.signer,
        certificate: await certifySupply(currentContextId),
      });
      console.log(
        `[kms-context-switch] solana: destroyed context ${baseline.contextId}; its certificate was refused ` +
          `and context ${currentContextId} still certifies`,
      );
    },
  };
};
