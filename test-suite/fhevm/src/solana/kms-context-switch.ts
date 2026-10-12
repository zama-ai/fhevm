// The Solana host's leg of the `kms-context-switch` profile. The EVM ProtocolConfig drives every
// KMS context transition, and zama-host follows it through its admin, as an operator mirrors a
// replica (KMS_CONTEXT_SWITCH_RUNBOOK.md §5): `define_kms_context` with the new pair after each EVM
// context activation, `define_kms_epoch` after each EVM epoch activation, and `destroy_kms_context`
// after the EVM destroy. After each `define_kms_context`, zama-host's signers and thresholds must
// match the EVM context's. The leg checks Solana decrypts at the transitions that change what a
// certificate proves: the first switch and the node swap (values written at the baseline still
// decrypt, and the new context certifies them), and the destroy (the destroyed context's
// certificate is refused, and the current context still serves). Each decrypt that must see a
// switch runs in a fresh process, so the SDK's cached zama-host reads never hide it. The two permits
// signed at the baseline stay in this process: the SDK verifies an answer against the signers of
// the context its permit names, which a switch does not change.
//
// Every Solana decrypt goes through relayer-http, which lists every party's proxy, spares included
// (generate/config.ts). After the first switch and after the node swap, the leg reads relayer-http's
// log of those decrypts: the party outside the active context must be refused at its first attempt,
// and the aggregation must not wait for its deadline.
import { bytesToHex, hexToBytes } from '@fhevm/sdk/base';
import { TOTAL_SUPPLY_KEY } from '@fhevm/confidential-token';
import { address, getAddressEncoder } from '@solana/kit';
import {
  ZAMA_HOST_ERROR__INVALID_KMS_CONTEXT,
  ZAMA_HOST_ERROR__NON_INCREASING_KMS_CONTEXT_ID,
  ZAMA_HOST_PROGRAM_ADDRESS,
  getDefineKmsEpochInstructionAsync,
  getDestroyKmsContextInstructionAsync,
} from '@fhevm/solana-zama-host';

import { defineKmsContextInstruction } from '../../../../solana/deploy/src/bootstrap';
import type { ContextAndEpoch } from '../commands/kms-context-switch';
import { relayerHttpNodeName } from '../kms-party';
import { SOLANA_ACL_PROGRAM } from '../layout';
import type { State } from '../types';
import { until } from '../utils/until';
import { readEvmKmsSignersForContext, uint256Bytes } from './addresses';
import { aggregationOutcome, nodeOutcomes, relayerHttpRequests } from './decryption-trace';
import {
  assertActiveKmsPairMatchesEvmHost,
  assertKmsContextMatchesEvmHost,
  bootstrapThresholdsForState,
  solanaDeployerKeypairPath,
} from './deploy';
import { LOCAL_SOLANA_ENDPOINTS } from './endpoints';
import { type FheVerticalConfig, currentHandle } from './fhe-vertical';
import { certifiedPublicDecryptInFreshProcess, userDecryptExpectInFreshProcess } from './fresh-decrypt';
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
import {
  buildSolanaUserDecryptRequest,
  decryptionChain,
  decryptionClient,
  solanaPermitWalletFromSecretKey,
  submitUserDecryptOnce,
} from './relayer-http-request';
import { waitForSnsCommit } from './sns';
import { sdkVerifyModule } from './lazy-modules';
import { discloseCertifiedHandle, sealTotalSupplyHandle, totalSupplyStore } from './token-vertical';

const WRAP_AMOUNT = 1000n;
const HOLDER_SOL = 5;

