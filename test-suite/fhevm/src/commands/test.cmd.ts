import { Command } from "@effect/cli";
import { Option } from "effect";
import {
  grepOption,
  networkOption,
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
    verbose: verboseOption,
    parallel: parallelOption,
    testName: testNameArg,
  },
  ({ grep, network, verbose, parallel, testName }) =>
    test(Option.getOrUndefined(testName), {
      grep: Option.getOrUndefined(grep),
      network,
      verbose,
      parallel: Option.getOrUndefined(parallel),
    }),
).pipe(
  Command.withDescription("Run a named e2e test profile inside the test-suite container."),
);
