import { FunctionFragment, Interface, type InterfaceAbi } from 'ethers';
import fs from 'fs';
import { task, types } from 'hardhat/config';
import type { HardhatRuntimeEnvironment, TaskArguments } from 'hardhat/types';
import path from 'path';

import {
  buildKmsNodes,
  buildKmsThresholds,
  deployEmptyUUPS,
  ensureAddressesDirectoryExists,
  readExistingHostEnv,
  readHostEnv,
} from './taskDeploy';
import {
  type KmsNode,
  type ProtocolConfigMigrationState,
  assertSame,
  loadKmsGenerationMigrationState,
  loadProtocolConfigMigrationState,
  normalizeAddress,
  normalizeAddresses,
  normalizeKmsGenerationMigrationState,
  normalizeNodes,
  normalizeThresholds,
} from './utils/daoKmsMigrationState';
import {
  type KmsGenerationMigrationState,
  buildKMSGenerationMigrationStateFromEnv,
} from './utils/kmsGenerationMigrationEnv';
import { getRequiredEnvVar } from './utils/loadVariables';

////////////////////////////////////////////////////////////////////////////////
// Proposal artifact helpers
////////////////////////////////////////////////////////////////////////////////

export function stringifyForProposal(value: unknown): string {
  return JSON.stringify(
    value,
    (_, nestedValue: unknown) => (typeof nestedValue === 'bigint' ? nestedValue.toString() : nestedValue),
    2,
  );
}

function getFunctionFragment(abi: InterfaceAbi, functionName: string): FunctionFragment {
  const fragment = new Interface(abi).getFunction(functionName);
  if (fragment === null) {
    throw new Error(`Function ${functionName} not found in ABI.`);
  }
  return fragment;
}

export const UPGRADE_TO_AND_CALL_INTERFACE = new Interface([
  'function upgradeToAndCall(address newImplementation, bytes data) payable',
]);

type PreparedDaoUpgrade = {
  proxyAddress: string;
  newImplementationAddress: string;
  innerFunctionSignature: string;
  decodedArgs: unknown[];
  innerCalldata: string;
  outerCalldata: string;
};

async function prepareDaoUpgrade(
  hre: HardhatRuntimeEnvironment,
  params: {
    proxyAddress: string;
    contractName: string;
    innerFunctionSignature: string;
    decodedArgs: unknown[];
  },
): Promise<PreparedDaoUpgrade> {
  const { ethers, upgrades } = hre;
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplementation = await ethers.getContractFactory(params.contractName, deployer);
  await upgrades.forceImport(params.proxyAddress, currentImplementation);
  const newImplementationAddress = String(
    await upgrades.prepareUpgrade(params.proxyAddress, newImplementation, {
      kind: 'uups',
    }),
  );
  const innerCalldata = newImplementation.interface.encodeFunctionData(
    params.innerFunctionSignature,
    params.decodedArgs,
  );
  const outerCalldata = UPGRADE_TO_AND_CALL_INTERFACE.encodeFunctionData('upgradeToAndCall', [
    newImplementationAddress,
    innerCalldata,
  ]);

  return {
    proxyAddress: params.proxyAddress,
    newImplementationAddress,
    innerFunctionSignature: params.innerFunctionSignature,
    decodedArgs: params.decodedArgs,
    innerCalldata,
    outerCalldata,
  };
}

function printPreparedDaoUpgrade(data: PreparedDaoUpgrade): void {
  console.log('proxyAddress:', data.proxyAddress);
  console.log('newImplementationAddress:', data.newImplementationAddress);
  console.log('innerFunctionSignature:', data.innerFunctionSignature);
  console.log('decodedArgs:', stringifyForProposal(data.decodedArgs));
  console.log(`${data.innerFunctionSignature} calldata:`, data.innerCalldata);
  console.log('upgradeToAndCall(address,bytes) calldata:', data.outerCalldata);
  console.log(
    'Proposal JSON:',
    stringifyForProposal({
      proxyAddress: data.proxyAddress,
      newImplementationAddress: data.newImplementationAddress,
      innerFunctionSignature: data.innerFunctionSignature,
      decodedArgs: data.decodedArgs,
      innerCalldata: data.innerCalldata,
      outerCalldata: data.outerCalldata,
    }),
  );
  console.log(
    `Cast command: cast calldata 'upgradeToAndCall(address,bytes)' ${data.newImplementationAddress} ${data.innerCalldata}`,
  );
}

