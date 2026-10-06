// The manifest-lifecycle fixture on the Solana host, written with the encrypted-counter specimen.
// It produces the four handles the EVM `ManifestLifecycleFixture` contract produces: `root` is a
// new count, `child` and `blocked` are increments of it in later blocks, and `independent` is a
// second owner's new count.
import { type Rpc, type Signature, type SolanaRpcApi, generateKeyPairSigner, getBase58Encoder } from '@solana/kit';

import { SOLANA_HOST_CHAIN_ID } from '../../../../solana/deploy/src/constants';
import { type Fixture, type ManifestFixturePhase, validateFixture } from '../commands/manifest-lifecycle';
import { LOCAL_SOLANA_ENDPOINTS } from './endpoints';
import { createProvisioningContext } from './provision';
import { incrementCounter, initializeCounter } from './specimens';

const OWNER_SOL = 5;

const hex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString('hex')}`;

/**
 * The finalized block that holds `signature`: its height, which the listener records as the block
 * number, and its hash.
 */
const producerBlock = async (
  rpc: Rpc<SolanaRpcApi>,
  signature: Signature,
): Promise<Pick<Fixture, 'rootBlock' | 'rootBlockHash'>> => {
  const transaction = await rpc
    .getTransaction(signature, { encoding: 'json', maxSupportedTransactionVersion: 0 })
    .send();
  if (!transaction) throw new Error(`transaction ${signature} is not finalized`);
  const block = await rpc
    .getBlock(transaction.slot, {
      maxSupportedTransactionVersion: 0,
      rewards: false,
      transactionDetails: 'none',
    })
    .send();
  if (!block) throw new Error(`slot ${transaction.slot} has no finalized block`);
  console.log(`[solana-manifest-fixture] root at slot ${transaction.slot}, block height ${block.blockHeight}`);
  return { rootBlock: Number(block.blockHeight), rootBlockHash: hex(Uint8Array.from(getBase58Encoder().encode(block.blockhash))) };
};

/** Runs one phase of the fixture on the local validator and returns the fixture so far. */
export const createSolanaManifestFixture = (): ((phase: ManifestFixturePhase) => Promise<Fixture>) => {
  const context = createProvisioningContext(LOCAL_SOLANA_ENDPOINTS.validatorRpc, LOCAL_SOLANA_ENDPOINTS.validatorWs);
  const fundedOwner = async () => {
    const owner = await generateKeyPairSigner();
    await context.fundSol(owner.address, OWNER_SOL);
    return owner;
  };
  let graph: { owner: Awaited<ReturnType<typeof fundedOwner>>; fixture: Fixture } | undefined;
  return async (phase) => {
    if (phase === 'seed graph') {
      const owner = await fundedOwner();
      const root = await initializeCounter(context, owner);
      const child = await incrementCounter(context, owner, 1n);
      graph = {
        owner,
        fixture: {
          chainId: SOLANA_HOST_CHAIN_ID.toString(),
          root: hex(root.handle),
          child: hex(child.handle),
          ...(await producerBlock(context.rpc, root.signature)),
        },
      };
      return validateFixture(graph.fixture);
    }
    if (!graph) throw new Error('the Solana manifest fixture must seed its graph first');
    const blocked = await incrementCounter(context, graph.owner, 1n);
    const independent = await initializeCounter(context, await fundedOwner());
    return validateFixture({ ...graph.fixture, blocked: hex(blocked.handle), independent: hex(independent.handle) });
  };
};
