import { task } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';

task('task:mint')
  .addParam('mint', 'Tokens to mint')
  .setAction(async function (taskArguments: TaskArguments, hre) {
    const { ethers, deployments } = hre;
    const EncryptedERC20 = await deployments.get('EncryptedERC20');

    const signers = await ethers.getSigners();

    const encryptedERC20 = (await ethers.getContractAt('EncryptedERC20', EncryptedERC20.address)) as any;

    await encryptedERC20.connect(signers[0]).mint(+taskArguments.mint);

    console.log('Mint done: ', taskArguments.mint);
  });
