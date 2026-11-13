// Compiles every contract library-solidity uses: lib/, config/, the host contracts (fhevmTemp/contracts) and
// examples/. It is the only way to prepare fhevmTemp/, which the `build` override in hardhat.config.ts requires.
//
// The host contract addresses (fhevmTemp/addresses/), needed to compile the host contracts and examples/, come from
// deploying the empty host proxies on a throwaway in-process chain (as in HH2). No host contract is upgraded.
import type { HardhatRuntimeEnvironment } from 'hardhat/types/hre';

import { loadEnv } from './env.js';
import { prepareAndCompileAll } from './hostDeployment.js';

export default async function compileAll(_taskArguments: object, hre: HardhatRuntimeEnvironment): Promise<void> {
  loadEnv();
  // A new connection (not `getOrCreate()`): `hardhat test` runs this task before the tests, which then deploy the host
  // contracts on the shared connection; deploying twice on the same chain would change the addresses.
  const connection = await hre.network.create();
  try {
    await prepareAndCompileAll(hre, connection);
  } finally {
    await connection.close();
  }
}