function requireProposalString(proposal: Record<string, unknown>, field: string): string {
  const value = proposal[field];
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`Proposal JSON must contain a non-empty string field: ${field}.`);
  }
  return value;
}

function requireProposalArgs(proposal: Record<string, unknown>): unknown[] {
  const value = proposal.decodedArgs;
  if (!Array.isArray(value)) {
    throw new Error('Proposal JSON must contain decodedArgs as an array.');
  }
  return value;
}

function assertEqualCalldata(label: string, expected: string, actual: string): void {
  if (expected.toLowerCase() !== actual.toLowerCase()) {
    throw new Error(`${label} mismatch.\nExpected: ${expected}\nActual:   ${actual}`);
  }
}

////////////////////////////////////////////////////////////////////////////////
// Decoded-args builders
////////////////////////////////////////////////////////////////////////////////

function buildProtocolConfigMigrationArgs(
  migrationContextId: bigint,
  kmsNodes: ReturnType<typeof buildKmsNodes>,
  thresholds: ReturnType<typeof buildKmsThresholds>,
): unknown[] {
  return [
    migrationContextId,
    kmsNodes.map((node) => [node.txSenderAddress, node.signerAddress, node.ipAddress, node.storageUrl]),
    [thresholds.publicDecryption, thresholds.userDecryption, thresholds.kmsGen, thresholds.mpc],
  ];
}

function buildKMSGenerationMigrationArgs(migrationState: KmsGenerationMigrationState) {
  return [
    [
      migrationState.prepKeygenCounter,
      migrationState.keyCounter,
      migrationState.crsCounter,
      migrationState.activeKeyId,
      migrationState.activeCrsId,
      migrationState.activePrepKeygenId,
      migrationState.activeKeyDigests.map((digest: { keyType: number; digest: string }) => [
        digest.keyType,
        digest.digest,
      ]),
      migrationState.activeCrsDigest,
      migrationState.keyConsensusTxSenders,
      migrationState.keyConsensusDigest,
      migrationState.crsConsensusTxSenders,
      migrationState.crsConsensusDigest,
      migrationState.prepKeygenConsensusTxSenders,
      migrationState.prepKeygenConsensusDigest,
      migrationState.crsMaxBitLength,
      migrationState.prepKeygenParamsType,
      migrationState.crsParamsType,
      migrationState.contextId,
    ],
  ];
}

////////////////////////////////////////////////////////////////////////////////
// Snapshot cross-check (verifier-side)
////////////////////////////////////////////////////////////////////////////////

// The proposal artifact's decodedArgs has bigints serialized as strings via
// stringifyForProposal. The on-chain ProtocolConfig.initializeFromMigration
// signature is (uint256 contextId, KmsNode[] nodes, Thresholds), so the
// proposal shape is [contextId, [[txSender, signer, ip, storage], ...], [pub,
// user, kmsGen, mpc]].
function extractProtocolConfigSnapshotFromDecodedArgs(decodedArgs: unknown[]): ProtocolConfigMigrationState {
  if (decodedArgs.length !== 3) {
    throw new Error(
      `ProtocolConfig proposal decodedArgs must have 3 entries (contextId, nodes, thresholds); got ${decodedArgs.length}.`,
    );
  }
  const [rawContextId, rawNodes, rawThresholds] = decodedArgs;
  if (typeof rawContextId !== 'string' && typeof rawContextId !== 'number') {
    throw new Error('ProtocolConfig proposal decodedArgs[0] (contextId) must be a string or number.');
  }
  if (!Array.isArray(rawThresholds) || rawThresholds.length !== 4) {
    throw new Error('ProtocolConfig proposal decodedArgs[2] (thresholds) must be a 4-element array.');
  }
  const [pub, user, kmsGen, mpc] = rawThresholds;
  const thresholds = normalizeThresholds(
    { publicDecryption: pub, userDecryption: user, kmsGen, mpc },
    'proposal thresholds',
  );
  const nodes = normalizeNodes(rawNodes, 'proposal kmsNodes');
  return {
    contextId: BigInt(String(rawContextId)).toString(),
    nodes,
    signers: normalizeAddresses(
      nodes.map((node: KmsNode) => node.signerAddress),
      'proposal signer addresses',
    ),
    thresholds,
  };
}

