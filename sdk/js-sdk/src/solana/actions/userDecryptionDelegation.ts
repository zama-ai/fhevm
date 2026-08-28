import {
  createNoopSigner,
  fetchEncodedAccount,
  fixDecoderSize,
  getAddressDecoder,
  getAddressEncoder,
  getBytesDecoder,
  getProgramDerivedAddress,
  getStructDecoder,
  getU64Decoder,
  getU8Decoder,
  type Address,
  type FetchAccountConfig,
  type Instruction,
  type MaybeEncodedAccount,
} from '@solana/kit';

import type { SolanaRpc } from '../encryptedValueAccount.js';
import { getDelegateForUserDecryptionInstructionAsync } from '../internal/generated/zamaHost/instructions/delegateForUserDecryption.js';
import { getRevokeDelegationForUserDecryptionInstructionAsync } from '../internal/generated/zamaHost/instructions/revokeDelegationForUserDecryption.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '../internal/generated/zamaHost/programAddress.js';

/** Seed of the user-decryption delegation record PDA. */
export const SOLANA_USER_DECRYPTION_DELEGATION_SEED = new TextEncoder().encode('user-decryption-delegation');

/**
 * The reserved sentinel a wildcard delegation row carries in place of an encrypted value account
 * authority: 32 bytes of `0xff`. The host program refuses it as a delegate, and the Connector
 * refuses any encrypted value account that names it as its authority, so it exists only as the
 * scope of a wildcard grant.
 */
export const SOLANA_WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY =
  'JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG' as Address<'JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG'>;

/** The tuple a delegation record is keyed by. */
export type SolanaUserDecryptionDelegationTuple = {
  /** The user granting delegated decrypt rights. */
  readonly delegator: Address;
  /** The party allowed to request user decryption of the delegator's values. */
  readonly delegate: Address;
  /**
   * The encrypted value account authority the delegation is scoped over, or
   * [`SOLANA_WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY`] for a grant across every authority of
   * the delegator's.
   */
  readonly encryptedValueAccountAuthority: Address;
};

/** The canonical delegation record address of a tuple — the address the Connector reads. */
export async function solanaUserDecryptionDelegationAddress(
  tuple: SolanaUserDecryptionDelegationTuple,
): Promise<Address> {
  const encoder = getAddressEncoder();
  const [derived] = await getProgramDerivedAddress({
    programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
    seeds: [
      SOLANA_USER_DECRYPTION_DELEGATION_SEED,
      encoder.encode(tuple.delegator),
      encoder.encode(tuple.delegate),
      encoder.encode(tuple.encryptedValueAccountAuthority),
    ],
  });
  return derived;
}

/** The wording of the wildcard-authority warning. */
export const SOLANA_WILDCARD_AUTHORITY_WARNING =
  'This delegation is scoped by the wildcard sentinel: it covers every encrypted value account ' +
  'authority the delegator has access under, now and in the future. Revoking an ' +
  'authority-specific row later will not narrow it — the wildcard row keeps authorizing until ' +
  'it is revoked itself.';

/** A warning a delegation grant deserves. Reporting it is the application's decision. */
export type SolanaDelegationWarning = {
  readonly code: 'WildcardAuthority';
  readonly message: typeof SOLANA_WILDCARD_AUTHORITY_WARNING;
};

/**
 * The warnings a delegation grant deserves. Pure: no logging here — a wallet UI, a Squads
 * proposal renderer and a script want to surface these differently.
 */
export function solanaDelegationWarnings(params: {
  readonly encryptedValueAccountAuthority: Address;
}): SolanaDelegationWarning[] {
  if (params.encryptedValueAccountAuthority === SOLANA_WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY) {
    return [{ code: 'WildcardAuthority', message: SOLANA_WILDCARD_AUTHORITY_WARNING }];
  }
  return [];
}

/** Parameters of a delegation grant. Plain addresses and a slot — renderable in a proposal UI. */
export type SolanaDelegateForUserDecryptionParameters = SolanaUserDecryptionDelegationTuple & {
  /** Pays rent if the record must be created. May differ from the delegator. */
  readonly payer: Address;
  /** The last slot the delegation is live at, inclusive. Must lie beyond the current slot. */
  readonly expirationSlot: bigint;
  /** Canonical singleton host config; defaults to the host config PDA when omitted. */
  readonly hostConfig?: Address | undefined;
  /** The record address; defaults to the canonical PDA of the tuple when omitted. */
  readonly delegationRecord?: Address | undefined;
};

/**
 * Builds the `zama_host::delegate_for_user_decryption` instruction: grants, or refreshes, the
 * delegation of the tuple. The instruction is returned unsigned — the delegator signs as part of
 * whatever transaction carries it, which is what lets a program-controlled delegator (a multisig
 * vault) sign via CPI instead of a wallet. Check [`solanaDelegationWarnings`] before offering a
 * wildcard grant to a user.
 */
