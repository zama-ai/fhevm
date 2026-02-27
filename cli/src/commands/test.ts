import { defineCommand } from "citty";

import { buildVersionEnvVars, resolveAllVersions } from "../config/versions";
import { getComposeFilesForServices, getServiceByName } from "../config/service-map";
import { composeExecStreaming } from "../docker/compose";
import { getContainerState } from "../docker/containers";
import { DOCKER_PROJECT } from "../docker/types";
import { ExitCode, FhevmCliError, exitWithError, type CliError } from "../errors";

export interface TestCommandArgs {
  type: string;
  verbose: boolean;
  network?: string;
  grep?: string;
  randomOrder: boolean;
  noHardhatCompile: boolean;
}

interface TestCommandDeps {
  getContainerState: typeof getContainerState;
  composeExecStreaming: typeof composeExecStreaming;
  resolveAllVersions: typeof resolveAllVersions;
  buildVersionEnvVars: typeof buildVersionEnvVars;
}

const TEST_TYPE_PATTERNS: Readonly<Record<string, string>> = {
  "input-proof": "test user input uint64",
  "input-proof-compute-decrypt": "test add 42 to uint64 input and decrypt",
  "user-decryption": "test user decrypt",
  "delegated-user-decryption": "test delegated user decryption",
  "public-decryption": "MakePubliclyDecryptable",
  erc20: "EncryptedERC20",
  "public-decrypt-http-ebool": "test HTTPPublicDecrypt ebool",
  "public-decrypt-http-mixed": "test HTTPPublicDecrypt mixed",
  operators: "FHEVM operations",
  random: "Rand",
  "random-subset": "Rand",
  "paused-host-contracts": "Paused host",
  "paused-gateway-contracts": "Paused gateway",
  debug: "",
};

const VALID_TEST_TYPES = Object.keys(TEST_TYPE_PATTERNS);
const TEST_CONTAINER_SERVICE = "test-suite-e2e-debug";
const DEFAULT_NETWORK = "staging";

const DEFAULT_DEPS: TestCommandDeps = {
  getContainerState,
  composeExecStreaming,
  resolveAllVersions,
  buildVersionEnvVars,
};

export function validateTestType(type: string): void {
  if (VALID_TEST_TYPES.includes(type)) {
    return;
  }

  throw new FhevmCliError({
    exitCode: ExitCode.CONFIG,
    step: "test",
    message: `invalid test type: '${type}'. Valid types: ${VALID_TEST_TYPES.join(", ")}`,
  });
}

export function resolveTestPattern(type: string, grepOverride?: string): string {
  if (grepOverride) {
    return grepOverride;
  }

  if (type === "debug") {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "test",
      message: "debug test type requires --g pattern",
    });
  }

  return TEST_TYPE_PATTERNS[type] ?? "";
}

export function buildHardhatArgs(options: {
  pattern: string;
  network: string;
  verbose: boolean;
  noHardhatCompile: boolean;
  randomOrder: boolean;
}): string[] {
  const args = ["npx", "hardhat", "test"];

  if (options.randomOrder) {
    args.push("--parallel");
  }
  if (options.verbose) {
    args.push("--verbose");
  }
  if (options.noHardhatCompile) {
    args.push("--no-hardhat-compile");
  }
  if (options.pattern) {
    args.push("--grep", options.pattern);
  }

  args.push("--network", options.network);
  return args;
}

export function mapTestExitCode(exitCode: number): number {
  if (exitCode === 0) {
    return ExitCode.SUCCESS;
  }
  return exitCode + (ExitCode.TEST_FAILURE - 1);
}

export async function runTestCommand(
  args: TestCommandArgs,
  deps: TestCommandDeps = DEFAULT_DEPS,
): Promise<number> {
  validateTestType(args.type);
  const pattern = resolveTestPattern(args.type, args.grep);
  const network = args.network ?? DEFAULT_NETWORK;
  const service = getServiceByName(TEST_CONTAINER_SERVICE);
  if (!service) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "test",
      message: `missing service definition: ${TEST_CONTAINER_SERVICE}`,
    });
  }
  const state = await deps.getContainerState(service.containerName);

  if (state !== "running") {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "test",
      service: service.name,
      message: `test container is '${state}'; run 'fhevm-cli up' first`,
    });
  }

  const versions = await deps.resolveAllVersions();
  const envVars = deps.buildVersionEnvVars(versions);
  const rawExitCode = await deps.composeExecStreaming(
    TEST_CONTAINER_SERVICE,
    buildHardhatArgs({
      pattern,
      network,
      verbose: args.verbose,
      noHardhatCompile: args.noHardhatCompile,
      randomOrder: args.randomOrder,
    }),
    {
      project: DOCKER_PROJECT,
      files: getComposeFilesForServices([service]),
      envVars,
    },
  );

  return mapTestExitCode(rawExitCode);
}

export function toTestError(error: unknown): CliError {
  if (error instanceof FhevmCliError) {
    return error;
  }

  return {
    exitCode: ExitCode.DOCKER,
    step: "test",
    message: error instanceof Error ? error.message : String(error),
    cause: error,
  };
}

export default defineCommand({
  meta: {
    name: "test",
    description: "Run fhevm test-suite groups",
  },
  args: {
    type: { type: "positional", required: true, description: "Test group" },
    verbose: { type: "boolean", alias: "v", required: false, description: "Verbose output" },
    network: { type: "string", alias: "n", required: false, description: "Network" },
    grep: { type: "string", alias: "g", required: false, description: "Pattern filter" },
    random: { type: "boolean", alias: "r", required: false, description: "Random order" },
    json: { type: "boolean", required: false, description: "JSON output" },
    "no-hardhat-compile": {
      type: "boolean",
      required: false,
      description: "Skip hardhat compile",
    },
  },
  async run({ args }) {
    const json = args.json ?? false;
    try {
      const exitCode = await runTestCommand({
        type: args.type,
        verbose: args.verbose ?? false,
        network: args.network,
        grep: args.grep,
        randomOrder: args.random ?? false,
        noHardhatCompile: args["no-hardhat-compile"] ?? false,
      });

      if (exitCode !== 0) {
        if (json) {
          console.log(JSON.stringify({ ok: false, command: "test", exitCode }));
        }
        process.exit(exitCode);
      }
      if (json) {
        console.log(JSON.stringify({ ok: true, command: "test", exitCode }));
      }
    } catch (error) {
      exitWithError(toTestError(error), { json });
    }
  },
});
