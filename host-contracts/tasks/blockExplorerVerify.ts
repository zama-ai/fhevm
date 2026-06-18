import { task, types } from 'hardhat/config';

import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

const PROTOCOL_CONFIG_CONTRACT = 'contracts/ProtocolConfig.sol:ProtocolConfig';
const PROTOCOL_CONFIG_MULTICHAIN_CONTRACT = 'contracts/ProtocolConfigMultichain.sol:ProtocolConfigMultichain';

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
  .addOptionalParam(
    'contract',
    'Fully qualified implementation contract to verify, eg contracts/ProtocolConfig.sol:ProtocolConfig or contracts/ProtocolConfigMultichain.sol:ProtocolConfigMultichain',
    PROTOCOL_CONFIG_CONTRACT,
    types.string,
  )
  .setAction(async function ({ useInternalProxyAddress, contract }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('PROTOCOL_CONFIG_CONTRACT_ADDRESS');
    const implementationProtocolConfigAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationProtocolConfigAddress,
      contract,
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

task('task:verifyAllHostContracts')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'withKmsGeneration',
    'Whether this host deployment includes canonical-host-only KMSGeneration.',
    true,
    types.boolean,
  )
  .addOptionalParam(
    'protocolConfigContract',
    'Fully qualified ProtocolConfig implementation contract to verify.',
    '',
    types.string,
  )
  .setAction(async function ({ useInternalProxyAddress, withKmsGeneration, protocolConfigContract }, hre) {
    const selectedProtocolConfigContract =
      protocolConfigContract || (withKmsGeneration ? PROTOCOL_CONFIG_CONTRACT : PROTOCOL_CONFIG_MULTICHAIN_CONTRACT);

    console.log('Verify ACL contract:');
    try {
      // to not panic if Etherscan throws an error due to already verified implementation
      await hre.run('task:verifyACL', { useInternalProxyAddress });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    try {
      // to not panic if Etherscan throws an error due to already verified implementation
      console.log('Verify FHEVMExecutor contract:');
      await hre.run('task:verifyFHEVMExecutor', { useInternalProxyAddress });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    try {
      // to not panic if Etherscan throws an error due to already verified implementation
      console.log('Verify KMSVerifier contract:');
      await hre.run('task:verifyKMSVerifier', { useInternalProxyAddress });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    try {
      // to not panic if Etherscan throws an error due to already verified implementation
      console.log('Verify InputVerifier contract:');
      await hre.run('task:verifyInputVerifier', { useInternalProxyAddress });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    try {
      // to not panic if Etherscan throws an error due to already verified implementation
      console.log('Verify HCULimit contract:');
      await hre.run('task:verifyHCULimit', { useInternalProxyAddress });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    try {
      // to not panic if Etherscan throws an error due to already verified implementation
      console.log('Verify PauserSet contract:');
      await hre.run('task:verifyPauserSet', { useInternalProxyAddress });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    try {
      // to not panic if Etherscan throws an error due to already verified implementation
      console.log('Verify ProtocolConfig contract:');
      await hre.run('task:verifyProtocolConfig', {
        useInternalProxyAddress,
        contract: selectedProtocolConfigContract,
      });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    if (withKmsGeneration) {
      try {
        // to not panic if Etherscan throws an error due to already verified implementation
        console.log('Verify KMSGeneration contract:');
        await hre.run('task:verifyKMSGeneration', { useInternalProxyAddress });
      } catch (error) {
        console.error('An error occurred:', error);
      }
    }

    console.log('Contract verification done!');
  });
