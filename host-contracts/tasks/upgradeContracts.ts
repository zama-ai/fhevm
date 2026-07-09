import { Interface, Wallet } from 'ethers';
import { task, types } from 'hardhat/config';
import { HardhatRuntimeEnvironment, TaskArguments } from 'hardhat/types';

import { buildProtocolConfigReinitializeArgs } from './taskDeploy';
import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';
import { buildUpgradeProposal, printUpgradeProposal, verifyProposalImplementation } from './utils/upgradeProposal';

const REINITIALIZE_FUNCTION_PREFIX = 'reinitializeV'; // Prefix for reinitialize functions

// This file defines generic tasks that can be used to upgrade the implementation of already deployed contracts.

type AbiFunction = {
  type?: string;
  name?: string;
  inputs?: { type?: string }[];
};

function getImplementationDirectory(input: string): string {
  const colonIndex = input.lastIndexOf('/');
  if (colonIndex !== -1) {
    return input.substring(0, colonIndex);
  }
  return input;
}

function getReinitializeFunction(abi: AbiFunction[]) {
  return abi.find((item) => item.type === 'function' && item.name?.includes(REINITIALIZE_FUNCTION_PREFIX));
}

function getFunctionSignature(fn: AbiFunction): string {
  return `${fn.name}(${(fn.inputs ?? []).map((input) => input.type).join(',')})`;
}

function formatCastArg(arg: unknown): string {
  if (Array.isArray(arg)) {
    return `[${arg.map(formatCastArg).join(',')}]`;
  }
  return String(arg);
}

