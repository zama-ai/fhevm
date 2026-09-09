// specimens — the typed drivers for the two specimen consumer programs the live scenarios stand
// their encrypted values up through: encrypted-counter (the smallest complete consumer) and
// dep-chain (the 32-step dependent-chain load shape).
//
// RFC 035 proves every value authority to be a PDA of its program on create, and only that
// authority can write the value or make a handle public, so nothing here signs `fhe_execute` from
// a wallet: the wallet is the specimen's *owner* — it pays, and the program allows it on every
// handle it writes — while the program's authority PDA signs the host CPI. The Codama clients are
// rendered into `./internal/generated/{encryptedCounter,depChain}` by the SDK's `codegen:solana`
// script from the committed IDLs.

import { getAddressEncoder, type Address, type Instruction, type TransactionSigner } from "@solana/kit";

import { solanaEncryptedValueAccountAddress } from "@sdk-src/solana/encryptedValueAccount.js";

import { getExtendInstructionAsync, getInitializeInstructionAsync as getInitializeChainInstructionAsync } from "./internal/generated/depChain/instructions/index.js";
import { findChainAuthorityPda, findChainPda } from "./internal/generated/depChain/pdas/index.js";
import { DEP_CHAIN_PROGRAM_ADDRESS } from "./internal/generated/depChain/programAddress.js";
import {
  getIncrementInstructionAsync,
  getInitializeInstructionAsync as getInitializeCounterInstructionAsync,
} from "./internal/generated/encryptedCounter/instructions/index.js";
import { findCounterAuthorityPda, findCounterPda } from "./internal/generated/encryptedCounter/pdas/index.js";
import { ENCRYPTED_COUNTER_PROGRAM_ADDRESS } from "./internal/generated/encryptedCounter/programAddress.js";
import { ZAMA_HOST_PROGRAM_ADDRESS } from "./internal/generated/zamaHost/programAddress.js";
import { currentHandle } from "./fhe-vertical";
import { hostConfigAddress, zamaEventAuthorityAddress, type SolanaProvisioningContext } from "./provision";

const addressBytes = (value: Address): Uint8Array => new Uint8Array(getAddressEncoder().encode(value));

// Byte-identical to the specimens' `encrypted_*_label` functions.
const COUNT_LABEL = new TextEncoder().encode("count___________________________");
const TAIL_LABEL = new TextEncoder().encode("tail____________________________");

/** The host's per-execution step ceiling, which `dep_chain::extend` promises exactly. */
export const MAX_CHAIN_LINKS = 32;

/** One specimen-owned encrypted value: who owns it, which program controls it, where it lives. */
export type SpecimenValue = {
  /** The wallet the program allows on every handle it writes — the user-decrypt identity. */
  readonly owner: Address;
  /** The program's authority PDA: the value's `encrypted_value_account_authority`. */
  readonly authority: Address;
  /** The value's `EncryptedValue` account. */
  readonly encryptedValue: Address;
};

/** A specimen value right after a write, with the handle that write installed. */
export type SpecimenHandle = {
  readonly value: SpecimenValue;
  readonly handle: Uint8Array;
};

const specimenValue = async (
  program: Address,
  owner: Address,
  state: Address,
  authority: Address,
  label: Uint8Array,
): Promise<SpecimenValue> => ({
  owner,
  authority,
  // The specimen's application is `(program, scope = its state PDA)`; the value hangs off the
  // authority PDA under that scope.
  encryptedValue: await solanaEncryptedValueAccountAddress(addressBytes(ZAMA_HOST_PROGRAM_ADDRESS), {
    program: addressBytes(program),
    encryptedValueAccountAuthority: addressBytes(authority),
    scope: addressBytes(state),
    label,
  }),
});

/** `owner`'s count under the encrypted-counter specimen. */
export const counterValue = async (owner: Address): Promise<SpecimenValue> => {
  const [counter] = await findCounterPda({ owner });
  const [counterAuthority] = await findCounterAuthorityPda({ counter });
  return specimenValue(ENCRYPTED_COUNTER_PROGRAM_ADDRESS, owner, counter, counterAuthority, COUNT_LABEL);
};

