/**
 * Fork orchestration for the dual-Anvil topology.
 *
 * Submits divergent transactions to two Anvil instances at the same height,
 * advances one past finality to trigger reorg detection, and verifies which
 * branch each coprocessor observed.
 *
 * Architecture:
 *   - Anvil A (canonical): host-node, container port 8545
 *   - Anvil B (fork):      fork-anvil, container port 8546, host port 8548
 *   Both share a chain ID, mnemonic and genesis; divergence comes only from
 *   the transactions each receives after the fork point.
 *
 * This is orchestration only, and deliberately makes no claim about what the
 * fork should DO to ciphertext bytes -- that belongs to the suites, and the
 * answer changed with the RFC 019 revision. It also touches no `*_branch`
 * table or `coprocessor_settlement`: the branch schema is deprecated in v0.15
 * and dropped in v0.16, and nothing here should grow a dependency on it.
 */
import { ethers } from 'ethers';
import * as fs from 'fs';
import { sleep } from './helpers';

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

const DEFAULT_CANONICAL_RPC = process.env.CANONICAL_RPC_URL || 'http://host-node:8545';
const DEFAULT_FORK_RPC = process.env.FORK_RPC_URL || 'http://fork-anvil:8546';
/** Host-side endpoint of the same Anvil; see DEFAULT_FORK_RPC_PORT in the CLI's layout. */
export const FORK_RPC_HOST_URL = process.env.FORK_RPC_HOST_URL || 'http://localhost:8548';

export interface ForkConfig {
  canonicalRpcUrl: string;
  forkRpcUrl: string;
}

export function defaultForkConfig(): ForkConfig {
  return {
    canonicalRpcUrl: DEFAULT_CANONICAL_RPC,
    forkRpcUrl: DEFAULT_FORK_RPC,
  };
}

// ---------------------------------------------------------------------------
// Signer helpers
// ---------------------------------------------------------------------------

const DEFAULT_MNEMONIC =
  'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';

/**
 * Get a signer (wallet) connected to a specific provider, derived from the
 * shared Anvil mnemonic. Index 0 = Alice, 1 = Bob, etc.
 */
export function getSignerForProvider(
  provider: ethers.JsonRpcProvider,
  accountIndex: number = 0,
): ethers.Wallet {
  const mnemonic = process.env.MNEMONIC || DEFAULT_MNEMONIC;
  const hdNode = ethers.HDNodeWallet.fromPhrase(mnemonic, undefined, "m/44'/60'/0'/0");
  const derived = hdNode.deriveChild(accountIndex);
  return new ethers.Wallet(derived.privateKey, provider);
}

// ---------------------------------------------------------------------------
// Env file helpers
// ---------------------------------------------------------------------------

/**
 * Update a key=value entry in a .env file. If the key exists, its value is
 * replaced. If it doesn't exist, the entry is appended.
 */
export function updateEnvFile(filePath: string, key: string, value: string): void {
  let content = fs.readFileSync(filePath, 'utf-8');
  const regex = new RegExp(`^${key}=.*$`, 'm');
  if (regex.test(content)) {
    content = content.replace(regex, `${key}=${value}`);
  } else {
    content += `\n${key}=${value}\n`;
  }
  fs.writeFileSync(filePath, content, 'utf-8');
}

/**
 * Get the path to a coprocessor instance's env file.
 */
export function coprocessorEnvPath(instanceIndex: number): string {
  const stateDir = process.env.FHEVM_STATE_DIR
    || require('path').resolve(__dirname, '../../../../.fhevm');
  const fileName = instanceIndex === 0 ? 'coprocessor.env' : `coprocessor.${instanceIndex}.env`;
  return require('path').join(stateDir, 'runtime', 'env', fileName);
}

// ---------------------------------------------------------------------------
// Provider helpers
// ---------------------------------------------------------------------------

