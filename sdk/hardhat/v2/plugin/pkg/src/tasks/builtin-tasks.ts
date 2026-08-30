import setupDebug from 'debug';
import * as fs from 'fs';
import {
  TASK_CLEAN,
  TASK_COMPILE_GET_REMAPPINGS,
  TASK_COMPILE_SOLIDITY_GET_SOURCE_PATHS,
  TASK_NODE_GET_PROVIDER,
  TASK_NODE_SERVER_READY,
  TASK_TEST,
} from 'hardhat/builtin-tasks/task-names';
import { subtask, task } from 'hardhat/config';
import type { HardhatRuntimeEnvironment, TaskArguments } from 'hardhat/types';
import * as picocolors from 'picocolors';

import { HardhatFhevmError } from '../error';
import { fhevmContext } from '../internal/EnvironmentExtender';
import constants from '../internal/constants';
import { assertHHFhevm } from '../internal/error';
import { checkSolidityCoverageSettings } from '../internal/utils/solidityCoverage';

const debug = setupDebug('@fhevm/hardhat:builtin-tasks');

// `TaskArguments` is hardhat's `any`, so this action names the one argument it reads.
task(TASK_TEST, async (taskArgs: { parallel?: boolean }, hre: HardhatRuntimeEnvironment, runSuper) => {
  // Not supported for the moment. Much too tricky. This would generate tons of support.
  if (taskArgs.parallel === true && hre.network.name === 'hardhat') {
    throw new HardhatFhevmError(
      'The fhevm hardhat plugin does not support parallel testing when running in cleartext mode.',
    );
  }

  await checkSolidityCoverageSettings(hre);

  const fhevmEnv = fhevmContext.get();
  fhevmEnv.setRunningInHHTest();

  await fhevmEnv.deploy();
  assertHHFhevm(fhevmEnv.isDeployed, 'FhevmEnvironment is not initialized');

  const res: unknown = await runSuper();
  return res;
});

task(TASK_CLEAN, async (_taskArgs: TaskArguments, _hre: HardhatRuntimeEnvironment, runSuper) => {
  debug(`execute TASK_CLEAN`);

  // no 'minimalInit' needed here. We only need paths.
  const fhevmEnv = fhevmContext.get();

  // Should not block the whole thing...
  try {
    if (fs.existsSync(fhevmEnv.paths.cacheDir)) {
      fs.rmSync(fhevmEnv.paths.cacheDir, { force: true, recursive: true });

      debug(`${picocolors.greenBright(TASK_CLEAN)} remove directory ${fhevmEnv.paths.cacheDir}`);
    } else {
      debug(`${picocolors.greenBright(TASK_CLEAN)} directory ${fhevmEnv.paths.cacheDir} already removed.`);
    }
  } catch {
    console.log(`${constants.HARDHAT_PLUGIN_NAME}: Unable to remove directory '${fhevmEnv.paths.cacheDir}'.`);
  }

  const res: unknown = await runSuper();
  return res;
});

subtask(TASK_COMPILE_GET_REMAPPINGS).setAction(async (_taskArgs, _hre, runSuper): Promise<Record<string, string>> => {
  debug(`execute TASK_COMPILE_GET_REMAPPINGS`);

  const fhevmEnv = fhevmContext.get();
  await fhevmEnv.minimalInit();

  // run super first.
  const res = (await runSuper()) as Record<string, string>;

  // No remapping any more: @fhevm/solidity's own `_getLocalConfig` already carries the cleartext
  // stack's addresses, so the rewritten copy it used to point at is redundant.

  return res;
});

// `runSuper()` is typed `any`, and another plugin may own this subtask, so its result is verified.
function isStringArray(value: unknown): value is string[] {
  return Array.isArray(value) && value.every((entry) => typeof entry === 'string');
}

subtask(TASK_COMPILE_SOLIDITY_GET_SOURCE_PATHS).setAction(
  async (/*{ sourcePath }: { sourcePath?: string }*/ _taskArgs: TaskArguments, _hre, runSuper): Promise<string[]> => {
    debug(`execute TASK_COMPILE_SOLIDITY_GET_SOURCE_PATHS`);

    const fhevmEnv = fhevmContext.get();
    await fhevmEnv.minimalInitWithAddresses();

    // run super first, then append our solidity files.
    const superPaths: unknown = await runSuper();
    if (!isStringArray(superPaths)) {
      throw new HardhatFhevmError(
        `Subtask ${TASK_COMPILE_SOLIDITY_GET_SOURCE_PATHS} did not return a list of source paths.`,
      );
    }

    return [...superPaths, ...fhevmEnv.getSoliditySourcePaths()];
  },
);

subtask(TASK_NODE_GET_PROVIDER).setAction(async (_taskArgs: TaskArguments, _hre, runSuper) => {
  // This task is not supposed to be called multiple times.
  const fhevmEnv = fhevmContext.get();

  if (!fhevmEnv.isDeployed) {
    fhevmEnv.setRunningInHHNode();
    await fhevmEnv.deploy();
    assertHHFhevm(fhevmEnv.isDeployed, 'FhevmEnvironment is not initialized');
  }

  const res: unknown = await runSuper();
  return res;
});

subtask(TASK_NODE_SERVER_READY).setAction(
  async (
    _taskArgs: TaskArguments,
    // {
    //   address,
    //   port,
    //   provider,
    //   server,
    // }: {
    //   address: string;
    //   port: number;
    //   provider: EthereumProvider;
    //   server: JsonRpcServer;
    // },
    _hre,
    runSuper,
  ) => {
    // This task is not supposed to be called multiple times.
    const fhevmEnv = fhevmContext.get();

    if (!fhevmEnv.isDeployed) {
      fhevmEnv.setRunningInHHNode();
      await fhevmEnv.deploy();
      assertHHFhevm(fhevmEnv.isDeployed, 'FhevmEnvironment is not initialized');
    }

    const res: unknown = await runSuper();
    return res;
  },
);
