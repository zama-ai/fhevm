// The evidence a `hardhat node` operator wants: which FHEVM stack this node serves, printed as soon as
// the chain is prepared — before hardhat announces the server. One line always; the address table when
// the run is verbose (`-vvv` or more: hardhat's verbosity is the COUNT of v's, default 2, and its own
// call traces start at 3 too). Only the `node` task asks for
// it (a test run would print one banner per in-process connection); the request is process-wide
// because the node task runs once per process.

import type { Deployed } from '@fhevm/host-contracts-cleartext/ts';

import type { FhevmNetworkInfo } from '../types.js';
import { FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE_NAME, PLUGIN_ID } from './constants.js';

export type NodeBannerLevel = 'none' | 'summary' | 'detailed';

/** Hardhat's default verbosity (`-vv`); `-vvv` is the first level above it. Not exported by hardhat, so restated here. */
export const DEFAULT_VERBOSITY = 2;

let level: NodeBannerLevel = 'none';

export function requestNodeBanner(verbosity: number): void {
  level = verbosity > DEFAULT_VERBOSITY ? 'detailed' : 'summary';
}

export function nodeBannerLevel(): NodeBannerLevel {
  return level;
}

export function formatStackBanner(
  network: FhevmNetworkInfo,
  stack: Deployed,
  reused: boolean,
  detailed: boolean,
): string {
  const origin = reused
    ? 'already on the chain, reused'
    : `deployed by this node from ${FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE_NAME}`;
  const summary = `${PLUGIN_ID}: cleartext FHEVM stack on '${network.networkName}' (chainId ${String(network.chainId)}) — ${origin}, verified`;
  if (!detailed) return `${summary} (-vvv lists the addresses)\n`;

  const { fhevmAddresses, cleartextAddresses } = stack;
  const rows: Array<[string, string]> = [
    ['ACL', fhevmAddresses.aclAddress],
    ['FHEVMExecutor', fhevmAddresses.fhevmExecutorAddress],
    ['InputVerifier', fhevmAddresses.inputVerifierAddress],
    ['KMSVerifier', fhevmAddresses.kmsVerifierAddress],
    ['HCULimit', fhevmAddresses.hcuLimitAddress],
    ['ProtocolConfig', fhevmAddresses.protocolConfigAddress],
    ['KMSGeneration', fhevmAddresses.kmsGenerationAddress],
    ['PauserSet', stack.pauserSetAddress],
    ['CleartextArithmetic', cleartextAddresses.cleartextArithmeticAddress],
    ['CleartextDB', cleartextAddresses.cleartextDbAddress],
    ['deployer (ACL owner)', stack.aclOwnerAddress],
  ];
  const width = Math.max(...rows.map(([name]) => name.length));
  return [summary, ...rows.map(([name, address]) => `  ${name.padEnd(width)}  ${address}`), ''].join('\n');
}
