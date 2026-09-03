// One user decryption, end to end: a fresh transport key pair, a decryption permit the user signs, one
// `decryptValues` call. The SDK's viem adapter signs through anything with a viem-style
// `signTypedData`, so a user is a wallet client that carries its account or a local account.

import { HardhatPluginError } from 'hardhat/plugins';
import { type Address, type Hex, type LocalAccount, type TypedDataDefinition, isAddress } from 'viem';

import type { FhevmClient, FhevmUser, FhevmUserDecryptOptions } from '../types.js';
import { PLUGIN_ID } from './constants.js';
import { assertHandleIsInitialized } from './fhevmHandle.js';
import { timestampNow } from './time.js';

const DEFAULT_DURATION_DAYS = 365;
const SECONDS_PER_DAY = 24 * 60 * 60;

type TypedValueLike = { readonly type: string; readonly value: unknown };
type ResolvedUser = { readonly address: Address; readonly signer: unknown };

export async function userDecryptOne(
  client: FhevmClient,
  method: string,
  handle: Hex,
  contractAddress: Address,
  user: FhevmUser,
  options: FhevmUserDecryptOptions | undefined,
): Promise<TypedValueLike> {
  assertHandleIsInitialized(handle);
  if (!isAddress(contractAddress)) {
    throw new HardhatPluginError(
      PLUGIN_ID,
      `${method}: the 'contractAddress' argument is not a valid address. Got '${contractAddress}' instead.`,
    );
  }
  const { address, signer } = resolveUser(method, user);

  const transportKeyPair = await client.generateTransportKeyPair();
  const startTimestamp = Number(options?.validity?.startTimestamp ?? timestampNow());
  const durationDays = Number(options?.validity?.durationDays ?? DEFAULT_DURATION_DAYS);

  const signedPermit = await client.signLegacyDecryptionPermit({
    contractAddresses: [contractAddress],
    startTimestamp,
    // The plugin measures validity in days; the SDK takes seconds.
    durationSeconds: durationDays * SECONDS_PER_DAY,
    signerAddress: address,
    signer,
    transportKeyPair,
  });

  const values = await client.decryptValues({
    encryptedValues: [handle],
    contractAddress,
    transportKeyPair,
    signedPermit,
  });
  const value = values[0];
  if (value === undefined) {
    throw new HardhatPluginError(PLUGIN_ID, `${method}: failed to decrypt handle '${handle}' for user ${address}.`);
  }
  return value;
}

// A local account signs by itself; a wallet client signs for the account it was created with — one
// without an account cannot say who the permit is for, so it is refused here rather than by the node.
function isLocalAccount(user: FhevmUser): user is LocalAccount {
  return !('account' in user);
}

function resolveUser(method: string, user: FhevmUser): ResolvedUser {
  if (isLocalAccount(user)) {
    return {
      address: user.address,
      signer: { signTypedData: (data: TypedDataDefinition) => user.signTypedData(data) },
    };
  }
  if (user.account === undefined) {
    throw new HardhatPluginError(
      PLUGIN_ID,
      `${method}: the wallet client carries no account; create it with an 'account' so the plugin knows who signs the decryption permit.`,
    );
  }
  return { address: user.account.address, signer: user };
}
