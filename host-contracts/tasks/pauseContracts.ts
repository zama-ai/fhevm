import dotenv from 'dotenv';
import fs from 'fs';
import { task } from 'hardhat/config';
import path from 'path';

import { getRequiredEnvVar } from './utils/loadVariables';

// Pause the ACL contract
task('task:pauseACL')
  .addOptionalParam(
    'useInternalACLAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalACLAddress }, { ethers }) {
    // Get the pauser wallet
    const pauserPrivateKey = getRequiredEnvVar('PAUSER_PRIVATE_KEY');
    const pauser = new ethers.Wallet(pauserPrivateKey).connect(ethers.provider);

    // Get the proxy address
    if (useInternalACLAddress) {
      const envFilePath = path.join('addresses/', `.env.host`);
      if (!fs.existsSync(envFilePath)) {
        throw new Error(`Environment file not found: ${envFilePath}`);
      }
      dotenv.config({ path: envFilePath, override: true });
    }

    const proxyAddress = getRequiredEnvVar('ACL_CONTRACT_ADDRESS');
    // Pause the contract
    const contract = await ethers.getContractAt('ACL', proxyAddress, pauser);
    await contract.pause();

    console.log(`ACL contract successfully paused at address: ${proxyAddress}\n`);
  });

// Unpause the ACL contract
task('task:unpauseACL')
  .addOptionalParam(
    'useInternalACLAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalACLAddress }, { ethers }) {
    // Get the deployer wallet
    const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    // NOTE: this task won't work once ownership will be transferred from initial deployer to the multisig
    const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);

    // Get the proxy address
    // Get the proxy address
    if (useInternalACLAddress) {
      const envFilePath = path.join('addresses/', `.env.host`);
      if (!fs.existsSync(envFilePath)) {
        throw new Error(`Environment file not found: ${envFilePath}`);
      }
      dotenv.config({ path: envFilePath, override: true });
    }

    const proxyAddress = getRequiredEnvVar('ACL_CONTRACT_ADDRESS');

    // Unpause the contract
    const contract = await ethers.getContractAt('ACL', proxyAddress, deployer);
    await contract.unpause();

    console.log(`ACL contract successfully unpaused at address: ${proxyAddress}\n`);
  });