function assertProtocolConfigProposalMatchesSnapshot(decodedArgs: unknown[], migrationStatePath: string): void {
  const expected = loadProtocolConfigMigrationState(path.resolve(process.cwd(), migrationStatePath));
  const actual = extractProtocolConfigSnapshotFromDecodedArgs(decodedArgs);
  assertSame('ProtocolConfig proposal contextId', actual.contextId, expected.contextId);
  assertSame('ProtocolConfig proposal KMS nodes', actual.nodes, expected.nodes);
  assertSame('ProtocolConfig proposal thresholds', actual.thresholds, expected.thresholds);
}

function assertKmsGenerationProposalMatchesSnapshot(decodedArgs: unknown[], migrationStatePath: string): void {
  if (decodedArgs.length !== 1) {
    throw new Error(
      `KMSGeneration proposal decodedArgs must have 1 entry (MigrationState); got ${decodedArgs.length}.`,
    );
  }
  const expected = loadKmsGenerationMigrationState(path.resolve(process.cwd(), migrationStatePath));
  const actual = normalizeKmsGenerationMigrationState(decodedArgs[0], 'proposal KMSGeneration migration state');
  assertSame('KMSGeneration proposal migration state', actual, expected);
}

function isProtocolConfigInitializeFromMigration(innerFunctionSignature: string): boolean {
  return innerFunctionSignature.startsWith('initializeFromMigration(uint256,');
}

function isKmsGenerationInitializeFromMigration(innerFunctionSignature: string): boolean {
  return innerFunctionSignature.startsWith('initializeFromMigration((uint256,');
}

function readProposalAddresses(filePath: string, label: string) {
  const proposal = JSON.parse(fs.readFileSync(filePath, 'utf-8')) as Record<string, unknown>;
  return {
    proxyAddress: normalizeAddress(requireProposalString(proposal, 'proxyAddress'), `${label}.proxyAddress`),
    newImplementationAddress: normalizeAddress(
      requireProposalString(proposal, 'newImplementationAddress'),
      `${label}.newImplementationAddress`,
    ),
  };
}

////////////////////////////////////////////////////////////////////////////////
// Migration empty-proxy bootstrap
////////////////////////////////////////////////////////////////////////////////

task('task:deployEmptyProxiesProtocolConfigKMSGeneration').setAction(async function (_, { ethers, upgrades, run }) {
  ensureAddressesDirectoryExists();

  const existingEnv = readExistingHostEnv();

  const targets = [
    { envKey: 'PROTOCOL_CONFIG_CONTRACT_ADDRESS', setterTask: 'task:setProtocolConfigAddress' },
    { envKey: 'KMS_GENERATION_CONTRACT_ADDRESS', setterTask: 'task:setKMSGenerationAddress' },
  ] as const;

  const missingTargets = targets.filter(({ envKey }) => !existingEnv[envKey]);

  if (missingTargets.length === 0) {
    console.warn(
      'Migration bootstrap is a no-op; addresses/.env.host already contains ProtocolConfig and KMSGeneration. Remove task:deployEmptyProxiesProtocolConfigKMSGeneration once UPGRADE_FROM_TAG includes #2243.',
    );
    return;
  }

  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);

  await run('compile:specific', { contract: 'contracts/emptyProxy' });

  for (const { envKey: _envKey, setterTask } of missingTargets) {
    const proxyAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run(setterTask, { address: proxyAddress });
  }
});

////////////////////////////////////////////////////////////////////////////////
// ProtocolConfig (migration)
////////////////////////////////////////////////////////////////////////////////

task(
  'task:prepareDeployProtocolConfigFromMigration',
  'Deploys a ProtocolConfig migration implementation and prints DAO upgrade calldata without mutating the proxy',
)
  .addOptionalParam(
    'useAddress',
    'Use addresses instead of private keys env variables for kms signers',
    true,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, hre) {
    const initialKmsNodes = buildKmsNodes(taskArguments.useAddress);
    const thresholds = buildKmsThresholds();
    const migrationContextId = BigInt(getRequiredEnvVar('MIGRATION_CONTEXT_ID'));
    const parsedEnv = readHostEnv();
    const proxyAddress = parsedEnv.PROTOCOL_CONFIG_CONTRACT_ADDRESS;
    const decodedArgs = buildProtocolConfigMigrationArgs(migrationContextId, initialKmsNodes, thresholds);
    const artifact = await hre.artifacts.readArtifact('ProtocolConfig');
    const innerFunctionSignature = getFunctionFragment(artifact.abi, 'initializeFromMigration').format('sighash');
    const preparedUpgrade = await prepareDaoUpgrade(hre, {
      proxyAddress,
      contractName: 'ProtocolConfig',
      innerFunctionSignature,
      decodedArgs,
    });

    printPreparedDaoUpgrade(preparedUpgrade);
    return preparedUpgrade;
  });

