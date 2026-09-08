import { describe, expect, test } from "bun:test";
import { recoveryContainers, withConsumerOnlyRecovery } from "./consumer-only-recovery";
import type { ComposeDoc } from "../generate/compose";

const fixture = (): ComposeDoc => {
  const services: ComposeDoc["services"] = {};
  for (const prefix of ["coprocessor", "coprocessor1", "coprocessor2"]) {
    for (const chain of ["", "-host1"]) {
      for (const [suffix, executable] of [
        ["host-listener", "host_listener"],
        ["host-listener-poller", "host_listener_poller"],
        ["host-listener-consumer", "host_listener_consumer"],
      ]) {
        const name = `${prefix}-${suffix}${chain}`;
        services[name] = { container_name: name, command: [executable, "--url=test"] };
      }
    }
  }
  services.gateway = { command: ["gw_listener"] };
  return { services };
};

describe("consumer-only drift", () => {
  test("selects all operators and chains without stopping gateway or consumers", () => {
    const { legacy, consumers } = recoveryContainers(fixture());
    expect(legacy).toHaveLength(12);
    expect(consumers).toHaveLength(6);
    expect(legacy).not.toContain("gateway");
    expect(legacy.some((name) => name.includes("consumer"))).toBe(false);
  });

  for (const failure of ["none", "test", "stop"] as const) {
    test(`restores the original running services after ${failure}`, async () => {
      const doc = fixture();
      const { legacy, consumers } = recoveryContainers(doc);
      const initiallyStopped = legacy[0]!;
      const running = new Set([...legacy.slice(1), ...consumers]);
      let tested = false;
      const result = withConsumerOnlyRecovery(doc, {
        running: async (name) => running.has(name),
        stop: async (names) => {
          for (const name of names) running.delete(name);
          if (failure === "stop") throw new Error("stop failed");
        },
        start: async (names) => { for (const name of names) running.add(name); },
      }, async () => {
        tested = true;
        expect(legacy.every((name) => !running.has(name))).toBe(true);
        expect(consumers.every((name) => running.has(name))).toBe(true);
        if (failure === "test") throw new Error("test failed");
      });
      if (failure === "none") await result;
      else await expect(result).rejects.toThrow(`${failure} failed`);
      expect(tested).toBe(failure !== "stop");
      expect(running.has(initiallyStopped)).toBe(false);
      expect(legacy.slice(1).every((name) => running.has(name))).toBe(true);
    });
  }

  test("also stops legacy-only extra chains", async () => {
    const extra: ComposeDoc = { services: {
      "coprocessor-host-listener-chain-b": { command: ["host_listener"] },
      "coprocessor1-host-listener-poller-chain-b": { command: ["host_listener_poller"] },
    } };
    const { legacy, consumers } = recoveryContainers(fixture(), [extra]);
    expect(legacy).toContain("coprocessor1-host-listener-poller-chain-b");
    const running = new Set([...legacy, ...consumers]);
    await withConsumerOnlyRecovery(fixture(), {
      running: async (name) => running.has(name),
      stop: async (names) => { for (const name of names) running.delete(name); },
      start: async (names) => { for (const name of names) running.add(name); },
    }, async () => {
      expect(legacy.every((name) => !running.has(name))).toBe(true);
    }, [extra]);
    expect(legacy.every((name) => running.has(name))).toBe(true);
  });

  test("missing consumer fails before stopping anything", async () => {
    const doc = fixture();
    let stopped = false;
    await expect(withConsumerOnlyRecovery(doc, {
      running: async (name) => !name.includes("consumer"),
      stop: async () => { stopped = true; }, start: async () => {},
    }, async () => {})).rejects.toThrow("Host consumer is not running");
    expect(stopped).toBe(false);
    delete doc.services["coprocessor2-host-listener-consumer-host1"];
    expect(() => recoveryContainers(doc)).toThrow("Missing host consumer");
  });
});