export async function buildDelegateForUserDecryptionInstruction(
  params: SolanaDelegateForUserDecryptionParameters,
): Promise<Instruction> {
  const delegationRecord = params.delegationRecord ?? (await solanaUserDecryptionDelegationAddress(params));
  return getDelegateForUserDecryptionInstructionAsync({
    payer: createNoopSigner(params.payer),
    delegator: createNoopSigner(params.delegator),
    ...(params.hostConfig !== undefined ? { hostConfig: params.hostConfig } : {}),
    delegationRecord,
    delegate: params.delegate,
    encryptedValueAccountAuthority: params.encryptedValueAccountAuthority,
    expirationSlot: params.expirationSlot,
  });
}

/** Parameters of a delegation revocation: the tuple, or an explicit record address. */
export type SolanaRevokeDelegationForUserDecryptionParameters = SolanaUserDecryptionDelegationTuple & {
  /** Canonical singleton host config; defaults to the host config PDA when omitted. */
  readonly hostConfig?: Address | undefined;
  /** The record address; defaults to the canonical PDA of the tuple when omitted. */
  readonly delegationRecord?: Address | undefined;
};

/**
 * Builds the `zama_host::revoke_delegation_for_user_decryption` instruction. Revocation takes
 * effect on the Connector's next request against the record — there is no cached authorization
 * to outlive it. A wildcard row is a separate record: narrowing one authority takes revoking
 * both.
 */
export async function buildRevokeDelegationForUserDecryptionInstruction(
  params: SolanaRevokeDelegationForUserDecryptionParameters,
): Promise<Instruction> {
  const delegationRecord = params.delegationRecord ?? (await solanaUserDecryptionDelegationAddress(params));
  return getRevokeDelegationForUserDecryptionInstructionAsync({
    delegator: createNoopSigner(params.delegator),
    ...(params.hostConfig !== undefined ? { hostConfig: params.hostConfig } : {}),
    delegationRecord,
  });
}

////////////////////////////////////////////////////////////////////////////////
// Reading the record
////////////////////////////////////////////////////////////////////////////////
//
// Hand-rolled like the EncryptedValue decoder, and for the same reason: the record is written by
// the host program but read here without the framework, so the layout lives in two places by
// construction. The account is a fixed 130 bytes — the 8-byte discriminator and a 122-byte body —
// pinned byte-for-byte against the program's serializer by the Rust cross-pin fixtures.
//
// Reading a delegation before submitting is a convenience, not an authorization: the Connector
// re-checks the record against its own atomic observation on every request, and only that check
// decides. What this saves a dapp is paying for a relayer job that a revoked or expired
// delegation would terminally reject.

/** The delegation record's Anchor discriminator, `sha256("account:UserDecryptionDelegation")[..8]`. */
const DELEGATION_RECORD_DISCRIMINATOR = new Uint8Array([0x25, 0x05, 0x8b, 0x21, 0x49, 0x35, 0x01, 0xf8]);
/** Discriminator plus the fixed borsh body: three pubkeys, three u64s, a bool and the bump. */
const DELEGATION_RECORD_SIZE = 8 + 32 * 3 + 8 * 3 + 1 + 1;

/** The decoded delegation record, fields exactly as the host program wrote them. */
export interface SolanaUserDecryptionDelegationRecord {
  readonly delegator: Address;
  readonly delegate: Address;
  readonly encryptedValueAccountAuthority: Address;
  /** The last slot the delegation is live at, inclusive. Zeroed by a revocation. */
  readonly expirationSlot: bigint;
  /** Strictly monotonic across grants, re-grants and revocations. Authorizes nothing. */
  readonly delegationCounter: bigint;
  /** The slot the record last changed in; a record mutates at most once per slot. */
  readonly lastUpdateSlot: bigint;
  /** Whether the delegator revoked it. A re-grant reinstates. */
  readonly revoked: boolean;
  /** The record PDA's bump. */
  readonly bump: number;
}

const delegationRecordBodyDecoder = getStructDecoder([
  ['delegator', fixDecoderSize(getBytesDecoder(), 32)],
  ['delegate', fixDecoderSize(getBytesDecoder(), 32)],
  ['encryptedValueAccountAuthority', fixDecoderSize(getBytesDecoder(), 32)],
  ['expirationSlot', getU64Decoder()],
  ['delegationCounter', getU64Decoder()],
  ['lastUpdateSlot', getU64Decoder()],
  // A raw byte, validated below: kit's boolean decoder reads any nonzero-but-not-one byte as
  // `false`, which would show a record the Rust decoder refuses as a LIVE delegation.
  ['revoked', getU8Decoder()],
  ['bump', getU8Decoder()],
]);