////////////////////////////////////////////////////////////////////////////////
// KMSGeneration (migration)
////////////////////////////////////////////////////////////////////////////////

async function prepareKMSGenerationMigrationUpgrade(
  hre: HardhatRuntimeEnvironment,
  migrationState: KmsGenerationMigrationState,
): Promise<PreparedDaoUpgrade> {
  const parsedEnv = readHostEnv();
  const proxyAddress = parsedEnv.KMS_GENERATION_CONTRACT_ADDRESS;
  const decodedArgs = buildKMSGenerationMigrationArgs(migrationState);
  const artifact = await hre.artifacts.readArtifact('KMSGeneration');
  const innerFunctionSignature = getFunctionFragment(artifact.abi, 'initializeFromMigration').format('sighash');
  return prepareDaoUpgrade(hre, {
    proxyAddress,
    contractName: 'KMSGeneration',
    innerFunctionSignature,
    decodedArgs,
  });
}

task(
  'task:prepareDeployKMSGenerationFromMigration',
  'Deploys a KMSGeneration migration implementation from MIGRATION_* env and prints DAO upgrade calldata without mutating the proxy',
).setAction(async function (_taskArguments: TaskArguments, hre) {
  const preparedUpgrade = await prepareKMSGenerationMigrationUpgrade(hre, buildKMSGenerationMigrationStateFromEnv());

  printPreparedDaoUpgrade(preparedUpgrade);
  return preparedUpgrade;
});

////////////////////////////////////////////////////////////////////////////////
// Proposal verifier (DAO-side)
////////////////////////////////////////////////////////////////////////////////

task(
  'task:verifyDaoKmsMigrationProposal',
  'Verifies DAO migration proposal calldata and cross-checks migration decodedArgs against migration-state.json',
)
  .addParam('proposal', 'Path to a proposal JSON emitted by a prepare-only DAO migration task')
  .addParam(
    'migrationState',
    'Path to the exported gateway migration-state.json. Used to cross-check migration proposal decodedArgs.',
  )
  .setAction(async function (taskArguments: TaskArguments) {
    const proposalPath = path.resolve(process.cwd(), taskArguments.proposal);
    const proposal = JSON.parse(fs.readFileSync(proposalPath, 'utf-8')) as Record<string, unknown>;
    const newImplementationAddress = requireProposalString(proposal, 'newImplementationAddress');
    const innerFunctionSignature = requireProposalString(proposal, 'innerFunctionSignature');
    const decodedArgs = requireProposalArgs(proposal);
    const actualInnerCalldata = requireProposalString(proposal, 'innerCalldata');
    const actualOuterCalldata = requireProposalString(proposal, 'outerCalldata');
    const expectedInnerCalldata = new Interface([`function ${innerFunctionSignature}`]).encodeFunctionData(
      innerFunctionSignature,
      decodedArgs,
    );
    const expectedOuterCalldata = UPGRADE_TO_AND_CALL_INTERFACE.encodeFunctionData('upgradeToAndCall', [
      newImplementationAddress,
      expectedInnerCalldata,
    ]);

    assertEqualCalldata('inner calldata', expectedInnerCalldata, actualInnerCalldata);
    assertEqualCalldata('upgradeToAndCall calldata', expectedOuterCalldata, actualOuterCalldata);

    if (isProtocolConfigInitializeFromMigration(innerFunctionSignature)) {
      assertProtocolConfigProposalMatchesSnapshot(decodedArgs, taskArguments.migrationState);
      console.log(`ProtocolConfig proposal decodedArgs match migration-state snapshot.`);
    } else if (isKmsGenerationInitializeFromMigration(innerFunctionSignature)) {
      assertKmsGenerationProposalMatchesSnapshot(decodedArgs, taskArguments.migrationState);
      console.log(`KMSGeneration proposal decodedArgs match migration-state snapshot.`);
    } else {
      console.log(
        `Skipping snapshot cross-check: ${innerFunctionSignature} has no migration-state schema. Reviewers must verify decodedArgs against migration-state.json by hand.`,
      );
    }

    console.log(`Proposal calldata verified: ${proposalPath}`);
    console.log(
      `Cast command: cast calldata 'upgradeToAndCall(address,bytes)' ${newImplementationAddress} ${expectedInnerCalldata}`,
    );
  });

