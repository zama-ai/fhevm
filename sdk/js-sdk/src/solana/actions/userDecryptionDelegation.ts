import {
  createNoopSigner,
  fetchEncodedAccount,
  getAddressDecoder,
  getAddressEncoder,
  getProgramDerivedAddress,
  getStructDecoder,
  getU64Decoder,
  getU8Decoder,
  type Address,
  type FetchAccountConfig,
  type Instruction,
  type MaybeEncodedAccount,
  type ProgramDerivedAddress,
  type TransactionSigner,
} from '@solana/kit';

import type { SolanaRpc } from '../encryptedStore.js';
import { getDelegateForUserDecryptionInstructionAsync } from '../internal/generated/zamaHost/instructions/delegateForUserDecryption.js';
import { getRevokeDelegationForUserDecryptionInstructionAsync } from '../internal/generated/zamaHost/instructions/revokeDelegationForUserDecryption.js';
import { findHostConfigPda } from '../internal/generated/zamaHost/pdas/hostConfig.js';

/**
 * Which zama-host deployment to address: the program id the chain definition names as
 * `fhevm.programs.host.address`, in base58 (`solanaHostProgram(chain)`).
 */
export type SolanaZamaHostAddressConfig = {
  readonly programAddress: Address;
};

/** Seed of the user-decryption delegation record PDA. */
export const SOLANA_USER_DECRYPTION_DELEGATION_SEED = new TextEncoder().encode('user-decryption-delegation');

/** The application a delegation covers: the `(program, scope)` of the encrypted stores it reaches. */
export type SolanaDelegationApplication = {
  /** The application program the encrypted stores belong to. */
  readonly program: Address;
  /** The scope: an account that program owns, e.g. the mint for the token program. */
  readonly scope: Address;
};

/** `0xff` × 32, the sentinel address of the wildcard row. */
const WILDCARD_ADDRESS =
  'JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG' as Address<'JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG'>;

/**
 * The application a wildcard delegation row carries: `0xff` × 32 in both the program and the scope
 * position, as EVM's wildcard fills the contract address. No one holds the key of that address, so
 * no program is deployed at it and no encrypted store belongs to it. The host refuses a grant that
 * sets only one of the two.
 */
export const SOLANA_WILDCARD_APP: SolanaDelegationApplication = {
  program: WILDCARD_ADDRESS,
  scope: WILDCARD_ADDRESS,
};

function isWildcardApp(application: SolanaDelegationApplication): boolean {
  return application.program === WILDCARD_ADDRESS && application.scope === WILDCARD_ADDRESS;
}

/**
 * The tuple a delegation record is keyed by.
 *
 * A delegation is keyed by an application, as EVM keys it by contract address, never by an
 * encrypted value id. One grant therefore covers every value of that application that allows the
 * delegator — the balance, a transferred amount, a burned amount, and their historical handles
 * alike — for as long as the row is live. For the confidential-token program the scope is the mint,
 * so a grant covers one mint's accounts.
 */
export type SolanaUserDecryptionDelegationTuple = SolanaDelegationApplication & {
  /** The user granting delegated decrypt rights. */
  readonly delegator: Address;
  /** The party allowed to request user decryption of the delegator's values. */
  readonly delegate: Address;
};

/** The full record PDA of a tuple — address and canonical bump — under one deployment. */
async function solanaUserDecryptionDelegationPda(
  tuple: SolanaUserDecryptionDelegationTuple,
  programAddress: Address,
): Promise<ProgramDerivedAddress> {
  const encoder = getAddressEncoder();
  return await getProgramDerivedAddress({
    programAddress,
    seeds: [
      SOLANA_USER_DECRYPTION_DELEGATION_SEED,
      encoder.encode(tuple.delegator),
      encoder.encode(tuple.delegate),
      encoder.encode(tuple.program),
      encoder.encode(tuple.scope),
    ],
  });
}

/** The canonical delegation record address of a tuple — the address the Connector reads. */
export async function solanaUserDecryptionDelegationAddress(
  tuple: SolanaUserDecryptionDelegationTuple,
  config: SolanaZamaHostAddressConfig,
): Promise<Address> {
  const [derived] = await solanaUserDecryptionDelegationPda(tuple, config.programAddress);
  return derived;
}

/** The wording of the wildcard-application warning. */
export const SOLANA_WILDCARD_APP_WARNING =
  'This delegation covers every application: every encrypted value the delegator has access to, ' +
  'in any program, now and in the future. Revoking an application row later will not narrow it — ' +
  'the wildcard row keeps authorizing until it is revoked itself.';