export type SolanaKmsContextLeg = {
  /**
   * Defines the context of `pair`, just activated on the EVM host, on zama-host with `pair`'s epoch
   * and the EVM context's signers. Then checks that zama-host holds those signers in the same order,
   * the EVM thresholds, and the EVM host's active pair.
   */
  readonly mirrorContext: (pair: ContextAndEpoch) => Promise<void>;
  /**
   * Makes the epoch of `pair`, just activated on the EVM host, the active epoch on zama-host, then
   * checks that zama-host's active pair is the EVM host's.
   */
  readonly mirrorEpoch: (pair: ContextAndEpoch) => Promise<void>;
  /**
   * Steps 1 and 5: values written at the baseline decrypt under `contextId`. zama-host accepts the
   * certificate its committee signs, and the holder user-decrypts the balance. relayer-http also
   * asked `outsideParties`, which are not in `contextId`: each was refused at its first attempt, and
   * no aggregation waited for its deadline.
   */
  readonly checkDecrypts: (contextId: bigint, outsideParties: readonly number[]) => Promise<void>;
  /**
   * Step 1: a permit signed at the baseline, before the switch, still decrypts. The baseline
   * context is not destroyed yet, so it still serves, as on EVM.
   */
  readonly checkBaselinePermit: () => Promise<void>;
  /**
   * Step 3: destroys the baseline context on zama-host. Its certificate from before the switch is
   * then refused, and a certificate of `currentContextId` is still accepted. A new request under a
   * baseline permit is refused with 410 kms_context_destroyed.
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
  await waitForSnsCommit(bytesToHex(supplyHandle));
  await sealTotalSupplyHandle(context, { authority: holder.signer, mint, handle: supplyHandle });
  const balance = await readTokenBalanceStore(context, { mint, owner: holder.signer.address });
  await waitForSnsCommit(balance.currentHandle);

  const chainId = await readHostChainId(context);
  const decryptConfig: FheVerticalConfig = {
    rpcUrl: endpoints.validatorRpc,
    relayerUrl: endpoints.relayer,
    decryptionRelayerUrl: endpoints.decryptionRelayer,
    chainId,
    verifyingProgramId: SOLANA_ACL_PROGRAM,
    fheParameter: state.scenario.kms.fheParams.toLowerCase(),
  };

  // Two permits the holder signs at the baseline: one decrypts after the switch, the other is first
  // sent after the baseline context is destroyed. Each must name the baseline context. They are
  // valid for a day, longer than the profile runs.
  const baselineClient = decryptionClient(decryptConfig);
  const holderWallet = solanaPermitWalletFromSecretKey(holder.bytes.subarray(0, 32));
  const baselinePermit = await baselineClient.signPermit({ wallet: holderWallet, durationSeconds: 86_400n });
  const destroyProbePermit = await baselineClient.signPermit({ wallet: holderWallet, durationSeconds: 86_400n });
  for (const permit of [baselinePermit, destroyProbePermit]) {
    const named = bytesToHex(permit.signedPermit.fields.kmsRouting.kmsContextId);
    if (named !== bytesToHex(uint256Bytes(baseline.contextId))) {
      throw new Error(`a permit signed at the baseline names KMS context ${named}, expected ${baseline.contextId}`);
    }
  }
  const balanceEntry = {
    handle: hexToBytes(balance.currentHandle),
    encryptedStore: new Uint8Array(getAddressEncoder().encode(address(balance.encryptedStore))),
    ownerAddress: new Uint8Array(getAddressEncoder().encode(holder.signer.address)),
  };

  /**
   * relayer-http's log of the decrypts of the leg's values made after `before` (the request ids it
   * had logged until then): in each answered decrypt, each party in `outsideParties` was refused at
   * its first attempt, and the aggregation did not reach its deadline. One attempt, because a core
   * with no role in the request's context answers NotFound, which the connector's worker turns into
   * 422 unprocessable, a code relayer-http does not retry. The SDK authenticates each
   * share against the signers of its permit's context and needs 2t+1 of them
   * (userDecrypt/response.ts), so a share from outside that context could never count.
   */
  const assertOutsidePartiesRefusedAtOnce = async (before: ReadonlySet<string>, outsideParties: readonly number[]) => {
    const handles = [bytesToHex(supplyHandle), balance.currentHandle.toLowerCase()];
    const answered = await until(
      async () => {
        const requests = [...(await relayerHttpRequests()).entries()]
          .filter(([requestId]) => !before.has(requestId))
          .map(([, lines]) => lines)
          .filter((lines) => lines.some((line) => handles.some((handle) => String(line.fields.handles).includes(handle))))
          .filter((lines) => aggregationOutcome(lines)?.message === 'aggregation succeeded');
        const flows = new Set(requests.map((lines) => String(lines[0]?.fields.flow)));
        return flows.has('user_decrypt') && flows.has('public_decrypt') ? requests : undefined;
      },
      { description: "relayer-http's answered user and public decrypts of the leg's values", timeoutMs: 10_000, intervalMs: 500 },
    );
    for (const lines of answered) {
      const started = lines.find((line) => line.message === 'aggregation started');
      const outcome = aggregationOutcome(lines)!;
      if (started === undefined) throw new Error('relayer-http logged no aggregation start for a decrypt');
      const timeoutMs = Number(started.fields.timeout_ms);
      if (outcome.fields.deadline_hit !== false) {
        throw new Error(`a ${String(started.fields.flow)} decrypt waited for its ${timeoutMs} ms deadline`);
      }
      for (const party of outsideParties) {
        const node = nodeOutcomes(lines).find((outcomeOfNode) => outcomeOfNode.node === relayerHttpNodeName(party));
        if (node?.message !== 'call failed' || node.attempts !== 1) {
          throw new Error(
            `party ${party}, outside the context, ended its ${String(started.fields.flow)} call with ` +
              `${node === undefined ? 'no line' : `"${node.message}" after ${node.attempts} attempt(s)`}, expected one refused attempt`,
          );
        }
        console.log(
          `[kms-context-switch] solana: ${String(started.fields.flow)}: party ${party} refused (${node.code}) in ` +
            `${node.elapsedMs} ms; the aggregation took ${String(outcome.fields.elapsed_ms)} ms of its ${timeoutMs} ms`,
        );
      }
    }
  };

  /**
   * Certifies the total supply. The SDK names zama-host's active context in the certificate, which
   * must be `contextId`.
   */
  const certifySupply = async (contextId: bigint) => {
    const { cleartext, certificate } = await certifiedPublicDecryptInFreshProcess(decryptConfig, {
      encryptedStore: supplyStore,
      handle: supplyHandle,
    });
    if (cleartext !== WRAP_AMOUNT) throw new Error(`Solana total supply decrypted to ${cleartext}, expected ${WRAP_AMOUNT}`);
    const { solanaPublicDecryptContextId } = await sdkVerifyModule();
    const namedContext = bytesToHex(solanaPublicDecryptContextId(certificate));
    const expectedContext = bytesToHex(uint256Bytes(contextId));
    if (namedContext !== expectedContext) {
      throw new Error(`the certificate names KMS context ${namedContext}, expected ${expectedContext}`);
    }
    return certificate;
  };

  const evmSigners = (contextId: bigint) => readEvmKmsSignersForContext({ hostRpcUrl: endpoints.hostRpc, contextId });
  const adminClient = await context.client(admin);
  const defineContext = async ({ contextId, epochId }: ContextAndEpoch, signers: readonly Uint8Array[]) =>
    adminClient.sendTransaction([
      await defineKmsContextInstruction({
        admin,
        contextId: uint256Bytes(contextId),
        epochId: uint256Bytes(epochId),
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
      await assertKmsContextMatchesEvmHost(ZAMA_HOST_PROGRAM_ADDRESS, uint256Bytes(pair.contextId));
      await assertActiveKmsPairMatchesEvmHost(ZAMA_HOST_PROGRAM_ADDRESS);
      console.log(
        `[kms-context-switch] solana: defined context ${pair.contextId} with epoch ${pair.epochId}, the EVM ` +
          `active pair; its ${signers.length} signers, in order, and its thresholds match the EVM context`,
      );
    },

    mirrorEpoch: async ({ contextId, epochId }) => {
      await adminClient.sendTransaction([
        await getDefineKmsEpochInstructionAsync({ admin, contextId: uint256Bytes(contextId), epochId: uint256Bytes(epochId) }),
      ]);
      await assertActiveKmsPairMatchesEvmHost(ZAMA_HOST_PROGRAM_ADDRESS);
      console.log(
        `[kms-context-switch] solana: epoch ${epochId} is active under context ${contextId}, the EVM active pair`,
      );
    },

    checkDecrypts: async (contextId, outsideParties) => {
      const before = new Set((await relayerHttpRequests()).keys());
      const certificate = await certifySupply(contextId);
      await discloseCertifiedHandle(context, { payer: holder.signer, certificate });
      await userDecryptExpectInFreshProcess(decryptConfig, {
        encryptedStore: balance.encryptedStore,
        handle: Buffer.from(balance.currentHandle.slice(2), 'hex'),
        secretKey: bytesToHex(holder.bytes.subarray(0, 32)),
        expected: WRAP_AMOUNT,
      });
      await assertOutsidePartiesRefusedAtOnce(before, outsideParties);
      console.log(
        `[kms-context-switch] solana: context ${contextId} certified the baseline supply and the holder ` +
          'user-decrypted the baseline balance',
      );
    },

    checkBaselinePermit: async () => {
      const [value] = await baselineClient.decryptValues({ session: baselinePermit, entries: [balanceEntry] });
      if (BigInt(value!.value as bigint) !== WRAP_AMOUNT) {
        throw new Error(`a baseline permit decrypted the balance to ${String(value!.value)}, expected ${WRAP_AMOUNT}`);
      }
      console.log(
        `[kms-context-switch] solana: a permit signed under baseline context ${baseline.contextId} still ` +
          'decrypts the balance after the switch',
      );
    },

    destroyBaseline: async (currentContextId) => {
      await adminClient.sendTransaction([
        await getDestroyKmsContextInstructionAsync({ admin, contextId: uint256Bytes(baseline.contextId) }),
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
      // The probe permit was never sent, so its request has a decryption id no Connector has seen.
      // A body answered before the destroy would come back with its stored 200: each Connector serves
      // the response row it holds for a decryption id. The body is posted as built because the SDK's
      // client reads the permit's context before sending and refuses a destroyed one.
      const probe = await submitUserDecryptOnce(
        decryptionChain(decryptConfig),
        buildSolanaUserDecryptRequest({ signedPermit: destroyProbePermit.signedPermit, entries: [balanceEntry] }),
      );
      if (probe.status !== 410 || probe.code !== 'kms_context_destroyed') {
        throw new Error(
          `a new request under destroyed context ${baseline.contextId} was answered ${probe.status} ${probe.code ?? ''}, ` +
            'expected 410 kms_context_destroyed',
        );
      }
      console.log(
        `[kms-context-switch] solana: destroyed context ${baseline.contextId}; its certificate was refused, ` +
          `a new request under it was refused with 410 kms_context_destroyed, and context ${currentContextId} still certifies`,
      );
    },
  };
};
