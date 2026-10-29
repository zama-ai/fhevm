/**
 * The handle a test profile uses to change the running stack between checks: upgrade steps, test
 * profiles and contract tasks. Each action is recorded in the receipt under `.fhevm/rollout/`.
 */
import { PreflightError } from "../errors";
import type { UpgradeStep } from "../flow/bootstrap";
import { runContractTask } from "../flow/contracts";
import { waitForTestSuite } from "../flow/readiness";
import { composeUp } from "../flow/runtime-compose";
import { upgradeBootstrappedStack } from "../flow/up-flow";
import { STATE_DIR } from "../layout";
import { loadState } from "../state/state";
import type { State } from "../types";
import { type RolloutReceipt, createRolloutReceipt } from "./rollout-receipt";
import { prepareRolloutSolidity } from "./rollout-solidity";
import { test as runTest } from "./test";

type StackTestOptions = {
  /** Commit whose Solidity library the fixtures compile against, for a stack on an older host ABI. */
  solidityLibraryRevision?: string;
};
type ContractTaskOptions = {
  env?: Record<string, string>;
};

export type Stack = {
  /** Runs bootstrap upgrade steps in order, after checking the whole list against `upgradeOrder`. */
  upgrade(first: UpgradeStep, ...rest: UpgradeStep[]): Promise<void>;
  test(profile: string, options?: StackTestOptions): Promise<void>;
  runGatewayContractTask(command: string, options?: ContractTaskOptions): Promise<void>;
  runHostContractTask(command: string, options?: ContractTaskOptions): Promise<void>;
  readState(): Promise<State>;
  stateDir(): string;
};

const readState = async () => {
  const state = await loadState();
  if (!state) {
    throw new PreflightError("Stack is not running; run `fhevm-cli up` first");
  }
  return state;
};

const createStack = (receipt: RolloutReceipt): Stack => ({
  async upgrade(...steps) {
    const state = await readState();
    await upgradeBootstrappedStack(state, steps);
    await receipt.record("upgrade", steps.join(", "), { docker: true, versions: state.versions.env });
  },
  async test(profile, options = {}) {
    console.log("[test-suite] recreate container to load current generated env");
    await composeUp("test-suite", ["test-suite-e2e-debug"], { noDeps: true, forceRecreate: true });
    await waitForTestSuite();
    if (options.solidityLibraryRevision) {
      const library = await prepareRolloutSolidity(options.solidityLibraryRevision);
      await receipt.record("fixture-solidity", "compiled fixtures against pinned Solidity", {
        details: { profile, ...library },
      });
    }
    await runTest(profile, { network: "staging", verbose: false, noHardhatCompile: true, parallel: false });
    await receipt.record("test", `${profile} passed`);
  },
  async runGatewayContractTask(command, options = {}) {
    await runContractTask("gateway-sc", "gateway-sc-deploy", command, options);
    await receipt.record("gateway-contract-task", command, {
      details: { envKeys: Object.keys(options.env ?? {}).sort() },
    });
  },
  async runHostContractTask(command, options = {}) {
    await runContractTask("host-sc", "host-sc-deploy", command, options);
    await receipt.record("host-contract-task", command, {
      details: { envKeys: Object.keys(options.env ?? {}).sort() },
    });
  },
  readState,
  stateDir() {
    return STATE_DIR;
  },
});

/** Runs a test that drives the stack, recording the stack before it and the outcome. */
export const runStackTest = async (name: string, run: (stack: Stack) => Promise<void>) => {
  const receipt = createRolloutReceipt();
  await receipt.start(name);
  try {
    await receipt.record("boot", "stack before the test", {
      docker: true,
      versions: (await readState()).versions.env,
    });
    await run(createStack(receipt));
    await receipt.record("complete", `${name} passed`);
  } catch (error) {
    try {
      await receipt.record("failed", `${name} failed`, {
        details: { error: error instanceof Error ? error.message : String(error) },
        diagnostics: true,
      });
    } catch (recordError) {
      console.error("[receipt] failed to record the test failure", recordError);
    }
    throw error;
  }
};