/** A warning a delegation grant deserves. Reporting it is the application's decision. */
export type SolanaDelegationWarning = {
  readonly code: 'WildcardApp';
  readonly message: typeof SOLANA_WILDCARD_APP_WARNING;
};

/**
 * The warnings a delegation grant deserves. Pure: no logging here — a wallet UI, a Squads
 * proposal renderer and a script want to surface these differently.
 */
export function solanaDelegationWarnings(application: SolanaDelegationApplication): SolanaDelegationWarning[] {
  if (isWildcardApp(application)) {
    return [{ code: 'WildcardApp', message: SOLANA_WILDCARD_APP_WARNING }];
  }
  return [];
}

/**
 * A signing account in the form the caller can actually produce. Pass the `TransactionSigner`
 * of a wallet you hold — the kit's signing pipeline then signs the built instruction as
 * written. Pass a bare `Address` for a signer nothing at hand can sign: the instruction then
 * carries a noop placeholder in that signer meta — the form a Squads proposal renderer or a
 * CPI-signing program needs — and the validator refuses the transaction unless something else
 * supplies the signature.
 */
export type SolanaSignerOrAddress = Address | TransactionSigner;

function resolvedSigner(value: SolanaSignerOrAddress): TransactionSigner {
  return typeof value === 'string' ? createNoopSigner(value) : value;
}

function resolvedAddress(value: SolanaSignerOrAddress): Address {
  return typeof value === 'string' ? value : value.address;
}

/** Parameters of a delegation grant. Signers where held, bare addresses where only named. */
export type SolanaDelegateForUserDecryptionParameters = Omit<SolanaUserDecryptionDelegationTuple, 'delegator'> & {
  /** The user granting delegated decrypt rights (see [`SolanaSignerOrAddress`]). */
  readonly delegator: SolanaSignerOrAddress;
  /**
   * Pays rent if the record must be created. May differ from the delegator (see
   * [`SolanaSignerOrAddress`]).
   */
  readonly payer: SolanaSignerOrAddress;
  /**
   * The Unix second the delegation ends at, exclusive, on the host's clock. Must lie after the
   * host's current time.
   */
  readonly expiresAt: bigint;
  /** Canonical singleton host config; defaults to the host config PDA when omitted. */
  readonly hostConfig?: Address | undefined;
  /** The record address; defaults to the canonical PDA of the tuple when omitted. */
  readonly delegationRecord?: Address | undefined;
} & SolanaZamaHostAddressConfig;

/**
 * Builds the `zama_host::delegate_for_user_decryption` instruction: grants, or refreshes, the
 * delegation of the tuple. A wallet-held delegator passes its `TransactionSigner` and the kit's
 * signing pipeline signs the instruction as built; a delegator nothing at hand signs for — a
 * Squads proposal, a program-controlled vault signing via CPI — passes its bare address, and
 * the instruction carries a noop placeholder for the transaction that eventually signs it.
 * Check [`solanaDelegationWarnings`] before offering a wildcard grant to a user.
 */
export async function buildDelegateForUserDecryptionInstruction(
  params: SolanaDelegateForUserDecryptionParameters,
): Promise<Instruction> {
  const { programAddress } = params;
  const tuple: SolanaUserDecryptionDelegationTuple = {
    delegator: resolvedAddress(params.delegator),
    delegate: params.delegate,
    program: params.program,
    scope: params.scope,
  };
  const delegationRecord =
    params.delegationRecord ?? (await solanaUserDecryptionDelegationAddress(tuple, { programAddress }));
  // The host config is resolved here, not left to the generated builder: its default resolver
  // derives the PDA under the canonical program id even when the instruction targets another.
  const hostConfig = params.hostConfig ?? (await findHostConfigPda({ programAddress }))[0];
  return getDelegateForUserDecryptionInstructionAsync(
    {
      payer: resolvedSigner(params.payer),
      delegator: resolvedSigner(params.delegator),
      hostConfig,
      scope: params.scope,
      delegationRecord,
      delegate: params.delegate,
      program: params.program,
      expiresAt: params.expiresAt,
    },
    { programAddress },
  );
}

/** Parameters of a delegation revocation: the tuple, or an explicit record address. */
export type SolanaRevokeDelegationForUserDecryptionParameters = Omit<
  SolanaUserDecryptionDelegationTuple,
  'delegator'
> & {
  /** The user revoking their grant (see [`SolanaSignerOrAddress`]). */
  readonly delegator: SolanaSignerOrAddress;
  /** Canonical singleton host config; defaults to the host config PDA when omitted. */
  readonly hostConfig?: Address | undefined;
  /** The record address; defaults to the canonical PDA of the tuple when omitted. */
  readonly delegationRecord?: Address | undefined;
} & SolanaZamaHostAddressConfig;

