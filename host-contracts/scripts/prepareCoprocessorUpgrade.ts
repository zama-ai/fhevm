#!/usr/bin/env -S node --enable-source-maps --no-warnings

/* eslint-disable no-console */
//
// Computes calldata for `proposeCoprocessorUpgrade(...)`. Run with `--help`
// for the full flag reference; see scripts/README.md for the workflow path.
// To add a new chain or environment, edit scripts/utils/environments.ts.
import { JsonRpcProvider, ethers } from 'ethers';
import { parseArgs } from 'node:util';

import {
  type BlockWindowResult,
  type ChainConfig,
  blockTimeDriftedFromFallback,
  bufferLeadSeconds,
  bufferSatisfied,
  computeBlockWindow,
  parseDurationToSeconds,
  parseIsoTimestampToEpochSeconds,
} from './utils/blockWindow';
import { ENVIRONMENTS, SUPPORTED_ENVIRONMENTS } from './utils/environments';

// Pinned ABI fragment for proposeCoprocessorUpgrade — kept inline so the script
// has no compile-time dependency on the typechain bindings (which require a
// prior `hardhat compile`). Keep this in sync with IProtocolConfig.sol.
const PROPOSE_UPGRADE_ABI = [
  'function proposeCoprocessorUpgrade(uint256 proposalId, string softwareVersion, tuple(uint64 chainId, uint64 startBlock, uint64 endBlock)[] chainUpgradeWindows, uint64 gwStartBlock, uint16 ciphertextVersion)',
];

interface GatewayConfig {
  rpcUrl: string;
  fallbackBlockTimeSeconds: number;
}

interface ScriptInputs {
  environment: string;
  startEpochSeconds: number;
  durationSeconds: number;
  bufferSeconds: number;
  hostChains: ChainConfig[];
  gateway: GatewayConfig;
  proposalId: bigint;
  softwareVersion: string;
  ciphertextVersion: number;
}

interface HostReport {
  chainId: number;
  rpcUrl: string;
  result: BlockWindowResult;
  bufferOk: boolean;
  driftWarn: boolean;
}

interface GatewayReport {
  rpcUrl: string;
  result: BlockWindowResult;
  bufferOk: boolean;
  driftWarn: boolean;
}

function fail(message: string): never {
  console.error(`error: ${message}`);
  process.exit(1);
}

function resolveEnvRpc(envName: string, label: string, defaultRpcUrl?: string): string {
  const value = process.env[envName];
  if (value) return value;
  if (defaultRpcUrl) return defaultRpcUrl;
  fail(`env var ${envName} (RPC URL for ${label}) is not set`);
}

