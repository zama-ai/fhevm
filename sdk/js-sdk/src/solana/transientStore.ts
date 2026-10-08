import {
  extendClient,
  nonDivisibleSequentialInstructionPlan,
  type Address,
  type ClientWithTransactionSending,
  type ClientWithTransactionSigning,
  type ExtendedClient,
  type Instruction,
  type SequentialInstructionPlan,
  type SuccessfulSingleTransactionPlanResult,
  type TransactionPlanResultContext,
  type TransactionSigner,
} from '@solana/kit';

import {
  findTransientStorePda,
  CLOSE_TRANSIENT_STORE_DISCRIMINATOR,
  getCloseTransientStoreInstruction,
  getOpenTransientStoreInstruction,
  OPEN_TRANSIENT_STORE_DISCRIMINATOR,
} from '@fhevm/solana-zama-host';

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

/** The payer's per-transaction journal PDA on zama-host. Open and close stay inside `transientStoreTransactions`. */
export type TransientStore = {
  readonly address: Address;
};

/**
 * The payer's transient store: zama-host's per-transaction journal for FHE results and grants.
 * FHE transactions must open it first and close it last; `transientStoreTransactions` does both.
 */
export async function prepareTransientStore(parameters: {
  readonly payer: TransactionSigner;
  /** The zama-host program id of the deployment (`solanaHostProgram(chain)`). */
  readonly host: Address;
}): Promise<TransientStore> {
  const { host, payer } = parameters;
  const [address] = await findTransientStorePda({ payer: payer.address }, { programAddress: host });
  const accounts = { transientStore: address, instructions: INSTRUCTIONS_SYSVAR_ADDRESS };
  const transientStore: TransientStore = { address };
  lifecycleByStore.set(transientStore, {
    open: getOpenTransientStoreInstruction({ payer, ...accounts }, { programAddress: host }),
    close: getCloseTransientStoreInstruction({ ...accounts, payer: payer.address }, { programAddress: host }),
    host,
  });
  return transientStore;
}

function lifecycleOf(transientStore: TransientStore): TransientStoreLifecycle {
  const lifecycle = lifecycleByStore.get(transientStore);
  if (lifecycle === undefined) {
    throw new Error('An FHE transaction requires the TransientStore returned by prepareTransientStore');
  }
  return lifecycle;
}

function isTransientStoreLifecycleInstruction(host: Address, instruction: Instruction): boolean {
  return (
    instruction.programAddress === host &&
    LIFECYCLE_DISCRIMINATORS.some((tag) => tag.every((byte, index) => instruction.data?.[index] === byte))
  );
}

// zama-host requires the close to be the transaction's last instruction. Single-transaction sign and
// send refuse a sandwich the planner had to split, and nothing else can join its plan. Non-divisible
// marks it atomic for any executor that does run split plans.
function fheTransactionPlan(
  transientStore: TransientStore,
  instructions: readonly Instruction[],
): SequentialInstructionPlan {
  const { open, close, host } = lifecycleOf(transientStore);
  if (instructions.some((instruction) => isTransientStoreLifecycleInstruction(host, instruction))) {
    throw new Error('FHE transaction instructions must not open or close the transient store; the client does both');
  }
  return nonDivisibleSequentialInstructionPlan([open, ...instructions, close]);
}

type TransactionConfig = Parameters<ClientWithTransactionSending['sendTransaction']>[1];

/** What `transientStoreTransactions` adds to a Kit client. */
export type TransientStoreTransactions<
  TSigned extends TransactionPlanResultContext,
  TSent extends TransactionPlanResultContext,
> = {
  readonly signFheTransaction: (
    transientStore: TransientStore,
    instructions: readonly Instruction[],
    config?: TransactionConfig,
  ) => Promise<SuccessfulSingleTransactionPlanResult<TSigned>>;
  readonly sendFheTransaction: (
    transientStore: TransientStore,
    instructions: readonly Instruction[],
    config?: TransactionConfig,
  ) => Promise<SuccessfulSingleTransactionPlanResult<TSent>>;
};

/**
 * Kit plugin: `signFheTransaction` and `sendFheTransaction` put `instructions` between the transient
 * store's open and close and sign or send them as one transaction through the client's planner.
 * A sandwich that does not fit one transaction fails at planning, before anything is signed.
 */
export function transientStoreTransactions() {
  return <TSigned extends TransactionPlanResultContext, TSent extends TransactionPlanResultContext, T extends object>(
    client: T & ClientWithTransactionSigning<TSigned> & ClientWithTransactionSending<TSent>,
  ): ExtendedClient<T, TransientStoreTransactions<TSigned, TSent>> =>
    extendClient<T, TransientStoreTransactions<TSigned, TSent>>(client, {
      signFheTransaction: async (
        transientStore: TransientStore,
        instructions: readonly Instruction[],
        config?: TransactionConfig,
      ) => await client.signTransaction(fheTransactionPlan(transientStore, instructions), config),
      sendFheTransaction: async (
        transientStore: TransientStore,
        instructions: readonly Instruction[],
        config?: TransactionConfig,
      ) => await client.sendTransaction(fheTransactionPlan(transientStore, instructions), config),
    });
}