/**
 * Builds the `zama_host::revoke_delegation_for_user_decryption` instruction. The delegator
 * signs the way it signed the grant: a wallet passes its `TransactionSigner`, a proposal or
 * CPI-signing program passes its bare address (see [`SolanaSignerOrAddress`]). Revocation takes
 * effect on the Connector's next request against the record — there is no cached authorization
 * to outlive it. A wildcard row is a separate record: narrowing one application takes revoking
 * both. The host's `acl_writes` pause refuses a revocation, as EVM's `whenNotPaused` does.
 */
export async function buildRevokeDelegationForUserDecryptionInstruction(
  params: SolanaRevokeDelegationForUserDecryptionParameters,
): Promise<Instruction> {
  const { programAddress } = params;
  const tuple: SolanaUserDecryptionDelegationTuple = {
    delegator: resolvedAddress(params.delegator),
    delegate: params.delegate,
    program: params.program,
    scope: params.scope,
  };
  const delegationRecord =
    params.delegationRecord ?? (await solanaUserDecryptionDelegationAddress(tuple, { programAddress }));
  // Resolved here for the same reason as in the delegate builder: the generated default is
  // pinned to the canonical program id.
  const hostConfig = params.hostConfig ?? (await findHostConfigPda({ programAddress }))[0];
  return getRevokeDelegationForUserDecryptionInstructionAsync(
    {
      delegator: resolvedSigner(params.delegator),
      hostConfig,
      delegationRecord,
    },
    { programAddress },
  );
}

////////////////////////////////////////////////////////////////////////////////
// Reading the record
////////////////////////////////////////////////////////////////////////////////
//
// Hand-rolled like the EncryptedStore decoder, and for the same reason: the record is written by
// the host program but read here without the framework, so the layout lives in two places by
// construction. The account is a fixed 161 bytes — the 8-byte discriminator and a 153-byte body —
// pinned byte-for-byte against the program's serializer by the Rust cross-pin fixtures.
//
// Reading a delegation before submitting is a convenience, not an authorization: the Connector
// re-checks the record against its own atomic observation on every request, and only that check
// decides. What this saves a dapp is paying for a relayer job that a revoked or expired
// delegation would only see denied.

/** The delegation record's Anchor discriminator, `sha256("account:UserDecryptionDelegation")[..8]`. */
const DELEGATION_RECORD_DISCRIMINATOR = new Uint8Array([0x25, 0x05, 0x8b, 0x21, 0x49, 0x35, 0x01, 0xf8]);
/** Discriminator plus the fixed borsh body: four 32-byte fields, three u64s and the bump. */
const DELEGATION_RECORD_SIZE = 8 + 32 * 4 + 8 * 3 + 1;

/** The decoded delegation record, fields exactly as the host program wrote them. */
export interface SolanaUserDecryptionDelegationRecord extends SolanaUserDecryptionDelegationTuple {
  /** The Unix second the delegation ends at, exclusive. Zeroed by a revocation. */
  readonly expiresAt: bigint;
  /** Strictly monotonic across grants, re-grants and revocations. Authorizes nothing. */
  readonly delegationCounter: bigint;
  /** The slot the record last changed in; a record mutates at most once per slot. */
  readonly lastUpdateSlot: bigint;
  /** The record PDA's bump. */
  readonly bump: number;
}

const delegationRecordBodyDecoder = getStructDecoder([
  ['delegator', getAddressDecoder()],
  ['delegate', getAddressDecoder()],
  ['program', getAddressDecoder()],
  ['scope', getAddressDecoder()],
  ['expiresAt', getU64Decoder()],
  ['delegationCounter', getU64Decoder()],
  ['lastUpdateSlot', getU64Decoder()],
  ['bump', getU8Decoder()],
]);

/**
 * Decodes an account's raw data, discriminator included, into a delegation record.
 *
 * @param data - The account data exactly as the RPC returned it.
 * @param accountName - How to name the account in an error; the fetch wrapper passes its address.
 * @throws If the size or the discriminator is not the delegation record's — exactly the accounts
 * the Rust twin decoder refuses.
 */
export function decodeSolanaUserDecryptionDelegation(
  data: Uint8Array,
  accountName: string,
): SolanaUserDecryptionDelegationRecord {
  if (data.length !== DELEGATION_RECORD_SIZE) {
    throw new Error(
      `delegation record ${accountName}: expected exactly ${DELEGATION_RECORD_SIZE} bytes, got ${data.length} ` +
        `— the on-chain layout has drifted from this decoder`,
    );
  }
  for (let index = 0; index < DELEGATION_RECORD_DISCRIMINATOR.length; index += 1) {
    if (data[index] !== DELEGATION_RECORD_DISCRIMINATOR[index]) {
      throw new Error(`account ${accountName} does not carry the delegation record discriminator`);
    }
  }
  return delegationRecordBodyDecoder.decode(data.slice(8));
}

