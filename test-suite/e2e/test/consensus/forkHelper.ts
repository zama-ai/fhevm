/**
 * Fork orchestration helper for dual-Anvil equivocation testing (E2/E3).
 *
 * This module provides utilities to:
 * - Submit divergent transactions to two Anvil instances at the same block height
 * - Advance one Anvil past finality to trigger reorg detection
 * - Verify that coprocessors observed the expected branch
 *
 * Architecture:
 *   - Anvil A (canonical): host-node on container port 8545
 *   - Anvil B (fork):      fork-anvil on container port 8546, host port 8547
 *   Both share the same chain ID, mnemonic, and genesis state.
 *   Divergence is created by submitting different transactions at the fork point.
 */
import { ethers } from 'ethers';
import * as fs from 'fs';
import { sleep } from './helpers';

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

const DEFAULT_CANONICAL_RPC = process.env.CANONICAL_RPC_URL || 'http://host-node:8545';
const DEFAULT_FORK_RPC = process.env.FORK_RPC_URL || 'http://fork-anvil:8546';

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

/** Enable or pause the fork's one-second interval mining without enabling automine. */
export async function setForkMining(enabled: boolean, config?: ForkConfig): Promise<void> {
  const fork = getForkProvider(config);
  if (!enabled) {
    await fork.send('evm_setIntervalMining', [0]);
  }
  await fork.send('evm_setAutomine', [false]);
  if (enabled) {
    await fork.send('evm_setIntervalMining', [1]);
  }
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
 * Copy the full state from one Anvil to another using anvil_dumpState/anvil_loadState.
 * This is used to ensure the fork Anvil has the same deployed contracts and
 * chain state as the primary before the fork point.
 * Set resumeTargetMining=false when the caller will mine the fork explicitly.
 *
 * Must be called AFTER the main stack is fully deployed and BEFORE submitting
 * divergent transactions.
 */
export async function syncAnvilState(
  sourceRpcUrl: string,
  targetRpcUrl: string,
  resumeTargetMining: boolean = true,
): Promise<void> {
  const source = new ethers.JsonRpcProvider(sourceRpcUrl);
  const target = new ethers.JsonRpcProvider(targetRpcUrl);

  // Stop the target while replacing its history. Resume interval mining only
  // after the new canonical snapshot is fully installed.
  await target.send('evm_setIntervalMining', [0]);
  await target.send('evm_setAutomine', [false]);

  // Dump state from the source Anvil (returns hex-encoded state blob).
  const stateHex: string = await source.send('anvil_dumpState', []);

  // Load state into the target Anvil.
  await target.send('anvil_loadState', [stateHex]);
  await target.send('evm_setAutomine', [false]);
  if (resumeTargetMining) {
    await target.send('evm_setIntervalMining', [1]);
  }
}

/**
 * Initialize the fork Anvil by copying the primary Anvil's state.
 * Call this after the main stack is deployed, and after any test-specific
 * contract deployments, so the fork Anvil has all contracts.
 */
export async function initializeForkAnvil(config?: ForkConfig): Promise<void> {
  const c = config ?? defaultForkConfig();
  await syncAnvilState(c.canonicalRpcUrl, c.forkRpcUrl);
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
