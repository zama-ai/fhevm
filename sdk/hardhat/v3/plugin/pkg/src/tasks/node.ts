// `hardhat node`, unchanged except for one thing: the fhevm banner. The task creates its connection
// through the network hooks, which deploy the stack; asking for the banner here, before `runSuper`,
// makes that hook print what it just served — one line, or the address table when the run is verbose.

import type { TaskOverrideActionFunction } from 'hardhat/types/tasks';

import { requestNodeBanner } from '../internal/nodeBanner.js';

const nodeAction: TaskOverrideActionFunction = (args, hre, runSuper) => {
  requestNodeBanner(hre.globalOptions.verbosity);
  return runSuper(args);
};

export default nodeAction;
