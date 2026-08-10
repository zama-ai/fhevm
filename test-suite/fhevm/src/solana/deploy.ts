// deploy — configures a freshly-deployed zama-host for the running stack: the singleton
// HostConfig plus the active KMS context, both from the REAL live gateway/ProtocolConfig values.
// This is the typed replacement for the retired live-client's `BOOTSTRAP=1` mode (the last
// production duty that Rust crate had), built from the same generated Codama client the scenarios
// use. Mock inputs and test shims stay OFF: everything is read live via `addresses.ts`, so the
// bootstrap is reproducible from a clean `fhevm-cli up --scenario solana`.
//
// Run directly (from test-suite/fhevm, validator + gateway up):
//   bun run src/solana/deploy.ts

import { fetchEncodedAccount, type TransactionSigner } from "@solana/kit";

import { readGatewayBootstrapInputs, SOLANA_HOST_CHAIN_ID, type GatewayBootstrapInputs } from "./addresses";
import { zamaEventAuthorityAddress } from "./fhe-execute";
import {
  getDefineKmsContextInstructionAsync,
  getInitializeHostConfigInstructionAsync,
} from "./internal/generated/zamaHost/instructions/index.js";
import { findHostConfigPda } from "./internal/generated/zamaHost/pdas/index.js";
import { ZAMA_HOST_PROGRAM_ADDRESS } from "./internal/generated/zamaHost/programAddress.js";
import { createProvisioningContext, type SolanaProvisioningContext } from "./provision";

/** The KMS context id the bring-up defines and every consumer resolves (`demo/seed.ts`, scenarios). */
export const BOOTSTRAP_KMS_CONTEXT_ID = 1n;

export type KmsCertificateThresholds = {
  /** Matching signatures a certificate needs: 2t+1 of the registered signer set. */
  readonly certificateThreshold: number;
};

/**
 * Derives the on-chain certificate threshold from the KMS corruption threshold t. A centralized
 * KMS (t=0) signs with one key; a threshold-mode KMS needs 2t+1 matching signatures, and KMS core
 * requires parties == 3t+1 (see scenarios/four-party-threshold-kms.yaml).
 */
export const kmsCertificateThresholds = (
  kmsCorruptionThreshold: number,
  registeredSignerCount: number,
): KmsCertificateThresholds => {
  const certificateThreshold = 2 * kmsCorruptionThreshold + 1;
  if (certificateThreshold > registeredSignerCount) {
    throw new Error(
      `KMS_THRESHOLD=${kmsCorruptionThreshold} needs 2t+1=${certificateThreshold} certificate ` +
        `signatures but only ${registeredSignerCount} KMS signers are registered on the gateway`,
    );
  }
  return { certificateThreshold };
};

export type BootstrapZamaHostParams = {
  readonly payer: TransactionSigner;
  readonly gateway: GatewayBootstrapInputs;
  readonly hostChainId?: bigint;
  /**
   * Input-attestation n-of-m. The PoC coprocessor emits a single attestation signature, so 1
   * keeps the live flow green while the full registered set is stored (EVM `InputVerifier`
   * parity).
   */
  readonly coprocessorThreshold?: number;
  /** KMS corruption threshold t; 0 is the centralized PoC default. */
  readonly kmsCorruptionThreshold?: number;
  readonly kmsContextId?: bigint;
};

/**
 * Initializes the zama-host HostConfig (idempotent: the gateway outlives validator resets, the
 * config account does not, so a re-run against a fresh validator recreates it and a re-run against
 * a configured one skips it) and defines the active KMS context from the registered signer set.
 */
