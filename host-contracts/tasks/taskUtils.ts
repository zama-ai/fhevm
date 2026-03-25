import { task, types } from 'hardhat/config';

import { InputVerifier, KMSVerifier } from '../types';
import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

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
  .addParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { ethers }) {
    const factory = await ethers.getContractFactory('./contracts/KMSVerifier.sol:KMSVerifier');
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const kmsAdd = getRequiredEnvVar('KMS_VERIFIER_CONTRACT_ADDRESS');
    const kmsVerifier = factory.attach(kmsAdd).connect(ethers.provider) as KMSVerifier;
    const listCurrentKMSSigners = await kmsVerifier.getKmsSigners();
    console.log('The list of current KMS Signers stored inside KMSVerifier contract is: ', listCurrentKMSSigners);
  });

////////////////////////////////////////////////////////////////////////////////
// CoprocessorSigners
////////////////////////////////////////////////////////////////////////////////

task('task:getCoprocessorSigners')
  .addParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { ethers }) {
    const factory = await ethers.getContractFactory('./contracts/InputVerifier.sol:InputVerifier');
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const inputVerifierAdd = getRequiredEnvVar('INPUT_VERIFIER_CONTRACT_ADDRESS');
    const inputVerifier = factory.attach(inputVerifierAdd).connect(ethers.provider) as InputVerifier;
    const listCurrentCoprocessorSigners = await inputVerifier.getCoprocessorSigners();
    console.log(
      'The list of current Coprocessor Signers stored inside InputVerifier contract is: ',
      listCurrentCoprocessorSigners,
    );
  });
