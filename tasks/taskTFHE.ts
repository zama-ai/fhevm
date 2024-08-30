import fs from 'fs';
import { task } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';
import path from 'path';

task('task:computeACLAddress').setAction(async function (taskArguments: TaskArguments, { ethers }) {
  const deployer = (await ethers.getSigners())[9].address;
  const aclAddress = ethers.getCreateAddress({
    from: deployer,
    nonce: 0, // using nonce of 0 for the ACL contract
  });
  const envFilePath = path.join(__dirname, '../lib/.env.acl');
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
    fs.writeFileSync('./lib/ACLAddress.sol', solidityTemplate, { encoding: 'utf8', flag: 'w' });
    console.log('./lib/ACLAddress.sol file generated successfully!');
  } catch (error) {
    console.error('Failed to write ./lib/ACLAddress.sol', error);
  }
});

task('task:computeTFHEExecutorAddress').setAction(async function (taskArguments: TaskArguments, { ethers }) {
  const deployer = (await ethers.getSigners())[9].address;
  const execAddress = ethers.getCreateAddress({
    from: deployer,
    nonce: 1, // using nonce of 1 for the TFHEExecutor contract
  });
  const envFilePath = path.join(__dirname, '../lib/.env.exec');
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
    fs.writeFileSync('./lib/TFHEExecutorAddress.sol', solidityTemplateCoprocessor, { encoding: 'utf8', flag: 'w' });
    console.log('./lib/TFHEExecutorAddress.sol file generated successfully!');
  } catch (error) {
    console.error('Failed to write ./lib/TFHEExecutorAddress.sol', error);
  }
});

task('task:computeKMSVerifierAddress').setAction(async function (taskArguments: TaskArguments, { ethers }) {
  const deployer = (await ethers.getSigners())[9].address;
  const kmsVerfierAddress = ethers.getCreateAddress({
    from: deployer,
    nonce: 2, // using nonce of 2 for the Kms Verifier contract
  });
  const envFilePath = path.join(__dirname, '../lib/.env.kmsverifier');
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
    fs.writeFileSync('./lib/KMSVerifierAddress.sol', solidityTemplate, { encoding: 'utf8', flag: 'w' });
    console.log('./lib/KMSVerifierAddress.sol file generated successfully!');
  } catch (error) {
    console.error('Failed to write ./lib/KMSVerifierAddress.sol', error);
  }
});

task('task:computeFHEPaymentAddress').setAction(async function (taskArguments: TaskArguments, { ethers }) {
  const deployer = (await ethers.getSigners())[9].address;
  const fhePaymentAddress = ethers.getCreateAddress({
    from: deployer,
    nonce: 3, // using nonce of 3 for the FHEPayment contract
  });
  const envFilePath = path.join(__dirname, '../lib/.env.fhepayment');
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
    fs.writeFileSync('./lib/FHEPaymentAddress.sol', solidityTemplate, { encoding: 'utf8', flag: 'w' });
    console.log('./lib/FHEPaymentAddress.sol file generated successfully!');
  } catch (error) {
    console.error('Failed to write ./lib/FHEPaymentAddress.sol', error);
  }
});
