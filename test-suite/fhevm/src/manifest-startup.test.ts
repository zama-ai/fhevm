import { expect, test } from "bun:test";
import { startManifestRuntime } from "./flow/manifest-startup";

test("manifest detectors wait for all listeners and a 15 second catch-up window", async () => {
  const events: unknown[] = [];
  const services = ["coprocessor-host-listener-poller", "coprocessor1-host-listener-consumer",
    "coprocessor-tfhe-worker", "coprocessor-consensus-detector", "coprocessor1-consensus-detector"];
  await startManifestRuntime(services, {
    start: async names => { events.push(["start", names]); },
    waitForListener: async name => { events.push(["listener", name]); },
    sleep: async ms => { events.push(["sleep", ms]); },
  });
  expect(events).toEqual([
    ["start", services.slice(0, 3)],
    ["listener", services[0]], ["listener", services[1]],
    ["sleep", 15_000], ["start", services.slice(3)],
  ]);
});

test("failed listener readiness prevents detector startup", async () => {
  const started: string[][] = [];
  await expect(startManifestRuntime(["coprocessor-host-listener", "coprocessor-consensus-detector"], {
    start: async names => { started.push(names); },
    waitForListener: async () => { throw new Error("listener failed"); },
    sleep: async () => { throw new Error("must not wait after failure"); },
  })).rejects.toThrow("listener failed");
  expect(started).toEqual([["coprocessor-host-listener"]]);
});
