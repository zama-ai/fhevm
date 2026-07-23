import { JsonRpcProvider, ethers } from 'ethers';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import {
  type BlockWindowResult,
  type ChainConfig,
  blockTimeDriftedFromFallback,
  bufferLeadSeconds,
  bufferSatisfied,
  computeBlockWindow,
  parseDurationToSeconds,
  parseIsoTimestampToEpochSeconds,
} from './blockWindow';
import { ENVIRONMENTS, SUPPORTED_ENVIRONMENTS } from './environments';
import { getRequiredEnvVar } from './loadVariables';

// Pinned inline so the util needs no typechain compile and encodes from tests without hre. Keep in sync with IProtocolConfig.sol.
export const PROPOSE_UPGRADE_ABI = [
  'function proposeCoprocessorUpgrade(uint256 proposalId, string softwareVersion, tuple(uint64 chainId, uint64 startBlock, uint64 endBlock)[] chainUpgradeWindows, uint64 gwStartBlock)',
];

export interface GatewayConfig {
  rpcUrl: string;
  fallbackBlockTimeSeconds: number;
}

export interface CoprocessorUpgradeInputs {
  environment: string;
  startEpochSeconds: number;
  durationSeconds: number;
  bufferSeconds: number;
  hostChains: ChainConfig[];
  gateway: GatewayConfig;
  proposalId: bigint;
  softwareVersion: string;
}

export interface HostReport {
  chainId: number;
  rpcUrl: string;
  result: BlockWindowResult;
  bufferOk: boolean;
  driftWarn: boolean;
}

export interface GatewayReport {
  rpcUrl: string;
  result: BlockWindowResult;
  bufferOk: boolean;
  driftWarn: boolean;
}

export interface CoprocessorUpgradeProposal {
  inputs: CoprocessorUpgradeInputs;
  host: HostReport[];
  gateway: GatewayReport;
  calldata: string;
}

// Raw string params from the task/workflow, before validation.
export interface RawCoprocessorUpgradeParams {
  environment: string;
  startTime: string;
  duration: string;
  buffer: string;
  proposalId: string;
  softwareVersion: string;
}

function resolveEnvRpc(envName: string, label: string, defaultRpcUrl?: string): string {
  const value = process.env[envName];
  if (value) return value;
  if (defaultRpcUrl) return defaultRpcUrl;
  throw new Error(`env var ${envName} (RPC URL for ${label}) is not set`);
}

export function resolveEnvironment(name: string): { chains: ChainConfig[]; gateway: GatewayConfig } {
  const def = ENVIRONMENTS[name];
  if (!def) {
    throw new Error(`--environment must be one of: ${SUPPORTED_ENVIRONMENTS.join(', ')}`);
  }
  const chains = def.chains.map((c) => ({
    chainId: c.chainId,
    rpcUrl: resolveEnvRpc(c.rpcUrlEnv, c.label, c.defaultRpcUrl),
    fallbackBlockTimeSeconds: c.fallbackBlockTimeSeconds,
  }));
  const gateway = {
    rpcUrl: resolveEnvRpc(def.gateway.rpcUrlEnv, def.gateway.label, def.gateway.defaultRpcUrl),
    fallbackBlockTimeSeconds: def.gateway.fallbackBlockTimeSeconds,
  };
  return { chains, gateway };
}

// Validate + parse raw params into typed inputs, resolving env RPC URLs. Throws on bad/missing input.
export function parseCoprocessorUpgradeInputs(raw: RawCoprocessorUpgradeParams): CoprocessorUpgradeInputs {
  const { chains: hostChains, gateway } = resolveEnvironment(raw.environment);

  const startEpochSeconds = parseIsoTimestampToEpochSeconds(raw.startTime);
  const durationSeconds = parseDurationToSeconds(raw.duration);
  const bufferSeconds = parseDurationToSeconds(raw.buffer);

  let proposalId: bigint;
  try {
    proposalId = BigInt(raw.proposalId);
  } catch {
    throw new Error('--proposal-id must be an integer (decimal or 0x-hex)');
  }
  if (proposalId <= 0n) {
    throw new Error('--proposal-id must be > 0 (contract rejects 0)');
  }

  if (raw.softwareVersion.trim() === '') {
    throw new Error('--software-version is required');
  }

  return {
    environment: raw.environment,
    startEpochSeconds,
    durationSeconds,
    bufferSeconds,
    hostChains,
    gateway,
    proposalId,
    softwareVersion: raw.softwareVersion,
  };
}