function shellQuote(arg: string): string {
  return `'${arg.replace(/'/g, `'\\''`)}'`;
}

// Parses a comma-separated integer list task arg (e.g. --dst-eids "30101,30109") into its entries.
function parseCsvIntegers(raw: string | undefined): string[] {
  if (!raw) {
    return [];
  }
  return raw
    .split(',')
    .map((entry) => entry.trim())
    .filter((entry) => entry.length > 0);
}

async function upgradeCurrentToNew(
  proxyAddress: string,
  currentImplementation: string,
  newImplementation: string,
  verifyContract: boolean,
  hre: HardhatRuntimeEnvironment,
  reinitializeArgs: unknown[] = [],
) {
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new Wallet(deployerPrivateKey).connect(hre.ethers.provider);

  console.log(`Importing ${currentImplementation} contract implementation at address ${proxyAddress}...`);
  const currentImplementationFactory = await hre.ethers.getContractFactory(currentImplementation, deployer);
  const currentProxyContract = await hre.upgrades.forceImport(proxyAddress, currentImplementationFactory);
  console.log('Proxy contract successfully loaded!');

  console.log(
    `Upgrading proxy to "${newImplementation}" implementation with reinitialize arguments:`,
    reinitializeArgs,
  );

  // Get reinitialize function from the new implementation artifact
  const newImplementationArtifact = await hre.artifacts.readArtifact(newImplementation);
  const reinitializeFunction = getReinitializeFunction(newImplementationArtifact.abi);
  if (!reinitializeFunction?.name) {
    throw new Error(`No reinitialize function found in ${newImplementation}`);
  }

  // Prepare the new implementation factory and execute the upgrade by calling the reinitialize function
  const newImplementationFactory = await hre.ethers.getContractFactory(newImplementation, deployer);

  await hre.upgrades.upgradeProxy(currentProxyContract, newImplementationFactory, {
    call: {
      fn: reinitializeFunction.name,
      args: reinitializeArgs,
    },
  });
  console.log('Proxy contract successfully upgraded!');

  if (verifyContract) {
    console.log('Waiting 2 minutes before contract verification... Please wait...');
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    const implementationAddress = await hre.upgrades.erc1967.getImplementationAddress(proxyAddress);
    await hre.run('verify:verify', {
      address: implementationAddress,
      contract: newImplementation,
      constructorArguments: [],
    });
  }
}

// Relies on incremental compilation: run only on a clean working tree whose generated
// addresses/FHEVMHostAddresses.sol matches the target environment, otherwise the implementation
// embeds the wrong addresses.
async function deployImplementationForPreparedUpgrade(
  proxyAddress: string,
  expectedArtifactName: string,
  currentImplementation: string,
  newImplementation: string,
  verifyContract: boolean,
  hre: HardhatRuntimeEnvironment,
  reinitializeArgs: unknown[] = [],
): Promise<void> {
  await compileImplementations(currentImplementation, newImplementation, hre);

  await checkImplementationArtifacts(expectedArtifactName, currentImplementation, newImplementation, hre);

  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new Wallet(deployerPrivateKey).connect(hre.ethers.provider);
  const currentImplementationFactory = await hre.ethers.getContractFactory(currentImplementation, deployer);
  await hre.upgrades.forceImport(proxyAddress, currentImplementationFactory);

  const newImplementationArtifact = await hre.artifacts.readArtifact(newImplementation);
  const reinitializeFunction = getReinitializeFunction(newImplementationArtifact.abi);
  if (!reinitializeFunction?.name) {
    throw new Error(`No reinitialize function found in ${newImplementation}`);
  }
  const newImplementationFactory = await hre.ethers.getContractFactory(newImplementation, deployer);

  console.log(`Deploying "${newImplementation}" for prepared upgrade on proxy ${proxyAddress}...`);
  const implementationAddress = String(
    await hre.upgrades.prepareUpgrade(proxyAddress, newImplementationFactory, {
      kind: 'uups',
    }),
  );
  console.log('New implementation deployed at:', implementationAddress);

  const reinitializeFunctionSignature = getFunctionSignature(reinitializeFunction);
  const reinitializeCalldata = hre.ethers.Interface.from(newImplementationArtifact.abi).encodeFunctionData(
    reinitializeFunction.name,
    reinitializeArgs,
  );
  const outerCalldata = new Interface([
    'function upgradeToAndCall(address newImplementation, bytes data) payable',
  ]).encodeFunctionData('upgradeToAndCall', [implementationAddress, reinitializeCalldata]);

  console.log('proxyAddress:', proxyAddress);
  console.log('newImplementationAddress:', implementationAddress);
  console.log('innerFunctionSignature:', reinitializeFunctionSignature);
  console.log(`${reinitializeFunction.name} calldata:`, reinitializeCalldata);
  console.log('upgradeToAndCall(address,bytes) calldata:', outerCalldata);
  console.log(
    `To double check, run: cast calldata ${shellQuote(reinitializeFunctionSignature)} ${reinitializeArgs
      .map((arg) => shellQuote(formatCastArg(arg)))
      .join(' ')}`.trim(),
  );

  if (verifyContract) {
    console.log('Waiting 2 minutes before contract verification... Please wait...');
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    await hre.run('verify:verify', {
      address: implementationAddress,
      contract: newImplementation,
      constructorArguments: [],
    });
  }
}

async function compileImplementations(
  currentImplementation: string,
  newImplementation: string,
  hre: HardhatRuntimeEnvironment,
): Promise<void> {
  await hre.run('compile:specific', { contract: getImplementationDirectory(currentImplementation) });
  await hre.run('compile:specific', { contract: getImplementationDirectory(newImplementation) });
}

async function checkImplementationArtifacts(
  expectedArtifactName: string,
  currentImplementation: string,
  newImplementation: string,
  hre: HardhatRuntimeEnvironment,
): Promise<void> {
  const currentImplementationArtifact = await hre.artifacts.readArtifact(currentImplementation);
  if (currentImplementationArtifact.contractName !== expectedArtifactName) {
    throw new Error(
      `The current implementation artifact does not match the expected contract name "${expectedArtifactName}". Found: ${currentImplementationArtifact.contractName}`,
    );
  }

  const newImplementationArtifact = await hre.artifacts.readArtifact(newImplementation);
  if (newImplementationArtifact.contractName !== expectedArtifactName) {
    throw new Error(
      `The new implementation artifact does not match the expected contract name "${expectedArtifactName}". Found: ${newImplementationArtifact.contractName}`,
    );
  }

  const hasReinitializeFunction = newImplementationArtifact.abi.some(
    (item) => item.type === 'function' && item.name.includes(REINITIALIZE_FUNCTION_PREFIX),
  );
  if (!hasReinitializeFunction) {
    throw new Error(
      `The new implementation artifact does not contain a reinitialize function. Please ensure the contract has a reinitialize function defined.`,
    );
  }
}

// Helper to perform a standard upgrade: compile, check artifacts, load address, upgrade
async function upgradeContract(
  contractName: string,
  addressEnvVar: string,
  taskArgs: TaskArguments,
  hre: HardhatRuntimeEnvironment,
  reinitializeArgs: unknown[] = [],
) {
  await compileImplementations(taskArgs.currentImplementation, taskArgs.newImplementation, hre);
  await checkImplementationArtifacts(contractName, taskArgs.currentImplementation, taskArgs.newImplementation, hre);

  if (taskArgs.useInternalProxyAddress) {
    loadHostAddresses();
  }
  const proxyAddress = getRequiredEnvVar(addressEnvVar);

  await upgradeCurrentToNew(
    proxyAddress,
    taskArgs.currentImplementation,
    taskArgs.newImplementation,
    taskArgs.verifyContract,
    hre,
    reinitializeArgs,
  );
}

async function prepareUpgradeContract(
  contractName: string,
  addressEnvVar: string,
  taskArgs: TaskArguments,
  hre: HardhatRuntimeEnvironment,
  reinitializeArgs: unknown[] = [],
) {
  if (taskArgs.useInternalProxyAddress) {
    loadHostAddresses();
  }
  const proxyAddress = getRequiredEnvVar(addressEnvVar);

  await deployImplementationForPreparedUpgrade(
    proxyAddress,
    contractName,
    taskArgs.currentImplementation,
    taskArgs.newImplementation,
    taskArgs.verifyContract,
    hre,
    reinitializeArgs,
  );
}

task('task:upgradeACL')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/ACL.sol:ACL',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/ACLUpgradedExample.sol:ACLUpgradedExample',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await upgradeContract('ACL', 'ACL_CONTRACT_ADDRESS', taskArgs, hre);
  });

task('task:upgradeFHEVMExecutor')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/FHEVMExecutor.sol:FHEVMExecutor',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/FHEVMExecutorUpgradedExample.sol:FHEVMExecutorUpgradedExample',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await upgradeContract('FHEVMExecutor', 'FHEVM_EXECUTOR_CONTRACT_ADDRESS', taskArgs, hre);
  });

task('task:prepareUpgradeFHEVMExecutor')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/FHEVMExecutor.sol:FHEVMExecutor',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: contracts/FHEVMExecutor.sol:FHEVMExecutor',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await prepareUpgradeContract('FHEVMExecutor', 'FHEVM_EXECUTOR_CONTRACT_ADDRESS', taskArgs, hre);
  });

task('task:prepareUpgradeACL')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/ACL.sol:ACL',
  )
  .addParam('newImplementation', 'The new implementation solidity contract path and name, eg: contracts/ACL.sol:ACL')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await prepareUpgradeContract('ACL', 'ACL_CONTRACT_ADDRESS', taskArgs, hre);
  });

// Governance prepare for the first bridge upgrade: EmptyUUPSProxy -> ConfidentialBridge via
// initializeFromEmptyProxy. New chains get the bridge from task:deployAllHostContracts.
// Any subsequent ConfidentialBridge-to-ConfidentialBridge upgrade should go through prepareUpgradeContract, not this task.
task('task:prepareUpgradeConfidentialBridge')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'dstEids',
    'Comma-separated LayerZero endpoint ids to seed the dstEid → dstChainId map (paired index-by-index with --dst-chain-ids). Empty by default; pairs can also be wired later via task:setDstChainId.',
    '',
    types.string,
  )
  .addOptionalParam(
    'dstChainIds',
    'Comma-separated destination chain ids paired index-by-index with --dst-eids.',
    '',
    types.string,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    if (taskArgs.useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS');
    const lzEndpoint = getRequiredEnvVar('LZ_ENDPOINT_ADDRESS');
    if (!hre.ethers.isAddress(lzEndpoint)) {
      throw new Error(`LZ_ENDPOINT_ADDRESS is not a valid address: ${lzEndpoint}`);
    }

    const dstEids = parseCsvIntegers(taskArgs.dstEids);
    const dstChainIds = parseCsvIntegers(taskArgs.dstChainIds);
    if (dstEids.length !== dstChainIds.length) {
      throw new Error(
        `--dst-eids and --dst-chain-ids must have the same length: got ${dstEids.length} eid(s) and ${dstChainIds.length} chain id(s). initializeFromEmptyProxy would revert with DstChainIdArrayLengthMismatch.`,
      );
    }

    await hre.run('compile:specific', { contract: 'contracts' });

    const preparedUpgrade = await buildUpgradeProposal(hre, {
      proxyAddress,
      contractName: 'ConfidentialBridge',
      innerFunctionName: 'initializeFromEmptyProxy',
      decodedArgs: [dstEids, dstChainIds],
      constructorArgs: [lzEndpoint],
      unsafeAllow: ['constructor', 'state-variable-immutable', 'missing-initializer-call'],
    });

    printUpgradeProposal(preparedUpgrade);
    if (taskArgs.verifyContract) {
      await verifyProposalImplementation(
        hre,
        preparedUpgrade,
        'contracts/bridge/ConfidentialBridge.sol:ConfidentialBridge',
      );
    }
    return preparedUpgrade;
  });

task('task:upgradeKMSVerifier')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/KMSVerifier.sol:KMSVerifier',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/KMSVerifierUpgradedExample.sol:KMSVerifierUpgradedExample',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await upgradeContract('KMSVerifier', 'KMS_VERIFIER_CONTRACT_ADDRESS', taskArgs, hre);
  });

task('task:prepareUpgradeKMSVerifier')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/KMSVerifier.sol:KMSVerifier',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: contracts/KMSVerifier.sol:KMSVerifier',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await prepareUpgradeContract('KMSVerifier', 'KMS_VERIFIER_CONTRACT_ADDRESS', taskArgs, hre);
  });

task('task:upgradeProtocolConfig')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/ProtocolConfig.sol:ProtocolConfig',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/ProtocolConfigUpgradedExample.sol:ProtocolConfigUpgradedExample',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    const reinitializeArgs = buildProtocolConfigReinitializeArgs();
    await upgradeContract('ProtocolConfig', 'PROTOCOL_CONFIG_CONTRACT_ADDRESS', taskArgs, hre, reinitializeArgs);
  });

task('task:prepareUpgradeProtocolConfig')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/ProtocolConfig.sol:ProtocolConfig',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: contracts/ProtocolConfig.sol:ProtocolConfig',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    const reinitializeArgs = buildProtocolConfigReinitializeArgs();
    await prepareUpgradeContract('ProtocolConfig', 'PROTOCOL_CONFIG_CONTRACT_ADDRESS', taskArgs, hre, reinitializeArgs);
  });

task('task:upgradeKMSGeneration')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/KMSGeneration.sol:KMSGeneration',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/KMSGenerationUpgradedExample.sol:KMSGenerationUpgradedExample',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await upgradeContract('KMSGeneration', 'KMS_GENERATION_CONTRACT_ADDRESS', taskArgs, hre);
  });

task('task:prepareUpgradeKMSGeneration')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/KMSGeneration.sol:KMSGeneration',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: contracts/KMSGeneration.sol:KMSGeneration',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await prepareUpgradeContract('KMSGeneration', 'KMS_GENERATION_CONTRACT_ADDRESS', taskArgs, hre);
  });

task('task:upgradeInputVerifier')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/InputVerifier.sol:InputVerifier',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: contracts/InputVerifier2.sol:InputVerifier',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    const initialSigners: string[] = [];
    const numSigners = getRequiredEnvVar('NUM_COPROCESSORS');
    for (let idx = 0; idx < +numSigners; idx++) {
      initialSigners.push(getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`));
    }
    const coprocessorThreshold = getRequiredEnvVar('COPROCESSOR_THRESHOLD');

    await upgradeContract('InputVerifier', 'INPUT_VERIFIER_CONTRACT_ADDRESS', taskArgs, hre, [
      initialSigners,
      coprocessorThreshold,
    ]);
  });

task('task:prepareUpgradeInputVerifier')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/InputVerifier.sol:InputVerifier',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: contracts/InputVerifier2.sol:InputVerifier',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    const initialSigners: string[] = [];
    const numSigners = getRequiredEnvVar('NUM_COPROCESSORS');
    for (let idx = 0; idx < +numSigners; idx++) {
      initialSigners.push(getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`));
    }
    const coprocessorThreshold = getRequiredEnvVar('COPROCESSOR_THRESHOLD');

    await prepareUpgradeContract('InputVerifier', 'INPUT_VERIFIER_CONTRACT_ADDRESS', taskArgs, hre, [
      initialSigners,
      coprocessorThreshold,
    ]);
  });

task('task:upgradeHCULimit')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/HCULimit.sol:HCULimit',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/HCULimitUpgradedExample.sol:HCULimitUpgradedExample',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await upgradeContract('HCULimit', 'HCU_LIMIT_CONTRACT_ADDRESS', taskArgs, hre);
  });

task('task:prepareUpgradeHCULimit')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/HCULimit.sol:HCULimit',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: contracts/HCULimit.sol:HCULimit',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await prepareUpgradeContract('HCULimit', 'HCU_LIMIT_CONTRACT_ADDRESS', taskArgs, hre);
  });
