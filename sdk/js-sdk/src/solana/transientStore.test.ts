import { describe, expect, it } from 'vitest';
import {
  AccountRole,
  address,
  createClient,
  createNoopSigner,
  createTransactionMessage,
  createTransactionPlanExecutor,
  createTransactionPlanner,
  fillTransactionMessageProvisoryResourceLimits,
  isSolanaError,
  pipe,
  sequentialInstructionPlan,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLoadedAccountsDataSizeLimit,
  SOLANA_ERROR__INSTRUCTION_PLANS__UNEXPECTED_TRANSACTION_PLAN,
  type SolanaError,
  type Instruction,
  type Signature,
  type TransactionMessage,
} from '@solana/kit';
import {
  transactionPlanner,
  transactionPlanSendingExecutor,
  transactionPlanSigningExecutor,
} from '@solana/kit-plugin-instruction-plan';

import { INSTRUCTIONS_SYSVAR_ADDRESS, prepareTransientStore, transientStoreTransactions } from './transientStore.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';

// Pinned against the host Rust codec and derivation in transient_mollusk.rs.
const payer = createNoopSigner(address('5bV6jUfhDHCQVA1WfKBUnXUsboJgoKgkzkKcxr3joew5'));
const transientStoreAddress = address('FQtss6FsWsNugVEsasKQ4vF8urWD7gkCzjTVgEaVy6xp');
const SYSTEM_PROGRAM = address('11111111111111111111111111111111');

const body = (tag: number, size = 1): Instruction => ({
  programAddress: SYSTEM_PROGRAM,
  data: new Uint8Array(size).fill(tag),
});

// The version-1 planner zama-host's `v1TransactionSigning` uses, built from the same Kit parts: the SDK
// does not depend on zama-host. The executor records each transaction it is handed.
function recordingClient() {
  const executed: TransactionMessage[] = [];
  const executor = createTransactionPlanExecutor({
    executeTransactionMessage: async (context, message) => {
      executed.push(message);
      return { ...context, signature: '1111111111111111111111111111111111111111111111111111111111111111' as Signature };
    },
  });
  const planner = createTransactionPlanner({
    createTransactionMessage: () =>
      pipe(
        createTransactionMessage({ version: 1 }),
        (message) => setTransactionMessageFeePayerSigner(payer, message),
        (message) => setTransactionMessageLoadedAccountsDataSizeLimit(64 * 1024 * 1024, message),
        fillTransactionMessageProvisoryResourceLimits,
      ),
  });
  const sending = createClient()
    .use(transactionPlanner(planner))
    .use(transactionPlanSigningExecutor(executor))
    .use(transactionPlanSendingExecutor(executor));
  return { sending, executed, client: sending.use(transientStoreTransactions()) };
}

describe('prepareTransientStore', () => {
  it('binds open, the journal PDA, and the final refund to the same canonical transient store', async () => {
    const { client, executed } = recordingClient();
    const transientStore = await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS });
    expect(transientStore.address).toBe(transientStoreAddress);
    await client.sendFheTransaction(transientStore, []);
    const [open, close] = executed[0]!.instructions;
    expect(open!.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect([...open!.data!]).toEqual([54, 100, 76, 213, 84, 233, 196, 94]);
    expect(open!.accounts?.map((meta) => [meta.address, meta.role])).toEqual([
      [payer.address, AccountRole.WRITABLE_SIGNER],
      [transientStoreAddress, AccountRole.WRITABLE],
      [INSTRUCTIONS_SYSVAR_ADDRESS, AccountRole.READONLY],
      [SYSTEM_PROGRAM, AccountRole.READONLY],
    ]);
    expect(open!.accounts?.[0]).toHaveProperty('signer', payer);
    expect(close!.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect([...close!.data!]).toEqual([107, 197, 28, 166, 51, 173, 83, 189]);
    expect(close!.accounts?.map((meta) => [meta.address, meta.role])).toEqual([
      [INSTRUCTIONS_SYSVAR_ADDRESS, AccountRole.READONLY],
      [transientStoreAddress, AccountRole.WRITABLE],
      [payer.address, AccountRole.WRITABLE],
    ]);
  });

  it('keeps a custom host consistent across derivation and lifecycle validation', async () => {
    const { client, executed } = recordingClient();
    const host = address('SysvarC1ock11111111111111111111111111111111');
    const transientStore = await prepareTransientStore({ payer, host });
    expect(transientStore.address).not.toBe(transientStoreAddress);
    await client.sendFheTransaction(transientStore, []);
    const sandwiched = executed[0]!.instructions;
    expect(sandwiched.map((ix) => ix.programAddress)).toEqual([host, host]);
    for (const ix of sandwiched)
      expect(ix.accounts?.some((account) => account.address === transientStore.address)).toBe(true);
    await expect(client.sendFheTransaction(transientStore, sandwiched)).rejects.toThrow(
      'must not open or close the transient store',
    );
    expect(executed).toHaveLength(1);
  });

  it('reuses the address for the same sponsor and derives another for a different one', async () => {
    const transientStore = await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS });
    // A new transaction with the same sponsor deliberately reuses the address, not its contents.
    expect((await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS })).address).toBe(
      transientStore.address,
    );
    const other = await prepareTransientStore({
      payer: createNoopSigner(INSTRUCTIONS_SYSVAR_ADDRESS),
      host: ZAMA_HOST_PROGRAM_ADDRESS,
    });
    expect(other.address).not.toBe(transientStoreAddress);
  });
});

