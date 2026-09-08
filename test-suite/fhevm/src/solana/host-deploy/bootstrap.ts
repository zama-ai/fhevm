// Initializes HostConfig and defines the bring-up KMS context from live gateway values.
// A re-run validates the existing host binding before skipping account initialization. `define_kms_context` is `init` on the context PDA, so a second call would fail closed
// without the skip.
import {
  type Address,
  type Instruction,
  type TransactionSigner,
  fetchEncodedAccount,
  fixDecoderSize,
  getAddressEncoder,
  getArrayDecoder,
  getBooleanDecoder,
  getBytesDecoder,
  getProgramDerivedAddress,
  getStructDecoder,
  getU8Decoder,
} from '@solana/kit';
import { createHash } from 'node:crypto';

import { HOST_CONFIG_DISCRIMINATOR, getHostConfigDecoder } from '../internal/generated/zamaHost/accounts/hostConfig.js';
import {
  getDefineKmsContextInstructionAsync,
  getInitializeHostConfigInstructionAsync,
} from '../internal/generated/zamaHost/instructions/index.js';
import { findHostConfigPda, findKmsContextPda, findRandNoncePda } from '../internal/generated/zamaHost/pdas/index.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '../internal/generated/zamaHost/programAddress.js';
import { getKmsThresholdsDecoder } from '../internal/generated/zamaHost/types/kmsThresholds.js';
import { BRINGUP_KMS_CONTEXT_ID, SOLANA_HOST_CHAIN_ID } from './constants';
import type { GatewayBootstrapInputs } from './gateway';
import type { HostDeployContext } from './send';

const BPF_UPGRADEABLE_LOADER = 'BPFLoaderUpgradeab1e11111111111111111111111' as Address;

/**
 * Derives the on-chain certificate threshold (matching signatures a certificate needs) from the
 * KMS corruption threshold t. A centralized KMS (t=0) signs with one key; a threshold-mode KMS
 * needs 2t+1 matching signatures, and KMS core requires parties == 3t+1.
 */
export const kmsCertificateThreshold = (kmsCorruptionThreshold: number, registeredSignerCount: number): number => {
  if (!Number.isSafeInteger(kmsCorruptionThreshold) || kmsCorruptionThreshold < 0 || kmsCorruptionThreshold > 255) {
    throw new Error('KMS_THRESHOLD must be an unsigned byte');
  }
  const certificateThreshold = 2 * kmsCorruptionThreshold + 1;
  if (certificateThreshold > registeredSignerCount) {
    throw new Error(
      `KMS_THRESHOLD=${kmsCorruptionThreshold} needs 2t+1=${certificateThreshold} certificate ` +
        `signatures but only ${registeredSignerCount} KMS signers are registered on the gateway`,
    );
  }
  return certificateThreshold;
};

const zamaEventAuthorityAddress = async (programAddress: Address) => {
  const [eventAuthority] = await getProgramDerivedAddress({
    programAddress,
    seeds: [new TextEncoder().encode('__event_authority')],
  });
  return eventAuthority;
};

/** BPF upgradeable loader `ProgramData` PDA (`[program_id]` under the loader). */
export const programDataAddressFor = async (programAddress: Address): Promise<Address> => {
  const [programData] = await getProgramDerivedAddress({
    programAddress: BPF_UPGRADEABLE_LOADER,
    seeds: [getAddressEncoder().encode(programAddress)],
  });
  return programData;
};

export type BootstrapZamaHostParams = {
  readonly payer: TransactionSigner;
  readonly gateway: GatewayBootstrapInputs;
  /**
   * Input-attestation n-of-m. The PoC coprocessor emits a single attestation signature, so 1
   * keeps the live flow green while the full registered set is stored (EVM `InputVerifier` parity).
   */
  readonly coprocessorThreshold?: number;
  /** KMS corruption threshold t; 0 is the centralized PoC default. */
  readonly kmsCorruptionThreshold?: number;
  /** Program id to bootstrap. Defaults to the localnet/generated id. */
  readonly programAddress?: Address;
  /** Validate existing bindings without sending initialization transactions. */
  readonly validateOnly?: boolean;
};

