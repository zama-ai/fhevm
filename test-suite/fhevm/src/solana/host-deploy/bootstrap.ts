// Initializes HostConfig and defines the bring-up KMS context from live gateway values.
// Idempotent: a re-run (helm Job recreate, Tailscale retry) skips whichever accounts already
// exist. `define_kms_context` is `init` on the context PDA, so a second call would fail closed
// without the skip.

import {
  fetchEncodedAccount,
  getAddressEncoder,
  getProgramDerivedAddress,
  type Address,
  type Instruction,
  type TransactionSigner,
} from "@solana/kit";

import {
  getDefineKmsContextInstructionAsync,
  getInitializeHostConfigInstructionAsync,
} from "../internal/generated/zamaHost/instructions/index.js";
import { findHostConfigPda, findKmsContextPda } from "../internal/generated/zamaHost/pdas/index.js";
import { ZAMA_HOST_PROGRAM_ADDRESS } from "../internal/generated/zamaHost/programAddress.js";

import { BRINGUP_KMS_CONTEXT_ID, SOLANA_HOST_CHAIN_ID } from "./constants";
import type { GatewayBootstrapInputs } from "./gateway";
import type { HostDeployContext } from "./send";

const BPF_UPGRADEABLE_LOADER = "BPFLoaderUpgradeab1e11111111111111111111111" as Address;

/**
 * Derives the on-chain certificate threshold (matching signatures a certificate needs) from the
 * KMS corruption threshold t. A centralized KMS (t=0) signs with one key; a threshold-mode KMS
 * needs 2t+1 matching signatures, and KMS core requires parties == 3t+1.
 */
export const kmsCertificateThreshold = (kmsCorruptionThreshold: number, registeredSignerCount: number): number => {
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
    seeds: [new TextEncoder().encode("__event_authority")],
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
};

export const bootstrapZamaHost = async (
  context: HostDeployContext,
  params: BootstrapZamaHostParams,
): Promise<void> => {
  const programAddress = params.programAddress ?? ZAMA_HOST_PROGRAM_ADDRESS;
  const eventAuthority = await zamaEventAuthorityAddress(programAddress);
  const programData = await programDataAddressFor(programAddress);
  const [hostConfig] = await findHostConfigPda({ programAddress });
  const shared = { eventAuthority, program: programAddress, hostConfig } as const;
  const ixConfig = { programAddress } as const;

  const existing = await fetchEncodedAccount(context.rpc, hostConfig);
  if (existing.exists) {
    console.log("host_config already initialized — skipping initialize_host_config");
  } else {
    await context.sendTransaction(params.payer, [
      await getInitializeHostConfigInstructionAsync(
        {
          payer: params.payer,
          admin: params.payer,
          programData,
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
    console.log("OK initialize_host_config");
  }

  const [kmsContext] = await findKmsContextPda({ contextId: BRINGUP_KMS_CONTEXT_ID }, { programAddress });
  const existingContext = await fetchEncodedAccount(context.rpc, kmsContext);
  if (existingContext.exists) {
    console.log("kms_context already defined — skipping define_kms_context");
    return;
  }

  const kmsCorruptionThreshold = params.kmsCorruptionThreshold ?? 0;
  const certificateThreshold = kmsCertificateThreshold(kmsCorruptionThreshold, params.gateway.kmsSigners.length);
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
