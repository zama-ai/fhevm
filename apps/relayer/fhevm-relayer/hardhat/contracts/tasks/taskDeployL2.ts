import dotenv from 'dotenv';
import fs from 'fs';
import { task } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';
import path from 'path';

task('task:deployDecryptionManager')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const decryptionManagerFactory = await ethers.getContractFactory('DecryptionManager', deployer);
    const decryptionManager = await decryptionManagerFactory.deploy();
    await decryptionManager.waitForDeployment();
    const decryptionManagerAddress = await decryptionManager.getAddress();

    const envFilePath = path.join(__dirname, '../addressesL2/.env.decryptionmanager');
    const content = `DECRYPTION_MANAGER_ADDRESS=${decryptionManagerAddress}`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log('decryptionManagerAddress written to addressesL2/.env.decryptionmanager successfully!');
    } catch (err) {
      console.error('Failed to write to addressesL2/.env.decryptionmanager:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant DECRYPTION_MANAGER_ADDRESS = ${taskArguments.address};
`;

    try {
      fs.writeFileSync('./addressesL2/DecryptionManagerAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.log('addressesL2/DecryptionManagerAddress.sol file has been generated successfully.');
    } catch (error) {
      console.error('Failed to write addressesL2/DecryptionManagerAddress.sol', error);
    }
  });


  task('task:deployZkPoKManager')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const zkpokManagerFactory = await ethers.getContractFactory('ZkPoKManager', deployer);
    const zkpokManager = await zkpokManagerFactory.deploy();
    await zkpokManager.waitForDeployment();
    const zkpokManagerAddress = await zkpokManager.getAddress();

    const envFilePath = path.join(__dirname, '../addressesL2/.env.zkpoknmanager');
    const content = `ZKPOK_MANAGER_ADDRESS=${zkpokManagerAddress}`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log('zkpokManagerAddress written to addressesL2/.env.zkpoknmanager successfully!');
    } catch (err) {
      console.error('Failed to write to addressesL2/.env.zkpoknmanager:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant ZKPOK_MANAGER_ADDRESS = ${taskArguments.address};
`;

    try {
      fs.writeFileSync('./addressesL2/ZkpokManagerAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.log('addressesL2/ZkpokManagerAddress.sol file has been generated successfully.');
    } catch (error) {
      console.error('Failed to write addressesL2/ZkpokManagerAddress.sol', error);
    }
  });

task('task:addSignersL2')
  .addParam('privateKey', 'The deployer private key')
  .addParam('numSigners', 'Number of KMS signers to add')
  .addOptionalParam(
    'useAddress',
    'Use addresses instead of private keys env variables for kms signers',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'customDecryptionManagerAddress',
    'Use a custom address for the DecryptionManager contract instead of the default one - ie stored inside .env.decryptionmanager',
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('DecryptionManager', deployer);
    let decryptionManagerAdd;
    if (taskArguments.customKmsVerifierAddress) {
      decryptionManagerAdd = taskArguments.customDecryptionManagerAddress;
    } else {
      decryptionManagerAdd = dotenv.parse(
        fs.readFileSync('addressesL2/.env.decryptionmanager'),
      ).DECRYPTION_MANAGER_ADDRESS;
    }
    const decryptionManager = await factory.attach(decryptionManagerAdd);
    for (let idx = 0; idx < taskArguments.numSigners; idx++) {
      if (!taskArguments.useAddress) {
        const privKeySigner = process.env[`PRIVATE_KEY_KMS_SIGNER_${idx}`];
        const kmsSigner = new ethers.Wallet(privKeySigner).connect(ethers.provider);
        const tx = await decryptionManager.addSigner(kmsSigner.address);
        await tx.wait();
        console.log(`KMS signer no${idx} (${kmsSigner.address}) was added to DecryptionManager contract`);
      } else {
        const kmsSignerAddress = process.env[`ADDRESS_KMS_SIGNER_${idx}`];
        const tx = await decryptionManager.addSigner(kmsSignerAddress);
        await tx.wait();
        console.log(`KMS signer no${idx} (${kmsSignerAddress}) was added to DecryptionManager contract`);
      }
    }
  });
