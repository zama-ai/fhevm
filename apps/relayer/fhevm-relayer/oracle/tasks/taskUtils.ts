import { task } from 'hardhat/config';

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