export function getCanonicalProvider(config?: ForkConfig): ethers.JsonRpcProvider {
  const c = config ?? defaultForkConfig();
  return new ethers.JsonRpcProvider(c.canonicalRpcUrl);
}

export function getForkProvider(config?: ForkConfig): ethers.JsonRpcProvider {
  const c = config ?? defaultForkConfig();
  return new ethers.JsonRpcProvider(c.forkRpcUrl);
}

/**
 * Enable or pause one-second interval mining on a chain, without ever
 * enabling automine.
 *
 * The fork suites need BOTH chains under manual control, not just the fork.
 * A canonical chain still mining on its interval breaks the collision case
 * twice over: it advances past the tip the fork was seeded from, so the two
 * branches no longer share a parent, and an interval block can consume the
 * pinned timestamp before the transaction lands, so the transaction is mined
 * under a timestamp nobody chose. Either one turns a deliberate collision test
 * into a coin flip.
 */
export async function setIntervalMining(
  provider: ethers.JsonRpcProvider,
  enabled: boolean,
): Promise<void> {
  if (!enabled) {
    await provider.send('evm_setIntervalMining', [0]);
  }
  await provider.send('evm_setAutomine', [false]);
  if (enabled) {
    await provider.send('evm_setIntervalMining', [1]);
  }
}

/** Enable or pause the fork's interval mining. Thin wrapper over setIntervalMining. */
export async function setForkMining(enabled: boolean, config?: ForkConfig): Promise<void> {
  await setIntervalMining(getForkProvider(config), enabled);
}

// ---------------------------------------------------------------------------
// Fork orchestration
// ---------------------------------------------------------------------------

/**
 * Get the current block number from both Anvil instances.
 * They should be in sync before the fork point.
 */
export async function getBlockNumbers(
  config?: ForkConfig,
): Promise<{ canonical: number; fork: number }> {
  const c = config ?? defaultForkConfig();
  const canonical = new ethers.JsonRpcProvider(c.canonicalRpcUrl);
  const fork = new ethers.JsonRpcProvider(c.forkRpcUrl);
  const [cn, fn] = await Promise.all([
    canonical.getBlockNumber(),
    fork.getBlockNumber(),
  ]);
  return { canonical: cn, fork: fn };
}

/**
 * Wait until both Anvil instances reach at least the target block number.
 */
export async function waitForBlock(
  targetBlock: number,
  config?: ForkConfig,
  timeoutMs: number = 60_000,
): Promise<void> {
  const c = config ?? defaultForkConfig();
  const canonical = new ethers.JsonRpcProvider(c.canonicalRpcUrl);
  const fork = new ethers.JsonRpcProvider(c.forkRpcUrl);
  const deadline = Date.now() + timeoutMs;

  while (Date.now() < deadline) {
    const [cn, fn] = await Promise.all([
      canonical.getBlockNumber(),
      fork.getBlockNumber(),
    ]);
    if (cn >= targetBlock && fn >= targetBlock) return;
    await sleep(1000);
  }
  throw new Error(`Timeout waiting for block ${targetBlock} on both Anvil instances`);
}

export interface DivergentTxResult {
  /** Block number on the canonical Anvil where the divergent tx landed */
  canonicalBlock: number;
  /** Block number on the fork Anvil where the divergent tx landed */
  forkBlock: number;
  /** Transaction hash on the canonical Anvil */
  canonicalTxHash: string;
  /** Transaction hash on the fork Anvil */
  forkTxHash: string;
}

/**
 * Submit divergent transactions to the two Anvil instances.
 *
 * Both transactions are submitted at approximately the same time, so they
 * end up in the same block number on their respective chains. The block
 * hashes will differ because the transaction contents differ.
 *
 * @param canonicalSigner - Signer connected to the canonical Anvil
 * @param forkSigner - Signer connected to the fork Anvil (same account, different provider)
 * @param canonicalTx - Transaction to submit on the canonical chain
 * @param forkTx - Transaction to submit on the fork chain
 */