export const bootstrapZamaHost = async (
  context: SolanaProvisioningContext,
  params: BootstrapZamaHostParams,
): Promise<void> => {
  const eventAuthority = await zamaEventAuthorityAddress();
  const [hostConfig] = await findHostConfigPda();
  const shared = { eventAuthority, program: ZAMA_HOST_PROGRAM_ADDRESS } as const;

  const existing = await fetchEncodedAccount(context.rpc, hostConfig);
  if (existing.exists) {
    console.log("host_config already initialized — skipping initialize_host_config");
  } else {
    await context.sendTransaction(params.payer, [
      await getInitializeHostConfigInstructionAsync({
        payer: params.payer,
        admin: params.payer,
        chainId: params.hostChainId ?? SOLANA_HOST_CHAIN_ID,
        gatewayChainId: params.gateway.gatewayChainId,
        inputVerificationContract: params.gateway.inputVerificationContract,
        coprocessorSigners: [...params.gateway.coprocessorSigners],
        coprocessorThreshold: params.coprocessorThreshold ?? 1,
        decryptionContract: params.gateway.decryptionContract,
        grantDenyListEnabled: false,
        ...shared,
      }),
    ]);
    console.log("OK initialize_host_config");
  }

  const kmsCorruptionThreshold = params.kmsCorruptionThreshold ?? 0;
  const { certificateThreshold } = kmsCertificateThresholds(
    kmsCorruptionThreshold,
    params.gateway.kmsSigners.length,
  );
  await context.sendTransaction(params.payer, [
    await getDefineKmsContextInstructionAsync({
      admin: params.payer,
      contextId: params.kmsContextId ?? BOOTSTRAP_KMS_CONTEXT_ID,
      signers: [...params.gateway.kmsSigners],
      thresholds: {
        publicDecryption: certificateThreshold,
        userDecryption: certificateThreshold,
        kmsGen: certificateThreshold,
        // Mirrors the gateway's MPC_THRESHOLD, which is t itself and NOT 2t+1 (fhevm-cli
        // generates MPC_THRESHOLD=t alongside the =2t+1 decryption thresholds; see
        // src/kms-threshold.test.ts). Stored for fidelity, never gates on-chain verification.
        mpc: kmsCorruptionThreshold,
      },
      ...shared,
    }),
  ]);
  console.log(
    `OK define_kms_context (signers: ${params.gateway.kmsSigners.length}, ` +
      `t=${kmsCorruptionThreshold}, cert_threshold=${certificateThreshold})`,
  );
};

/** Reads the standard 64-byte Solana CLI keypair file into a kit signer. */
export const loadKeypairSigner = async (path: string): Promise<TransactionSigner> => {
  const { createKeyPairSignerFromBytes } = await import("@solana/kit");
  const bytes = Uint8Array.from(JSON.parse(await Bun.file(path).text()) as number[]);
  return createKeyPairSignerFromBytes(bytes);
};

const readIntegerEnv = (name: string, fallback: number): number => {
  const raw = process.env[name];
  if (raw === undefined || raw === "") return fallback;
  const value = Number.parseInt(raw, 10);
  if (!Number.isSafeInteger(value) || value < 0) throw new Error(`${name} must be a non-negative integer`);
  return value;
};

if (import.meta.main) {
  // Deployer/fee-payer wallet: the same path the validator deploy signs with, so the bootstrap
  // admin is the wallet that already holds the airdropped fee balance.
  const payer = await loadKeypairSigner(
    process.env.DEPLOYER_KEYPAIR ?? `${process.env.HOME}/.config/solana/id.json`,
  );
  const gateway = await readGatewayBootstrapInputs({
    gatewayRpcUrl: process.env.GW_RPC ?? "http://127.0.0.1:8546",
  });
  const context = createProvisioningContext(
    process.env.SOLANA_RPC_URL ?? "http://127.0.0.1:8899",
    process.env.SOLANA_WS_URL ?? "ws://127.0.0.1:8900",
  );
  await bootstrapZamaHost(context, {
    payer,
    gateway,
    coprocessorThreshold: readIntegerEnv("COPROCESSOR_THRESHOLD", 1),
    kmsCorruptionThreshold: readIntegerEnv("KMS_THRESHOLD", 0),
  });
}