/**
 * Whether the record authorizes at `unixTimestamp` — the Connector's own liveness boundary, taken
 * against the host's Clock: `expiresAt` is still ahead. A revoked record holds 0, so it reads like
 * one never granted.
 */
export function isSolanaUserDecryptionDelegationLiveAt(
  record: SolanaUserDecryptionDelegationRecord,
  unixTimestamp: bigint,
): boolean {
  return record.expiresAt > unixTimestamp;
}

/** The two rows that can carry one grant; `null` where no account exists. */
export interface SolanaUserDecryptionDelegationRows {
  /** The row of the tuple's own application. */
  readonly exact: SolanaUserDecryptionDelegationRecord | null;
  /** The delegator's wildcard row, which covers every application. */
  readonly wildcard: SolanaUserDecryptionDelegationRecord | null;
}

/**
 * Reads both rows that could authorize the tuple — the application's own and the delegator's
 * wildcard row — exactly the pair the Connector reads. Either being live (see
 * [`isSolanaUserDecryptionDelegationLiveAt`]) is what authorizes a delegated request.
 *
 * A delegation record only ever lives in a zama-host-owned account, so an account at the
 * canonical address owned by anyone else — e.g. a system account somebody created by
 * transferring lamports to the PDA — reads as absent (`null`), not as an error: no record
 * exists, and a third party must not be able to make this read throw.
 *
 * A host-owned account that decodes but contradicts its own address — naming another tuple, or
 * storing a non-canonical bump — throws like the layout checks do. The Connector refuses such a
 * record too (its rule: the address is not taken as proof of what the record says); only a host
 * program defect can write one, and reporting it beats presenting it as a delegation of the
 * queried tuple.
 *
 * @param rpc - The Solana RPC to read through.
 * @param tuple - The delegation tuple.
 * @param config - Standard fetch passthrough, e.g. `{ commitment: 'confirmed' }`, plus the
 * `programAddress` of the deployment.
 * @throws If an existing zama-host-owned account does not decode as a delegation record of the
 * queried tuple with the canonical bump.
 */
export async function fetchSolanaUserDecryptionDelegation(
  rpc: SolanaRpc,
  tuple: SolanaUserDecryptionDelegationTuple,
  config: FetchAccountConfig & SolanaZamaHostAddressConfig,
): Promise<SolanaUserDecryptionDelegationRows> {
  // Split off before the fetch: `programAddress` is this module's key, not RPC passthrough.
  const { programAddress, ...fetchConfig } = config;
  const wildcardTuple: SolanaUserDecryptionDelegationTuple = { ...tuple, ...SOLANA_WILDCARD_APP };
  const [exactPda, wildcardPda] = await Promise.all([
    solanaUserDecryptionDelegationPda(tuple, programAddress),
    solanaUserDecryptionDelegationPda(wildcardTuple, programAddress),
  ]);
  const [exactAccount, wildcardAccount] = await Promise.all([
    fetchEncodedAccount(rpc, exactPda[0], fetchConfig),
    fetchEncodedAccount(rpc, wildcardPda[0], fetchConfig),
  ]);
  const rowOrNull = (
    account: MaybeEncodedAccount,
    [address, bump]: ProgramDerivedAddress,
    queried: SolanaUserDecryptionDelegationTuple,
  ): SolanaUserDecryptionDelegationRecord | null => {
    if (!account.exists || account.programAddress !== programAddress) {
      return null;
    }
    const record = decodeSolanaUserDecryptionDelegation(account.data, address);
    if (
      record.delegator !== queried.delegator ||
      record.delegate !== queried.delegate ||
      record.program !== queried.program ||
      record.scope !== queried.scope
    ) {
      throw new Error(
        `delegation record ${address} names a (delegator, delegate, program, scope) tuple other than ` +
          `the one its address derives from — only the host program writes here, so one of the ` +
          `two is not what this reader believes it is`,
      );
    }
    if (record.bump !== bump) {
      throw new Error(
        `delegation record ${address}: stored bump ${record.bump} is not the canonical bump ${bump} ` +
          `of its own address`,
      );
    }
    return record;
  };
  return {
    exact: rowOrNull(exactAccount, exactPda, tuple),
    wildcard: rowOrNull(wildcardAccount, wildcardPda, wildcardTuple),
  };
}
