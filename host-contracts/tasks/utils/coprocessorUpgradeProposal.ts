import { JsonRpcProvider, ethers } from 'ethers';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import {
  type BlockWindowResult,
  type ChainConfig,
  type ChainTip,
  blockTimeDriftedFromFallback,
  bufferLeadSeconds,
  bufferSatisfied,
  computeBlockWindow,
  parseDurationToSeconds,
  parseIsoTimestampToEpochSeconds,
  readChainTip,
} from './blockWindow';
import { assertContractMatchesVersionPrefix } from './contractVersion';
import { ENVIRONMENTS, SUPPORTED_ENVIRONMENTS } from './environments';
import { formatError } from './formatError';
import { getRequiredEnvVar } from './loadVariables';

// Pinned inline so the util needs no typechain compile and encodes from tests without hre. Keep in sync with IProtocolConfig.sol.
export const PROPOSE_UPGRADE_ABI = [
  'function proposeCoprocessorUpgrade(uint256 proposalId, string softwareVersion, tuple(uint64 chainId, uint64 startBlock, uint64 endBlock)[] chainUpgradeWindows, uint64 gwStartBlock)',
];

export interface GatewayConfig {
  // Short name for reports, e.g. 'gateway-testnet'; shown in place of the RPC URL.
  label: string;
  rpcUrl: string;
  // Chain id the RPC must report. Unset only for the local stack, which skips the check.
  chainId?: number;
}

export interface CoprocessorUpgradeInputs {
  environment: string;
  startEpochSeconds: number;
  durationSeconds: number;
  bufferSeconds: number;
  hostChains: ChainConfig[];
  gateway: GatewayConfig;
  // Host chain holding the canonical ProtocolConfig.
  canonicalChainId: number;
  proposalId: bigint;
  softwareVersion: string;
}

export interface HostReport {
  chainId: number;
  // Short name for reports, e.g. 'sepolia'; shown in place of the RPC URL.
  label: string;
  rpcUrl: string;
  result: BlockWindowResult;
  bufferOk: boolean;
  driftWarn: boolean;
}

