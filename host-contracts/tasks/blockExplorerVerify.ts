import { task, types } from 'hardhat/config';
import type { HardhatRuntimeEnvironment, TaskArguments } from 'hardhat/types';

import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

task('task:verifyACL')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('ACL_CONTRACT_ADDRESS');
    const implementationACLAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationACLAddress,
      constructorArguments: [],
    });
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });
  });

task('task:verifyFHEVMExecutor')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('FHEVM_EXECUTOR_CONTRACT_ADDRESS');
    const implementationFHEVMExecutorAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationFHEVMExecutorAddress,
      constructorArguments: [],
    });
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });
  });

task('task:verifyKMSVerifier')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('KMS_VERIFIER_CONTRACT_ADDRESS');
    const implementationKMSVerifierAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationKMSVerifierAddress,
      constructorArguments: [],
    });
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });
  });

task('task:verifyInputVerifier')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('INPUT_VERIFIER_CONTRACT_ADDRESS');
    const implementationInputVerifierAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationInputVerifierAddress,
      constructorArguments: [],
    });
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });
  });

task('task:verifyHCULimit')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('HCU_LIMIT_CONTRACT_ADDRESS');
    const implementationHCULimitAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationHCULimitAddress,
      constructorArguments: [],
    });
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });
  });

task('task:verifyPauserSet')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const implementationPauserSetAddress = getRequiredEnvVar('PAUSER_SET_CONTRACT_ADDRESS');
    await run('verify:verify', {
      address: implementationPauserSetAddress,
      constructorArguments: [],
    });
  });

task('task:verifyProtocolConfig')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('PROTOCOL_CONFIG_CONTRACT_ADDRESS');
    const implementationProtocolConfigAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationProtocolConfigAddress,
      constructorArguments: [],
    });
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });
  });

task('task:verifyKMSGeneration')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('KMS_GENERATION_CONTRACT_ADDRESS');
    const implementationKMSGenerationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationKMSGenerationAddress,
      constructorArguments: [],
    });
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });
  });

async function verifyTask(
  hre: HardhatRuntimeEnvironment,
  taskName: string,
  label: string,
  args: TaskArguments,
): Promise<void> {
  try {
    console.log(`Verify ${label} contract:`);
    await hre.run(taskName, args);
  } catch (error) {
    console.error('An error occurred:', error);
  }
}

async function verifyHostContracts(
  hre: HardhatRuntimeEnvironment,
  useInternalProxyAddress: boolean,
): Promise<void> {
  const args = { useInternalProxyAddress };

  await verifyTask(hre, 'task:verifyACL', 'ACL', args);
  await verifyTask(hre, 'task:verifyFHEVMExecutor', 'FHEVMExecutor', args);
  await verifyTask(hre, 'task:verifyKMSVerifier', 'KMSVerifier', args);
  await verifyTask(hre, 'task:verifyInputVerifier', 'InputVerifier', args);
  await verifyTask(hre, 'task:verifyHCULimit', 'HCULimit', args);
  await verifyTask(hre, 'task:verifyPauserSet', 'PauserSet', args);
  await verifyTask(hre, 'task:verifyProtocolConfig', 'ProtocolConfig', args);
}

task('task:verifySecondaryHost')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
    await verifyHostContracts(hre, useInternalProxyAddress);
    console.log('Secondary host contract verification done!');
  });

task('task:verifyCanonicalHost')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
    await verifyHostContracts(hre, useInternalProxyAddress);
    await verifyTask(hre, 'task:verifyKMSGeneration', 'KMSGeneration', {
      useInternalProxyAddress,
    });
    console.log('Canonical host contract verification done!');
  });
