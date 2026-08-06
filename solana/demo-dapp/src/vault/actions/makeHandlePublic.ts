import type { Address, Instruction, ReadonlyUint8Array, TransactionSigner } from '@solana/kit';

import { getMakeTokenAccountHandlePublicInstructionAsync } from '../internal/generated/confidentialToken/instructions/makeTokenAccountHandlePublic.js';
import type { DisclosedValueKindArgs } from '../internal/generated/confidentialToken/types/disclosedValueKind.js';

export type SolanaMakeTokenAccountHandlePublicParameters = {
  readonly payer: TransactionSigner;
  readonly owner: TransactionSigner;
  readonly mint: Address;
  readonly tokenAccount: Address;
  readonly encryptedValue: Address;
  readonly hostConfig: Address;
  readonly kind: DisclosedValueKindArgs;
  readonly handle: ReadonlyUint8Array;
};

/** Builds the owner-authorized request that seals one exact token state handle publicly. */
export async function buildMakeTokenAccountHandlePublicInstruction(
  parameters: SolanaMakeTokenAccountHandlePublicParameters,
): Promise<Instruction> {
  return getMakeTokenAccountHandlePublicInstructionAsync({
    payer: parameters.payer,
    owner: parameters.owner,
    mint: parameters.mint,
    tokenAccount: parameters.tokenAccount,
    encryptedValue: parameters.encryptedValue,
    hostConfig: parameters.hostConfig,
    kind: parameters.kind,
    handle: parameters.handle,
  });
}
