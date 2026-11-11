// Prepares the in-process chain for the tests (HH2: task:prepareTestContracts): compiles everything, deploys the
// host contracts (empty proxies, then upgraded to their real implementations) and adds the pausers.
//
// Everything happens on the connection returned by `network.getOrCreate()`: tests running in the same Hardhat process
// get the same connection, so they see these deployments. Run on its own, the chain is discarded when the task ends.
import type { HardhatRuntimeEnvironment } from 'hardhat/types/hre';

import { addHostPausers, loadDeploymentEnv, prepareAndCompileAll, upgradeHostProxies } from './hostDeployment.js';

export default async function prepareTestContracts(
  _taskArguments: object,
  hre: HardhatRuntimeEnvironment,
): Promise<void> {
  loadDeploymentEnv();
  const connection = await hre.network.getOrCreate();
  await prepareAndCompileAll(hre, connection);
  await upgradeHostProxies(hre, connection);
  await addHostPausers(connection);
}
