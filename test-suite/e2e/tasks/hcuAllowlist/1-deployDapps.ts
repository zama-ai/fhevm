// Prerequisite of zama-ai/planning-blockchain#1839. Deploys the test dapps A and B with the user wallet and prints their .env lines.
import { task } from 'hardhat/config';

import { requireEnv } from './checks';

task('task:allowlist-deploy-dapps', 'Deploys the test dapps A and B and prints their .env lines').setAction(
  async (_, hre) => {
    await hre.run('compile');
    const user = new hre.ethers.Wallet(requireEnv('DEPLOYER_PRIVATE_KEY'), hre.ethers.provider);
    const factory = await hre.ethers.getContractFactory('HCUAllowlistHostDapp', user);
    for (const name of ['A', 'B']) {
      const dapp = await factory.deploy();
      await dapp.waitForDeployment();
      console.log(`HCU_ALLOWLIST_DAPP_${name}=${await dapp.getAddress()}`);
    }
  },
);
