import {
  createNoopSigner,
  fetchEncodedAccounts,
  getAddressDecoder,
  type Address,
  type Instruction,
  type MaybeEncodedAccount,
  type ProgramDerivedAddress,
  type ReadonlyUint8Array,
  type TransactionSigner,
} from '@solana/kit';

import type { SolanaRpc } from '../encryptedStore.js';
import {
  type DelegationRecordSeeds,
  findDelegationRecordPda,
  getDelegateForUserDecryptionInstructionAsync,
  getRevokeDelegationForUserDecryptionInstructionAsync,
  getUserDecryptionDelegationDecoder,
  getUserDecryptionDelegationSize,
  USER_DECRYPTION_DELEGATION_DISCRIMINATOR,
  type UserDecryptionDelegation,
  WILDCARD_APP,
} from '@fhevm/solana-zama-host';

/**
 * Which zama-host deployment to address: the program id the chain definition names as
 * `fhevm.programs.host.address`, in base58 (`solanaHostProgram(chain)`).
 */
export type SolanaZamaHostAddressConfig = {
  readonly programAddress: Address;
};

/** The application a delegation covers: the `(program, scope)` of the encrypted stores it reaches. */
export type SolanaDelegationApplication = Pick<Readonly<DelegationRecordSeeds>, 'program' | 'scope'>;

/** The sentinel address of the wildcard row, zama-host's `WILDCARD_APP` (`0xff` × 32). */
const WILDCARD_ADDRESS = getAddressDecoder().decode(WILDCARD_APP);

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
export type SolanaUserDecryptionDelegationTuple = Readonly<DelegationRecordSeeds>;

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

export function resolvedSigner(value: SolanaSignerOrAddress): TransactionSigner {
  return typeof value === 'string' ? createNoopSigner(value) : value;
}

export function resolvedAddress(value: SolanaSignerOrAddress): Address {
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
  readonly hostConfig?: Address;
  /** The record address; defaults to the canonical PDA of the tuple when omitted. */
  readonly delegationRecord?: Address;
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
  const { payer, delegator, programAddress, ...accountsAndArgs } = params;
  return getDelegateForUserDecryptionInstructionAsync(
    { ...accountsAndArgs, payer: resolvedSigner(payer), delegator: resolvedSigner(delegator) },
    { programAddress },
  );
}

/** Parameters of a delegation revocation: the tuple whose record it revokes. */
export type SolanaRevokeDelegationForUserDecryptionParameters = Omit<
  SolanaUserDecryptionDelegationTuple,
  'delegator'
> & {
  /** The user revoking their grant (see [`SolanaSignerOrAddress`]). */
  readonly delegator: SolanaSignerOrAddress;
  /** Canonical singleton host config; defaults to the host config PDA when omitted. */
  readonly hostConfig?: Address;
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
  const { delegator, delegate, program, scope, programAddress, ...accounts } = params;
  // The host derives the record's seeds from the record's own fields and checks only the delegator,
  // so the record must come from the tuple: a record of another tuple would revoke that one.
  const tuple = { delegator: resolvedAddress(delegator), delegate, program, scope };
  return getRevokeDelegationForUserDecryptionInstructionAsync(
    {
      ...accounts,
      delegator: resolvedSigner(delegator),
      delegationRecord: (await findDelegationRecordPda(tuple, { programAddress }))[0],
    },
    { programAddress },
  );
}

////////////////////////////////////////////////////////////////////////////////
// Reading the record
////////////////////////////////////////////////////////////////////////////////
//
// The layout, size and discriminator come from the generated zama-host client. The decoder below
// adds the checks the generated one leaves out: the exact account size and the discriminator.
//
// Reading a delegation before submitting is a convenience, not an authorization: the Connector
// re-checks the record against its own atomic observation on every request, and only that check
// decides. What this saves a dapp is paying for a relayer job that a revoked or expired
// delegation would only see denied.

/** The decoded delegation record, exactly as the generated decoder reads it. */
export type SolanaUserDecryptionDelegationRecord = Readonly<UserDecryptionDelegation>;

/**
 * Decodes an account's raw data, discriminator included, into a delegation record.
 *
 * @param data - The account data exactly as the RPC returned it.
 * @param accountName - How to name the account in an error; the fetch wrapper passes its address.
 * @throws If the size or the discriminator is not the delegation record's — exactly the accounts
 * the Rust twin decoder refuses.
 */
export function decodeSolanaUserDecryptionDelegation(
  data: ReadonlyUint8Array,
  accountName: string,
): SolanaUserDecryptionDelegationRecord {
  const size = getUserDecryptionDelegationSize();
  if (data.length !== size) {
    throw new Error(
      `delegation record ${accountName}: expected exactly ${size} bytes, got ${data.length} ` +
        `— the on-chain layout has drifted from the committed zama-host IDL`,
    );
  }
  if (!USER_DECRYPTION_DELEGATION_DISCRIMINATOR.every((byte, index) => data[index] === byte)) {
    throw new Error(`account ${accountName} does not carry the delegation record discriminator`);
  }
  return getUserDecryptionDelegationDecoder().decode(data);
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
 * wildcard row — in one read at `finalized`, exactly the pair the Connector reads. Either being live (see
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
 * @param config - The `programAddress` of the deployment.
 * @throws If an existing zama-host-owned account does not decode as a delegation record of the
 * queried tuple with the canonical bump.
 */
export async function fetchSolanaUserDecryptionDelegation(
  rpc: SolanaRpc,
  tuple: SolanaUserDecryptionDelegationTuple,
  config: SolanaZamaHostAddressConfig,
): Promise<SolanaUserDecryptionDelegationRows> {
  const { programAddress } = config;
  const wildcardTuple: SolanaUserDecryptionDelegationTuple = { ...tuple, ...SOLANA_WILDCARD_APP };
  const [exactPda, wildcardPda] = await Promise.all([
    findDelegationRecordPda(tuple, { programAddress }),
    findDelegationRecordPda(wildcardTuple, { programAddress }),
  ]);
  // One RPC call, so both rows reflect one slot: two calls could straddle a revoke.
  const accounts = await fetchEncodedAccounts(rpc, [exactPda[0], wildcardPda[0]], { commitment: 'finalized' });
  const [exactAccount, wildcardAccount] = accounts;
  if (accounts.length !== 2 || exactAccount === undefined || wildcardAccount === undefined) {
    throw new Error(`getMultipleAccounts returned ${accounts.length} accounts for the two delegation rows`);
  }
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
