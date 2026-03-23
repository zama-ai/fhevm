/**
 * Pauses and unpauses the host or gateway contract surfaces.
 */
import { PreflightError } from "../errors";
import { loadState } from "../state/state";
import { composeEnv, runStreaming } from "../utils/process";
import { dockerArgs, envPath } from "../layout";
import { readEnvFileIfExists } from "../utils/fs";
import { resolvedComposeEnv } from "../generate/compose";

/** Quotes a shell argument for safe inclusion in a `sh -lc` command. */
export const shellEscape = (value: string) => `'${value.replaceAll("'", `'\\''`)}'`;

/** Runs a host or gateway contract task inside its deploy container. */
const runContractTask = async (
  component: "host-sc" | "gateway-sc",
  service: "host-sc-deploy" | "gateway-sc-deploy",
  command: string,
) => {
  const state = await loadState();
  if (!state) {
    throw new PreflightError("Stack is not running; run `fhevm-cli up` first");
  }
  await runStreaming(
    [...dockerArgs(component), "run", "--rm", "--no-deps", "--entrypoint", "sh", service, "-lc", command],
    { env: { ...resolvedComposeEnv(state), ...(await readEnvFileIfExists(envPath(component))) } },
  );
};

/** Pauses the requested contract surface through its deploy task. */
export const pause = async (scope: string | undefined) => {
  if (scope === "host") {
    await runContractTask("host-sc", "host-sc-deploy", "npx hardhat compile && npx hardhat task:pauseACL");
    return;
  }
  if (scope === "gateway") {
    await runContractTask("gateway-sc", "gateway-sc-deploy", "npx hardhat compile && npx hardhat task:pauseAllGatewayContracts");
    return;
  }
  throw new PreflightError("pause expects `host` or `gateway`");
};

/** Unpauses the requested contract surface through its deploy task. */
export const unpause = async (scope: string | undefined) => {
  if (scope === "host") {
    await runContractTask("host-sc", "host-sc-deploy", "npx hardhat compile && npx hardhat task:unpauseACL");
    return;
  }
  if (scope === "gateway") {
    await runContractTask("gateway-sc", "gateway-sc-deploy", "npx hardhat compile && npx hardhat task:unpauseAllGatewayContracts");
    return;
  }
  throw new PreflightError("unpause expects `host` or `gateway`");
};
