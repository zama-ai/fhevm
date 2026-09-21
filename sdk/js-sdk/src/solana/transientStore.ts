import {
  appendTransactionMessageInstructions,
  getAddressEncoder,
  getProgramDerivedAddress,
  type Address,
  type Instruction,
  type TransactionSigner,
} from '@solana/kit';

import {
  CLOSE_TRANSIENT_STORE_DISCRIMINATOR,
  getCloseTransientStoreInstruction,
} from './internal/generated/zamaHost/instructions/closeTransientStore.js';
import {
  OPEN_TRANSIENT_STORE_DISCRIMINATOR,
  getOpenTransientStoreInstruction,
} from './internal/generated/zamaHost/instructions/openTransientStore.js';

/** Public API surface: Solana app authors composing FHE transactions. */

/** Instructions sysvar (`Sysvar1nstructions…`). Every FHE instruction in the transaction must name it. */
export const INSTRUCTIONS_SYSVAR_ADDRESS =
  'Sysvar1nstructions1111111111111111111111111' as Address<'Sysvar1nstructions1111111111111111111111111'>;

const LIFECYCLE_DISCRIMINATORS = [OPEN_TRANSIENT_STORE_DISCRIMINATOR, CLOSE_TRANSIENT_STORE_DISCRIMINATOR] as const;

type TransientStoreLifecycle = {
  readonly open: Instruction;
  readonly close: Instruction;
  readonly host: Address;
};

const lifecycleByStore = new WeakMap<TransientStore, TransientStoreLifecycle>();

/** The payer's per-transaction journal PDA on zama-host. Open and close stay inside `appendTransientStoreInstructions`. */
export type TransientStore = {
  readonly address: Address;
};

/**
 * The payer's transient store: zama-host's per-transaction journal for FHE results and grants.
 * FHE transactions must open it first and close it last; `appendTransientStoreInstructions` does both.
 */
export async function prepareTransientStore(parameters: {
  readonly payer: TransactionSigner;
  /** The zama-host program id of the deployment (`solanaHostProgram(chain)`). */
  readonly host: Address;
}): Promise<TransientStore> {
  const { host, payer } = parameters;
  const [address] = await getProgramDerivedAddress({
    programAddress: host,
    seeds: [new TextEncoder().encode('transient'), getAddressEncoder().encode(payer.address)],
  });
  const accounts = { transientStore: address, instructions: INSTRUCTIONS_SYSVAR_ADDRESS };
  const transientStore: TransientStore = { address };
  lifecycleByStore.set(transientStore, {
    open: getOpenTransientStoreInstruction({ payer, ...accounts }, { programAddress: host }),
    close: getCloseTransientStoreInstruction({ ...accounts, refund: payer.address }, { programAddress: host }),
    host,
  });
  return transientStore;
}

function lifecycleOf(transientStore: TransientStore): TransientStoreLifecycle {
  const lifecycle = lifecycleByStore.get(transientStore);
  if (lifecycle === undefined) {
    throw new Error('appendTransientStoreInstructions requires the TransientStore returned by prepareTransientStore');
  }
  return lifecycle;
}

function isTransientStoreLifecycleInstruction(host: Address, instruction: Instruction): boolean {
  return (
    instruction.programAddress === host &&
    LIFECYCLE_DISCRIMINATORS.some((tag) => tag.every((byte, index) => instruction.data?.[index] === byte))
  );
}

function sandwichTransientStore(
  transientStore: TransientStore,
  instructions: readonly Instruction[],
  alreadyPresent: readonly Instruction[] = [],
): Instruction[] {
  const { open, close, host } = lifecycleOf(transientStore);
  if (
    [...alreadyPresent, ...instructions].some((instruction) => isTransientStoreLifecycleInstruction(host, instruction))
  ) {
    throw new Error(
      'FHE transaction instructions must not open or close the transient store; appendTransientStoreInstructions already does both',
    );
  }
  return [open, ...instructions, close];
}

type KitTransactionMessage = Parameters<typeof appendTransactionMessageInstructions>[1];

/** Surround `instructions` with the matching open (first) and close (last). Two arguments return the list; three append onto a Kit message. */
export function appendTransientStoreInstructions(
  transientStore: TransientStore,
  instructions: readonly Instruction[],
): Instruction[];
export function appendTransientStoreInstructions(
  transientStore: TransientStore,
  instructions: readonly Instruction[],
  message: KitTransactionMessage,
): ReturnType<typeof appendTransactionMessageInstructions>;
export function appendTransientStoreInstructions(
  transientStore: TransientStore,
  instructions: readonly Instruction[],
  message?: KitTransactionMessage,
): Instruction[] | ReturnType<typeof appendTransactionMessageInstructions> {
  const alreadyPresent =
    message === undefined ? [] : ((message as { readonly instructions?: readonly Instruction[] }).instructions ?? []);
  const sandwiched = sandwichTransientStore(transientStore, instructions, alreadyPresent);
  return message === undefined ? sandwiched : appendTransactionMessageInstructions(sandwiched, message);
}
