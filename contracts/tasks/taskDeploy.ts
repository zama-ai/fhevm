import { HardhatUpgrades } from '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import { Wallet } from 'ethers';
import fs from 'fs';
import { task, types } from 'hardhat/config';
import type { HardhatEthersHelpers, TaskArguments } from 'hardhat/types';
import path from 'path';

import { InputVerifier, KMSVerifier } from '../types';

async function deployEmptyUUPS(ethers: HardhatEthersHelpers, upgrades: HardhatUpgrades, deployer: Wallet) {
  console.log('Deploying an EmptyUUPS proxy contract...');
  const factory = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const UUPSEmpty = await upgrades.deployProxy(factory, [deployer.address], {
    initializer: 'initialize',
    kind: 'uups',
  });
  await UUPSEmpty.waitForDeployment();
  const UUPSEmptyAddress = await UUPSEmpty.getAddress();
  console.log('EmptyUUPS proxy contract successfully deployed!');
  return UUPSEmptyAddress;
}

task('task:deployEmptyUUPSProxies')
  .addParam('privateKey', 'The deployer private key')
  .addOptionalParam(
    'useCoprocessorAddress',
    'Use addresses instead of private key env variable for coprocessor',
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const aclAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setACLAddress', {
      address: aclAddress,
    });

    const tfheExecutorAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setTFHEExecutorAddress', {
      address: tfheExecutorAddress,
    });

    const kmsVerifierAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setKMSVerifierAddress', {
      address: kmsVerifierAddress,
    });

    const inputVerifierAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setInputVerifierAddress', {
      address: inputVerifierAddress,
      useAddress: taskArguments.useCoprocessorAddress,
    });

    const fheGasLimitAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setFHEGasLimitAddress', {
      address: fheGasLimitAddress,
    });

    const decryptionOracleAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setDecryptionOracleAddress', {
      address: decryptionOracleAddress,
    });
  });

task('task:deployDecryptionOracle')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
    const newImplem = await ethers.getContractFactory('DecryptionOracle', deployer);
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.decryptionoracle'));
    const proxyAddress = parsedEnv.DECRYPTION_ORACLE_ADDRESS;
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    await upgrades.upgradeProxy(proxy, newImplem);
    console.log('DecryptionOracle code set successfully at address:', proxyAddress);
  });

task('task:deployACL')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
    const newImplem = await ethers.getContractFactory('ACL', deployer);
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.acl'));
    const proxyAddress = parsedEnv.ACL_CONTRACT_ADDRESS;
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    await upgrades.upgradeProxy(proxy, newImplem);
    console.log('ACL code set successfully at address:', proxyAddress);
  });

task('task:deployTFHEExecutor')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
    const newImplem = await ethers.getContractFactory('./contracts/TFHEExecutor.sol:TFHEExecutor', deployer);
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.exec'));
    const proxyAddress = parsedEnv.TFHE_EXECUTOR_CONTRACT_ADDRESS;
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    await upgrades.upgradeProxy(proxy, newImplem);
    console.log('TFHEExecutor code set successfully at address:', proxyAddress);
  });

task('task:deployKMSVerifier')
  .addParam('privateKey', 'The deployer private key')
  .addParam('decryptionManagerAddress', 'The decryption manager contract address from the Gateway chain')
  .addOptionalParam(
    'useAddress',
    'Use addresses instead of private keys env variables for kms signers',
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
    const newImplem = await ethers.getContractFactory('KMSVerifier', deployer);
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.kmsverifier'));
    const proxyAddress = parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS;
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    const verifyingContractSource = taskArguments.decryptionManagerAddress;
    const parsedEnv2 = dotenv.parse(fs.readFileSync('.env'));
    const chainIDSource = +parsedEnv2.CHAIN_ID_GATEWAY;
    const initialThreshold = +parsedEnv2.INITIAL_KMS_THRESHOLD;
    let initialSigners: string[] = [];
    const numSigners = +parsedEnv2.NUM_KMS_SIGNERS;
    for (let idx = 0; idx < numSigners; idx++) {
      if (!taskArguments.useAddress) {
        const privKeySigner = parsedEnv2[`PRIVATE_KEY_KMS_SIGNER_${idx}`];
        const kmsSigner = new ethers.Wallet(privKeySigner).connect(ethers.provider);
        initialSigners.push(kmsSigner.address);
      } else {
        const kmsSignerAddress = parsedEnv2[`ADDRESS_KMS_SIGNER_${idx}`];
        initialSigners.push(kmsSignerAddress);
      }
    }
    await upgrades.upgradeProxy(proxy, newImplem, {
      call: { fn: 'reinitialize', args: [verifyingContractSource, chainIDSource, initialSigners, initialThreshold] },
    });
    console.log('KMSVerifier code set successfully at address:', proxyAddress);
    console.log(`${numSigners} KMS signers were added to KMSVerifier at initialization`);
  });