describe('transientStoreTransactions', () => {
  it('signs and sends the sandwich as one version 1 transaction that ends with the close', async () => {
    const { client, executed } = recordingClient();
    const transientStore = await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS });
    for (const instructions of [
      [body(1), body(2)],
      [body(2), body(1)],
    ]) {
      executed.length = 0;
      await client.signFheTransaction(transientStore, instructions);
      await client.sendFheTransaction(transientStore, instructions);
      expect(executed).toHaveLength(2);
      for (const message of executed) {
        expect(message.version).toBe(1);
        // v1 carries the compute limit in the message config, so no ComputeBudget instruction is added.
        expect(message.instructions.slice(1, -1)).toEqual(instructions);
        expect([...message.instructions.at(-1)!.data!.slice(0, 8)]).toEqual([107, 197, 28, 166, 51, 173, 83, 189]);
      }
    }
  });

  it('fails at planning when the sandwich does not fit one transaction, before anything is signed or sent', async () => {
    const { client, executed } = recordingClient();
    const transientStore = await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS });
    // Three 1,500-byte instructions exceed the 4,096-byte v1 limit together but fit one per transaction.
    // The planner then splits the sandwich and leaves the close in a later transaction than the open;
    // single-transaction sign and send refuse it before executing any of it.
    const oversized = [body(1, 1500), body(2, 1500), body(3, 1500)];
    for (const send of [client.signFheTransaction, client.sendFheTransaction]) {
      const error = await send(transientStore, oversized).catch((caught: unknown) => caught);
      expect(isSolanaError(error, SOLANA_ERROR__INSTRUCTION_PLANS__UNEXPECTED_TRANSACTION_PLAN)).toBe(true);
      // Non-divisible: a bundle-aware executor would also have to land the pieces atomically.
      expect(
        (error as SolanaError<typeof SOLANA_ERROR__INSTRUCTION_PLANS__UNEXPECTED_TRANSACTION_PLAN>).context
          .transactionPlan,
      ).toMatchObject({ kind: 'sequential', divisible: false });
    }
    expect(executed).toHaveLength(0);
  });

  it('rejects a body that opens or closes the transient store itself', async () => {
    const { client, executed } = recordingClient();
    const transientStore = await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS });
    await client.sendFheTransaction(transientStore, [body(1)]);
    const [open, , close] = executed[0]!.instructions;
    for (const nested of [
      [open!, body(1)],
      [body(1), close!, body(2)],
    ]) {
      await expect(client.sendFheTransaction(transientStore, nested)).rejects.toThrow(
        'must not open or close the transient store',
      );
    }
    expect(executed).toHaveLength(1);
  });

  it('rejects a hand-built object that was never prepared', async () => {
    const { client } = recordingClient();
    await expect(client.sendFheTransaction({ address: transientStoreAddress }, [])).rejects.toThrow(
      'requires the TransientStore returned by prepareTransientStore',
    );
  });

  it('exposes no composed transaction a caller could extend past the close', async () => {
    const { sending, client } = recordingClient();
    const transientStore = await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS });
    expect(Object.keys(client).filter((key) => !(key in sending))).toEqual([
      'signFheTransaction',
      'sendFheTransaction',
    ]);
    // Never called: the assertion is the type error.
    void (() =>
      // @ts-expect-error The body is a list of instructions, never a composed `InstructionPlan`.
      client.sendFheTransaction(transientStore, sequentialInstructionPlan([body(1)])));
  });
});
