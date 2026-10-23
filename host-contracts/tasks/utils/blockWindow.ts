// Pure helpers for computing block windows from wall-clock parameters.
import { type JsonRpcProvider } from 'ethers';

// Sample size for measuring average block time from the chain tip; larger smooths variance but costs more RPC.
const BLOCK_TIME_SAMPLE_SIZE = 1000;

// Drift threshold: warn if observed differs from fallback by more than this fraction (0.2 = 20%).
const BLOCK_TIME_DRIFT_WARN_FRACTION = 0.2;

// A host tip older than this many block times means the RPC is behind, and every lead projected
// from it would be inflated by the lag. The floor keeps sub-second chains tolerant of ordinary lag.
const STALE_TIP_BLOCK_TIMES = 10;
const STALE_TIP_MIN_SECONDS = 60;

export interface ChainConfig {
  chainId: number;
  rpcUrl: string;
  // Configured fallback block time in seconds, used only if RPC sampling fails.
  fallbackBlockTimeSeconds: number;
  // Shortest --duration the chain's finality allows: the dry run cannot start before startBlock is
  // finalized. 0 means no floor (chains passed by the e2e harness through LOCAL_HOST_CHAINS).
  minWindowSeconds: number;
}

export interface BlockWindowInput {
  // Caller owns the connection (provider is not destroyed here).
  provider: JsonRpcProvider;
  // UNIX seconds.
  startEpochSeconds: number;
  durationSeconds: number;
  // Required lead time between "now" and startBlock, in seconds.
  bufferSeconds: number;
  // Used only if RPC sampling fails.
  fallbackBlockTimeSeconds: number;
  // Wall-clock "now" in UNIX seconds. The caller reads it once per run so every chain is measured
  // against the same instant.
  nowEpochSeconds: number;
}

export interface BlockWindowResult {
  currentTipBlock: number;
  currentTipTimestamp: number;
  // Wall clock at computation time; the DAO lead is measured from it, not from the tip.
  computedAtEpochSeconds: number;
  // How far the tip timestamp lags the wall clock.
  tipAgeSeconds: number;
  // Null if sampling failed.
  observedBlockTimeSeconds: number | null;
  // Observed value if available, else fallback.
  effectiveBlockTimeSeconds: number;
  startBlock: number;
  endBlock: number;
  startBlockEstimatedTimestamp: number;
  endBlockEstimatedTimestamp: number;
  // estimatedStartTs - requestedStartTs; bounded by half of effectiveBlockTimeSeconds (integer-block rounding).
  startSkewSeconds: number;
  usedFallback: boolean;
}

export type BlockReader = Pick<JsonRpcProvider, 'getBlockNumber' | 'getBlock'>;

export interface ChainTip {
  tipBlock: number;
  tipTimestamp: number;
}

export async function readChainTip(provider: BlockReader): Promise<ChainTip> {
  const tipBlock = await provider.getBlockNumber();
  const tip = await provider.getBlock(tipBlock);
  if (!tip) {
    throw new Error(`Failed to fetch tip block #${tipBlock}`);
  }
  return { tipBlock, tipTimestamp: Number(tip.timestamp) };
}

// Read tip + sample back BLOCK_TIME_SAMPLE_SIZE blocks to estimate average block time. Null if chain too young.
async function sampleBlockTime(
  provider: JsonRpcProvider,
): Promise<ChainTip & { averageBlockTimeSeconds: number | null }> {
  const { tipBlock, tipTimestamp } = await readChainTip(provider);
  const sampleStart = Math.max(1, tipBlock - BLOCK_TIME_SAMPLE_SIZE);
  if (sampleStart >= tipBlock) {
    return { tipBlock, tipTimestamp, averageBlockTimeSeconds: null };
  }
  let sampleStartBlock: Awaited<ReturnType<JsonRpcProvider['getBlock']>>;
  try {
    sampleStartBlock = await provider.getBlock(sampleStart);
  } catch {
    // A failed historical read falls back to the configured block time, like a missing block.
    sampleStartBlock = null;
  }
  if (!sampleStartBlock) {
    return { tipBlock, tipTimestamp, averageBlockTimeSeconds: null };
  }
  const elapsedBlocks = tipBlock - sampleStart;
  const elapsedSeconds = tipTimestamp - Number(sampleStartBlock.timestamp);
  if (elapsedSeconds <= 0 || elapsedBlocks <= 0) {
    return { tipBlock, tipTimestamp, averageBlockTimeSeconds: null };
  }
  return { tipBlock, tipTimestamp, averageBlockTimeSeconds: elapsedSeconds / elapsedBlocks };
}

// Project a wall-clock instant to a block number on the same chain (rounds to nearest block).
function projectBlockNumber(
  tipBlock: number,
  tipTimestamp: number,
  targetTimestamp: number,
  blockTimeSeconds: number,
): number {
  const deltaSeconds = targetTimestamp - tipTimestamp;
  return tipBlock + Math.round(deltaSeconds / blockTimeSeconds);
}