task('task:deployInputVerifier')
  .addParam('privateKey', 'The deployer private key')
  .addParam('zkpokManagerAddress', 'The ZKPOK manager contract address from the Gateway chain')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
    const newImplem = await ethers.getContractFactory('./contracts/InputVerifier.sol:InputVerifier', deployer);
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.inputverifier'));
    const proxyAddress = parsedEnv.INPUT_VERIFIER_CONTRACT_ADDRESS;
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    const verifyingContractSource = taskArguments.zkpokManagerAddress;
    const parsedEnv2 = dotenv.parse(fs.readFileSync('.env'));
    const chainIDSource = +parsedEnv2.CHAIN_ID_GATEWAY;

    let initialSigners: string[] = [];
    const numSigners = +parsedEnv2.NUM_KMS_SIGNERS;
    for (let idx = 0; idx < numSigners; idx++) {
      if (!taskArguments.useAddress) {
        const privKeySigner = parsedEnv2[`PRIVATE_KEY_COPROCESSOR_ACCOUNT_${idx}`];
        const inputSigner = new ethers.Wallet(privKeySigner).connect(ethers.provider);
        initialSigners.push(inputSigner.address);
      } else {
        const inputSignerAddress = parsedEnv2[`ADDRESS_COPROCESSOR_ACCOUNT_${idx}`];
        initialSigners.push(inputSignerAddress);
      }
    }

    await upgrades.upgradeProxy(proxy, newImplem, {
      call: { fn: 'reinitialize', args: [verifyingContractSource, chainIDSource, initialSigners] },
    });
    console.log('InputVerifier code set successfully at address:', proxyAddress);
  });

task('task:deployFHEGasLimit')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
    const newImplem = await ethers.getContractFactory('FHEGasLimit', deployer);
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.fhegaslimit'));
    const proxyAddress = parsedEnv.FHE_GASLIMIT_CONTRACT_ADDRESS;
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    await upgrades.upgradeProxy(proxy, newImplem);
    console.log('FHEGasLimit code set successfully at address:', proxyAddress);
  });

task('task:addSigners')
  .addParam('privateKey', 'The deployer private key')
  .addParam('numSigners', 'Number of KMS signers to add')
  .addOptionalParam(
    'useAddress',
    'Use addresses instead of private keys env variables for kms signers',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'customKmsVerifierAddress',
    'Use a custom address for the KMSVerifier contract instead of the default one - ie stored inside .env.kmsverifier',
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('./contracts/KMSVerifier.sol:KMSVerifier', deployer);
    let kmsAdd;
    if (taskArguments.customKmsVerifierAddress) {
      kmsAdd = taskArguments.customKmsVerifierAddress;
    } else {
      kmsAdd = dotenv.parse(fs.readFileSync('addresses/.env.kmsverifier')).KMS_VERIFIER_CONTRACT_ADDRESS;
    }
    const kmsVerifier = await factory.attach(kmsAdd);
    for (let idx = 0; idx < taskArguments.numSigners; idx++) {
      if (!taskArguments.useAddress) {
        const privKeySigner = process.env[`PRIVATE_KEY_KMS_SIGNER_${idx}`];
        const kmsSigner = new ethers.Wallet(privKeySigner).connect(ethers.provider);
        const tx = await kmsVerifier.addSigner(kmsSigner.address);
        await tx.wait();
        console.log(`KMS signer no${idx} (${kmsSigner.address}) was added to KMSVerifier contract`);
      } else {
        const kmsSignerAddress = process.env[`ADDRESS_KMS_SIGNER_${idx}`];
        const tx = await kmsVerifier.addSigner(kmsSignerAddress);
        await tx.wait();
        console.log(`KMS signer no${idx} (${kmsSignerAddress}) was added to KMSVerifier contract`);
      }
    }
  });

