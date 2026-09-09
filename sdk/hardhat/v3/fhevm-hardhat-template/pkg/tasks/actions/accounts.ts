import type { NewTaskActionFunction } from 'hardhat/types/tasks';

const accounts: NewTaskActionFunction = async (_args, hre) => {
  const connection = await hre.network.create();
  const wallets = await connection.viem.getWalletClients();
  for (const wallet of wallets) console.log(wallet.account.address);
};

export default accounts;
