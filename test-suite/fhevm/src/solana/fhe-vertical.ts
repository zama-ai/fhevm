// fhe-vertical — the decrypt half the live scenarios share: read a value's state, request the
// KMS public-decrypt certificate of a public handle, and user-decrypt a handle through the permit
// path.

import { getAddressEncoder, type Address } from '@solana/kit';

import { asBytes32Hex, hexToBytes } from '@fhevm/sdk/base';
import {
  fetchSolanaEncryptedStore,
  encryptedStoreHandle,
} from '@fhevm/sdk/solana';

import { createFinalizedRpc, ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { relayerAuth } from '../layout';
import { certificateCleartext, type PublicDecryptCertificate } from './public-decrypt';
import type { SolanaProvisioningContext } from './provision';
import { loadSolanaSdk } from './target';
import { expectCleartext, userDecryptScalar } from './user-decrypt-result';

const hex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString('hex')}`;
const addressBytes = (value: Address): Uint8Array => new Uint8Array(getAddressEncoder().encode(value));

/** The environment facts every vertical decrypt binds to. */
export type FheVerticalConfig = {
  readonly rpcUrl: string;
  readonly relayerUrl: string;
  /** The Solana host chain id (`HostConfig.chain_id`, type byte `0x01`). */
  readonly chainId: bigint;
  /** The zama-host program id as bytes32 hex — the permit's verifying program. */
  readonly verifyingProgramId: `0x${string}`;
  /** The FHE parameter choice the local stack runs. */
  readonly fheParameter: string;
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

/** The target's public-decrypt client: the relayer's, or the cleartext stack's. */
const publicDecryptClient = async (config: FheVerticalConfig) => {
  const solana = await loadSolanaSdk();
  const chain = solana.defineFhevmSolanaChain({
    id: config.chainId,
    fhevm: { relayerUrl: config.relayerUrl, programs: { host: { address: asBytes32Hex(config.verifyingProgramId) } } },
  });
  solana.setFhevmRuntimeConfig({ auth: relayerAuth() });
  return solana.createFhevmPublicDecryptClient({ chain, rpc: createFinalizedRpc(config.rpcUrl) });
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
  const certificate = await (await publicDecryptClient(config)).publicDecryptCertificate({
    handle: hex(params.handle),
    encryptedStore: addressBytes(params.encryptedStore),
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
  const client = await publicDecryptClient(config);
  const values = await client.decryptPublicValues({
    entries: entries.map(({ encryptedStore, handle }) => ({
      handle: hex(handle),
      encryptedStore: addressBytes(encryptedStore),
    })),
  });
  return values.map(({ value }) => value);
};

export type UserDecryptParams = {
  readonly encryptedStore: Address;
  readonly handle: Uint8Array;
  /** The signer's 32-byte ed25519 seed, 0x-hex. */
  readonly secretKey: string;
  readonly ownerAddress?: Address | undefined;
};

/**
 * Runs the permit-path user decrypt of `handle` (current or since replaced — the Connector proves
 * the allow leaf either way) as the wallet behind `secretKey`, and returns the cleartext.
 * `ownerAddress` names the delegator on a delegated entry.
 */
export const userDecrypt = async (config: FheVerticalConfig, params: UserDecryptParams): Promise<bigint> => {
  const solana = await loadSolanaSdk();
  const chain = solana.defineFhevmSolanaChain({
    id: config.chainId,
    fhevm: { relayerUrl: config.relayerUrl, programs: { host: { address: asBytes32Hex(config.verifyingProgramId) } } },
  });
  solana.setFhevmRuntimeConfig({ auth: relayerAuth() });
  const client = solana.createFhevmDecryptClient({
    chain,
    rpc: createFinalizedRpc(config.rpcUrl),
    fheParameter: config.fheParameter,
  });
  // One wallet signature mints a permissive session; the request runs under it.
  const wallet = solana.solanaPermitWalletFromSecretKey(hexToBytes(params.secretKey));
  const session = await client.signPermit({ wallet, durationSeconds: 3600n });
  const clearValues = await client.decryptValues({
    session,
    entries: [
      {
        handle: params.handle,
        encryptedStore: addressBytes(params.encryptedStore),
        ...(params.ownerAddress === undefined ? {} : { ownerAddress: addressBytes(params.ownerAddress) }),
      },
    ],
  });
  return userDecryptScalar(clearValues);
};

/** `userDecrypt`, asserting the cleartext equals `expected`. */
export const userDecryptExpect = async (
  config: FheVerticalConfig,
  params: UserDecryptParams & { readonly expected: bigint },
): Promise<bigint> => expectCleartext(await userDecrypt(config, params), params.expected);
