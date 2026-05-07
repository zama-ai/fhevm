import { FunctionFragment, Interface, type InterfaceAbi } from 'ethers';
import { task } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import {
  deployEmptyUUPS,
  ensureAddressesDirectoryExists,
  readExistingHostEnv,
  readHostEnv,
  waitForProtocolConfigUpgradeLanded,
} from './taskDeploy';
import { buildKMSGenerationInitializeFromMigrationArgs } from './utils/kmsGenerationMigrationEnv';
import { getRequiredEnvVar } from './utils/loadVariables';
import { buildProtocolConfigInitializeFromMigrationArgs } from './utils/protocolConfigMigrationEnv';

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
).setAction(async function (_, hre) {
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
  // upgrades.upgradeProxy can return before the upgradeToAndCall tx is mined on interval-mining
  // networks; poll a state-dependent view so the task only returns once the new implementation
  // is live (mirrors task:deployProtocolConfig).
  await waitForProtocolConfigUpgradeLanded(hre, proxyAddress);
  console.log('ProtocolConfig migration code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// KMSGeneration (migration)
////////////////////////////////////////////////////////////////////////////////

task(
  'task:prepareDeployKMSGenerationFromMigration',
  'Deploys a KMSGeneration migration implementation from MIGRATION_* env and prints DAO upgrade calldata without mutating the proxy',
).setAction(async function (_, hre) {
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
