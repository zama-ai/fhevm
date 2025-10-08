import fs from 'fs';
import { task } from 'hardhat/config';
import path from 'path';

import { getRequiredEnvVar } from './utils/loadVariables';

task('task:deployDecryptionOracle').setAction(async function (_, { ethers, upgrades, run }) {
  await run('compile');
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const factory = await ethers.getContractFactory('DecryptionOracle', deployer);
  const decryptionOracle = await upgrades.deployProxy(factory, [deployer.address], {
    initializer: 'initialize',
    kind: 'uups',
  });
  await decryptionOracle.waitForDeployment();
  const proxyAddress = await decryptionOracle.getAddress();
  console.log('DecryptionOracle code set successfully at address:', proxyAddress);
  // Ensure the addresses/ directory exists or create it
  fs.mkdirSync('./addresses', { recursive: true });
  const envFilePath = path.join(__dirname, '../addresses/.env.decryptionoracle');
  const content = `DECRYPTION_ORACLE_ADDRESS=${proxyAddress}`;
  try {
    fs.writeFileSync(envFilePath, content, { flag: 'w' });
    console.log('decryptionOracleAddress written to addresses/.env.decryptionoracle successfully!');
  } catch (err) {
    console.error('Failed to write to addresses/.env.decryptionoracle:', err);
  }

  const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant DECRYPTION_ORACLE_ADDRESS = ${proxyAddress};
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
