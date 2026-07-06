// Pure helpers for computing block windows from wall-clock parameters.
import { type JsonRpcProvider } from 'ethers';

// Sample size for measuring average block time from the chain tip; larger smooths variance but costs more RPC.
const BLOCK_TIME_SAMPLE_SIZE = 1000;

// Drift threshold: warn if observed differs from fallback by more than this fraction (0.2 = 20%).
const BLOCK_TIME_DRIFT_WARN_FRACTION = 0.2;

export interface ChainConfig {
  chainId: number;
  rpcUrl: string;
  // Configured fallback block time in seconds, used only if RPC sampling fails.
  fallbackBlockTimeSeconds: number;
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
}

export interface BlockWindowResult {
  currentTipBlock: number;
  currentTipTimestamp: number;
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

// Read tip + sample back BLOCK_TIME_SAMPLE_SIZE blocks to estimate average block time. Null if chain too young.
async function sampleBlockTime(provider: JsonRpcProvider): Promise<{
  tipBlock: number;
  tipTimestamp: number;
  averageBlockTimeSeconds: number | null;
}> {
  const tipBlock = await provider.getBlockNumber();
  const tip = await provider.getBlock(tipBlock);
  if (!tip) {
    throw new Error(`Failed to fetch tip block #${tipBlock}`);
  }
  const sampleStart = Math.max(1, tipBlock - BLOCK_TIME_SAMPLE_SIZE);
  if (sampleStart >= tipBlock) {
    return { tipBlock, tipTimestamp: Number(tip.timestamp), averageBlockTimeSeconds: null };
  }
  const sampleStartBlock = await provider.getBlock(sampleStart);
  if (!sampleStartBlock) {
    return { tipBlock, tipTimestamp: Number(tip.timestamp), averageBlockTimeSeconds: null };
  }
  const elapsedBlocks = tipBlock - sampleStart;
  const elapsedSeconds = Number(tip.timestamp) - Number(sampleStartBlock.timestamp);
  if (elapsedSeconds <= 0 || elapsedBlocks <= 0) {
    return { tipBlock, tipTimestamp: Number(tip.timestamp), averageBlockTimeSeconds: null };
  }
  return {
    tipBlock,
    tipTimestamp: Number(tip.timestamp),
    averageBlockTimeSeconds: elapsedSeconds / elapsedBlocks,
  };
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

  const sample = await sampleBlockTime(input.provider);
  const effectiveBlockTimeSeconds = sample.averageBlockTimeSeconds ?? input.fallbackBlockTimeSeconds;
  const usedFallback = sample.averageBlockTimeSeconds === null;

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

export function bufferLeadSeconds(result: BlockWindowResult): number {
  return (result.startBlock - result.currentTipBlock) * result.effectiveBlockTimeSeconds;
}

// True iff startBlock is at least bufferSeconds away from the tip in wall-clock terms.
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
