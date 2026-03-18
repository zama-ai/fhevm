import { describe, expect, test } from "bun:test";
import { Effect, Layer } from "effect";
import { CommandRunner, type RunResult } from "./CommandRunner";
import { ContainerRunner } from "./ContainerRunner";
import { ImageBuilder } from "./ImageBuilder";
import { loadMergedComposeDoc } from "../render-compose";
import { stubState } from "../test-helpers";

describe("ImageBuilder", () => {
  test("maybeBuild records coprocessor images after per-service builds", async () => {
    const doc = await Effect.runPromise(loadMergedComposeDoc("coprocessor"));
    const services = Object.entries(doc.services)
      .filter(([, service]) => !!service.build)
      .map(([name]) => name);
    const refs = [
      ...new Set(
        services
          .map((service) => doc.services[service]?.image)
          .filter((value): value is string => typeof value === "string" && value.length > 0),
      ),
    ];
    const buildCalls: string[][] = [];
    const commandLayer = Layer.succeed(CommandRunner, {
      run: (argv: string[]) => {
        if (argv[0] === "docker" && argv[1] === "image" && argv[2] === "inspect") {
          return Effect.succeed({
            stdout: `sha256:${argv[3]}`,
            stderr: "",
            code: 0,
          } as RunResult);
        }
        return Effect.succeed({ stdout: "", stderr: "", code: 0 } as RunResult);
      },
      runLive: () => Effect.succeed(0),
      runWithHeartbeat: () => Effect.void,
    });
    const containerLayer = Layer.succeed(ContainerRunner, {
      composeUp: () => Effect.void,
      composeDown: () => Effect.succeed(true),
      composeBuild: (_component: string, batch: string[]) => {
        buildCalls.push(batch);
        return Effect.void;
      },
    });
    const state = stubState();

    await Effect.runPromise(
      Effect.gen(function* () {
        const builder = yield* ImageBuilder;
        yield* builder.maybeBuild("coprocessor", state, () => Effect.void);
      }).pipe(
        Effect.provide(ImageBuilder.Live),
        Effect.provide(commandLayer),
        Effect.provide(containerLayer),
      ),
    );

    expect(buildCalls).toEqual(services.map((service) => [service]));
    expect(state.builtImages?.map((image) => image.ref).sort()).toEqual([...refs].sort());
    expect(state.builtImages?.every((image) => image.group === "coprocessor")).toBe(true);
  });
});