/** `owner`'s chain tail under the dep-chain specimen. */
export const chainValue = async (owner: Address): Promise<SpecimenValue> => {
  const [chain] = await findChainPda({ owner });
  const [chainAuthority] = await findChainAuthorityPda({ chain });
  return specimenValue(DEP_CHAIN_PROGRAM_ADDRESS, owner, chain, chainAuthority, TAIL_LABEL);
};

const hostAccounts = async () => ({
  hostConfig: await hostConfigAddress(),
  zamaEventAuthority: await zamaEventAuthorityAddress(),
});

/**
 * `encrypted_counter::initialize`: creates `owner`'s counter with its count trivially encrypted
 * to 0 and the owner allowed on that handle. Built separately from sending so a multisig can
 * propose it with a bare-address owner (`createNoopSigner`) — the vault PDA signs at execution.
 */
export const buildInitializeCounterInstruction = async (owner: TransactionSigner): Promise<Instruction> =>
  getInitializeCounterInstructionAsync({
    owner,
    countValue: (await counterValue(owner.address)).encryptedValue,
    ...(await hostAccounts()),
  });

/** `encrypted_counter::increment`: adds `amount` to the count; the owner is allowed on the new handle. */
export const buildIncrementCounterInstruction = async (owner: TransactionSigner, amount: bigint): Promise<Instruction> =>
  getIncrementInstructionAsync({
    owner,
    countValue: (await counterValue(owner.address)).encryptedValue,
    ...(await hostAccounts()),
    amount,
  });

/**
 * Sends one specimen write signed by the owner and returns the handle it installed. Preflight is
 * skipped: the result-handle entropy reads the SlotHashes sysvar via `sol_get_sysvar`, which real
 * execution populates but preflight simulation does not.
 */
const writeSpecimenValue = async (
  context: SolanaProvisioningContext,
  owner: TransactionSigner,
  value: SpecimenValue,
  instruction: Instruction,
): Promise<SpecimenHandle> => {
  await context.sendTransaction(owner, [instruction], { skipPreflight: true });
  return { value, handle: await currentHandle(context, value.encryptedValue) };
};

/** Creates `owner`'s counter at 0. */
export const initializeCounter = async (
  context: SolanaProvisioningContext,
  owner: TransactionSigner,
): Promise<SpecimenHandle> =>
  writeSpecimenValue(context, owner, await counterValue(owner.address), await buildInitializeCounterInstruction(owner));

/** Adds `amount` to `owner`'s count (the update form of a persistent output). */
export const incrementCounter = async (
  context: SolanaProvisioningContext,
  owner: TransactionSigner,
  amount: bigint,
): Promise<SpecimenHandle> =>
  writeSpecimenValue(
    context,
    owner,
    await counterValue(owner.address),
    await buildIncrementCounterInstruction(owner, amount),
  );

/** Creates `owner`'s chain with its tail at 0. */
export const initializeChain = async (
  context: SolanaProvisioningContext,
  owner: TransactionSigner,
): Promise<SpecimenHandle> => {
  const value = await chainValue(owner.address);
  return writeSpecimenValue(
    context,
    owner,
    value,
    await getInitializeChainInstructionAsync({ owner, tailValue: value.encryptedValue, ...(await hostAccounts()) }),
  );
};

/**
 * `dep_chain::extend`: adds `amount` to the tail `links` times as ONE execution of `links`
 * dependent steps, each reading the previous step's transient result — the shape the coprocessor
 * cannot parallelize. The tail grows by `links * amount`.
 */
export const extendChain = async (
  context: SolanaProvisioningContext,
  owner: TransactionSigner,
  params: { readonly links: number; readonly amount: bigint },
): Promise<SpecimenHandle> => {
  const value = await chainValue(owner.address);
  return writeSpecimenValue(
    context,
    owner,
    value,
    await getExtendInstructionAsync({
      owner,
      tailValue: value.encryptedValue,
      ...(await hostAccounts()),
      links: params.links,
      amount: params.amount,
    }),
  );
};
