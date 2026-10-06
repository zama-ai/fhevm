// fhe-vertical — the decrypt half the live scenarios share: read a value's state and request the
// KMS public-decrypt certificate of a public handle.
//
// The decrypt requests themselves go through the strict, unit-tested request builders the
// fhevm-cli already ships (`./public-decrypt`, `./current-user-decrypt`).

import { getAddressEncoder, type Address } from '@solana/kit';

import { asBytes32Hex, hexToBytes } from '@fhevm/sdk/base';
import {
  fetchSolanaEncryptedStore,
  encryptedStoreHandle,
} from '@fhevm/sdk/solana';

import { runSolanaCurrentUserDecrypt } from './current-user-decrypt';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '../../../../solana/deploy/src/generated/zamaHost/programAddress.js';
import {
  certificateCleartext,
  createPublicDecryptClient,
  runSolanaPublicDecrypt,
  type PublicDecryptCertificate,
} from './public-decrypt';
import type { SolanaProvisioningContext } from './provision';

const hex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString('hex')}`;
const addressBytes = (value: Address): Uint8Array => new Uint8Array(getAddressEncoder().encode(value));
const addressHex = (value: Address): string => hex(addressBytes(value));

/** The environment facts every vertical decrypt binds to. */
export type FheVerticalConfig = {
  readonly rpcUrl: string;
  readonly relayerUrl: string;
  /** The Solana host chain id (`HostConfig.chain_id`, type byte `0x01`). */
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

/** The current handle bytes of an encrypted value at `finalized`. */
export const currentHandle = async (
  context: SolanaProvisioningContext,
  encryptedStore: Address,
  key: Uint8Array,
): Promise<Uint8Array> =>
  encryptedStoreHandle(
    await fetchSolanaEncryptedStore(
      context.rpc,
      encryptedStore,
      { commitment: 'finalized' },
      ZAMA_HOST_PROGRAM_ADDRESS,
    ),
    key,
  );

/** A certified public decrypt: the interpreted cleartext plus the raw KMS certificate. */
export type PublicDecryptOutcome = {
  readonly cleartext: bigint;
  /** The full certificate — what on-chain consume steps (redeem/disclose) verify. */
  readonly certificate: PublicDecryptCertificate;
};

/**
 * Requests the KMS public-decrypt certificate of `handle`, made public in `encryptedStore`, through
 * the SDK's public-decrypt action. Returns the cleartext together with the certificate; asserting
 * the value is the scenario's job.
 */
export const certifiedPublicDecrypt = async (
  config: FheVerticalConfig,
  params: { readonly encryptedStore: Address; readonly handle: Uint8Array },
): Promise<PublicDecryptOutcome> => {
  const certificate = await runSolanaPublicDecrypt({
    PD_RPC_URL: config.rpcUrl,
    PD_RELAYER_URL: config.relayerUrl,
    PD_VERIFYING_PROGRAM_ID: config.verifyingProgramId,
    PD_CONTRACTS_CHAIN_ID: config.chainId.toString(),
    PD_HANDLE: hex(params.handle),
    PD_CONTEXT_ID: config.publicDecryptContextId,
    PD_ENCRYPTED_STORE: addressHex(params.encryptedStore),
  });
  return { cleartext: certificateCleartext(certificate), certificate };
};

/**
 * Decrypts several public handles, each made public in its own store, from one KMS certificate
 * through the SDK's batch action. Returns the values in entry order.
 */
export const publicDecryptValues = async (
  config: FheVerticalConfig,
  entries: readonly { readonly encryptedStore: Address; readonly handle: Uint8Array }[],
): Promise<unknown[]> => {
  const client = await createPublicDecryptClient({
    rpcUrl: config.rpcUrl,
    chainId: config.chainId,
    relayerUrl: config.relayerUrl,
    verifyingProgramId: asBytes32Hex(config.verifyingProgramId),
    apiKey: process.env.ZAMA_FHEVM_API_KEY ?? 'local',
  });
  const values = await client.decryptPublicValues({
    contextId: hexToBytes(config.publicDecryptContextId),
    entries: entries.map(({ encryptedStore, handle }) => ({
      handle: hex(handle),
      encryptedStore: addressBytes(encryptedStore),
    })),
  });
  return values.map(({ value }) => value);
};

/**
 * Runs the permit-path user decrypt of `handle` (current or since replaced — the Connector proves
 * the allow leaf either way) as the wallet behind `secretKey`, and asserts the cleartext equals
 * `expected`. `ownerAddress` names the delegator on a delegated entry.
 */
export const userDecryptExpect = (
  config: FheVerticalConfig,
  params: {
    readonly encryptedStore: Address;
    readonly handle: Uint8Array;
    /** The signer's 32-byte ed25519 seed, 0x-hex. */
    readonly secretKey: string;
    readonly expected: bigint;
    readonly ownerAddress?: Address | undefined;
  },
): Promise<bigint> =>
  runSolanaCurrentUserDecrypt({
    UD_RPC_URL: config.rpcUrl,
    UD_RELAYER_URL: config.relayerUrl,
    UD_CONTRACTS_CHAIN_ID: config.chainId.toString(),
    UD_HANDLE: hex(params.handle),
    UD_ENCRYPTED_STORE: addressHex(params.encryptedStore),
    UD_SECRET_KEY: params.secretKey,
    UD_CONTEXT_ID: `0x${BigInt(config.userDecryptContextId).toString(16).padStart(64, '0')}`,
    UD_EPOCH_ID: config.kmsEpochId,
    UD_VERIFYING_PROGRAM_ID: config.verifyingProgramId,
    UD_KMS_SIGNERS: config.kmsSigners.join(','),
    UD_FHE_PARAMETER: config.fheParameter,
    UD_GATEWAY_CHAIN_ID: config.gatewayChainId,
    UD_GATEWAY_DECRYPTION_CONTRACT: config.gatewayDecryptionContract,
    UD_EXPECTED: params.expected.toString(),
    ...(params.ownerAddress === undefined ? {} : { UD_OWNER_ADDRESS: addressHex(params.ownerAddress) }),
  });