task('task:getAllSigners')
  .addOptionalParam(
    'customKmsVerifierAddress',
    'Use a custom address for the KMSVerifier contract instead of the default one - ie stored inside .env.kmsverifier',
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const factory = await ethers.getContractFactory('./contracts/KMSVerifier.sol:KMSVerifier');
    let kmsAdd;
    if (taskArguments.customKmsVerifierAddress) {
      kmsAdd = taskArguments.customKmsVerifierAddress;
    } else {
      kmsAdd = dotenv.parse(fs.readFileSync('addresses/.env.kmsverifier')).KMS_VERIFIER_CONTRACT_ADDRESS;
    }
    const kmsVerifier = (await factory.attach(kmsAdd).connect(ethers.provider)) as KMSVerifier;
    const listCurrentKMSSigners = await kmsVerifier.getSigners();
    console.log('The list of current KMS Signers stored inside KMSVerifier contract is: ', listCurrentKMSSigners);
  });

task('task:removeSigner')
  .addParam('privateKey', 'The KMSVerifier owner private key')
  .addParam('kmsSignerAddress', 'The KMS Signer address you wish to remove')
  .addOptionalParam(
    'customKmsVerifierAddress',
    'Use a custom address for the KMSVerifier contract instead of the default one - ie stored inside .env.kmsverifier',
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('./contracts/KMSVerifier.sol:KMSVerifier', deployer);
    let kmsAdd;
    if (taskArguments.customKmsVerifierAddress) {
      kmsAdd = taskArguments.customKmsVerifierAddress;
    } else {
      kmsAdd = dotenv.parse(fs.readFileSync('addresses/.env.kmsverifier')).KMS_VERIFIER_CONTRACT_ADDRESS;
    }
    const kmsVerifier = (await factory.attach(kmsAdd)) as KMSVerifier;
    const tx = await kmsVerifier.removeSigner(taskArguments.kmsSignerAddress);
    await tx.wait();
    console.log(`KMS signer with address (${taskArguments.kmsSignerAddress}) was removed from KMSVerifier contract`);
  });

