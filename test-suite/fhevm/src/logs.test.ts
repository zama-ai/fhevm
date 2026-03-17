import { describe, expect, test } from "bun:test";
import { Effect } from "effect";

import { logs } from "./commands/logs";
import { depsToLayer, fakeRunner } from "./test-helpers";

describe("logs", () => {
  test("falls back to exited containers for aliased service logs", async () => {
    const liveCalls: string[][] = [];
    await Effect.runPromise(
      logs("coprocessor", { follow: false }).pipe(
        Effect.provide(
          depsToLayer({
            runner: fakeRunner({
              "docker ps --filter label=com.docker.compose.project=fhevm --format {{.Names}}":
                "coprocessor-host-listener\ncoprocessor-tfhe-worker",
              "docker ps -a --filter label=com.docker.compose.project=fhevm --format {{.Names}}":
                "coprocessor-host-listener\ncoprocessor-gw-listener\ncoprocessor-tfhe-worker",
            }),
            liveRunner: async (argv: string[]) => {
              liveCalls.push(argv);
              return 0;
            },
          }),
        ),
      ),
    );
    expect(liveCalls).toEqual([["docker", "logs", "--tail", "200", "coprocessor-gw-listener"]]);
  });
});
