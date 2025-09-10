import dotenv from 'dotenv';
import fs from 'fs';
import { task } from 'hardhat/config';

import { getRequiredEnvVar } from './utils/loadVariables';

// Pause the ACL contract
task('task:pauseACL').setAction(async function (_, { ethers }) {
  // Get the pauser wallet
  const pauserPrivateKey = getRequiredEnvVar('PAUSER_PRIVATE_KEY');
  const pauser = new ethers.Wallet(pauserPrivateKey).connect(ethers.provider);

  // Get the proxy address
  const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.host'));
  const proxyAddress = parsedEnv.ACL_CONTRACT_ADDRESS;

  // Pause the contract
  const contract = await ethers.getContractAt('ACL', proxyAddress, pauser);
  await contract.pause();

  console.log(`ACL contract successfully paused at address: ${proxyAddress}\n`);
});

// Unpause the ACL contract
task('task:unpauseACL').setAction(async function (_, { ethers }) {
  // Get the deployer wallet
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);

  // Get the proxy address
  const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.host'));
  const proxyAddress = parsedEnv.ACL_CONTRACT_ADDRESS;

  // Unpause the contract
  const contract = await ethers.getContractAt('ACL', proxyAddress, deployer);
  await contract.unpause();

  console.log(`ACL contract successfully unpaused at address: ${proxyAddress}\n`);
});