async function gatherWindows(
  inputs: CoprocessorUpgradeInputs,
): Promise<{ host: HostReport[]; gateway: GatewayReport }> {
  const host: HostReport[] = [];
  for (const chain of inputs.hostChains) {
    const provider = new JsonRpcProvider(chain.rpcUrl);
    try {
      const result = await computeBlockWindow({
        provider,
        startEpochSeconds: inputs.startEpochSeconds,
        durationSeconds: inputs.durationSeconds,
        bufferSeconds: inputs.bufferSeconds,
        fallbackBlockTimeSeconds: chain.fallbackBlockTimeSeconds,
      });
      host.push({
        chainId: chain.chainId,
        rpcUrl: chain.rpcUrl,
        result,
        bufferOk: bufferSatisfied(result, inputs.bufferSeconds),
        driftWarn: blockTimeDriftedFromFallback(result.observedBlockTimeSeconds, chain.fallbackBlockTimeSeconds),
      });
    } finally {
      provider.destroy();
    }
  }

  const gwProvider = new JsonRpcProvider(inputs.gateway.rpcUrl);
  let gateway: GatewayReport;
  try {
    const result = await computeBlockWindow({
      provider: gwProvider,
      startEpochSeconds: inputs.startEpochSeconds,
      durationSeconds: inputs.durationSeconds,
      bufferSeconds: inputs.bufferSeconds,
      fallbackBlockTimeSeconds: inputs.gateway.fallbackBlockTimeSeconds,
    });
    gateway = {
      rpcUrl: inputs.gateway.rpcUrl,
      result,
      bufferOk: bufferSatisfied(result, inputs.bufferSeconds),
      driftWarn: blockTimeDriftedFromFallback(result.observedBlockTimeSeconds, inputs.gateway.fallbackBlockTimeSeconds),
    };
  } finally {
    gwProvider.destroy();
  }

  return { host, gateway };
}

// Pure ABI encoder (no RPC); exported for decode round-trip tests.
export function encodeProposeCoprocessorUpgrade(
  inputs: CoprocessorUpgradeInputs,
  reports: { host: HostReport[]; gateway: GatewayReport },
): string {
  const iface = new ethers.Interface(PROPOSE_UPGRADE_ABI);
  const windows = reports.host.map((r) => ({
    chainId: BigInt(r.chainId),
    startBlock: BigInt(r.result.startBlock),
    endBlock: BigInt(r.result.endBlock),
  }));
  return iface.encodeFunctionData('proposeCoprocessorUpgrade', [
    inputs.proposalId,
    inputs.softwareVersion,
    windows,
    BigInt(reports.gateway.result.startBlock),
  ]);
}

// Gather windows (real RPC) + encode calldata. Buffer failures surface via `bufferViolations`, not thrown.
export async function buildCoprocessorUpgradeProposal(
  inputs: CoprocessorUpgradeInputs,
): Promise<CoprocessorUpgradeProposal> {
  const reports = await gatherWindows(inputs);
  const calldata = encodeProposeCoprocessorUpgrade(inputs, reports);
  return { inputs, host: reports.host, gateway: reports.gateway, calldata };
}

// Names the chains (and/or gateway) whose startBlock is closer to the tip than the requested DAO buffer.
export function bufferViolations(proposal: CoprocessorUpgradeProposal): string[] {
  const failures: string[] = [];
  for (const r of proposal.host) {
    if (!r.bufferOk) failures.push(`chain ${r.chainId}`);
  }
  if (!proposal.gateway.bufferOk) failures.push('gateway');
  return failures;
}

// No-DAO path (devnet/test-suite where the deployer owns ProtocolConfig): broadcast the byte-identical
// calldata with the deployer key to `target` on `--network`. Sibling of executeUpgradeProposal. Returns tx hash.
export async function executeCoprocessorUpgradeProposal(
  hre: HardhatRuntimeEnvironment,
  proposal: CoprocessorUpgradeProposal,
  target: string,
): Promise<string> {
  const deployer = new hre.ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(hre.ethers.provider);
  const tx = await deployer.sendTransaction({ to: target, data: proposal.calldata });
  await tx.wait();
  return tx.hash;
}

function isoUtc(epochSeconds: number): string {
  return new Date(epochSeconds * 1000).toISOString();
}

function formatBlockTime(seconds: number | null): string {
  return seconds === null ? '(sample failed)' : `${seconds.toFixed(2)}s`;
}

// Render an integer-second duration as a compact human string, e.g. 3600 -> "1h", 2280 -> "38m".
function formatDuration(seconds: number): string {
  const s = Math.max(0, Math.round(seconds));
  if (s < 60) return `${s}s`;
  if (s < 3600) return `${Math.round(s / 60)}m`;
  const hours = Math.floor(s / 3600);
  const minutes = Math.round((s % 3600) / 60);
  return minutes === 0 ? `${hours}h` : `${hours}h${minutes}m`;
}

function bufferShortageHint(leadSeconds: number, bufferSeconds: number): string {
  const lead = Math.round(leadSeconds);
  if (lead < 0) {
    return `startBlock is ${formatDuration(-lead)} in the past — push --start-time forward`;
  }
  const shortBy = Math.max(0, bufferSeconds - lead);
  return `lead ${formatDuration(lead)}, need ${formatDuration(bufferSeconds)}, short by ${formatDuration(shortBy)}`;
}