/**
 * Decodes an account's raw data, discriminator included, into a delegation record.
 *
 * @param data - The account data exactly as the RPC returned it.
 * @param accountName - How to name the account in an error; the fetch wrapper passes its address.
 * @throws If the size, the discriminator or the revoked byte is not the delegation record's —
 * exactly the accounts the Rust twin decoder refuses.
 */
export function decodeSolanaUserDecryptionDelegation(
  data: Uint8Array,
  accountName: string,
): SolanaUserDecryptionDelegationRecord {
  if (data.length !== DELEGATION_RECORD_SIZE) {
    throw new Error(
      `delegation record ${accountName}: expected exactly ${DELEGATION_RECORD_SIZE} bytes (130), got ${data.length} ` +
        `— the on-chain layout has drifted from this decoder`,
    );
  }
  for (let index = 0; index < DELEGATION_RECORD_DISCRIMINATOR.length; index += 1) {
    if (data[index] !== DELEGATION_RECORD_DISCRIMINATOR[index]) {
      throw new Error(`account ${accountName} does not carry the delegation record discriminator`);
    }
  }
  const decoded = delegationRecordBodyDecoder.decode(data.slice(8));
  if (decoded.revoked !== 0 && decoded.revoked !== 1) {
    throw new Error(
      `delegation record ${accountName}: revoked byte is ${decoded.revoked}, not a borsh bool ` +
        `— the on-chain layout has drifted from this decoder`,
    );
  }
  const addressDecoder = getAddressDecoder();
  return {
    delegator: addressDecoder.decode(decoded.delegator),
    delegate: addressDecoder.decode(decoded.delegate),
    encryptedValueAccountAuthority: addressDecoder.decode(decoded.encryptedValueAccountAuthority),
    expirationSlot: decoded.expirationSlot,
    delegationCounter: decoded.delegationCounter,
    lastUpdateSlot: decoded.lastUpdateSlot,
    revoked: decoded.revoked === 1,
    bump: decoded.bump,
  };
}

/**
 * Whether the record authorizes at `slot` — the Connector's own liveness boundary: not revoked,
 * and the expiration slot has not passed (the expiration slot itself is inside the life).
 */
export function isSolanaUserDecryptionDelegationLiveAt(
  record: SolanaUserDecryptionDelegationRecord,
  slot: bigint,
): boolean {
  return !record.revoked && record.expirationSlot >= slot;
}

/** The two rows that can carry one grant; `null` where no account exists. */
export interface SolanaUserDecryptionDelegationRows {
  /** The row of the tuple's own authority. */
  readonly exact: SolanaUserDecryptionDelegationRecord | null;
  /** The delegator's wildcard row, which covers every authority of theirs. */
  readonly wildcard: SolanaUserDecryptionDelegationRecord | null;
}

/**
 * Reads both rows that could authorize the tuple — the authority-specific one and the delegator's
 * wildcard row — exactly the pair the Connector reads. Either being live (see
 * [`isSolanaUserDecryptionDelegationLiveAt`]) is what authorizes a delegated request.
 *
 * A delegation record only ever lives in a zama-host-owned account, so an account at the
 * canonical address owned by anyone else — e.g. a system account somebody created by
 * transferring lamports to the PDA — reads as absent (`null`), not as an error: no record
 * exists, and a third party must not be able to make this read throw.
 *
 * @param rpc - The Solana RPC to read through.
 * @param tuple - The delegation tuple.
 * @param config - Standard fetch passthrough, e.g. `{ commitment: 'confirmed' }`.
 * @throws If an existing zama-host-owned account does not decode as a delegation record.
 */
export async function fetchSolanaUserDecryptionDelegation(
  rpc: SolanaRpc,
  tuple: SolanaUserDecryptionDelegationTuple,
  config?: FetchAccountConfig,
): Promise<SolanaUserDecryptionDelegationRows> {
  const [exactAddress, wildcardAddress] = await Promise.all([
    solanaUserDecryptionDelegationAddress(tuple),
    solanaUserDecryptionDelegationAddress({
      ...tuple,
      encryptedValueAccountAuthority: SOLANA_WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY,
    }),
  ]);
  const [exactAccount, wildcardAccount] = await Promise.all([
    fetchEncodedAccount(rpc, exactAddress, config),
    fetchEncodedAccount(rpc, wildcardAddress, config),
  ]);
  const rowOrNull = (account: MaybeEncodedAccount, address: Address) =>
    account.exists && account.programAddress === ZAMA_HOST_PROGRAM_ADDRESS
      ? decodeSolanaUserDecryptionDelegation(account.data, address)
      : null;
  return {
    exact: rowOrNull(exactAccount, exactAddress),
    wildcard: rowOrNull(wildcardAccount, wildcardAddress),
  };
}