// Compute the block window for one chain. Throws on invalid durationSeconds; buffer-violation surfaced via result, not thrown.
export async function computeBlockWindow(input: BlockWindowInput): Promise<BlockWindowResult> {
  if (input.durationSeconds <= 0) {
    throw new Error('durationSeconds must be positive');
  }

  const now = input.nowEpochSeconds;
  const sample = await sampleBlockTime(input.provider);
  const effectiveBlockTimeSeconds = sample.averageBlockTimeSeconds ?? input.fallbackBlockTimeSeconds;
  const usedFallback = sample.averageBlockTimeSeconds === null;

  // The window is projected from the tip the RPC reports. If that tip is older than the chain's
  // normal block cadence allows, the RPC is behind the chain and the projection would place the
  // window earlier than intended, so stop here rather than propose from a stale view.
  const tipAgeSeconds = now - sample.tipTimestamp;
  const maxTipAgeSeconds = Math.max(STALE_TIP_MIN_SECONDS, STALE_TIP_BLOCK_TIMES * effectiveBlockTimeSeconds);
  if (tipAgeSeconds > maxTipAgeSeconds) {
    throw new Error(
      `chain tip is ${tipAgeSeconds}s old (block ${sample.tipBlock}, allowed ${Math.round(maxTipAgeSeconds)}s): the RPC is behind, use another endpoint`,
    );
  }

  const startBlock = projectBlockNumber(
    sample.tipBlock,
    sample.tipTimestamp,
    input.startEpochSeconds,
    effectiveBlockTimeSeconds,
  );
  const endTimestamp = input.startEpochSeconds + input.durationSeconds;
  const endBlock = projectBlockNumber(sample.tipBlock, sample.tipTimestamp, endTimestamp, effectiveBlockTimeSeconds);
  if (endBlock <= startBlock) {
    throw new Error(
      `duration too short for chain block time: projected startBlock=${startBlock} endBlock=${endBlock} (effective block time ${effectiveBlockTimeSeconds}s, duration ${input.durationSeconds}s)`,
    );
  }

  const startBlockEstimatedTimestamp =
    sample.tipTimestamp + Math.round((startBlock - sample.tipBlock) * effectiveBlockTimeSeconds);
  const endBlockEstimatedTimestamp =
    sample.tipTimestamp + Math.round((endBlock - sample.tipBlock) * effectiveBlockTimeSeconds);

  return {
    currentTipBlock: sample.tipBlock,
    currentTipTimestamp: sample.tipTimestamp,
    computedAtEpochSeconds: now,
    tipAgeSeconds,
    observedBlockTimeSeconds: sample.averageBlockTimeSeconds,
    effectiveBlockTimeSeconds,
    startBlock,
    endBlock,
    startBlockEstimatedTimestamp,
    endBlockEstimatedTimestamp,
    startSkewSeconds: startBlockEstimatedTimestamp - input.startEpochSeconds,
    usedFallback,
  };
}

// Wall-clock seconds until the chain is expected to reach startBlock. Measured from "now", not from
// the tip, so an RPC that lags does not inflate the lead.
export function bufferLeadSeconds(result: BlockWindowResult): number {
  return result.startBlockEstimatedTimestamp - result.computedAtEpochSeconds;
}

// True iff startBlock is at least bufferSeconds away in wall-clock terms.
export function bufferSatisfied(result: BlockWindowResult, bufferSeconds: number): boolean {
  return bufferLeadSeconds(result) >= bufferSeconds;
}

// True iff observed block time drifted significantly from the configured fallback (signals congestion).
export function blockTimeDriftedFromFallback(observed: number | null, fallback: number): boolean {
  if (observed === null) {
    return false;
  }
  const driftFraction = Math.abs(observed - fallback) / fallback;
  return driftFraction > BLOCK_TIME_DRIFT_WARN_FRACTION;
}

// Parse a duration like "30m", "2h", "1d", or a bare integer (seconds). Throws on malformed input.
export function parseDurationToSeconds(input: string): number {
  const trimmed = input.trim();
  if (/^\d+$/.test(trimmed)) {
    return parseInt(trimmed, 10);
  }
  const match = /^(\d+)([smhd])$/.exec(trimmed);
  if (!match) {
    throw new Error(`Unrecognized duration "${input}". Use formats like "30s", "30m", "2h", "1d".`);
  }
  const value = parseInt(match[1], 10);
  const unit = match[2];
  switch (unit) {
    case 's':
      return value;
    case 'm':
      return value * 60;
    case 'h':
      return value * 3600;
    case 'd':
      return value * 86400;
    default:
      throw new Error(`Unrecognized duration unit "${unit}"`);
  }
}

// Parse an ISO 8601 timestamp into UNIX seconds. Throws on invalid input.
export function parseIsoTimestampToEpochSeconds(input: string): number {
  const ms = Date.parse(input);
  if (Number.isNaN(ms)) {
    throw new Error(`Unparsable timestamp "${input}". Use ISO 8601 (e.g. 2026-07-01T12:00:00Z).`);
  }
  return Math.floor(ms / 1000);
}