export function printCoprocessorUpgradeProposal(proposal: CoprocessorUpgradeProposal, target?: string): void {
  const { inputs } = proposal;
  console.log('# proposeCoprocessorUpgrade — computed block windows');
  console.log('');
  console.log(`Environment        : ${inputs.environment}`);
  console.log(`Proposal id        : ${inputs.proposalId}`);
  console.log(`Software version   : ${inputs.softwareVersion}`);
  console.log(
    `Evaluation window  : ${isoUtc(inputs.startEpochSeconds)} → ${isoUtc(inputs.startEpochSeconds + inputs.durationSeconds)} (${inputs.durationSeconds}s)`,
  );
  console.log(`DAO buffer         : ${inputs.bufferSeconds}s`);
  console.log(
    `Skew (per-chain)   : projected block timestamp minus requested --start-time; bounded by half of block-time (integer-block rounding).`,
  );
  console.log('');
  console.log('## Host chains');
  for (const report of proposal.host) {
    console.log(`  chainId=${report.chainId}`);
    console.log(`    rpc            : ${report.rpcUrl}`);
    console.log(
      `    tip            : block ${report.result.currentTipBlock} @ ${isoUtc(report.result.currentTipTimestamp)}`,
    );
    console.log(`    block time     : ${formatBlockTime(report.result.effectiveBlockTimeSeconds)}`);
    console.log(
      `    startBlock     : ${report.result.startBlock} (estimated ${isoUtc(report.result.startBlockEstimatedTimestamp)}, skew ${report.result.startSkewSeconds >= 0 ? '+' : ''}${report.result.startSkewSeconds}s)`,
    );
    console.log(
      `    endBlock       : ${report.result.endBlock} (estimated ${isoUtc(report.result.endBlockEstimatedTimestamp)})`,
    );
    if (!report.bufferOk) {
      console.log(
        `    buffer         : NO — ${bufferShortageHint(bufferLeadSeconds(report.result), inputs.bufferSeconds)}`,
      );
    }
    if (report.result.usedFallback) {
      console.log(`    WARN           : block-time sampling failed; used configured fallback`);
    }
    if (report.driftWarn) {
      console.log(`    WARN           : observed block time drifted >20% from fallback`);
    }
  }
  console.log('');
  console.log('## Gateway');
  console.log(`  rpc              : ${proposal.gateway.rpcUrl}`);
  console.log(
    `  tip              : block ${proposal.gateway.result.currentTipBlock} @ ${isoUtc(proposal.gateway.result.currentTipTimestamp)}`,
  );
  console.log(`  block time       : ${formatBlockTime(proposal.gateway.result.effectiveBlockTimeSeconds)}`);
  console.log(
    `  gwStartBlock     : ${proposal.gateway.result.startBlock} (estimated ${isoUtc(proposal.gateway.result.startBlockEstimatedTimestamp)}, skew ${proposal.gateway.result.startSkewSeconds >= 0 ? '+' : ''}${proposal.gateway.result.startSkewSeconds}s)`,
  );
  if (!proposal.gateway.bufferOk) {
    console.log(
      `  buffer           : NO — ${bufferShortageHint(bufferLeadSeconds(proposal.gateway.result), inputs.bufferSeconds)}`,
    );
  }
  if (proposal.gateway.result.usedFallback) {
    console.log(`  WARN             : block-time sampling failed; used configured fallback`);
  }
  if (proposal.gateway.driftWarn) {
    console.log(`  WARN             : observed block time drifted >20% from fallback`);
  }
  console.log('');
  console.log('## Cross-chain alignment');
  const allStarts = [
    ...proposal.host.map((r) => ({ label: `chain ${r.chainId}`, ts: r.result.startBlockEstimatedTimestamp })),
    { label: 'gateway', ts: proposal.gateway.result.startBlockEstimatedTimestamp },
  ];
  const minStart = Math.min(...allStarts.map((s) => s.ts));
  const maxStart = Math.max(...allStarts.map((s) => s.ts));
  console.log(`  start skew range : ${maxStart - minStart}s across all chains + gateway`);
  for (const s of allStarts) {
    const lag = s.ts - minStart;
    const tag = lag === 0 ? 'starts first' : `starts ${lag}s later`;
    console.log(`    ${s.label.padEnd(15)}: ${isoUtc(s.ts)} (${tag})`);
  }
  console.log('');
  console.log('## Aragon proposal action');
  console.log(
    `  target   : ${target ?? '<unresolved — set PROTOCOL_CONFIG_CONTRACT_ADDRESS or pass --use-internal-proxy-address>'}`,
  );
  console.log(`  calldata : ${proposal.calldata}`);
}
