// fhe-vertical — the decrypt half the live scenarios share: read a value's state, request the KMS
// public-decrypt certificate of a public handle, and build the public-leaf inclusion proof the
// on-chain consume steps (redeem, disclose) verify.
//
// The client fetches no proof for a decrypt (RFC 035): the request names `(handle, account)` and
// the Connector reads the account and asks the coprocessors for the leaf. The proof built here is
// for the ON-CHAIN verifier only, rebuilt from the account's history exactly as the demo vault's
// settle does, and cross-checked against the live peaks so a history the scenario got wrong fails
// here with the leaf count named, not inside the program as a generic verifier error.
//
// The decrypt requests themselves go through the strict, unit-tested request builders the
// fhevm-cli already ships (`./public-decrypt`, `./current-user-decrypt`).

import { getAddressEncoder, type Address } from "@solana/kit";

import { fetchSolanaEncryptedValueState, type SolanaEncryptedValueState } from "@sdk-src/solana/encryptedValueAccount.js";
import { buildPublicLeafProof, type MmrProof, type SolanaEncryptedValueAccountEvent } from "@sdk-src/solana/proof.js";

import { runSolanaCurrentUserDecrypt } from "./current-user-decrypt";
import { ZAMA_HOST_PROGRAM_ADDRESS } from "./internal/generated/zamaHost/programAddress.js";
import { certificateCleartext, runSolanaPublicDecrypt, type PublicDecryptCertificate } from "./public-decrypt";
import type { SolanaProvisioningContext } from "./provision";

const hex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString("hex")}`;
const addressBytes = (value: Address): Uint8Array => new Uint8Array(getAddressEncoder().encode(value));
const addressHex = (value: Address): string => hex(addressBytes(value));

/** The environment facts every vertical decrypt binds to. */
export type FheVerticalConfig = {
  readonly relayerUrl: string;
  /** The Solana host chain id (`HostConfig.chain_id`, high bit set). */
  readonly chainId: bigint;
  /** KMS public-decrypt context id, 0x-hex bytes32. */
  readonly publicDecryptContextId: string;
  /** Gateway user-decrypt context id, unsigned decimal string. */
  readonly userDecryptContextId: string;
  /** The zama-host program id as bytes32 hex — the permit's verifying program. */
  readonly verifyingProgramId: `0x${string}`;
  /** The registered KMS signer set (EVM addresses, gateway registry order). */
  readonly kmsSigners: readonly `0x${string}`[];
  /** The KMS epoch id permits are minted for, bytes32 hex. */
  readonly kmsEpochId: `0x${string}`;
  /** The FHE parameter choice the local stack runs. */
  readonly fheParameter: string;
  /** The gateway chain id, unsigned decimal string. */
  readonly gatewayChainId: string;
  /** The gateway `Decryption` contract — the EIP-712 verifying contract of KMS node signatures. */
  readonly gatewayDecryptionContract: `0x${string}`;
};

/** Reads an `EncryptedValue` account at `confirmed`, asserting the host program owns it. */
export const readEncryptedValueState = (
  context: SolanaProvisioningContext,
  encryptedValue: Address,
): Promise<SolanaEncryptedValueState> =>
  fetchSolanaEncryptedValueState(context.rpc, encryptedValue, { commitment: "confirmed" }, ZAMA_HOST_PROGRAM_ADDRESS);

/** The current handle bytes of an encrypted value at `confirmed`. */
export const currentHandle = async (context: SolanaProvisioningContext, encryptedValue: Address): Promise<Uint8Array> =>
  (await readEncryptedValueState(context, encryptedValue)).currentHandle;

/** A certified public decrypt: the interpreted cleartext plus the raw KMS certificate. */
export type PublicDecryptOutcome = {
  readonly cleartext: bigint;
  /** The full certificate — what on-chain consume steps (redeem/disclose) verify. */
  readonly certificate: PublicDecryptCertificate;
};

/**
 * Requests the KMS public-decrypt certificate of `handle`, made public in `encryptedValue`, through
 * the SDK's public-decrypt action. Returns the cleartext together with the certificate; asserting
 * the value is the scenario's job.
 */
export const certifiedPublicDecrypt = async (
  config: FheVerticalConfig,
  params: { readonly encryptedValue: Address; readonly handle: Uint8Array },
): Promise<PublicDecryptOutcome> => {
  const certificate = await runSolanaPublicDecrypt({
    PD_RELAYER_URL: config.relayerUrl,
    PD_CONTRACTS_CHAIN_ID: config.chainId.toString(),
    PD_HANDLE: hex(params.handle),
    PD_CONTEXT_ID: config.publicDecryptContextId,
    PD_ENCRYPTED_VALUE_ACCOUNT: addressHex(params.encryptedValue),
  });
  return { cleartext: certificateCleartext(certificate), certificate };
};

/**
 * Runs the permit-path user decrypt of `handle` (current or since replaced — the Connector proves
 * the allow leaf either way) as the wallet behind `secretKey`, and asserts the cleartext equals
 * `expected`. `allowedKey` names the delegator on a delegated entry.
 */
export const userDecryptExpect = (
  config: FheVerticalConfig,
  params: {
    readonly encryptedValue: Address;
    readonly handle: Uint8Array;
    /** The signer's 32-byte ed25519 seed, 0x-hex. */
    readonly secretKey: string;
    readonly expected: bigint;
    readonly allowedKey?: Address | undefined;
  },
): Promise<bigint> =>
  runSolanaCurrentUserDecrypt({
    UD_RELAYER_URL: config.relayerUrl,
    UD_CONTRACTS_CHAIN_ID: config.chainId.toString(),
    UD_HANDLE: hex(params.handle),
    UD_ENCRYPTED_VALUE_ACCOUNT: addressHex(params.encryptedValue),
    UD_SECRET_KEY: params.secretKey,
    UD_CONTEXT_ID: `0x${BigInt(config.userDecryptContextId).toString(16).padStart(64, "0")}`,
    UD_EPOCH_ID: config.kmsEpochId,
    UD_VERIFYING_PROGRAM_ID: config.verifyingProgramId,
    UD_KMS_SIGNERS: config.kmsSigners.join(","),
    UD_FHE_PARAMETER: config.fheParameter,
    UD_GATEWAY_CHAIN_ID: config.gatewayChainId,
    UD_GATEWAY_DECRYPTION_CONTRACT: config.gatewayDecryptionContract,
    UD_EXPECTED: params.expected.toString(),
    ...(params.allowedKey === undefined ? {} : { UD_ALLOWED_KEY: addressHex(params.allowedKey) }),
  });


/** The SDK's {@link buildPublicLeafProof} against the live account. */
export const livePublicLeafProof = async (
  context: SolanaProvisioningContext,
  encryptedValue: Address,
  history: readonly SolanaEncryptedValueAccountEvent[],
  publicLeafIndex: bigint,
): Promise<MmrProof> =>
  buildPublicLeafProof(addressBytes(encryptedValue), await readEncryptedValueState(context, encryptedValue), history, publicLeafIndex);