function resolveEnvironment(name: string): { chains: ChainConfig[]; gateway: GatewayConfig } {
  const def = ENVIRONMENTS[name];
  if (!def) {
    fail(`--environment must be one of: ${SUPPORTED_ENVIRONMENTS.join(', ')}`);
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

function printHelp(): void {
  const envList = SUPPORTED_ENVIRONMENTS.join(' | ');
  const rows: string[] = [];
  for (const [name, def] of Object.entries(ENVIRONMENTS)) {
    const chainBits = def.chains.map((c) => `${c.label} (chainId=${c.chainId}, $${c.rpcUrlEnv})`).join(', ');
    rows.push(`    ${name.padEnd(8)}${chainBits}, gateway $${def.gateway.rpcUrlEnv}`);
  }

  console.log(`Usage: prepare-coprocessor-upgrade --environment ENV \\
                                --start-time ISO --duration D --buffer D \\
                                --proposal-id N --software-version V \\
                                --ciphertext-version N

Computes block windows for \`proposeCoprocessorUpgrade(...)\` across the host
chains + gateway of the chosen environment, then ABI-encodes ready-to-submit
DAO calldata.

Required flags:
  --environment         One of: ${envList}. Selects the chain set + the
                        RPC env-var names the script reads. See env table below.
  --start-time          ISO 8601 timestamp (UTC) the evaluation window should
                        begin. Example: 2026-07-01T12:00:00Z
  --duration            Window length. Format: 30s, 30m, 2h, 1d, or a bare
                        integer (seconds).
  --buffer              DAO lead time required between "now" and startBlock.
                        Same format as --duration. The run exits non-zero if
                        any chain's startBlock is closer to its tip than this.
  --proposal-id         Positive integer (decimal or 0x-hex). Contract rejects 0.
  --software-version    Coprocessor software version string, e.g. v0.14.0.
  --ciphertext-version  Integer in [0, 32767] (fits Postgres SMALLINT).

Other:
  -h, --help            Show this message and exit.

Environments and the env vars they consume (set these before running):
${rows.join('\n')}

Exit codes:
  0  All chains satisfied the buffer; calldata is safe to submit.
  1  Input validation failed (missing flag, unknown environment, RPC env unset).
  2  DAO buffer violated on one or more chains. Calldata is still printed for
     inspection but should NOT be submitted.

See scripts/README.md for the full reference.`);
}

function parseInputs(): ScriptInputs {
  const { values } = parseArgs({
    options: {
      environment: { type: 'string' },
      'start-time': { type: 'string' },
      duration: { type: 'string' },
      buffer: { type: 'string' },
      'proposal-id': { type: 'string' },
      'software-version': { type: 'string' },
      'ciphertext-version': { type: 'string' },
      help: { type: 'boolean', short: 'h' },
    },
    strict: true,
    allowPositionals: false,
  });

  if (values.help) {
    printHelp();
    process.exit(0);
  }

  const req = (key: string): string => {
    const v = values[key as keyof typeof values];
    if (typeof v !== 'string' || v === '') {
      fail(`--${key} is required`);
    }
    return v;
  };

  const environment = req('environment');
  const { chains: hostChains, gateway } = resolveEnvironment(environment);

  const startEpochSeconds = parseIsoTimestampToEpochSeconds(req('start-time'));
  const durationSeconds = parseDurationToSeconds(req('duration'));
  const bufferSeconds = parseDurationToSeconds(req('buffer'));

  let proposalId: bigint;
  try {
    proposalId = BigInt(req('proposal-id'));
  } catch {
    fail(`--proposal-id must be an integer (decimal or 0x-hex)`);
  }
  if (proposalId <= 0n) {
    fail('--proposal-id must be > 0 (contract rejects 0)');
  }

  const softwareVersion = req('software-version');

  const ciphertextVersion = Number(req('ciphertext-version'));
  if (!Number.isInteger(ciphertextVersion) || ciphertextVersion < 0 || ciphertextVersion > 0x7fff) {
    fail('--ciphertext-version must be an integer in [0, 32767]');
  }

  return {
    environment,
    startEpochSeconds,
    durationSeconds,
    bufferSeconds,
    hostChains,
    gateway,
    proposalId,
    softwareVersion,
    ciphertextVersion,
  };
}

async function gatherWindows(inputs: ScriptInputs): Promise<{ host: HostReport[]; gateway: GatewayReport }> {
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

function printSummary(
  inputs: ScriptInputs,
  reports: { host: HostReport[]; gateway: GatewayReport },
  calldata: string,
): void {
  console.log('# proposeCoprocessorUpgrade — computed block windows');
  console.log('');
  console.log(`Environment        : ${inputs.environment}`);
  console.log(`Proposal id        : ${inputs.proposalId}`);
  console.log(`Software version   : ${inputs.softwareVersion}`);
  console.log(`Ciphertext version : ${inputs.ciphertextVersion}`);
  console.log(
    `Evaluation window  : ${isoUtc(inputs.startEpochSeconds)} → ${isoUtc(inputs.startEpochSeconds + inputs.durationSeconds)} (${inputs.durationSeconds}s)`,
  );
  console.log(`DAO buffer         : ${inputs.bufferSeconds}s`);
  console.log(
    `Skew (per-chain)   : projected block timestamp minus requested --start-time; bounded by half of block-time (integer-block rounding).`,
  );
  console.log('');
  console.log('## Host chains');
  for (const report of reports.host) {
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
  console.log(`  rpc              : ${reports.gateway.rpcUrl}`);
  console.log(
    `  tip              : block ${reports.gateway.result.currentTipBlock} @ ${isoUtc(reports.gateway.result.currentTipTimestamp)}`,
  );
  console.log(`  block time       : ${formatBlockTime(reports.gateway.result.effectiveBlockTimeSeconds)}`);
  console.log(
    `  gwStartBlock     : ${reports.gateway.result.startBlock} (estimated ${isoUtc(reports.gateway.result.startBlockEstimatedTimestamp)}, skew ${reports.gateway.result.startSkewSeconds >= 0 ? '+' : ''}${reports.gateway.result.startSkewSeconds}s)`,
  );
  if (!reports.gateway.bufferOk) {
    console.log(
      `  buffer           : NO — ${bufferShortageHint(bufferLeadSeconds(reports.gateway.result), inputs.bufferSeconds)}`,
    );
  }
  if (reports.gateway.result.usedFallback) {
    console.log(`  WARN             : block-time sampling failed; used configured fallback`);
  }
  if (reports.gateway.driftWarn) {
    console.log(`  WARN             : observed block time drifted >20% from fallback`);
  }
  console.log('');
  console.log('## Cross-chain alignment');
  const allStarts = [
    ...reports.host.map((r) => ({ label: `chain ${r.chainId}`, ts: r.result.startBlockEstimatedTimestamp })),
    { label: 'gateway', ts: reports.gateway.result.startBlockEstimatedTimestamp },
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
  console.log('## Calldata');
  console.log(calldata);
}

function encodeCalldata(inputs: ScriptInputs, reports: { host: HostReport[]; gateway: GatewayReport }): string {
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
    inputs.ciphertextVersion,
  ]);
}

async function main(): Promise<void> {
  const inputs = parseInputs();
  const reports = await gatherWindows(inputs);
  const calldata = encodeCalldata(inputs, reports);

  printSummary(inputs, reports, calldata);

  const bufferFailures: string[] = [];
  for (const r of reports.host) {
    if (!r.bufferOk) bufferFailures.push(`chain ${r.chainId}`);
  }
  if (!reports.gateway.bufferOk) bufferFailures.push('gateway');
  if (bufferFailures.length > 0) {
    console.error(`\nerror: DAO buffer violated for: ${bufferFailures.join(', ')}`);
    process.exit(2);
  }
}

main().catch((err) => {
  console.error(err instanceof Error ? err.message : err);
  process.exit(1);
});
