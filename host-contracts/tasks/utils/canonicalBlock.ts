import type { Provider } from 'ethers';

import { formatError } from './formatError';

// Pins reads of the canonical chain (Ethereum) to one finalized block, for the snapshot exports and
// the mirror tasks of every canonical contract.

// RPC providers must disable caching so this check re-fetches the block after the state reads.
export async function assertCanonicalBlockHash(
  canonicalProvider: Provider,
  snapshot: { blockNumber: number; blockHash: string },
): Promise<void> {
  const block = await canonicalProvider.getBlock(snapshot.blockNumber);
  if (block === null || block.hash === null || block.hash !== snapshot.blockHash) {
    throw new Error(`Canonical block ${snapshot.blockNumber} is missing or its hash changed during the read; retry.`);
  }
}

// Resolves the canonical block a snapshot is pinned to. Pass `requestedBlockNumber` to pin to a
// finalized historical block (an export artifact's blockNumber) so a DAO signer can reproduce a
// snapshot byte-for-byte even after a later change on canonical. Omit it to use the latest
// finalized block.
export async function resolveCanonicalBlock(
  canonicalProvider: Provider,
  requestedBlockNumber?: number,
): Promise<{ canonicalChainId: bigint; blockNumber: number; blockHash: string }> {
  if (requestedBlockNumber !== undefined && (!Number.isSafeInteger(requestedBlockNumber) || requestedBlockNumber < 0)) {
    throw new Error('Canonical block number must be a nonnegative safe integer.');
  }

  // Handshake before the identity check so a dead or mistyped RPC URL is reported as an RPC
  // problem, not as a contract identity failure. Require finality even for an explicit block:
  // a finalized block can't be reorged out, so the exported artifact stays
  // reproducible. We resolve the full block (not just its number) to capture the hash too, which
  // uniquely identifies the read's state across reorgs — a height alone is ambiguous.
  let canonicalChainId: bigint;
  let finalizedBlock: Awaited<ReturnType<Provider['getBlock']>>;
  try {
    canonicalChainId = (await canonicalProvider.getNetwork()).chainId;
    finalizedBlock = await canonicalProvider.getBlock('finalized');
  } catch (err) {
    throw new Error(`Canonical RPC handshake failed (${formatError(err)}).`);
  }
  if (finalizedBlock === null || finalizedBlock.hash === null) {
    throw new Error('Canonical RPC returned no finalized block.');
  }
  if (requestedBlockNumber !== undefined && requestedBlockNumber > finalizedBlock.number) {
    throw new Error(
      `Canonical block ${requestedBlockNumber} is above the latest finalized block ${finalizedBlock.number}; wait for finalization.`,
    );
  }
  const block =
    requestedBlockNumber === undefined || requestedBlockNumber === finalizedBlock.number
      ? finalizedBlock
      : await canonicalProvider.getBlock(requestedBlockNumber);
  if (block === null || block.hash === null) {
    throw new Error(`Canonical RPC returned no block or hash for "${requestedBlockNumber}".`);
  }
  return { canonicalChainId, blockNumber: block.number, blockHash: block.hash };
}
