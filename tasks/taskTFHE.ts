import fs from 'fs';
import { task, types } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';
import path from 'path';

task('task:computeACLAddress')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).address;
    const aclAddress = ethers.getCreateAddress({
      from: deployer,
      nonce: 1, // using nonce of 1 for the ACL contract (0 for original implementation, +1 for proxy)
    });
    const envFilePath = path.join(__dirname, '../node_modules/fhevm-core-contracts/addresses/.env.acl');
    const content = `ACL_CONTRACT_ADDRESS=${aclAddress}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log(`ACL address ${aclAddress} written successfully!`);
    } catch (err) {
      console.error('Failed to write ACL address:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant aclAdd = ${aclAddress};\n`;

    try {
      fs.writeFileSync('./node_modules/fhevm-core-contracts/addresses/ACLAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.log('./node_modules/fhevm-core-contracts/addresses/ACLAddress.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./node_modules/fhevm-core-contracts/addresses/ACLAddress.sol', error);
    }
  });

task('task:computeTFHEExecutorAddress')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).address;
    const execAddress = ethers.getCreateAddress({
      from: deployer,
      nonce: 3, // using nonce of 3 for the TFHEExecutor contract (2 for original implementation, +1 for proxy)
    });
    const envFilePath = path.join(__dirname, '../node_modules/fhevm-core-contracts/addresses/.env.exec');
    const content = `TFHE_EXECUTOR_CONTRACT_ADDRESS=${execAddress}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log(`TFHEExecutor address ${execAddress} written successfully!`);
    } catch (err) {
      console.error('Failed to write TFHEExecutor address:', err);
    }

    const solidityTemplateCoprocessor = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant tfheExecutorAdd = ${execAddress};\n`;

    try {
      fs.writeFileSync(
        './node_modules/fhevm-core-contracts/addresses/TFHEExecutorAddress.sol',
        solidityTemplateCoprocessor,
        { encoding: 'utf8', flag: 'w' },
      );
      console.log('./node_modules/fhevm-core-contracts/addresses/TFHEExecutorAddress.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./node_modules/fhevm-core-contracts/addresses/TFHEExecutorAddress.sol', error);
    }
  });

task('task:computeKMSVerifierAddress')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).address;
    const kmsVerfierAddress = ethers.getCreateAddress({
      from: deployer,
      nonce: 5, // using nonce of 5 for the KMSVerifier contract (4 for original implementation, +1 for proxy)
    });
    const envFilePath = path.join(__dirname, '../node_modules/fhevm-core-contracts/addresses/.env.kmsverifier');
    const content = `KMS_VERIFIER_CONTRACT_ADDRESS=${kmsVerfierAddress}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log(`KMSVerifier address ${kmsVerfierAddress} written successfully!`);
    } catch (err) {
      console.error('Failed to write KMSVerifier address:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant kmsVerifierAdd = ${kmsVerfierAddress};\n`;

    try {
      fs.writeFileSync('./node_modules/fhevm-core-contracts/addresses/KMSVerifierAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.log('./node_modules/fhevm-core-contracts/addresses/KMSVerifierAddress.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./node_modules/fhevm-core-contracts/addresses/KMSVerifierAddress.sol', error);
    }
  });

task('task:computeInputVerifierAddress')
  .addParam('privateKey', 'The deployer private key')
  .addOptionalParam(
    'useAddress',
    'Use addresses instead of private key env variable for coprocessor',
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    // this script also compute the coprocessor address from its private key
    const deployer = new ethers.Wallet(taskArguments.privateKey).address;
    const inputVerfierAddress = ethers.getCreateAddress({
      from: deployer,
      nonce: 7, // using nonce of 7 for the InputVerifier contract (6 for original implementation, +1 for proxy)
    });
    const envFilePath = path.join(__dirname, '../node_modules/fhevm-core-contracts/addresses/.env.inputverifier');
    const content = `INPUT_VERIFIER_CONTRACT_ADDRESS=${inputVerfierAddress}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log(`InputVerifier address ${inputVerfierAddress} written successfully!`);
    } catch (err) {
      console.error('Failed to write InputVerifier address:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant inputVerifierAdd = ${inputVerfierAddress};\n`;

    try {
      fs.writeFileSync('./node_modules/fhevm-core-contracts/addresses/InputVerifierAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.log(
        './node_modules/fhevm-core-contracts/addresses/InputVerifierAddress.sol file generated successfully!',
      );
    } catch (error) {
      console.error('Failed to write ./node_modules/fhevm-core-contracts/addresses/InputVerifierAddress.sol', error);
    }
    let coprocAddress;
    if (!taskArguments.useAddress) {
      coprocAddress = new ethers.Wallet(process.env.PRIVATE_KEY_COPROCESSOR_ACCOUNT!).address;
    } else {
      coprocAddress = process.env.ADDRESS_COPROCESSOR_ACCOUNT;
    }
    const envFilePath2 = path.join(__dirname, '../node_modules/fhevm-core-contracts/addresses/.env.coprocessor');
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
      fs.writeFileSync('./node_modules/fhevm-core-contracts/addresses/CoprocessorAddress.sol', solidityTemplate2, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.log('./node_modules/fhevm-core-contracts/addresses/CoprocessorAddress.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./node_modules/fhevm-core-contracts/addresses/CoprocessorAddress.sol', error);
    }
  });

task('task:computeFHEPaymentAddress')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).address;
    const fhePaymentAddress = ethers.getCreateAddress({
      from: deployer,
      nonce: 9, // using nonce of 9 for the FHEPayment contract (8 for original implementation, +1 for proxy)
    });
    const envFilePath = path.join(__dirname, '../node_modules/fhevm-core-contracts/addresses/.env.fhepayment');
    const content = `FHE_PAYMENT_CONTRACT_ADDRESS=${fhePaymentAddress}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log(`FHEPayment address ${fhePaymentAddress} written successfully!`);
    } catch (err) {
      console.error('Failed to write FHEPayment address:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant fhePaymentAdd = ${fhePaymentAddress};\n`;

    try {
      fs.writeFileSync('./node_modules/fhevm-core-contracts/addresses/FHEPaymentAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.log('./node_modules/fhevm-core-contracts/addresses/FHEPaymentAddress.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./node_modules/fhevm-core-contracts/addresses/FHEPaymentAddress.sol', error);
    }
  });
