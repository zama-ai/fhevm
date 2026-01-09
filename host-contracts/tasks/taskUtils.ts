import dotenv from 'dotenv';
import fs from 'fs';
import { task } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';

import { InputVerifier, KMSVerifierV2 } from '../types';

////////////////////////////////////////////////////////////////////////////////
// Faucet
////////////////////////////////////////////////////////////////////////////////

task('task:faucetToPrivate')
  .addParam('privateKey', 'The receiver private key')
  .setAction(async function (taskArgs, hre) {
    const receiverAddress = new hre.ethers.Wallet(taskArgs.privateKey).address;

    if (hre.network.name === 'hardhat') {
      const bal = '0x1000000000000000000000000000000000000000';
      await hre.network.provider.send('hardhat_setBalance', [receiverAddress, bal]);
    } else {
      throw new Error('The faucetToPrivate task is only meant to be used with a hardhat network');
    }
  });

////////////////////////////////////////////////////////////////////////////////
// KMSSigners
////////////////////////////////////////////////////////////////////////////////

task('task:getKmsSigners')
  .addOptionalParam(
    'customKmsVerifierAddress',
    'Use a custom address for the KMSVerifierV2 contract instead of the default one - ie stored inside .env.host',
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const factory = await ethers.getContractFactory('./contracts/KMSVerifierV2.sol:KMSVerifierV2');
    let kmsAdd;
    if (taskArguments.customKmsVerifierAddress) {
      kmsAdd = taskArguments.customKmsVerifierAddress;
    } else {
      kmsAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).KMS_VERIFIER_CONTRACT_ADDRESS;
    }
    const kmsVerifier = factory.attach(kmsAdd).connect(ethers.provider) as KMSVerifierV2;
    const listCurrentKMSSigners = await kmsVerifier.getKmsSigners();
    console.log('The list of current KMS Signers stored inside KMSVerifierV2 contract is: ', listCurrentKMSSigners);
  });

////////////////////////////////////////////////////////////////////////////////
// CoprocessorSigners
////////////////////////////////////////////////////////////////////////////////

task('task:getCoprocessorSigners')
  .addOptionalParam(
    'customInputVerifierAddress',
    'Use a custom address for the InputVerifier contract instead of the default one - ie stored inside .env.host',
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const factory = await ethers.getContractFactory('./contracts/InputVerifier.sol:InputVerifier');
    let inputVerifierAdd;
    if (taskArguments.customInputVerifierAddress) {
      inputVerifierAdd = taskArguments.customInputVerifierAddress;
    } else {
      inputVerifierAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).INPUT_VERIFIER_CONTRACT_ADDRESS;
    }
    const inputVerifier = factory.attach(inputVerifierAdd).connect(ethers.provider) as InputVerifier;
    const listCurrentCoprocessorSigners = await inputVerifier.getCoprocessorSigners();
    console.log(
      'The list of current Coprocessor Signers stored inside InputVerifier contract is: ',
      listCurrentCoprocessorSigners,
    );
  });
