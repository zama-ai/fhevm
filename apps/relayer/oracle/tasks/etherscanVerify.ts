import dotenv from 'dotenv';
import fs from 'fs';
import { task } from 'hardhat/config';

task('task:verifyDecryptionOracle').setAction(async function (taskArguments, { upgrades, run }) {
  const parsedEnvDecryptionOracle = dotenv.parse(fs.readFileSync('addresses/.env.decryptionoracle'));
  const proxyDecryptionOracle = parsedEnvDecryptionOracle.DECRYPTION_ORACLE_ADDRESS;
  const implementationDecryptionOracleAddress = await upgrades.erc1967.getImplementationAddress(proxyDecryptionOracle);
  await run('verify:verify', {
    address: implementationDecryptionOracleAddress,
    constructorArguments: [],
  });
  await run('verify:verify', {
    address: proxyDecryptionOracle,
    constructorArguments: [],
  });
});