export async function submitDivergentTransactions(
  canonicalSigner: ethers.Signer,
  forkSigner: ethers.Signer,
  canonicalTx: ethers.TransactionRequest,
  forkTx: ethers.TransactionRequest,
): Promise<DivergentTxResult> {
  // Submit both in parallel — they'll land in the next mined block on each Anvil.
  const [cReceipt, fReceipt] = await Promise.all([
    canonicalSigner.sendTransaction(canonicalTx).then((tx) => tx.wait()),
    forkSigner.sendTransaction(forkTx).then((tx) => tx.wait()),
  ]);

  if (!cReceipt || !fReceipt) {
    throw new Error('One or both divergent transactions failed');
  }

  return {
    canonicalBlock: cReceipt.blockNumber,
    forkBlock: fReceipt.blockNumber,
    canonicalTxHash: cReceipt.hash,
    forkTxHash: fReceipt.hash,
  };
}

// ---------------------------------------------------------------------------
// Finality advancement
// ---------------------------------------------------------------------------

/**
 * Advance Anvil past the finality lag by mining empty blocks.
 * Uses the `evm_mine` JSON-RPC method.
 *
 * @param provider - Provider connected to the Anvil to advance
 * @param blocksToMine - Number of blocks to mine
 */
export async function mineBlocks(
  provider: ethers.JsonRpcProvider,
  blocksToMine: number,
): Promise<void> {
  for (let i = 0; i < blocksToMine; i++) {
    await provider.send('evm_mine', []);
  }
}

/**
 * Advance the canonical Anvil past the finality lag so that the fork
 * point block becomes finalized. This triggers reorg detection on
 * coprocessor instances that switch from the fork Anvil to the
 * canonical one.
 *
 * @param finalityLag - Number of blocks after which a block is considered final
 * @param config - Fork configuration
 */
export async function advancePastFinality(
  finalityLag: number,
  config?: ForkConfig,
): Promise<void> {
  const c = config ?? defaultForkConfig();
  const canonical = new ethers.JsonRpcProvider(c.canonicalRpcUrl);
  await mineBlocks(canonical, finalityLag + 1);
}

// ---------------------------------------------------------------------------
// State synchronization
// ---------------------------------------------------------------------------

/**
 * Seed the fork Anvil from the canonical chain by *forking* it, not by copying
 * its state.
 *
 * `anvil_reset({ forking: { jsonRpcUrl, blockNumber } })` points the target at
 * the canonical chain at a chosen height and fetches pre-fork state lazily over
 * RPC. That replaces `anvil_dumpState`/`anvil_loadState`, which had two
 * failures this harness kept paying for:
 *
 * Size. Anvil refuses a large dump with `{"code":-32600,"message":"Invalid
 * request"}`, so once a stack had run long enough the fork suites stopped
 * working with an error that read as a malformed request (Consensus Defect
 * Log, L-2). Forking transfers nothing, so there is no size to exceed.
 *
 * Block-hash history. `anvil_loadState` restores *headers*, so the two chains'
 * `eth_getBlockByNumber` agreed while their EVMs did not: BLOCKHASH inside the
 * fork still returned its own original parent hashes (L-1). Since the compute
 * handle preimage includes `blockhash(block.number - 1)`, a colliding handle
 * was unconstructible and F1' could only skip. A forked Anvil serves the
 * source chain's real block hashes, so the EVMs agree and the case is testable.
 *
 * Returns the shared tip, and verifies it: a fork that came up on a different
 * tip is a silent setup fault that would surface later as "no collision",
 * which reads as a consensus finding rather than as a broken fixture.
 */