// Mirror program input constraints so malformed first-deploy config cannot upload bytecode first.
export const validateBootstrapInputs = (params: BootstrapZamaHostParams): void => {
  for (const [name, signers, maximum] of [
    ['coprocessor', params.gateway.coprocessorSigners, 8],
    ['KMS', params.gateway.kmsSigners, 16],
  ] as const) {
    const addresses = signers.map((signer) => Buffer.from(signer).toString('hex'));
    if (
      signers.length < 1 ||
      signers.length > maximum ||
      signers.some((s) => s.length !== 20 || s.every((b) => b === 0)) ||
      new Set(addresses).size !== signers.length
    ) {
      throw new Error(`${name} signer set must contain 1..${maximum} distinct nonzero 20-byte addresses`);
    }
  }
  const threshold = params.coprocessorThreshold ?? 1;
  if (!Number.isSafeInteger(threshold) || threshold < 1 || threshold > params.gateway.coprocessorSigners.length) {
    throw new Error('coprocessor threshold must be between 1 and signer count');
  }
  if (params.gateway.gatewayChainId < 0n || params.gateway.gatewayChainId >= 1n << 63n) {
    throw new Error('gateway chain id must be an EVM u64 with the chain-type bit clear');
  }
  if (
    [params.gateway.decryptionContract, params.gateway.inputVerificationContract].some(
      (contract) => contract.length !== 20 || contract.every((byte) => byte === 0),
    )
  ) {
    throw new Error('gateway contract addresses must be nonzero 20-byte addresses');
  }
  kmsCertificateThreshold(params.kmsCorruptionThreshold ?? 0, params.gateway.kmsSigners.length);
};

