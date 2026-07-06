import { task, types } from 'hardhat/config';

import {
  bufferViolations,
  buildCoprocessorUpgradeProposal,
  executeCoprocessorUpgradeProposal,
  parseCoprocessorUpgradeInputs,
  printCoprocessorUpgradeProposal,
} from './utils/coprocessorUpgradeProposal';
import { loadHostAddresses } from './utils/loadVariables';

// Coprocessor-upgrade DAO tasks on the host ProtocolConfig, mirroring kmsContext.ts: `prepare*` prints
// the Aragon calldata (DAO path), `propose*` broadcasts it with the deployer key (no-DAO devnet path).
// Logic lives in utils/coprocessorUpgradeProposal.ts; see scripts/RUNBOOK.md.

const PROTOCOL_CONFIG_ADDRESS_ENV_VAR = 'PROTOCOL_CONFIG_CONTRACT_ADDRESS';

// Resolve the host ProtocolConfig address from env or the addresses dir; throws if unset.
function requireProtocolConfigAddress(useInternalProxyAddress: boolean): string {
  if (useInternalProxyAddress) {
    loadHostAddresses();
  }
  const address = process.env[PROTOCOL_CONFIG_ADDRESS_ENV_VAR];
  if (!address || address.trim() === '') {
    throw new Error(
      `No ProtocolConfig address configured. Set ${PROTOCOL_CONFIG_ADDRESS_ENV_VAR} or pass --use-internal-proxy-address.`,
    );
  }
  return address;
}

task(
  'task:prepareCoprocessorUpgrade',
  'Builds Aragon proposal calldata for ProtocolConfig.proposeCoprocessorUpgrade by computing block windows across the environment host chains + gateway (DAO path, never broadcasts)',
)
  .addParam('environment', 'Target environment: devnet | testnet | mainnet')
  .addParam('startTime', 'ISO 8601 UTC start of the evaluation window (e.g. 2026-07-01T12:00:00Z)')
  .addParam('duration', 'Evaluation window length (e.g. 30s, 30m, 2h, 1d, or a bare integer of seconds)')
  .addParam('buffer', 'DAO lead time required between now and startBlock (same format as --duration)')
  .addParam('proposalId', 'Coprocessor upgrade proposal id (positive integer, decimal or 0x-hex)')
  .addParam('softwareVersion', 'Coprocessor software version string (e.g. v0.14.0)')
  .setAction(async function ({ environment, startTime, duration, buffer, proposalId, softwareVersion }): Promise<void> {
    const inputs = parseCoprocessorUpgradeInputs({
      environment,
      startTime,
      duration,
      buffer,
      proposalId,
      softwareVersion,
    });
    const proposal = await buildCoprocessorUpgradeProposal(inputs);
    printCoprocessorUpgradeProposal(proposal);

    const violations = bufferViolations(proposal);
    if (violations.length > 0) {
      // Hard-fail the buffer gate (calldata already printed for inspection).
      throw new Error(
        `DAO buffer violated for: ${violations.join(', ')}. Calldata printed above for inspection but must NOT be submitted.`,
      );
    }
  });

task(
  'task:proposeCoprocessorUpgrade',
  'Broadcasts ProtocolConfig.proposeCoprocessorUpgrade computed for the environment with the deployer key (no-DAO path for devnet / test-suite). Sends to the host ProtocolConfig on --network.',
)
  .addParam('environment', 'Target environment: devnet | testnet | mainnet')
  .addParam('startTime', 'ISO 8601 UTC start of the evaluation window (e.g. 2026-07-01T12:00:00Z)')
  .addParam('duration', 'Evaluation window length (e.g. 30s, 30m, 2h, 1d, or a bare integer of seconds)')
  .addParam('buffer', 'DAO lead time required between now and startBlock (same format as --duration)')
  .addParam('proposalId', 'Coprocessor upgrade proposal id (positive integer, decimal or 0x-hex)')
  .addParam('softwareVersion', 'Coprocessor software version string (e.g. v0.14.0)')
  .addOptionalParam(
    'useInternalProxyAddress',
    'Resolve the ProtocolConfig address from the /addresses directory instead of the environment',
    false,
    types.boolean,
  )
  .setAction(async function (
    { environment, startTime, duration, buffer, proposalId, softwareVersion, useInternalProxyAddress },
    hre,
  ): Promise<void> {
    const inputs = parseCoprocessorUpgradeInputs({
      environment,
      startTime,
      duration,
      buffer,
      proposalId,
      softwareVersion,
    });
    const proposal = await buildCoprocessorUpgradeProposal(inputs);
    printCoprocessorUpgradeProposal(proposal);

    const violations = bufferViolations(proposal);
    if (violations.length > 0) {
      throw new Error(`DAO buffer violated for: ${violations.join(', ')}. Refusing to broadcast.`);
    }

    const target = requireProtocolConfigAddress(useInternalProxyAddress);
    const hash = await executeCoprocessorUpgradeProposal(hre, proposal, target);
    console.log(`\nBroadcast proposeCoprocessorUpgrade on ${target} (tx: ${hash}).`);
  });