// The Gateway mints blocks on demand, so gwStartBlock is pinned to the current tip instead of projected.
export interface GatewayReport extends ChainTip {
  rpcUrl: string;
  startBlock: number;
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

// One LOCAL_HOST_CHAINS entry: a ChainConfig whose label and finality floor the harness may leave out.
type LocalHostChainJson = Omit<ChainConfig, 'label' | 'minWindowSeconds'> & {
  label?: string;
  minWindowSeconds?: number;
};

export function resolveEnvironment(name: string): {
  canonicalChainId: number;
  chains: ChainConfig[];
  gateway: GatewayConfig;
} {
  const def = ENVIRONMENTS[name];
  if (!def) {
    throw new Error(`--environment must be one of: ${SUPPORTED_ENVIRONMENTS.join(', ')}`);
  }
  // Local e2e chains are scenario-defined (1..N), so the test passes them as JSON, ProtocolConfig chain first.
  // They have no label or finality floor unless the harness sets one.
  const scenarioChains = name === 'local' ? process.env.LOCAL_HOST_CHAINS : undefined;
  const chains: ChainConfig[] = scenarioChains
    ? (JSON.parse(scenarioChains) as LocalHostChainJson[]).map((c) => ({
        ...c,
        label: c.label ?? `chain-${c.chainId}`,
        minWindowSeconds: c.minWindowSeconds ?? 0,
      }))
    : def.chains.map((c) => ({
        chainId: c.chainId,
        label: c.label,
        rpcUrl: resolveEnvRpc(c.rpcUrlEnv, c.label, c.defaultRpcUrl),
        fallbackBlockTimeSeconds: c.fallbackBlockTimeSeconds,
        minWindowSeconds: c.minWindowSeconds,
      }));
  if (chains.length === 0) {
    throw new Error('LOCAL_HOST_CHAINS is empty');
  }
  const canonicalChainId = scenarioChains ? chains[0].chainId : def.canonicalChainId;
  const gateway: GatewayConfig = {
    label: def.gateway.label,
    rpcUrl: resolveEnvRpc(def.gateway.rpcUrlEnv, def.gateway.label, def.gateway.defaultRpcUrl),
    chainId: def.gateway.chainId,
  };
  return { canonicalChainId, chains, gateway };
}

// Release tags the coprocessor's version parser reads as intended: vMAJOR.MINOR.PATCH, optional pre-release suffix.
// Anything else parses as zeros on the coprocessor and fails the cutover gate after the DAO vote.
const RELEASE_TAG = /^v?\d+\.\d+\.\d+(?:-[0-9A-Za-z.]+)?$/;

export function isReleaseTag(version: string): boolean {
  return RELEASE_TAG.test(version);
}

// Validate + parse raw params into typed inputs, resolving env RPC URLs. Throws on bad/missing input.
export function parseCoprocessorUpgradeInputs(raw: RawCoprocessorUpgradeParams): CoprocessorUpgradeInputs {
  const { canonicalChainId, chains: hostChains, gateway } = resolveEnvironment(raw.environment);

  const startEpochSeconds = parseIsoTimestampToEpochSeconds(raw.startTime);
  const durationSeconds = parseDurationToSeconds(raw.duration);
  const bufferSeconds = parseDurationToSeconds(raw.buffer);
  assertWindowCoversFinality(durationSeconds, hostChains);

  let proposalId: bigint;
  try {
    proposalId = BigInt(raw.proposalId);
  } catch {
    throw new Error('--proposal-id must be an integer (decimal or 0x-hex)');
  }
  if (proposalId <= 0n) {
    throw new Error('--proposal-id must be > 0 (contract rejects 0)');
  }

  const softwareVersion = raw.softwareVersion.trim();
  if (!isReleaseTag(softwareVersion)) {
    throw new Error(
      `--software-version "${raw.softwareVersion}" is not a release tag (expected vMAJOR.MINOR.PATCH, e.g. v0.15.0)`,
    );
  }

  return {
    environment: raw.environment,
    startEpochSeconds,
    durationSeconds,
    bufferSeconds,
    hostChains,
    gateway,
    canonicalChainId,
    proposalId,
    softwareVersion,
  };
}

// A window shorter than a chain's floor ends before the chain finalizes startBlock, so the dry run
// never starts and the attempt rolls back.
export function assertWindowCoversFinality(durationSeconds: number, chains: ChainConfig[]): void {
  for (const chain of chains) {
    const floor = chain.minWindowSeconds;
    if (durationSeconds < floor) {
      throw new Error(
        `--duration ${durationSeconds}s is below the minimum window of chain ${chain.chainId} (${floor}s): ` +
          'the dry run cannot start before the chain finalizes startBlock',
      );
    }
  }
}

function chainLabel(chain: ChainConfig): string {
  return `${chain.label} (chain ${chain.chainId})`;
}

export type NetworkReader = Pick<JsonRpcProvider, 'getNetwork'>;

// The registry names the chain id and the operator names the RPC URL: refuse a URL that serves another chain.
export async function assertRpcChainId(provider: NetworkReader, expectedChainId: number, label: string): Promise<void> {
  let reported: bigint;
  try {
    reported = (await provider.getNetwork()).chainId;
  } catch (err) {
    throw new Error(`RPC for ${label} did not answer eth_chainId (${formatError(err)})`);
  }
  if (reported !== BigInt(expectedChainId)) {
    throw new Error(
      `RPC for ${label} reports chain id ${reported}, expected ${expectedChainId}; check its RPC URL env var`,
    );
  }
}

// RPC URLs can carry an API key in the path or query, and the runbook has the operator copy this output.
export function redactRpcUrl(url: string): string {
  try {
    const parsed = new URL(url);
    return `${parsed.protocol}//${parsed.host}`;
  } catch {
    return '<unparsable url>';
  }
}

async function gatherWindows(
  inputs: CoprocessorUpgradeInputs,
): Promise<{ host: HostReport[]; gateway: GatewayReport }> {
  const nowEpochSeconds = Math.floor(Date.now() / 1000);
  const host: HostReport[] = [];
  for (const chain of inputs.hostChains) {
    const provider = new JsonRpcProvider(chain.rpcUrl);
    try {
      await assertRpcChainId(provider, chain.chainId, chainLabel(chain));
      const result = await computeBlockWindow({
        provider,
        startEpochSeconds: inputs.startEpochSeconds,
        durationSeconds: inputs.durationSeconds,
        bufferSeconds: inputs.bufferSeconds,
        fallbackBlockTimeSeconds: chain.fallbackBlockTimeSeconds,
        nowEpochSeconds,
      });
      host.push({
        chainId: chain.chainId,
        label: chain.label,
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
    if (inputs.gateway.chainId !== undefined) {
      await assertRpcChainId(gwProvider, inputs.gateway.chainId, inputs.gateway.label);
    } else {
      console.warn('WARN: gateway chain id not checked (local stack).');
    }
    const tip = await readChainTip(gwProvider);
    if (tip.tipBlock === 0) {
      throw new Error(
        `Gateway tip is block 0 (contract rejects gwStartBlock=0); check ${redactRpcUrl(inputs.gateway.rpcUrl)}`,
      );
    }
    gateway = { rpcUrl: inputs.gateway.rpcUrl, ...tip, startBlock: tip.tipBlock };
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
    BigInt(reports.gateway.startBlock),
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

// Names the host chains whose startBlock is closer to the tip than the requested DAO buffer.
export function bufferViolations(proposal: CoprocessorUpgradeProposal): string[] {
  return proposal.host.filter((r) => !r.bufferOk).map((r) => `chain ${r.chainId}`);
}

// No-DAO path (devnet/test-suite where the deployer owns ProtocolConfig): broadcast the byte-identical
// calldata with the deployer key to `target` on `--network`. Sibling of executeUpgradeProposal. Returns tx hash.
export async function executeCoprocessorUpgradeProposal(
  hre: HardhatRuntimeEnvironment,
  proposal: CoprocessorUpgradeProposal,
  target: string,
): Promise<string> {
  await assertBroadcastTarget(hre, proposal.inputs, target);
  const deployer = new hre.ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(hre.ethers.provider);
  const tx = await deployer.sendTransaction({ to: target, data: proposal.calldata });
  await tx.wait();
  return tx.hash;
}

// --network and --environment are independent flags: refuse to broadcast unless the connected chain is the
// environment's ProtocolConfig chain and the target answers as a ProtocolConfig.
export async function assertBroadcastTarget(
  hre: HardhatRuntimeEnvironment,
  inputs: CoprocessorUpgradeInputs,
  target: string,
): Promise<void> {
  const connected = (await hre.ethers.provider.getNetwork()).chainId;
  if (connected !== BigInt(inputs.canonicalChainId)) {
    throw new Error(
      `--network is chain ${connected} but the ${inputs.environment} ProtocolConfig lives on chain ${inputs.canonicalChainId}; refusing to broadcast`,
    );
  }
  await assertContractMatchesVersionPrefix(hre, target, 'ProtocolConfig');
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

export function printCoprocessorUpgradeProposal(proposal: CoprocessorUpgradeProposal, target: string): void {
  const { inputs } = proposal;
  console.log('# proposeCoprocessorUpgrade — computed block windows');
  console.log('');
  console.log(`Environment        : ${inputs.environment}`);
  console.log(`Proposal id        : ${inputs.proposalId}`);
  console.log(`Software version   : ${inputs.softwareVersion}`);
  console.log(
    `Evaluation window  : ${isoUtc(inputs.startEpochSeconds)} → ${isoUtc(
      inputs.startEpochSeconds + inputs.durationSeconds,
    )} (${inputs.durationSeconds}s)`,
  );
  console.log(`DAO buffer         : ${inputs.bufferSeconds}s`);
  console.log(
    `Skew (per-chain)   : projected block timestamp minus requested --start-time; bounded by half of block-time (integer-block rounding).`,
  );
  console.log('');
  console.log('## Host chains');
  for (const report of proposal.host) {
    console.log(`  chainId=${report.chainId} (${report.label})`);
    console.log(`    rpc            : ${redactRpcUrl(report.rpcUrl)}`);
    console.log(
      `    tip            : block ${report.result.currentTipBlock} @ ${isoUtc(report.result.currentTipTimestamp)} (${
        report.result.tipAgeSeconds
      }s behind the clock)`,
    );
    console.log(`    block time     : ${formatBlockTime(report.result.effectiveBlockTimeSeconds)}`);
    console.log(
      `    startBlock     : ${report.result.startBlock} (estimated ${isoUtc(
        report.result.startBlockEstimatedTimestamp,
      )}, skew ${report.result.startSkewSeconds >= 0 ? '+' : ''}${report.result.startSkewSeconds}s)`,
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
  console.log(`  rpc              : ${inputs.gateway.label} ${redactRpcUrl(proposal.gateway.rpcUrl)}`);
  console.log(`  tip              : block ${proposal.gateway.tipBlock} @ ${isoUtc(proposal.gateway.tipTimestamp)}`);
  console.log(
    `  gwStartBlock     : ${proposal.gateway.startBlock} (pinned to the tip: the Gateway mints blocks on demand, a projected block may never exist)`,
  );
  console.log('');
  console.log('## Cross-chain alignment');
  const allStarts = proposal.host.map((r) => ({
    label: `chain ${r.chainId}`,
    ts: r.result.startBlockEstimatedTimestamp,
  }));
  const minStart = Math.min(...allStarts.map((s) => s.ts));
  const maxStart = Math.max(...allStarts.map((s) => s.ts));
  console.log(`  start skew range : ${maxStart - minStart}s across host chains`);
  for (const s of allStarts) {
    const lag = s.ts - minStart;
    const tag = lag === 0 ? 'starts first' : `starts ${lag}s later`;
    console.log(`    ${s.label.padEnd(15)}: ${isoUtc(s.ts)} (${tag})`);
  }
  console.log('');
  console.log('## Aragon proposal action');
  console.log('  target:', target);
  console.log('  calldata:', proposal.calldata);
}
