import { describe, expect, test } from "bun:test";
import type { ComposeDoc } from "../generate/compose";
import { legacyHostListenerContainers, withLegacyHostListeners } from "./legacy-host-listeners";

const fixture = (): ComposeDoc[] => ["", "-chain-b"].map((chain) => ({
  services: Object.fromEntries(["coprocessor", "coprocessor1", "coprocessor2"].flatMap((operator) =>
    ["host-listener", "host-listener-poller", "host-listener-consumer"].map((suffix) => [
      `${operator}-${suffix}${chain}`,
      { command: [suffix.replaceAll("-", "_")], profiles: suffix.endsWith("consumer") ? [] : ["legacy-host-listeners"] },
    ]),
  )),
}));

describe("legacy host ingestion fallback", () => {
  for (const mode of ["listener", "poller"] as const) {
    for (const failure of ["none", "test", "stop", "start"] as const) {
      test(`${mode}: exclusive ingestion and consumer restoration after ${failure}`, async () => {
        const docs = fixture();
        const { consumers, selected, legacy } = legacyHostListenerContainers(docs, mode);
        expect(consumers).toHaveLength(6);
        expect(selected).toHaveLength(6);
        expect(legacy).toHaveLength(12);
        const running = new Set(consumers);
        let tested = false;
        const result = withLegacyHostListeners(docs, mode, {
          running: async (name) => running.has(name),
          stop: async (names) => {
            for (const name of names) running.delete(name);
            if (failure === "stop" && names.some((name) => consumers.includes(name))) throw new Error("stop failed");
          },
          start: async (names) => {
            for (const name of names) running.add(name);
            if (failure === "start" && names.some((name) => selected.includes(name))) throw new Error("start failed");
          },
        }, async () => {
          tested = true;
          expect([...running].sort()).toEqual([...selected].sort());
          if (failure === "test") throw new Error("test failed");
        });
        if (failure === "none") await result;
        else await expect(result).rejects.toThrow(`${failure} failed`);
        expect(tested).toBe(failure === "none" || failure === "test");
        expect([...running].sort()).toEqual([...consumers].sort());
      });
    }
  }

  test("rejects a missing fallback on a secondary chain before any mutation", () => {
    const docs = fixture();
    delete docs[1].services["coprocessor2-host-listener-poller-chain-b"];
    expect(() => legacyHostListenerContainers(docs, "poller")).toThrow("Missing legacy fallback");
  });

  test("does not let a running legacy service mask a consumer failure", async () => {
    const docs = fixture();
    const { consumers, legacy } = legacyHostListenerContainers(docs, "listener");
    const running = new Set([...consumers, legacy[0]]);
    let mutated = false;
    await expect(withLegacyHostListeners(docs, "listener", {
      running: async (name) => running.has(name),
      stop: async () => { mutated = true; },
      start: async () => { mutated = true; },
    }, async () => { throw new Error("must not run tests"); })).rejects.toThrow("Unexpected running host ingestion");
    expect(mutated).toBe(false);
  });

  test("refuses to restart consumers if legacy ingestion cannot be stopped", async () => {
    const docs = fixture();
    const { consumers, selected } = legacyHostListenerContainers(docs, "listener");
    const running = new Set(consumers);
    await expect(withLegacyHostListeners(docs, "listener", {
      running: async (name) => running.has(name),
      stop: async (names) => {
        if (names.some((name) => selected.includes(name))) throw new Error("legacy stop failed");
        for (const name of names) running.delete(name);
      },
      start: async (names) => { for (const name of names) running.add(name); },
    }, async () => {})).rejects.toThrow("legacy stop failed");
    expect([...running].sort()).toEqual([...selected].sort());
  });

});
