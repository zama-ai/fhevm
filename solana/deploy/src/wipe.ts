// Closes every account the host program owns so the next deployment starts from nothing.
// `close_owned_accounts` exists only in `admin-sweep` builds (the preview-env profile), so it is
// absent from the vendored IDL and the Codama client. Its wire format is pinned by
// solana/runtime-tests/tests/host_admin_mollusk.rs.
import { AccountRole, type Address, type Instruction, type Slot, type TransactionSigner } from '@solana/kit';

import { programDataAddressFor } from './bootstrap';
import type { HostDeployContext } from './send';

const CLOSE_OWNED_ACCOUNTS_DISCRIMINATOR = new Uint8Array([36, 68, 214, 114, 46, 227, 146, 228]);
/** Each target adds 33 bytes to the message; 25 keeps a batch well under the 1232-byte packet. */
const TARGETS_PER_TRANSACTION = 25;

export type WipeZamaHostParams = {
  /** The program's upgrade authority; receives the rent of every closed account. */
  readonly payer: TransactionSigner;
  readonly programAddress: Address;
};

/**
 * Lists the program's accounts no earlier than a slot already observed, so a pooled RPC endpoint
 * cannot answer from a node that has not yet seen the last batch.
 */
const ownedAccountsSince = async (
  context: HostDeployContext,
  programAddress: Address,
  minContextSlot: Slot,
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

const confirmedSlot = (context: HostDeployContext): Promise<Slot> =>
  context.rpc.getSlot({ commitment: 'confirmed' }).send();

/** Closes all program-owned accounts and returns how many the listing named. */
export const wipeZamaHost = async (context: HostDeployContext, params: WipeZamaHostParams): Promise<number> => {
  const programData = await programDataAddressFor(params.programAddress);
  const targets = await ownedAccountsSince(context, params.programAddress, await confirmedSlot(context));
  for (let start = 0; start < targets.length; start += TARGETS_PER_TRANSACTION) {
    const instruction: Instruction = {
      programAddress: params.programAddress,
      accounts: [
        { address: params.payer.address, role: AccountRole.WRITABLE_SIGNER },
        { address: programData, role: AccountRole.READONLY },
        ...targets
          .slice(start, start + TARGETS_PER_TRANSACTION)
          .map((address) => ({ address, role: AccountRole.WRITABLE })),
      ],
      data: CLOSE_OWNED_ACCOUNTS_DISCRIMINATOR,
    };
    await context.sendTransaction(params.payer, [instruction]);
  }
  const remaining = await ownedAccountsSince(context, params.programAddress, await confirmedSlot(context));
  if (remaining.length > 0) {
    throw new Error(`${remaining.length} program-owned accounts remain after the wipe: ${remaining.join(', ')}`);
  }
  return targets.length;
};
