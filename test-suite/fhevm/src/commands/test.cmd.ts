import { Command } from "@effect/cli";
import { Option } from "effect";
import {
  grepOption,
  networkOption,
  noRelayerOption,
  parallelOption,
  testNameArg,
  verboseOption,
} from "../options";
import { test } from "./test";

export const testCommand = Command.make(
  "test",
  {
    grep: grepOption,
    network: networkOption,
    noRelayer: noRelayerOption,
    verbose: verboseOption,
    parallel: parallelOption,
    testName: testNameArg,
  },
  ({ grep, network, noRelayer, verbose, parallel, testName }) =>
    test(Option.getOrUndefined(testName), {
      grep: Option.getOrUndefined(grep),
      network,
      noRelayer,
      verbose,
      parallel: Option.getOrUndefined(parallel),
    }),
).pipe(
  Command.withDescription("Run a named e2e test profile or suite inside the test-suite container."),
);
