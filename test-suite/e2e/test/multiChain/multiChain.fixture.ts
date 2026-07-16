import { ethers } from 'ethers';

import { deployContract } from './multiChainHelper';

export interface ChainContracts {
  erc20: ethers.Contract;
  erc20Address: string;
  userDecrypt: ethers.Contract;
  userDecryptAddress: string;
  rand: ethers.Contract;
}

export async function deployChainFixture(
  deployer: ethers.Signer,
): Promise<ChainContracts> {
  const erc20 = await deployContract('EncryptedERC20', deployer, 'Token', 'TKN');
  const erc20Address = await erc20.getAddress();

  const mintTx = await erc20.connect(deployer).getFunction('mint')(1_000_000, { gasLimit: 10_000_000 });
  await mintTx.wait();

  const userDecrypt = await deployContract('UserDecrypt', deployer);
  const userDecryptAddress = await userDecrypt.getAddress();

  const rand = await deployContract('Rand', deployer);

  return { erc20, erc20Address, userDecrypt, userDecryptAddress, rand };
}