export const bootstrapZamaHost = async (context: HostDeployContext, params: BootstrapZamaHostParams): Promise<void> => {
  validateBootstrapInputs(params);
  const programAddress = params.programAddress ?? ZAMA_HOST_PROGRAM_ADDRESS;
  const eventAuthority = await zamaEventAuthorityAddress(programAddress);
  const programData = await programDataAddressFor(programAddress);
  const [hostConfig] = await findHostConfigPda({ programAddress });
  const [randNonce] = await findRandNoncePda({ programAddress });
  const shared = { eventAuthority, program: programAddress, hostConfig } as const;
  const ixConfig = { programAddress } as const;

  const kmsCorruptionThreshold = params.kmsCorruptionThreshold ?? 0;
  const certificateThreshold = kmsCertificateThreshold(kmsCorruptionThreshold, params.gateway.kmsSigners.length);
  const existing = await fetchEncodedAccount(context.rpc, hostConfig, { commitment: 'confirmed' });

  if (existing.exists) {
    const equalBytes = (a: ArrayLike<number>, b: ArrayLike<number>) => Buffer.from(a).equals(Buffer.from(b));
    if (
      existing.programAddress !== programAddress ||
      !equalBytes(existing.data.slice(0, 8), HOST_CONFIG_DISCRIMINATOR)
    ) {
      throw new Error('existing HostConfig has an unexpected owner or discriminator');
    }
    const config = getHostConfigDecoder().decode(existing.data);
    if (
      config.admin !== params.payer.address ||
      config.chainId !== SOLANA_HOST_CHAIN_ID ||
      config.gatewayChainId !== params.gateway.gatewayChainId ||
      !equalBytes(config.inputVerificationContract, params.gateway.inputVerificationContract) ||
      !equalBytes(config.decryptionContract, params.gateway.decryptionContract) ||
      config.coprocessorThreshold !== (params.coprocessorThreshold ?? 1) ||
      config.coprocessorSignerCount !== params.gateway.coprocessorSigners.length ||
      !params.gateway.coprocessorSigners.every((signer, i) => equalBytes(signer, config.coprocessorSigners[i]!))
    ) {
      throw new Error(
        'existing HostConfig does not match deployment inputs; refuse to bind an existing host to a different stack',
      );
    }
    console.log('host_config matches deployment inputs');
  } else if (!params.validateOnly) {
    await context.sendTransaction(params.payer, [
      await getInitializeHostConfigInstructionAsync(
        {
          payer: params.payer,
          admin: params.payer,
          programData,
          randNonce,
          chainId: SOLANA_HOST_CHAIN_ID,
          gatewayChainId: params.gateway.gatewayChainId,
          inputVerificationContract: params.gateway.inputVerificationContract,
          coprocessorSigners: [...params.gateway.coprocessorSigners],
          coprocessorThreshold: params.coprocessorThreshold ?? 1,
          decryptionContract: params.gateway.decryptionContract,
          grantDenyListEnabled: false,
          ...shared,
        },
        ixConfig,
      ),
    ]);
    console.log('OK initialize_host_config');
  }

  const [kmsContext] = await findKmsContextPda({ contextId: BRINGUP_KMS_CONTEXT_ID }, { programAddress });
  const existingContext = await fetchEncodedAccount(context.rpc, kmsContext, { commitment: 'confirmed' });
  if (existingContext.exists) {
    // The Codama subset exports the instruction/threshold codec but not the KmsContext account.
    // Account layout is defined in zama-host/src/state/kms_context.rs.
    const decoder = getStructDecoder([
      ['discriminator', fixDecoderSize(getBytesDecoder(), 8)],
      ['contextId', fixDecoderSize(getBytesDecoder(), 32)],
      ['signers', getArrayDecoder(fixDecoderSize(getBytesDecoder(), 20))],
      ['thresholds', getKmsThresholdsDecoder()],
      ['destroyed', getBooleanDecoder()],
      ['bump', getU8Decoder()],
    ]);
    const expectedDiscriminator = createHash('sha256').update('account:KmsContext').digest().subarray(0, 8);
    const data = decoder.decode(existingContext.data);
    const equal = (a: ArrayLike<number>, b: ArrayLike<number>) => Buffer.from(a).equals(Buffer.from(b));
    if (
      existingContext.programAddress !== programAddress ||
      !equal(data.discriminator, expectedDiscriminator) ||
      !equal(data.contextId, BRINGUP_KMS_CONTEXT_ID) ||
      data.destroyed ||
      data.signers.length !== params.gateway.kmsSigners.length ||
      !data.signers.every((signer, i) => equal(signer, params.gateway.kmsSigners[i]!)) ||
      data.thresholds.publicDecryption !== certificateThreshold ||
      data.thresholds.userDecryption !== certificateThreshold ||
      data.thresholds.kmsGen !== certificateThreshold ||
      data.thresholds.mpc !== kmsCorruptionThreshold
    )
      throw new Error('existing KMS context does not match deployment inputs');
    console.log('kms_context matches deployment inputs');
    return;
  }

  if (params.validateOnly) return;

  const instructions: Instruction[] = [
    await getDefineKmsContextInstructionAsync(
      {
        admin: params.payer,
        contextId: BRINGUP_KMS_CONTEXT_ID,
        signers: [...params.gateway.kmsSigners],
        thresholds: {
          publicDecryption: certificateThreshold,
          userDecryption: certificateThreshold,
          kmsGen: certificateThreshold,
          // Mirrors the gateway's MPC_THRESHOLD, which is t itself and NOT 2t+1 (fhevm-cli
          // generates MPC_THRESHOLD=t alongside the =2t+1 decryption thresholds). Stored for
          // fidelity, never gates on-chain verification.
          mpc: kmsCorruptionThreshold,
        },
        kmsContext,
        ...shared,
      },
      ixConfig,
    ),
  ];
  await context.sendTransaction(params.payer, instructions);
  console.log(
    `OK define_kms_context (signers: ${params.gateway.kmsSigners.length}, ` +
      `t=${kmsCorruptionThreshold}, cert_threshold=${certificateThreshold})`,
  );
};
