import fs from 'fs';
import { task, types } from 'hardhat/config';

import { buildCanonicalUpgradeProposal, parseSnapshotArtifact } from './protocolConfigMirror';
import {
  deployEmptyUUPS,
  ensureAddressesDirectoryExists,
  readExistingHostEnv,
  readHostEnv,
  waitForTaskReady,
} from './taskDeploy';
import { getRequiredEnvVar } from './utils/loadVariables';
import { buildProtocolConfigInitializeFromMigrationArgs } from './utils/protocolConfigMigrationEnv';
import { buildUpgradeProposal, printUpgradeProposal, verifyProposalImplementation } from './utils/upgradeProposal';

////////////////////////////////////////////////////////////////////////////////
// Migration empty-proxy bootstrap
////////////////////////////////////////////////////////////////////////////////

task('task:deployEmptyProxiesProtocolConfigKMSGeneration').setAction(async function (_, { ethers, upgrades, run }) {
  ensureAddressesDirectoryExists();

  const existingEnv = readExistingHostEnv();

  const targets = [
    { envKey: 'PROTOCOL_CONFIG_CONTRACT_ADDRESS', setterTask: 'task:setProtocolConfigAddress' },
    { envKey: 'KMS_GENERATION_CONTRACT_ADDRESS', setterTask: 'task:setKMSGenerationAddress' },
    { envKey: 'CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS', setterTask: 'task:setBridgeAddress' },
  ] as const;

  const missingTargets = targets.filter(({ envKey }) => !existingEnv[envKey]);

  if (missingTargets.length === 0) {
    console.warn(
      'Migration bootstrap is a no-op; addresses/.env.host already contains ProtocolConfig, KMSGeneration and ConfidentialBridge. Remove task:deployEmptyProxiesProtocolConfigKMSGeneration once UPGRADE_FROM_TAG includes #2243.',
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
    const preparedUpgrade = await buildUpgradeProposal(hre, {
      proxyAddress,
      contractName: 'ProtocolConfig',
      innerFunctionName: 'initializeFromMigration',
      decodedArgs,
    });

    printUpgradeProposal(preparedUpgrade);
    if (verifyContract) {
      await verifyProposalImplementation(hre, preparedUpgrade, 'contracts/ProtocolConfig.sol:ProtocolConfig');
    }
    return preparedUpgrade;
  });

// DAO path for initializing a non-canonical ProtocolConfig replica from the canonical chain
// (Ethereum). Consumes a reviewed task:exportCanonicalProtocolConfig artifact — not a live RPC
// read — so the DAO executes exactly the state its signers reproduced and diffed. Devnet
// equivalent: task:deployProtocolConfigFromCanonical.
task(
  'task:prepareDeployProtocolConfigFromCanonical',
  'Deploys a ProtocolConfig implementation and prints DAO upgrade calldata from a reviewed canonical snapshot artifact',
)
  .addParam(
    'snapshot',
    'Path to the reviewed task:exportCanonicalProtocolConfig artifact to encode into the DAO payload.',
    undefined,
    types.string,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function ({ snapshot: snapshotPath, verifyContract }, hre) {
    const parsedEnv = readHostEnv();
    const proxyAddress = parsedEnv.PROTOCOL_CONFIG_CONTRACT_ADDRESS;
    // The bootstrap task may have updated addresses/FHEVMHostAddresses.sol, so rebuild
    await hre.run('compile:specific', { contract: 'contracts' });
    const snapshot = parseSnapshotArtifact(fs.readFileSync(snapshotPath, 'utf-8'));
    const preparedUpgrade = await buildCanonicalUpgradeProposal(hre, { snapshot, proxyAddress });

    printUpgradeProposal(preparedUpgrade);
    if (verifyContract) {
      await verifyProposalImplementation(hre, preparedUpgrade, 'contracts/ProtocolConfig.sol:ProtocolConfig');
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
