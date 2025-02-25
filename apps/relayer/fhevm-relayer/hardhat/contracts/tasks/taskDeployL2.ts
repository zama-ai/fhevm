import dotenv from 'dotenv';
import fs from 'fs';
import { task } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';
import path from 'path';

function writeEnvFile(envFilePath: string, solFilePath: string, content: string): void {
  try {
    fs.writeFileSync(envFilePath, content, { flag: 'w' });
    console.log(`Content written to ${envFilePath} successfully!`);
  } catch (err) {
    console.error(`Failed to write to ${envFilePath}:`, err);
  }

  const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant ${content};
`;

  try {
    fs.writeFileSync(solFilePath, solidityTemplate, {
      encoding: 'utf8',
      flag: 'w',
    });
    console.log(`${solFilePath} file has been generated successfully.`);
  } catch (error) {
    console.error(`Failed to write ${solFilePath}`, error);
  }
}

task('task:deployDecryptionManager')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const decryptionManagerFactory = await ethers.getContractFactory('DecryptionManager', deployer);
    const decryptionManager = await decryptionManagerFactory.deploy();
    await decryptionManager.waitForDeployment();
    const decryptionManagerAddress = await decryptionManager.getAddress();
    console.log('DecryptionManager contract deployed to:', decryptionManagerAddress);
    const envFilePath = path.join(__dirname, '../addressesL2/.env.decryption_manager');
    const solFilePath = path.join(__dirname, '../addressesL2/DecryptionManagerAddress.sol');
    const content = `DECRYPTION_MANAGER_ADDRESS=${decryptionManagerAddress}`;
    writeEnvFile(envFilePath, solFilePath, content);
  });

// Deploy the HTTPZ contract
task('task:deployHttpz')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const HTTPZ = await ethers.getContractFactory('HTTPZ', deployer);
    const httpz = await HTTPZ.deploy();

    // Wait for the deployment to be confirmed
    await httpz.waitForDeployment();

    const httpzAddress = await httpz.getAddress();

    console.log('HTTPZ contract deployed to:', httpzAddress);

    // Save the HTTPZ address to the .env.httpz file
    const envFilePath = path.join(__dirname, '../addressesL2/.env.httpz');
    const solFilePath = path.join(__dirname, '../addressesL2/HttpzAddress.sol');
    const content = `HTTPZ_ADDRESS=${httpzAddress}`;
    writeEnvFile(envFilePath, solFilePath, content);
  });

// Deploy the ZKPoKManager contract
task('task:deployZkPoKManager')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const parsedEnvHttpz = dotenv.parse(fs.readFileSync('addressesL2/.env.httpz'));
    const httpzAddress = parsedEnvHttpz.HTTPZ_ADDRESS;

    const dummyPaymentManagerAddress = '0x0000000000000000000000000000000000000000';

    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);

    // Deploy ZKPoKManager contract
    const ZKPoKManager = await ethers.getContractFactory('ZKPoKManager', deployer);
    const zkpokManager = await ZKPoKManager.deploy(httpzAddress, dummyPaymentManagerAddress);

    // Wait for the deployment to be confirmed
    await zkpokManager.waitForDeployment();

    const zkpokManagerAddress = await zkpokManager.getAddress();

    console.log('ZKPoKManager contract deployed to:', zkpokManagerAddress);

    // Save the ZKPoKManager address to the .env.zkpok_manager file
    const envFilePath = path.join(__dirname, '../addressesL2/.env.zkpok_manager');
    const solFilePath = path.join(__dirname, '../addressesL2/ZkpokManagerAddress.sol');
    const content = `ZKPOK_MANAGER_ADDRESS=${zkpokManagerAddress}`;
    writeEnvFile(envFilePath, solFilePath, content);
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
    'Use a custom address for the DecryptionManager contract instead of the default one - ie stored inside .env.decryption_manager',
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('DecryptionManager', deployer);
    let decryptionManagerAdd;
    if (taskArguments.customKmsVerifierAddress) {
      decryptionManagerAdd = taskArguments.customDecryptionManagerAddress;
    } else {
      decryptionManagerAdd = dotenv.parse(
        fs.readFileSync('addressesL2/.env.decryption_manager'),
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
