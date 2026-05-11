import { FunctionFragment, Interface, type InterfaceAbi, getAddress } from 'ethers';
import { task, types } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import {
  assertContractMatchesVersionPrefix,
  deployEmptyUUPS,
  ensureAddressesDirectoryExists,
  readExistingHostEnv,
  readHostEnv,
  waitForTaskReady,
} from './taskDeploy';
import { buildKMSGenerationInitializeFromMigrationArgs } from './utils/kmsGenerationMigrationEnv';
import { getRequiredEnvVar } from './utils/loadVariables';
import {
  type ProtocolConfigMigrationKmsNode,
  buildProtocolConfigInitializeFromMigrationArgs,
} from './utils/protocolConfigMigrationEnv';

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

async function verifyPreparedImplementation(
  hre: HardhatRuntimeEnvironment,
  data: PreparedDaoUpgrade,
  contract: string,
): Promise<void> {
  console.log('Waiting 2 minutes before contract verification... Please wait...');
  await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
  await hre.run('verify:verify', {
    address: data.newImplementationAddress,
    contract,
    constructorArguments: [],
  });
}

function printPreparedDaoUpgrade(data: PreparedDaoUpgrade): void {
  console.log('proxyAddress:', data.proxyAddress);
  console.log('newImplementationAddress:', data.newImplementationAddress);
  console.log('innerFunctionSignature:', data.innerFunctionSignature);
  console.log('decodedArgs:', stringifyForProposal(data.decodedArgs));
  console.log(`${data.innerFunctionSignature} calldata:`, data.innerCalldata);
  console.log('upgradeToAndCall(address,bytes) calldata:', data.outerCalldata);
  console.log(
    'Prepared upgrade artifact:',
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

function assertEqual(label: string, actual: string | number | boolean, expected: string | number | boolean): void {
  if (actual !== expected) {
    throw new Error(`${label} mismatch: expected ${expected}, got ${actual}.`);
  }
}

function assertBigIntEqual(label: string, actual: bigint, expected: bigint): void {
  if (actual !== expected) {
    throw new Error(`${label} mismatch: expected ${expected.toString()}, got ${actual.toString()}.`);
  }
}

function assertJsonEqual(label: string, actual: unknown, expected: unknown): void {
  const actualJson = stringifyForProposal(actual);
  const expectedJson = stringifyForProposal(expected);
  if (actualJson !== expectedJson) {
    throw new Error(`${label} mismatch: expected ${expectedJson}, got ${actualJson}.`);
  }
}

function normalizeAddresses(addresses: readonly string[]): string[] {
  return addresses.map((address) => getAddress(address));
}

type KmsNodeLike = {
  txSenderAddress: string;
  signerAddress: string;
  ipAddress: string;
  storageUrl: string;
};

function normalizeKmsNode(node: KmsNodeLike): ProtocolConfigMigrationKmsNode {
  return {
    txSenderAddress: getAddress(node.txSenderAddress),
    signerAddress: getAddress(node.signerAddress),
    ipAddress: node.ipAddress,
    storageUrl: node.storageUrl,
  };
}

type KeyDigestLike = {
  keyType: number | bigint;
  digest: string;
};

function normalizeKeyDigest(digest: KeyDigestLike): { keyType: number; digest: string } {
  return {
    keyType: Number(digest.keyType),
    digest: digest.digest.toLowerCase(),
  };
}

function storageUrlsForSenders(
  nodes: readonly ProtocolConfigMigrationKmsNode[],
  txSenders: readonly string[],
): string[] {
  const storageUrlsByTxSender = new Map(
    nodes.map((node) => [getAddress(node.txSenderAddress), node.storageUrl] as const),
  );

  return txSenders.map((txSender) => {
    const storageUrl = storageUrlsByTxSender.get(getAddress(txSender));
    if (storageUrl === undefined) {
      throw new Error(`Migration snapshot has consensus tx sender ${txSender} without a matching KMS node.`);
    }
    return storageUrl;
  });
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

  for (const { envKey, setterTask } of missingTargets) {
    const proxyAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run(setterTask, { address: proxyAddress });
    process.env[envKey] = proxyAddress;
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
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function ({ verifyContract }, hre) {
    const parsedEnv = readHostEnv();
    const proxyAddress = parsedEnv.PROTOCOL_CONFIG_CONTRACT_ADDRESS;
    // The bootstrap task may have updated addresses/FHEVMHostAddresses.sol, so rebuild
    await hre.run('compile:specific', { contract: 'contracts' });
    const decodedArgs = buildProtocolConfigInitializeFromMigrationArgs();
    const artifact = await hre.artifacts.readArtifact('ProtocolConfig');
    const innerFunctionSignature = getFunctionFragment(artifact.abi, 'initializeFromMigration').format('sighash');
    const preparedUpgrade = await prepareDaoUpgrade(hre, {
      proxyAddress,
      contractName: 'ProtocolConfig',
      innerFunctionSignature,
      decodedArgs,
    });

    printPreparedDaoUpgrade(preparedUpgrade);
    if (verifyContract) {
      await verifyPreparedImplementation(hre, preparedUpgrade, 'contracts/ProtocolConfig.sol:ProtocolConfig');
    }
    return preparedUpgrade;
  });

task(
  'task:deployProtocolConfigFromMigration',
  'Upgrades the ProtocolConfig proxy to a migration implementation initialized via initializeFromMigration',
).setAction(async function (_, hre) {
  const { ethers, upgrades } = hre;
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  await hre.run('compile:specific', { contract: 'contracts' });
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('ProtocolConfig', deployer);
  const parsedEnv = readHostEnv();
  const proxyAddress = parsedEnv.PROTOCOL_CONFIG_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  const decodedArgs = buildProtocolConfigInitializeFromMigrationArgs();

  await upgrades.upgradeProxy(proxy, newImplem, {
    call: {
      fn: 'initializeFromMigration',
      args: decodedArgs,
    },
  });
  await waitForTaskReady(hre, 'task:assertProtocolConfigReady');
  console.log('ProtocolConfig migration code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// KMSGeneration (migration)
////////////////////////////////////////////////////////////////////////////////

task(
  'task:prepareDeployKMSGenerationFromMigration',
  'Deploys a KMSGeneration migration implementation from MIGRATION_* env and prints DAO upgrade calldata without mutating the proxy',
)
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function ({ verifyContract }, hre) {
    const parsedEnv = readHostEnv();
    const proxyAddress = parsedEnv.KMS_GENERATION_CONTRACT_ADDRESS;
    // The bootstrap task may have updated addresses/FHEVMHostAddresses.sol, so rebuild
    await hre.run('compile:specific', { contract: 'contracts' });
    const decodedArgs = buildKMSGenerationInitializeFromMigrationArgs();
    const artifact = await hre.artifacts.readArtifact('KMSGeneration');
    const innerFunctionSignature = getFunctionFragment(artifact.abi, 'initializeFromMigration').format('sighash');
    const preparedUpgrade = await prepareDaoUpgrade(hre, {
      proxyAddress,
      contractName: 'KMSGeneration',
      innerFunctionSignature,
      decodedArgs,
    });

    printPreparedDaoUpgrade(preparedUpgrade);
    if (verifyContract) {
      await verifyPreparedImplementation(hre, preparedUpgrade, 'contracts/KMSGeneration.sol:KMSGeneration');
    }
    return preparedUpgrade;
  });

task(
  'task:deployKMSGenerationFromMigration',
  'Upgrades the KMSGeneration proxy to a migration implementation initialized via initializeFromMigration from MIGRATION_* env',
).setAction(async function (_, hre) {
  const { ethers, upgrades } = hre;
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  await hre.run('compile:specific', { contract: 'contracts' });
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('KMSGeneration', deployer);
  const parsedEnv = readHostEnv();
  const proxyAddress = parsedEnv.KMS_GENERATION_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  const decodedArgs = buildKMSGenerationInitializeFromMigrationArgs();

  await upgrades.upgradeProxy(proxy, newImplem, {
    call: {
      fn: 'initializeFromMigration',
      args: decodedArgs,
    },
  });
  console.log('KMSGeneration migration code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// Post-execution verification
////////////////////////////////////////////////////////////////////////////////

task(
  'task:assertKmsMigrationSucceeded',
  'Asserts the live host migration state matches the MIGRATION_* snapshot env',
).setAction(async function (_, hre) {
  const parsedEnv = readHostEnv();
  const protocolConfigAddress = parsedEnv.PROTOCOL_CONFIG_CONTRACT_ADDRESS;
  const kmsGenerationAddress = parsedEnv.KMS_GENERATION_CONTRACT_ADDRESS;
  const kmsVerifierAddress = parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS;
  if (!protocolConfigAddress) {
    throw new Error('PROTOCOL_CONFIG_CONTRACT_ADDRESS is missing from addresses/.env.host.');
  }
  if (!kmsGenerationAddress) {
    throw new Error('KMS_GENERATION_CONTRACT_ADDRESS is missing from addresses/.env.host.');
  }
  if (!kmsVerifierAddress) {
    throw new Error('KMS_VERIFIER_CONTRACT_ADDRESS is missing from addresses/.env.host.');
  }

  const [expectedContextId, expectedNodes, expectedThresholds] = buildProtocolConfigInitializeFromMigrationArgs();
  const [expectedKmsGenerationState] = buildKMSGenerationInitializeFromMigrationArgs();

  await assertContractMatchesVersionPrefix(hre, protocolConfigAddress, 'ProtocolConfig');
  const protocolConfig = await hre.ethers.getContractAt('ProtocolConfig', protocolConfigAddress);

  assertBigIntEqual(
    'ProtocolConfig current KMS context ID',
    await protocolConfig.getCurrentKmsContextId(),
    expectedContextId,
  );
  assertEqual(
    'ProtocolConfig migrated context validity',
    await protocolConfig.isValidKmsContext(expectedContextId),
    true,
  );
  assertJsonEqual(
    'ProtocolConfig KMS nodes',
    (await protocolConfig.getKmsNodesForContext(expectedContextId)).map(normalizeKmsNode),
    expectedNodes.map(normalizeKmsNode),
  );
  assertJsonEqual(
    'ProtocolConfig KMS signers',
    normalizeAddresses(await protocolConfig.getKmsSignersForContext(expectedContextId)),
    normalizeAddresses(expectedNodes.map((node) => node.signerAddress)),
  );
  assertBigIntEqual(
    'ProtocolConfig public decryption threshold',
    await protocolConfig.getPublicDecryptionThresholdForContext(expectedContextId),
    BigInt(expectedThresholds.publicDecryption),
  );
  assertBigIntEqual(
    'ProtocolConfig user decryption threshold',
    await protocolConfig.getUserDecryptionThresholdForContext(expectedContextId),
    BigInt(expectedThresholds.userDecryption),
  );
  assertBigIntEqual(
    'ProtocolConfig KMS generation threshold',
    await protocolConfig.getKmsGenThreshold(),
    BigInt(expectedThresholds.kmsGen),
  );
  assertBigIntEqual(
    'ProtocolConfig MPC threshold',
    await protocolConfig.getMpcThreshold(),
    BigInt(expectedThresholds.mpc),
  );

  await assertContractMatchesVersionPrefix(hre, kmsGenerationAddress, 'KMSGeneration');
  const kmsGeneration = await hre.ethers.getContractAt('KMSGeneration', kmsGenerationAddress);

  assertBigIntEqual(
    'KMSGeneration key counter',
    await kmsGeneration.getKeyCounter(),
    expectedKmsGenerationState.keyCounter,
  );
  assertBigIntEqual(
    'KMSGeneration CRS counter',
    await kmsGeneration.getCrsCounter(),
    expectedKmsGenerationState.crsCounter,
  );
  assertBigIntEqual(
    'KMSGeneration active key ID',
    await kmsGeneration.getActiveKeyId(),
    expectedKmsGenerationState.activeKeyId,
  );
  assertBigIntEqual(
    'KMSGeneration active CRS ID',
    await kmsGeneration.getActiveCrsId(),
    expectedKmsGenerationState.activeCrsId,
  );

  const requestDoneChecks = [
    {
      label: 'prep keygen',
      requestId: expectedKmsGenerationState.activePrepKeygenId,
    },
    {
      label: 'key',
      requestId: expectedKmsGenerationState.activeKeyId,
    },
    {
      label: 'CRS',
      requestId: expectedKmsGenerationState.activeCrsId,
    },
  ];

  for (const { label, requestId } of requestDoneChecks) {
    assertEqual(`KMSGeneration ${label} request done`, await kmsGeneration.isRequestDone(requestId), true);
  }

  const consensusTxSenderChecks = [
    {
      label: 'key',
      requestId: expectedKmsGenerationState.activeKeyId,
      expectedTxSenders: expectedKmsGenerationState.keyConsensusTxSenders,
    },
    {
      label: 'CRS',
      requestId: expectedKmsGenerationState.activeCrsId,
      expectedTxSenders: expectedKmsGenerationState.crsConsensusTxSenders,
    },
    {
      label: 'prep keygen',
      requestId: expectedKmsGenerationState.activePrepKeygenId,
      expectedTxSenders: expectedKmsGenerationState.prepKeygenConsensusTxSenders,
    },
  ];

  for (const { label, requestId, expectedTxSenders } of consensusTxSenderChecks) {
    assertJsonEqual(
      `KMSGeneration ${label} consensus tx senders`,
      normalizeAddresses(await kmsGeneration.getConsensusTxSenders(requestId)),
      normalizeAddresses(expectedTxSenders),
    );
  }

  assertBigIntEqual(
    'KMSGeneration active key params type',
    BigInt(await kmsGeneration.getKeyParamsType(expectedKmsGenerationState.activeKeyId)),
    BigInt(expectedKmsGenerationState.prepKeygenParamsType),
  );
  assertBigIntEqual(
    'KMSGeneration active CRS params type',
    BigInt(await kmsGeneration.getCrsParamsType(expectedKmsGenerationState.activeCrsId)),
    BigInt(expectedKmsGenerationState.crsParamsType),
  );

  const [activeKeyStorageUrls, activeKeyDigests] = await kmsGeneration.getKeyMaterials(
    expectedKmsGenerationState.activeKeyId,
  );
  assertJsonEqual(
    'KMSGeneration active key storage URLs',
    Array.from(activeKeyStorageUrls),
    storageUrlsForSenders(expectedNodes, expectedKmsGenerationState.keyConsensusTxSenders),
  );
  assertJsonEqual(
    'KMSGeneration active key digests',
    activeKeyDigests.map(normalizeKeyDigest),
    expectedKmsGenerationState.activeKeyDigests.map(normalizeKeyDigest),
  );

  const [activeCrsStorageUrls, activeCrsDigest] = await kmsGeneration.getCrsMaterials(
    expectedKmsGenerationState.activeCrsId,
  );
  assertJsonEqual(
    'KMSGeneration active CRS storage URLs',
    Array.from(activeCrsStorageUrls),
    storageUrlsForSenders(expectedNodes, expectedKmsGenerationState.crsConsensusTxSenders),
  );
  assertEqual(
    'KMSGeneration active CRS digest',
    activeCrsDigest.toLowerCase(),
    expectedKmsGenerationState.activeCrsDigest.toLowerCase(),
  );

  await assertContractMatchesVersionPrefix(hre, kmsVerifierAddress, 'KMSVerifier');
  const kmsVerifier = await hre.ethers.getContractAt('KMSVerifier', kmsVerifierAddress);
  assertBigIntEqual(
    'KMSVerifier current KMS context ID',
    await kmsVerifier.getCurrentKmsContextId(),
    expectedContextId,
  );
  assertBigIntEqual(
    'KMSVerifier public decryption threshold',
    await kmsVerifier.getThreshold(),
    BigInt(expectedThresholds.publicDecryption),
  );
  assertJsonEqual(
    'KMSVerifier KMS signers',
    normalizeAddresses(await kmsVerifier.getKmsSigners()),
    normalizeAddresses(expectedNodes.map((node) => node.signerAddress)),
  );

  console.log('KMS migration verification succeeded.');
});
