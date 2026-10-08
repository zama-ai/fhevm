import type { Address, Instruction } from '@solana/kit';
import { getExtendLookupTableInstruction, type ExtendLookupTableInput } from '@solana-program/address-lookup-table';

/** The demo pairs the first extend with create; 20 addresses leave room for that transaction. */
export const MAX_EXTEND_ADDRESSES_PER_TRANSACTION = 20;

/**
 * Builds the `ExtendLookupTable` instructions for an address set of any size, chunked at
 * {@link MAX_EXTEND_ADDRESSES_PER_TRANSACTION} so no returned instruction can overflow the
 * transaction wire limit. Send each instruction in its own transaction (the first may share one
 * with the table's create) and confirm it before the next, in order.
 */
export function getExtendLookupTableInstructions(input: {
  readonly lookupTable: Address;
  readonly authority: ExtendLookupTableInput['authority'];
  readonly payer: ExtendLookupTableInput['payer'];
  readonly addresses: readonly Address[];
}): Instruction[] {
  const instructions: Instruction[] = [];
  for (let index = 0; index < input.addresses.length; index += MAX_EXTEND_ADDRESSES_PER_TRANSACTION) {
    instructions.push(
      getExtendLookupTableInstruction({
        address: input.lookupTable,
        authority: input.authority,
        payer: input.payer,
        addresses: input.addresses.slice(index, index + MAX_EXTEND_ADDRESSES_PER_TRANSACTION),
      }),
    );
  }
  return instructions;
}