task('task:setDecryptionOracleAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../addresses/.env.decryptionoracle');
    const content = `DECRYPTION_ORACLE_ADDRESS=${taskArguments.address}`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log('decryptionOracleAddress written to addresses/.env.decryptionoracle successfully!');
    } catch (err) {
      console.error('Failed to write to addresses/.env.decryptionoracle:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant DECRYPTION_ORACLE_ADDRESS = ${taskArguments.address};
`;

    try {
      fs.writeFileSync('./addresses/DecryptionOracleAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.log('addresses/DecryptionOracleAddress.sol file has been generated successfully.');
    } catch (error) {
      console.error('Failed to write addresses/DecryptionOracleAddress.sol', error);
    }
  });

task('task:setACLAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../addresses/.env.acl');
    const content = `ACL_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log(`ACL address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write ACL address:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant aclAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./addresses/ACLAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.log('./addresses/ACLAddress.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./addresses/ACLAddress.sol', error);
    }
  });

task('task:setTFHEExecutorAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../addresses/.env.exec');
    const content = `TFHE_EXECUTOR_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log(`TFHEExecutor address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write TFHEExecutor address:', err);
    }

    const solidityTemplateCoprocessor = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant tfheExecutorAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./addresses/TFHEExecutorAddress.sol', solidityTemplateCoprocessor, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.log('./addresses/TFHEExecutorAddress.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./addresses/TFHEExecutorAddress.sol', error);
    }
  });

task('task:setKMSVerifierAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../addresses/.env.kmsverifier');
    const content = `KMS_VERIFIER_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log(`KMSVerifier address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write KMSVerifier address:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant kmsVerifierAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./addresses/KMSVerifierAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.log('./addresses/KMSVerifierAddress.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./addresses/KMSVerifierAddress.sol', error);
    }
  });

task('task:setInputVerifierAddress')
  .addParam('address', 'The address of the contract')
  .addOptionalParam(
    'useAddress',
    'Use addresses instead of private key env variable for coprocessor',
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    // this script also computes the coprocessor address from its private key
    const envFilePath = path.join(__dirname, '../addresses/.env.inputverifier');
    const content = `INPUT_VERIFIER_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log(`InputVerifier address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write InputVerifier address:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant inputVerifierAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./addresses/InputVerifierAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.log('./addresses/InputVerifierAddress.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./addresses/InputVerifierAddress.sol', error);
    }

    if (process.env.IS_COPROCESSOR) {
      let coprocAddress;
      if (!taskArguments.useAddress) {
        coprocAddress = new ethers.Wallet(process.env.PRIVATE_KEY_COPROCESSOR_ACCOUNT!).address;
      } else {
        coprocAddress = process.env.ADDRESS_COPROCESSOR_ACCOUNT;
      }
      const envFilePath2 = path.join(__dirname, '../addresses/.env.coprocessor');
      const content2 = `COPROCESSOR_ADDRESS=${coprocAddress}\n`;
      try {
        fs.writeFileSync(envFilePath2, content2, { flag: 'w' });
        console.log(`Coprocessor address ${coprocAddress} written successfully!`);
      } catch (err) {
        console.error('Failed to write InputVerifier address:', err);
      }

      const solidityTemplate2 = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant coprocessorAdd = ${coprocAddress};\n`;

      try {
        fs.writeFileSync('./addresses/CoprocessorAddress.sol', solidityTemplate2, {
          encoding: 'utf8',
          flag: 'w',
        });
        console.log('./addresses/CoprocessorAddress.sol file generated successfully!');
      } catch (error) {
        console.error('Failed to write ./addresses/CoprocessorAddress.sol', error);
      }
    }
  });

task('task:setFHEGasLimitAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../addresses/.env.fhegaslimit');
    const content = `FHE_GASLIMIT_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log(`FHEGasLimit address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write FHEGasLimit address:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant fheGasLimitAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./addresses/FHEGasLimitAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.log('./addresses/FHEGasLimitAddress.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./addresses/FHEGasLimitAddress.sol', error);
    }
  });

task('task:addInputSigners')
  .addParam('privateKey', 'The deployer private key')
  .addParam('numSigners', 'Number of coprocessor signers to add')
  .addOptionalParam(
    'useAddress',
    'Use addresses instead of private keys env variables for kms signers',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'customInputVerifierAddress',
    'Use a custom address for the InputVerifier contract instead of the default one - ie stored inside .env.inputverifier',
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('./contracts/InputVerifier.sol:InputVerifier', deployer);
    let inputAdd;
    if (taskArguments.customInputVerifierAddress) {
      inputAdd = taskArguments.customInputVerifierAddress;
    } else {
      inputAdd = dotenv.parse(fs.readFileSync('addresses/.env.inputverifier')).INPUT_VERIFIER_CONTRACT_ADDRESS;
    }
    const inputVerifier = (await factory.attach(inputAdd)) as InputVerifier;
    for (let idx = 0; idx < taskArguments.numSigners; idx++) {
      if (!taskArguments.useAddress) {
        const privKeySigner = process.env[`PRIVATE_KEY_COPROCESSOR_ACCOUNT_${idx}`]!;
        const inputSigner = new ethers.Wallet(privKeySigner).connect(ethers.provider);
        const tx = await inputVerifier.addSigner(inputSigner.address);
        await tx.wait();
        console.log(`Coprocessor signer no${idx} (${inputSigner.address}) was added to InputVerifier contract`);
      } else {
        const inputSignerAddress = process.env[`ADDRESS_COPROCESSOR_ACCOUNT_${idx}`];
        const tx = await inputVerifier.addSigner(inputSignerAddress);
        await tx.wait();
        console.log(`Coprocessor signer no${idx} (${inputSignerAddress}) was added to InputVerifier contract`);
      }
    }
  });