export async function seedForkFromCanonical(
  sourceRpcUrl: string,
  targetRpcUrl: string,
  resumeTargetMining: boolean = true,
  /**
   * The canonical endpoint as the *fork container* resolves it, which is not
   * necessarily how this process reaches it: a test running on the host talks
   * to `localhost:8545`, where inside the fork container `localhost` is the
   * fork itself. Defaults to the in-network name.
   */
  sourceRpcUrlAsSeenByFork: string = DEFAULT_CANONICAL_RPC,
): Promise<{ height: number; hash: string }> {
  const source = new ethers.JsonRpcProvider(sourceRpcUrl);
  const target = new ethers.JsonRpcProvider(targetRpcUrl);

  // Stop the target before replacing its history, and again after: a reset
  // restores Anvil's default mining behaviour, and these suites need both
  // chains under manual control.
  await target.send('evm_setIntervalMining', [0]);
  await target.send('evm_setAutomine', [false]);

  const tip = await source.getBlock('latest');
  if (!tip) throw new Error(`could not read the canonical tip from ${sourceRpcUrl}`);

  await target.send('anvil_reset', [
    { forking: { jsonRpcUrl: sourceRpcUrlAsSeenByFork, blockNumber: tip.number } },
  ]);
  await target.send('evm_setIntervalMining', [0]);
  await target.send('evm_setAutomine', [false]);

  const seeded = await target.getBlock('latest');
  if (!seeded || seeded.number !== tip.number || seeded.hash !== tip.hash) {
    throw new Error(
      `fork did not come up on the canonical tip: expected ${tip.number}/${tip.hash}, got ` +
        `${seeded?.number}/${seeded?.hash}. The fork container fetches pre-fork state from ` +
        `${sourceRpcUrlAsSeenByFork}; if that name does not resolve from inside it, pass the ` +
        'URL it can actually reach.',
    );
  }

  if (resumeTargetMining) {
    await target.send('evm_setIntervalMining', [1]);
  }
  return { height: tip.number, hash: tip.hash! };
}

/**
 * Initialize the fork Anvil by forking the primary Anvil at its current tip.
 * Call this after the main stack is deployed, and after any test-specific
 * contract deployments, so the fork resolves all of them.
 */
export async function initializeForkAnvil(config?: ForkConfig): Promise<void> {
  const c = config ?? defaultForkConfig();
  await seedForkFromCanonical(c.canonicalRpcUrl, c.forkRpcUrl);
}

// ---------------------------------------------------------------------------
// Handle-collision control
// ---------------------------------------------------------------------------

/**
 * Compute handles bind the formation context. `FHEVMExecutor` mints
 *
 *   keccak256(DOMAIN_SEPARATOR, op, operands, boundaryBits, ACL, chainid,
 *             blockhash(block.number - 1), block.timestamp)
 *
 * so two competing blocks mint the SAME handle exactly when they share a
 * parent and a timestamp, and different handles otherwise. That is the
 * "residual first-competing-block case" of RFC 019, and it is the only way a
 * fork can produce a handle collision at all.
 *
 * The fork suites therefore choose which case they are testing by controlling
 * the timestamp, rather than hoping for one. Anvil mines with a real clock, so
 * two branches left to themselves would collide or not depending on how fast
 * the test ran -- a flake in both directions.
 */
export async function pinNextBlockTimestamp(
  provider: ethers.JsonRpcProvider,
  timestamp: number,
): Promise<void> {
  await provider.send('evm_setNextBlockTimestamp', [timestamp]);
}

/** Mines exactly one block, for chains left with automine and interval mining off. */
export async function mineOneBlock(provider: ethers.JsonRpcProvider): Promise<number> {
  await provider.send('evm_mine', []);
  return provider.getBlockNumber();
}

/**
 * Reads BLOCKHASH(n) from inside the EVM, which is NOT the same thing as the
 * block's header hash.
 *
 * `anvil_loadState` restores headers -- `eth_getBlockByNumber` on a seeded
 * fork reports the source chain's hashes -- but it does not restore the
 * block-hash history the BLOCKHASH opcode reads. A seeded fork therefore
 * agrees with its source on every header while the EVM running on it sees
 * different parent hashes, which is invisible to any check made over RPC.
 *
 * That matters because `FHEVMExecutor` folds `blockhash(block.number - 1)`
 * into the compute-handle preimage. On a state-dump fork the preimage differs
 * from the source chain's even when the parent, timestamp, operands and opcode
 * are provably identical -- so two branches cannot mint a colliding handle,
 * and a test that assumed otherwise would read a harness limit as a protocol
 * result.
 */