////////////////////////////////////////////////////////////////////////////////
// addresses.json generator
////////////////////////////////////////////////////////////////////////////////

task('task:generateDaoKmsMigrationAddresses', 'Generates DAO KMS migration addresses.json from proposal artifacts')
  .addParam('migrationState', 'Path to migration-state.json')
  .addParam('gatewayKmsGenerationProposal', 'Path to proposal-gateway-kmsgeneration.json')
  .addParam('hostProtocolConfigProposal', 'Path to proposal-host-protocol-config.json')
  .addParam('hostKmsGenerationProposal', 'Path to proposal-host-kmsgeneration.json')
  .addParam('hostKmsVerifierProposal', 'Path to proposal-host-kmsverifier.json')
  .addParam('output', 'Path to write addresses.json')
  .setAction(async function (taskArguments: TaskArguments) {
    const migrationStatePath = path.resolve(process.cwd(), taskArguments.migrationState);
    const outputPath = path.resolve(process.cwd(), taskArguments.output);
    const migrationState = JSON.parse(fs.readFileSync(migrationStatePath, 'utf-8')) as Record<string, unknown>;
    const metadata = migrationState.metadata as Record<string, unknown>;

    const gatewayKmsGenerationProposal = readProposalAddresses(
      path.resolve(process.cwd(), taskArguments.gatewayKmsGenerationProposal),
      'proposal-gateway-kmsgeneration.json',
    );
    const hostProtocolConfigProposal = readProposalAddresses(
      path.resolve(process.cwd(), taskArguments.hostProtocolConfigProposal),
      'proposal-host-protocol-config.json',
    );
    const hostKmsGenerationProposal = readProposalAddresses(
      path.resolve(process.cwd(), taskArguments.hostKmsGenerationProposal),
      'proposal-host-kmsgeneration.json',
    );
    const hostKmsVerifierProposal = readProposalAddresses(
      path.resolve(process.cwd(), taskArguments.hostKmsVerifierProposal),
      'proposal-host-kmsverifier.json',
    );

    const gatewayKmsGenerationProxy = normalizeAddress(
      metadata.gatewayKmsGenerationProxy,
      'migration-state.json.metadata.gatewayKmsGenerationProxy',
    );
    const gatewayConfigProxy = normalizeAddress(
      metadata.gatewayConfigProxy,
      'migration-state.json.metadata.gatewayConfigProxy',
    );
    const legacyHostKmsVerifierProxy =
      metadata.legacyHostKmsVerifierProxy === null || metadata.legacyHostKmsVerifierProxy === undefined
        ? null
        : normalizeAddress(
            metadata.legacyHostKmsVerifierProxy,
            'migration-state.json.metadata.legacyHostKmsVerifierProxy',
          );

    assertSame('gateway KMSGeneration proxy', gatewayKmsGenerationProposal.proxyAddress, gatewayKmsGenerationProxy);
    if (legacyHostKmsVerifierProxy !== null) {
      assertSame('host KMSVerifier proxy', hostKmsVerifierProposal.proxyAddress, legacyHostKmsVerifierProxy);
    }

    const addressesJson = {
      host: {
        proxies: {
          protocolConfig: hostProtocolConfigProposal.proxyAddress,
          kmsGeneration: hostKmsGenerationProposal.proxyAddress,
          kmsVerifier: hostKmsVerifierProposal.proxyAddress,
        },
        implementations: {
          protocolConfig: hostProtocolConfigProposal.newImplementationAddress,
          kmsGeneration: hostKmsGenerationProposal.newImplementationAddress,
          kmsVerifier: hostKmsVerifierProposal.newImplementationAddress,
        },
      },
      gateway: {
        proxies: {
          kmsGeneration: gatewayKmsGenerationProxy,
          gatewayConfig: gatewayConfigProxy,
        },
        implementations: {
          kmsGeneration: gatewayKmsGenerationProposal.newImplementationAddress,
        },
      },
    };

    fs.mkdirSync(path.dirname(outputPath), { recursive: true });
    fs.writeFileSync(outputPath, `${stringifyForProposal(addressesJson)}\n`);
    console.log(`Generated DAO KMS migration addresses.json: ${outputPath}`);
    return addressesJson;
  });
