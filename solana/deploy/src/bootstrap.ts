// Initializes HostConfig with EVM's initial HCU limits and defines the bring-up KMS context from
// live gateway values.
// A re-run validates the existing host binding before skipping account initialization:
// `define_kms_context` is `init` on the context PDA, so a second call would fail closed
// without the skip.
import {
  type Address,
  type Instruction,
  type TransactionSigner,
  fetchEncodedAccount,
  getAddressEncoder,
  getProgramDerivedAddress,
} from '@solana/kit';
import { LOADER_V3_PROGRAM_ADDRESS } from '@solana-program/loader-v3';

import { assertValidSolanaChainId, isEvmHostChainId } from '../../../sdk/js-sdk/src/core/chains/hostChainId';
import { BRINGUP_KMS_CONTEXT_ID, HCU_LIMITS } from './constants';
import type { GatewayBootstrapInputs } from './gateway';
import {
  findEventAuthorityPda,
  findHostConfigPda,
  findKmsContextPda,
  findRandNoncePda,
  getDefineKmsContextInstructionAsync,
  getHostConfigDecoder,
  getInitializeHostConfigInstructionAsync,
  getKmsContextDecoder,
  getSetMaxHcuDepthPerTxInstructionAsync,
  getSetMaxHcuPerTxInstructionAsync,
  HOST_CONFIG_DISCRIMINATOR,
  KMS_CONTEXT_DISCRIMINATOR,
  MAX_COPROCESSOR_SIGNERS,
  MAX_KMS_SIGNERS,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from '@fhevm/solana-zama-host';
import type { HostDeployContext } from './send';

/**
 * Derives the on-chain certificate threshold (matching signatures a certificate needs) from the
 * KMS corruption threshold t: 2t+1 matching signatures. The KMS core runs a committee of exactly
 * 3t+1 parties, so the gateway's signer set must have that size. t=0 is the single signer of a
 * cleartext host.
 */
export const kmsCertificateThreshold = (kmsCorruptionThreshold: number, registeredSignerCount: number): number => {
  if (!Number.isSafeInteger(kmsCorruptionThreshold) || kmsCorruptionThreshold < 0 || kmsCorruptionThreshold > 255) {
    throw new Error('KMS corruption threshold t must be an unsigned byte');
  }
  const committeeSize = 3 * kmsCorruptionThreshold + 1;
  if (registeredSignerCount !== committeeSize) {
    throw new Error(
      `KMS corruption threshold t=${kmsCorruptionThreshold} needs a committee of 3t+1=${committeeSize} ` +
        `KMS signers; the gateway has ${registeredSignerCount} registered`,
    );
  }
  return 2 * kmsCorruptionThreshold + 1;
};

/** BPF upgradeable loader `ProgramData` PDA (`[program_id]` under the loader). */
export const programDataAddressFor = async (programAddress: Address): Promise<Address> => {
  const [programData] = await getProgramDerivedAddress({
    programAddress: LOADER_V3_PROGRAM_ADDRESS,
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
  /** KMS corruption threshold t. */
  readonly kmsCorruptionThreshold: number;
  /** Program id to bootstrap. Defaults to the generated client's id. */
  readonly programAddress?: Address;
  /** Validate existing bindings without sending initialization transactions. */
  readonly validateOnly?: boolean;
  readonly chainId: bigint;
};

// Mirror program input constraints so malformed first-deploy config cannot upload bytecode first.
export const validateBootstrapInputs = (params: BootstrapZamaHostParams): void => {
  assertValidSolanaChainId(params.chainId);
  for (const [name, signers, maximum] of [
    ['coprocessor', params.gateway.coprocessorSigners, MAX_COPROCESSOR_SIGNERS],
    ['KMS', params.gateway.kmsSigners, MAX_KMS_SIGNERS],
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
  if (params.gateway.gatewayChainId < 0n || !isEvmHostChainId(params.gateway.gatewayChainId)) {
    throw new Error('gateway chain id must be a uint64-padded EVM id (high byte 0x00)');
  }
  if (
    [params.gateway.decryptionContract, params.gateway.inputVerificationContract].some(
      (contract) => contract.length !== 20 || contract.every((byte) => byte === 0),
    )
  ) {
    throw new Error('gateway contract addresses must be nonzero 20-byte addresses');
  }
  kmsCertificateThreshold(params.kmsCorruptionThreshold, params.gateway.kmsSigners.length);
};

// The program's unlimited HCU sentinel.
const UNLIMITED_HCU = 2n ** 64n - 1n;

export const bootstrapZamaHost = async (context: HostDeployContext, params: BootstrapZamaHostParams): Promise<void> => {
  validateBootstrapInputs(params);
  const programAddress = params.programAddress ?? ZAMA_HOST_PROGRAM_ADDRESS;
  const eventAuthority = (await findEventAuthorityPda({ programAddress }))[0];
  const programData = await programDataAddressFor(programAddress);
  const [hostConfig] = await findHostConfigPda({ programAddress });
  const [randNonce] = await findRandNoncePda({ programAddress });
  const shared = { eventAuthority, program: programAddress, hostConfig } as const;
  const ixConfig = { programAddress } as const;

  const { kmsCorruptionThreshold } = params;
  const certificateThreshold = kmsCertificateThreshold(kmsCorruptionThreshold, params.gateway.kmsSigners.length);
  const existing = await fetchEncodedAccount(context.rpc, hostConfig);

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
      config.chainId !== params.chainId ||
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
    // Initialization sets both limits in one transaction, so both unlimited means the host was
    // never bootstrapped. Any other values are an admin's tuning and stay as they are.
    if (config.maxHcuPerTx === UNLIMITED_HCU && config.maxHcuDepthPerTx === UNLIMITED_HCU) {
      throw new Error(
        'existing HostConfig still has unlimited HCU limits (never bootstrapped); set them with the HCU setters first',
      );
    }
    console.log(
      `host_config matches deployment inputs; HCU limits: maxHcuPerTx=${config.maxHcuPerTx} ` +
        `maxHcuDepthPerTx=${config.maxHcuDepthPerTx} hcuBlockCapPerApp=${config.hcuBlockCapPerApp}`,
    );
  } else if (!params.validateOnly) {
    await context.sendTransaction(params.payer, [
      await getInitializeHostConfigInstructionAsync(
        {
          payer: params.payer,
          admin: params.payer,
          programData,
          randNonce,
          chainId: params.chainId,
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
      await getSetMaxHcuDepthPerTxInstructionAsync(
        { admin: params.payer, value: HCU_LIMITS.maxHcuDepthPerTx, ...shared },
        ixConfig,
      ),
      await getSetMaxHcuPerTxInstructionAsync(
        { admin: params.payer, value: HCU_LIMITS.maxHcuPerTx, ...shared },
        ixConfig,
      ),
    ]);
    console.log('OK initialize_host_config');
  }

  const [kmsContext] = await findKmsContextPda({ contextId: BRINGUP_KMS_CONTEXT_ID }, { programAddress });
  const existingContext = await fetchEncodedAccount(context.rpc, kmsContext);
  if (existingContext.exists) {
    const data = getKmsContextDecoder().decode(existingContext.data);
    const equal = (a: ArrayLike<number>, b: ArrayLike<number>) => Buffer.from(a).equals(Buffer.from(b));
    if (
      existingContext.programAddress !== programAddress ||
      !equal(data.discriminator, KMS_CONTEXT_DISCRIMINATOR) ||
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
