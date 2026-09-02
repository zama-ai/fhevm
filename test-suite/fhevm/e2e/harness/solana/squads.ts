// squads — driving the real Squads v4 multisig on the e2e validator.
//
// The program itself is a genesis fixture (`src/solana/squads.ts`); this module is the client
// side: create an m-of-n multisig and run one instruction through the full governance cycle —
// vault transaction -> proposal -> approvals -> execution. The steps are exposed separately so a
// scenario can assert the negative between them: below the threshold, execution refuses, which
// is what makes a delegation granted by a multisig a *multisig's* delegation and not any single
// member's.
//
// What the inner program sees is the same at every threshold: the *vault PDA* signs the inner
// instruction via `invoke_signed`. The quorum gates whether that signature happens at all.
//
// `@sqds/multisig` is a web3.js v1 client, while the harness lives on `@solana/kit`; the bridge
// stays inside this module — kit instructions convert at `toWeb3Instruction`, kit keypairs enter
// as their raw 64-byte encoding.

import * as multisig from "@sqds/multisig";
import { AccountRole, type Instruction } from "@solana/kit";
import { Connection, Keypair, PublicKey, TransactionInstruction, TransactionMessage } from "@solana/web3.js";

import { SQUADS_PROGRAM_ID } from "../../../src/solana/squads";

/**
 * Asserts that the RUNNING validator actually carries the Squads program.
 *
 * The scenario's skip condition is the fixtures' presence on disk, but genesis happened at the
 * validator's boot, which may predate the fetch: fixtures on disk prove nothing about the chain
 * being tested. Without this probe that mismatch surfaces as an opaque Squads-SDK failure deep
 * inside multisig creation; with it, the failure names the fix.
 */
export const assertSquadsDeployed = async (connection: Connection): Promise<void> => {
  const program = await connection.getAccountInfo(new PublicKey(SQUADS_PROGRAM_ID));
  if (program === null || !program.executable) {
    throw new Error(
      `the Squads program ${SQUADS_PROGRAM_ID} is not deployed on the running validator: ` +
        `the fixtures are on disk, but the validator booted without them — ` +
        `restart the stack (solana/scripts/e2e/clean-e2e.sh)`,
    );
  }
};

/** A web3.js signer from the harness's standard 64-byte keypair encoding (seed ‖ pubkey). */
export const web3KeypairFromBytes = (bytes: Uint8Array): Keypair => Keypair.fromSeed(bytes.subarray(0, 32));

/** A kit instruction as web3.js v1 wants it; embedded kit signers reduce to their metas. */
export const toWeb3Instruction = (instruction: Instruction): TransactionInstruction =>
  new TransactionInstruction({
    programId: new PublicKey(instruction.programAddress),
    keys: (instruction.accounts ?? []).map((account) => ({
      pubkey: new PublicKey(account.address),
      isSigner: account.role === AccountRole.READONLY_SIGNER || account.role === AccountRole.WRITABLE_SIGNER,
      isWritable: account.role === AccountRole.WRITABLE || account.role === AccountRole.WRITABLE_SIGNER,
    })),
    data: Buffer.from(instruction.data ?? new Uint8Array()),
  });

export type Squad = {
  readonly multisigPda: PublicKey;
  /** The multisig's acting identity — the delegator of the delegation scenarios. */
  readonly vaultPda: PublicKey;
};

const confirm = async (connection: Connection, signature: string): Promise<void> => {
  const latest = await connection.getLatestBlockhash("confirmed");
  const status = await connection.confirmTransaction({ signature, ...latest }, "confirmed");
  if (status.value.err !== null) {
    throw new Error(`squads transaction ${signature} failed: ${JSON.stringify(status.value.err)}`);
  }
};

/**
 * Creates an m-of-n multisig — every member with full permissions, `threshold` approvals to act —
 * and returns it with its index-0 vault. The first member creates it and pays; the treasury comes
 * from the cloned mainnet program config (creation fee 0).
 */
export const createSquad = async (
  connection: Connection,
  params: { readonly members: readonly Keypair[]; readonly threshold: number },
): Promise<Squad> => {
  const [creator] = params.members;
  if (creator === undefined) throw new Error("a multisig needs at least one member");
  const createKey = Keypair.generate();
  const [multisigPda] = multisig.getMultisigPda({ createKey: createKey.publicKey });
  const [vaultPda] = multisig.getVaultPda({ multisigPda, index: 0 });
  const [programConfigPda] = multisig.getProgramConfigPda({});
  const programConfig = await multisig.accounts.ProgramConfig.fromAccountAddress(connection, programConfigPda);

  const signature = await multisig.rpc.multisigCreateV2({
    connection,
    createKey,
    creator,
    multisigPda,
    configAuthority: null,
    timeLock: 0,
    members: params.members.map((member) => ({
      key: member.publicKey,
      permissions: multisig.types.Permissions.all(),
    })),
    threshold: params.threshold,
    treasury: programConfig.treasury,
    rentCollector: null,
  });
  await confirm(connection, signature);
  return { multisigPda, vaultPda };
};

/**
 * Puts one instruction up for the multisig's vote: the vault transaction carries it, wrapped in
 * a message whose fee payer is the vault, and the proposal opens the vote. Returns the
 * transaction index the approvals and the execution refer to.
 *
 * Rent inside the inner instruction is paid by the vault (an outer member's signature never
 * crosses the CPI boundary), so fund the vault before proposing anything that creates accounts.
 */
export const proposeThroughSquad = async (
  connection: Connection,
  squad: Squad,
  proposer: Keypair,
  instruction: Instruction,
): Promise<bigint> => {
  const account = await multisig.accounts.Multisig.fromAccountAddress(connection, squad.multisigPda);
  const transactionIndex = BigInt(account.transactionIndex.toString()) + 1n;
  const { blockhash } = await connection.getLatestBlockhash("confirmed");

  const created = await multisig.rpc.vaultTransactionCreate({
    connection,
    feePayer: proposer,
    multisigPda: squad.multisigPda,
    transactionIndex,
    creator: proposer.publicKey,
    vaultIndex: 0,
    ephemeralSigners: 0,
    transactionMessage: new TransactionMessage({
      payerKey: squad.vaultPda,
      recentBlockhash: blockhash,
      instructions: [toWeb3Instruction(instruction)],
    }),
  });
  await confirm(connection, created);

  const proposed = await multisig.rpc.proposalCreate({
    connection,
    feePayer: proposer,
    multisigPda: squad.multisigPda,
    transactionIndex,
    creator: proposer,
  });
  await confirm(connection, proposed);
  return transactionIndex;
};

/** One member's approval of the proposal. */
export const approveProposal = async (
  connection: Connection,
  squad: Squad,
  member: Keypair,
  transactionIndex: bigint,
): Promise<void> => {
  const approved = await multisig.rpc.proposalApprove({
    connection,
    feePayer: member,
    multisigPda: squad.multisigPda,
    transactionIndex,
    member,
  });
  await confirm(connection, approved);
};

/**
 * Executes the approved vault transaction: this is where the vault PDA `invoke_signed`s the
 * inner instruction. Below the threshold the program refuses — assert that with
 * `expect(...).rejects` before the quorum lands.
 */
export const executeVaultTransaction = async (
  connection: Connection,
  squad: Squad,
  member: Keypair,
  transactionIndex: bigint,
): Promise<void> => {
  const executed = await multisig.rpc.vaultTransactionExecute({
    connection,
    feePayer: member,
    multisigPda: squad.multisigPda,
    transactionIndex,
    member: member.publicKey,
  });
  await confirm(connection, executed);
};
