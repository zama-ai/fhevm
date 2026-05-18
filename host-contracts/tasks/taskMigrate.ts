import { calculateERC7201StorageLocation } from '@openzeppelin/upgrades-core/dist/utils/erc7201';
import { AbiCoder, FunctionFragment, Interface, type InterfaceAbi, getAddress, keccak256, toBeHex } from 'ethers';
import fs from 'fs';
import { task, types } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';
import path from 'path';

import {
  assertContractMatchesVersionPrefix,
  deployEmptyUUPS,
  ensureAddressesDirectoryExists,
  readExistingHostEnv,
  readHostEnv,
  waitForTaskReady,
} from './taskDeploy';
import { buildKMSGenerationInitializeFromMigrationArgs } from './utils/kmsGenerationMigrationEnv';
import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';
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

function assertEqual<T>(label: string, actual: T, expected: NoInfer<T>): void {
  if (actual !== expected) {
    throw new Error(`${label} mismatch: expected ${expected}, got ${actual}.`);
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

const HOST_KMS_GENERATION_NAMESPACE = 'fhevm.storage.KMSGeneration';
const GATEWAY_KMS_GENERATION_NAMESPACE = 'fhevm_gateway.storage.KMSGeneration';

const GATEWAY_CONFIG_VIEW_ABI = [
  'function getCurrentKmsContextId() view returns (uint256)',
  'function getKmsTxSendersForContext(uint256 contextId) view returns (address[])',
  'function getKmsNodeForContext(uint256 contextId, address kmsTxSenderAddress) view returns (tuple(address txSenderAddress, address signerAddress, string ipAddress, string storageUrl))',
  'function getPublicDecryptionThresholdForContext(uint256 contextId) view returns (uint256)',
  'function getUserDecryptionThresholdForContext(uint256 contextId) view returns (uint256)',
  'function getMpcThreshold() view returns (uint256)',
  'function getKmsGenThreshold() view returns (uint256)',
];

// Post-migration Gateway KMSGeneration is a view-only frozen stub: only the materials/consensus
// getters below are exposed. Counters, active IDs and consensus digests are read via storage
// slots (see KMS_GENERATION_FIELD_OFFSET).
const GATEWAY_KMS_GENERATION_VIEW_ABI = [
  'function getConsensusTxSenders(uint256 requestId) view returns (address[])',
  'function getKeyParamsType(uint256 keyId) view returns (uint8)',
  'function getCrsParamsType(uint256 crsId) view returns (uint8)',
  'function getKeyMaterials(uint256 keyId) view returns (string[], tuple(uint8 keyType, bytes digest)[])',
  'function getCrsMaterials(uint256 crsId) view returns (string[], bytes)',
];

function assertAddressSetsEqual(label: string, actual: readonly string[], expected: readonly string[]): void {
  assertEqual(label, normalizeAddresses(actual).sort().join(','), normalizeAddresses(expected).sort().join(','));
}

// Field offsets within the `KMSGenerationStorage` struct — identical across host
// (`fhevm.storage.KMSGeneration`) and Gateway (`fhevm_gateway.storage.KMSGeneration`).
// Mirrors `KMS_GENERATION_SLOT` in `gateway-contracts/tasks/exportMigrationState.ts`.
const KMS_GENERATION_FIELD_OFFSET = {
  consensusDigest: 3n,
  prepKeygenCounter: 4n,
  keyCounter: 5n,
  keygenIdPairs: 6n,
  activeKeyId: 8n,
  crsCounter: 9n,
  activeCrsId: 12n,
} as const;

type StorageProvider = {
  getStorage: (address: string, slot: string, blockTag?: number | string) => Promise<string>;
};

function kmsGenerationFieldSlot(namespace: string, offset: bigint): string {
  return toBeHex(BigInt(calculateERC7201StorageLocation(namespace)) + offset, 32);
}

function kmsGenerationMappingSlot(namespace: string, offset: bigint, key: bigint): string {
  const baseSlot = kmsGenerationFieldSlot(namespace, offset);
  return keccak256(AbiCoder.defaultAbiCoder().encode(['uint256', 'uint256'], [key, baseSlot]));
}

// Returns a reader bound to one contract (host or Gateway side). Hides the
// provider/address/namespace/blockTag tuple that's constant for an entire side.
function makeKmsGenerationStorageReader(
  provider: StorageProvider,
  contractAddress: string,
  namespace: string,
  blockTag: number | string | undefined,
) {
  return {
    readUint: async (offset: bigint): Promise<bigint> =>
      BigInt(await provider.getStorage(contractAddress, kmsGenerationFieldSlot(namespace, offset), blockTag)),
    readMappingUint: async (offset: bigint, key: bigint): Promise<bigint> =>
      BigInt(await provider.getStorage(contractAddress, kmsGenerationMappingSlot(namespace, offset, key), blockTag)),
    readMappingBytes32: async (offset: bigint, key: bigint): Promise<string> =>
      (
        await provider.getStorage(contractAddress, kmsGenerationMappingSlot(namespace, offset, key), blockTag)
      ).toLowerCase(),
  };
}

task(
  'task:assertKmsMigrationSucceeded',
  'Asserts host ProtocolConfig + KMSGeneration + KMSVerifier match the live Gateway snapshot',
)
  .addParam('gatewayConfigProxy', 'Gateway GatewayConfig proxy address')
  .addParam('gatewayKmsGenerationProxy', 'Gateway KMSGeneration proxy address')
  .addOptionalParam(
    'useInternalProxyAddress',
    'Load host proxy addresses from the addresses/ env file. Defaults to false (read from process.env).',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'gatewayBlockTag',
    'Gateway block to read at. Defaults to metadata.exportBlockNumber from gateway-contracts/migration-state.json if present; else latest with a warning.',
    undefined,
    types.int,
  )
  .setAction(async function (taskArgs, hre) {
    await hre.run('compile:specific', { contract: 'contracts' });

    if (taskArgs.useInternalProxyAddress) {
      loadHostAddresses();
    }
    const protocolConfigAddress = getRequiredEnvVar('PROTOCOL_CONFIG_CONTRACT_ADDRESS');
    const kmsGenerationAddress = getRequiredEnvVar('KMS_GENERATION_CONTRACT_ADDRESS');
    const kmsVerifierAddress = getRequiredEnvVar('KMS_VERIFIER_CONTRACT_ADDRESS');

    const gatewayConfigProxy = getAddress(taskArgs.gatewayConfigProxy);
    const gatewayKmsGenerationProxy = getAddress(taskArgs.gatewayKmsGenerationProxy);

    // On hardhat the mock Gateway shares the in-process network; everywhere else require the URL.
    const rpcUrl = hre.network.name === 'hardhat' ? process.env.GATEWAY_RPC_URL : getRequiredEnvVar('GATEWAY_RPC_URL');
    const gatewayProvider = rpcUrl ? new hre.ethers.JsonRpcProvider(rpcUrl) : hre.ethers.provider;

    let gatewayBlockTag: number | 'latest' = taskArgs.gatewayBlockTag ?? 'latest';
    if (gatewayBlockTag === 'latest') {
      const snapshotPath = path.resolve(hre.config.paths.root, '..', 'gateway-contracts', 'migration-state.json');
      const snapshotBlock = fs.existsSync(snapshotPath)
        ? (JSON.parse(fs.readFileSync(snapshotPath, 'utf8')) as { metadata?: { exportBlockNumber?: number } }).metadata
            ?.exportBlockNumber
        : undefined;
      // Reject snapshots from a different chain (dev runs against an in-process hardhat node at block 0).
      if (snapshotBlock !== undefined && snapshotBlock <= (await gatewayProvider.getBlockNumber())) {
        gatewayBlockTag = snapshotBlock;
      } else {
        console.warn(
          'Gateway block tag defaulting to "latest" — pass --gateway-block-tag for deterministic verification.',
        );
      }
    }
    const gatewayCallOpts = { blockTag: gatewayBlockTag };

    const gatewayConfig = new hre.ethers.Contract(gatewayConfigProxy, GATEWAY_CONFIG_VIEW_ABI, gatewayProvider);
    const gatewayKmsGeneration = new hre.ethers.Contract(
      gatewayKmsGenerationProxy,
      GATEWAY_KMS_GENERATION_VIEW_ABI,
      gatewayProvider,
    );

    const gatewayContextId: bigint = await gatewayConfig.getCurrentKmsContextId(gatewayCallOpts);
    const gatewayKmsTxSenders: string[] = await gatewayConfig.getKmsTxSendersForContext(
      gatewayContextId,
      gatewayCallOpts,
    );
    const gatewayRawNodes = await Promise.all(
      gatewayKmsTxSenders.map((txSender) =>
        gatewayConfig.getKmsNodeForContext(gatewayContextId, txSender, gatewayCallOpts),
      ),
    );
    const gatewayNodes: ProtocolConfigMigrationKmsNode[] = gatewayRawNodes.map(normalizeKmsNode);

    const [
      gatewayPublicDecryptionThreshold,
      gatewayUserDecryptionThreshold,
      gatewayMpcThreshold,
      gatewayKmsGenThreshold,
    ] = (await Promise.all([
      gatewayConfig.getPublicDecryptionThresholdForContext(gatewayContextId, gatewayCallOpts),
      gatewayConfig.getUserDecryptionThresholdForContext(gatewayContextId, gatewayCallOpts),
      gatewayConfig.getMpcThreshold(gatewayCallOpts),
      gatewayConfig.getKmsGenThreshold(gatewayCallOpts),
    ])) as [bigint, bigint, bigint, bigint];

    const gatewayKmsStorage = makeKmsGenerationStorageReader(
      gatewayProvider,
      gatewayKmsGenerationProxy,
      GATEWAY_KMS_GENERATION_NAMESPACE,
      gatewayBlockTag,
    );
    const hostKmsStorage = makeKmsGenerationStorageReader(
      hre.ethers.provider,
      kmsGenerationAddress,
      HOST_KMS_GENERATION_NAMESPACE,
      undefined,
    );

    const gatewayCounterFields = [
      'prepKeygenCounter',
      'keyCounter',
      'crsCounter',
      'activeKeyId',
      'activeCrsId',
    ] as const;
    const [gatewayPrepKeygenCounter, gatewayKeyCounter, gatewayCrsCounter, gatewayActiveKeyId, gatewayActiveCrsId] =
      await Promise.all(gatewayCounterFields.map((f) => gatewayKmsStorage.readUint(KMS_GENERATION_FIELD_OFFSET[f])));

    const gatewayActivePrepKeygenId = await gatewayKmsStorage.readMappingUint(
      KMS_GENERATION_FIELD_OFFSET.keygenIdPairs,
      gatewayActiveKeyId,
    );

    const [
      gatewayKeyConsensusTxSenders,
      gatewayCrsConsensusTxSenders,
      gatewayPrepKeygenConsensusTxSenders,
      gatewayKeyParamsType,
      gatewayCrsParamsType,
    ] = (await Promise.all([
      gatewayKmsGeneration.getConsensusTxSenders(gatewayActiveKeyId, gatewayCallOpts),
      gatewayKmsGeneration.getConsensusTxSenders(gatewayActiveCrsId, gatewayCallOpts),
      gatewayKmsGeneration.getConsensusTxSenders(gatewayActivePrepKeygenId, gatewayCallOpts),
      gatewayKmsGeneration.getKeyParamsType(gatewayActiveKeyId, gatewayCallOpts),
      gatewayKmsGeneration.getCrsParamsType(gatewayActiveCrsId, gatewayCallOpts),
    ])) as [string[], string[], string[], number | bigint, number | bigint];

    const [
      [gatewayActiveKeyStorageUrls, gatewayActiveKeyDigests],
      [gatewayActiveCrsStorageUrls, gatewayActiveCrsDigest],
    ] = (await Promise.all([
      gatewayKmsGeneration.getKeyMaterials(gatewayActiveKeyId, gatewayCallOpts),
      gatewayKmsGeneration.getCrsMaterials(gatewayActiveCrsId, gatewayCallOpts),
    ])) as [[string[], Array<{ keyType: number | bigint; digest: string }>], [string[], string]];

    const gatewaySignerAddresses = gatewayNodes.map((node) => node.signerAddress);

    await assertContractMatchesVersionPrefix(hre, protocolConfigAddress, 'ProtocolConfig');
    const protocolConfig = await hre.ethers.getContractAt('ProtocolConfig', protocolConfigAddress);

    assertEqual(
      'ProtocolConfig current KMS context ID',
      await protocolConfig.getCurrentKmsContextId(),
      gatewayContextId,
    );
    assertEqual(
      'ProtocolConfig migrated context validity',
      await protocolConfig.isValidKmsContext(gatewayContextId),
      true,
    );

    const sortByTxSender = (nodes: readonly ProtocolConfigMigrationKmsNode[]) =>
      [...nodes].sort((a, b) => a.txSenderAddress.localeCompare(b.txSenderAddress));
    const hostNodes = (await protocolConfig.getKmsNodesForContext(gatewayContextId)).map(normalizeKmsNode);
    assertJsonEqual('ProtocolConfig KMS nodes', sortByTxSender(hostNodes), sortByTxSender(gatewayNodes));

    assertAddressSetsEqual(
      'ProtocolConfig KMS signers set',
      await protocolConfig.getKmsSignersForContext(gatewayContextId),
      gatewaySignerAddresses,
    );

    assertEqual(
      'ProtocolConfig public decryption threshold',
      await protocolConfig.getPublicDecryptionThresholdForContext(gatewayContextId),
      gatewayPublicDecryptionThreshold,
    );
    assertEqual(
      'ProtocolConfig user decryption threshold',
      await protocolConfig.getUserDecryptionThresholdForContext(gatewayContextId),
      gatewayUserDecryptionThreshold,
    );
    assertEqual(
      'ProtocolConfig KMS generation threshold',
      await protocolConfig.getKmsGenThreshold(),
      gatewayKmsGenThreshold,
    );
    assertEqual('ProtocolConfig MPC threshold', await protocolConfig.getMpcThreshold(), gatewayMpcThreshold);

    await assertContractMatchesVersionPrefix(hre, kmsGenerationAddress, 'KMSGeneration');
    const kmsGeneration = await hre.ethers.getContractAt('KMSGeneration', kmsGenerationAddress);

    assertEqual('KMSGeneration key counter', await kmsGeneration.getKeyCounter(), gatewayKeyCounter);
    assertEqual('KMSGeneration CRS counter', await kmsGeneration.getCrsCounter(), gatewayCrsCounter);
    assertEqual('KMSGeneration active key ID', await kmsGeneration.getActiveKeyId(), gatewayActiveKeyId);
    assertEqual('KMSGeneration active CRS ID', await kmsGeneration.getActiveCrsId(), gatewayActiveCrsId);

    const [hostActivePrepKeygenId, hostPrepKeygenCounter] = await Promise.all([
      hostKmsStorage.readMappingUint(KMS_GENERATION_FIELD_OFFSET.keygenIdPairs, gatewayActiveKeyId),
      hostKmsStorage.readUint(KMS_GENERATION_FIELD_OFFSET.prepKeygenCounter),
    ]);
    assertEqual('KMSGeneration active prep keygen ID', hostActivePrepKeygenId, gatewayActivePrepKeygenId);
    assertEqual('KMSGeneration prep keygen counter', hostPrepKeygenCounter, gatewayPrepKeygenCounter);

    const requestDoneChecks = [
      { label: 'prep keygen', requestId: gatewayActivePrepKeygenId },
      { label: 'key', requestId: gatewayActiveKeyId },
      { label: 'CRS', requestId: gatewayActiveCrsId },
    ];
    for (const { label, requestId } of requestDoneChecks) {
      assertEqual(`KMSGeneration ${label} request done`, await kmsGeneration.isRequestDone(requestId), true);
    }

    const consensusTxSenderChecks = [
      { label: 'key', requestId: gatewayActiveKeyId, expectedTxSenders: gatewayKeyConsensusTxSenders },
      { label: 'CRS', requestId: gatewayActiveCrsId, expectedTxSenders: gatewayCrsConsensusTxSenders },
      {
        label: 'prep keygen',
        requestId: gatewayActivePrepKeygenId,
        expectedTxSenders: gatewayPrepKeygenConsensusTxSenders,
      },
    ];
    for (const { label, requestId, expectedTxSenders } of consensusTxSenderChecks) {
      assertJsonEqual(
        `KMSGeneration ${label} consensus tx senders`,
        normalizeAddresses(await kmsGeneration.getConsensusTxSenders(requestId)),
        normalizeAddresses(expectedTxSenders),
      );
    }

    const consensusDigestChecks = [
      { label: 'prep keygen', requestId: gatewayActivePrepKeygenId },
      { label: 'key', requestId: gatewayActiveKeyId },
      { label: 'CRS', requestId: gatewayActiveCrsId },
    ];
    const consensusDigestPairs = await Promise.all(
      consensusDigestChecks.map(({ requestId }) =>
        Promise.all([
          hostKmsStorage.readMappingBytes32(KMS_GENERATION_FIELD_OFFSET.consensusDigest, requestId),
          gatewayKmsStorage.readMappingBytes32(KMS_GENERATION_FIELD_OFFSET.consensusDigest, requestId),
        ]),
      ),
    );
    for (const [i, { label }] of consensusDigestChecks.entries()) {
      const [hostDigest, gatewayDigest] = consensusDigestPairs[i];
      assertEqual(`KMSGeneration ${label} consensus digest`, hostDigest, gatewayDigest);
    }

    assertEqual(
      'KMSGeneration active key params type',
      BigInt(await kmsGeneration.getKeyParamsType(gatewayActiveKeyId)),
      BigInt(gatewayKeyParamsType),
    );
    assertEqual(
      'KMSGeneration active CRS params type',
      BigInt(await kmsGeneration.getCrsParamsType(gatewayActiveCrsId)),
      BigInt(gatewayCrsParamsType),
    );

    const [activeKeyStorageUrls, activeKeyDigests] = await kmsGeneration.getKeyMaterials(gatewayActiveKeyId);
    assertJsonEqual(
      'KMSGeneration active key storage URLs',
      Array.from(activeKeyStorageUrls),
      gatewayActiveKeyStorageUrls,
    );
    assertJsonEqual(
      'KMSGeneration active key digests',
      activeKeyDigests.map(normalizeKeyDigest),
      gatewayActiveKeyDigests.map(normalizeKeyDigest),
    );

    const [activeCrsStorageUrls, activeCrsDigest] = await kmsGeneration.getCrsMaterials(gatewayActiveCrsId);
    assertJsonEqual(
      'KMSGeneration active CRS storage URLs',
      Array.from(activeCrsStorageUrls),
      gatewayActiveCrsStorageUrls,
    );
    assertEqual('KMSGeneration active CRS digest', activeCrsDigest.toLowerCase(), gatewayActiveCrsDigest.toLowerCase());

    await assertContractMatchesVersionPrefix(hre, kmsVerifierAddress, 'KMSVerifier');
    const kmsVerifier = await hre.ethers.getContractAt('KMSVerifier', kmsVerifierAddress);
    assertEqual('KMSVerifier current KMS context ID', await kmsVerifier.getCurrentKmsContextId(), gatewayContextId);
    assertEqual(
      'KMSVerifier public decryption threshold',
      await kmsVerifier.getThreshold(),
      gatewayPublicDecryptionThreshold,
    );
    assertAddressSetsEqual('KMSVerifier KMS signers set', await kmsVerifier.getKmsSigners(), gatewaySignerAddresses);

    console.log('KMS migration verification succeeded.');
  });
