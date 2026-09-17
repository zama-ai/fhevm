// Closes every account the host program owns so the next deployment starts from nothing.
// `close_owned_accounts` exists only in `admin-sweep` builds (the preview-env profile), so it is
// absent from the vendored IDL and the Codama client. Its wire format is pinned by
// solana/runtime-tests/tests/host_admin_mollusk.rs.
import {
  AccountRole,
  type Address,
  type Instruction,
  type Slot,
  type TransactionSigner,
  fetchEncodedAccount,
  getAddressDecoder,
} from '@solana/kit';

import { programDataAddressFor } from './bootstrap';
import type { HostDeployContext } from './send';

const CLOSE_OWNED_ACCOUNTS_DISCRIMINATOR = new Uint8Array([36, 68, 214, 114, 46, 227, 146, 228]);
/** Each target adds 33 bytes to the message; 25 keeps a batch well under the 1232-byte packet. */
const TARGETS_PER_TRANSACTION = 25;
/** `UpgradeableLoaderState::ProgramData`: u32 tag, u64 slot, then `Option<Pubkey>` as u8 + 32 bytes. */
const PROGRAM_DATA_AUTHORITY_OFFSET = 4 + 8;

export type WipeZamaHostParams = {
  /** The program's upgrade authority; receives the rent of every closed account. */
  readonly payer: TransactionSigner;
  readonly programAddress: Address;
};

/**
 * Fails unless the program is deployed and `payer` is its upgrade authority. The instruction
 * enforces the same, but a wipe that finds zero accounts never sends it, and a wrong profile or
 * cluster must not print a clean sweep.
 */
const assertUpgradeAuthority = async (context: HostDeployContext, params: WipeZamaHostParams): Promise<Address> => {
  const programData = await programDataAddressFor(params.programAddress);
  const account = await fetchEncodedAccount(context.rpc, programData, { commitment: 'confirmed' });
  if (!account.exists) throw new Error(`program ${params.programAddress} is not deployed`);
  const hasAuthority = account.data[PROGRAM_DATA_AUTHORITY_OFFSET] === 1;
  const authority = hasAuthority
    ? getAddressDecoder().decode(account.data, PROGRAM_DATA_AUTHORITY_OFFSET + 1)
    : undefined;
  if (authority !== params.payer.address) {
    throw new Error(`deployer is not the upgrade authority of ${params.programAddress} (${authority ?? 'finalized'})`);
  }
  return programData;
};

const ownedAccounts = async (
  context: HostDeployContext,
  programAddress: Address,
  minContextSlot?: Slot,
): Promise<Address[]> => {
  const accounts = await context.rpc
    .getProgramAccounts(programAddress, {
      commitment: 'confirmed',
      encoding: 'base64',
      dataSlice: { offset: 0, length: 0 },
      minContextSlot,
    })
    .send();
  return accounts.map(({ pubkey }) => pubkey);
};

/** Closes all program-owned accounts and returns how many the listing named. */
export const wipeZamaHost = async (context: HostDeployContext, params: WipeZamaHostParams): Promise<number> => {
  const programData = await assertUpgradeAuthority(context, params);
  const targets = await ownedAccounts(context, params.programAddress);
  for (let start = 0; start < targets.length; start += TARGETS_PER_TRANSACTION) {
    const batch = targets.slice(start, start + TARGETS_PER_TRANSACTION);
    const instruction: Instruction = {
      programAddress: params.programAddress,
      accounts: [
        { address: params.payer.address, role: AccountRole.WRITABLE_SIGNER },
        { address: programData, role: AccountRole.READONLY },
        ...batch.map((address) => ({ address, role: AccountRole.WRITABLE })),
      ],
      data: CLOSE_OWNED_ACCOUNTS_DISCRIMINATOR,
    };
    await context.sendTransaction(params.payer, [instruction]);
    console.log(`closed ${start + batch.length}/${targets.length} program-owned accounts`);
  }
  // Re-list no earlier than a slot observed after the last confirmation, so a pooled RPC endpoint
  // cannot report zero accounts from a node that has not yet seen the last transaction.
  const slot = await context.rpc.getSlot({ commitment: 'confirmed' }).send();
  const remaining = await ownedAccounts(context, params.programAddress, slot);
  if (remaining.length > 0) {
    throw new Error(`${remaining.length} program-owned accounts remain after the wipe: ${remaining.join(', ')}`);
  }
  return targets.length;
};