export async function evmBlockhash(
  provider: ethers.JsonRpcProvider,
  blockNumber: number,
): Promise<string> {
  // PUSH2 <n>; BLOCKHASH; PUSH1 0; MSTORE; PUSH1 32; PUSH1 0; RETURN
  const probe = `0x61${blockNumber.toString(16).padStart(4, '0')}4060005260206000f3`;
  return provider.call({ data: probe });
}

/**
 * True when the two chains' EVMs agree on the parent hash the executor will
 * actually hash, not merely on what their headers report.
 */
export async function branchesShareEvmParentHash(
  parentHeight: number,
  config?: ForkConfig,
): Promise<{ agree: boolean; canonical: string; fork: string }> {
  const c = config ?? defaultForkConfig();
  const [canonical, fork] = await Promise.all([
    evmBlockhash(new ethers.JsonRpcProvider(c.canonicalRpcUrl), parentHeight),
    evmBlockhash(new ethers.JsonRpcProvider(c.forkRpcUrl), parentHeight),
  ]);
  return { agree: canonical === fork, canonical, fork };
}

/**
 * Asserts both chains are ready to mint a colliding handle: same height, and
 * therefore the same parent for the next block. Called before pinning
 * timestamps, because a shared timestamp alone does not produce a collision if
 * the parents already differ -- and a test that assumed otherwise would report
 * "no collision" as a consensus finding rather than as its own setup error.
 */
export async function requireSharedParent(config?: ForkConfig): Promise<{ height: number; parentHash: string }> {
  const c = config ?? defaultForkConfig();
  const canonical = new ethers.JsonRpcProvider(c.canonicalRpcUrl);
  const fork = new ethers.JsonRpcProvider(c.forkRpcUrl);
  const [canonicalBlock, forkBlock] = await Promise.all([canonical.getBlock('latest'), fork.getBlock('latest')]);
  if (!canonicalBlock || !forkBlock) throw new Error('could not read the tip of both chains');
  if (canonicalBlock.number !== forkBlock.number)
    throw new Error(
      `chains are at different heights (canonical ${canonicalBlock.number}, fork ${forkBlock.number}); ` +
        're-seed the fork before expecting a handle collision',
    );
  if (canonicalBlock.hash !== forkBlock.hash)
    throw new Error(
      `chains share a height but not a tip hash at ${canonicalBlock.number}; ` +
        'the next blocks would have different parents and could not mint a colliding handle',
    );
  return { height: canonicalBlock.number, parentHash: canonicalBlock.hash! };
}

// ---------------------------------------------------------------------------
// Verification helpers
// ---------------------------------------------------------------------------

/**
 * Verify that two Anvil instances have different block hashes at the same height,
 * confirming that the fork was successful.
 */
export async function verifyForkDivergence(
  blockNumber: number,
  config?: ForkConfig,
): Promise<{ canonicalHash: string; forkHash: string }> {
  const c = config ?? defaultForkConfig();
  const canonical = new ethers.JsonRpcProvider(c.canonicalRpcUrl);
  const fork = new ethers.JsonRpcProvider(c.forkRpcUrl);

  const [cBlock, fBlock] = await Promise.all([
    canonical.getBlock(blockNumber),
    fork.getBlock(blockNumber),
  ]);

  if (!cBlock || !fBlock) {
    throw new Error(`Block ${blockNumber} not found on one or both Anvil instances`);
  }

  if (cBlock.hash === fBlock.hash) {
    throw new Error(
      `Block ${blockNumber} has the same hash on both Anvils — fork did not diverge`,
    );
  }

  return {
    canonicalHash: cBlock.hash!,
    forkHash: fBlock.hash!,
  };
}
