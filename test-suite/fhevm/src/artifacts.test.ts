import { describe, expect, test } from "bun:test";

import { resolvedComposeEnv, serviceNameList } from "./artifacts";
import { stubState } from "./test-helpers";

describe("resolvedComposeEnv", () => {
  test("includes version env and COMPOSE_IGNORE_ORPHANS", () => {
    const env = resolvedComposeEnv(stubState());
    expect(env.GATEWAY_VERSION).toBe("v0.11.0");
    expect(env.CORE_VERSION).toBe("v0.13.0");
    expect(env.COMPOSE_IGNORE_ORPHANS).toBe("true");
  });

  test("defaults FHEVM_CARGO_PROFILE to release", () => {
    const env = resolvedComposeEnv(stubState());
    expect(env.FHEVM_CARGO_PROFILE).toBe("release");
  });

  test("uses override profile when present", () => {
    const env = resolvedComposeEnv(stubState({ overrides: [{ group: "coprocessor", profile: "debug" }] }));
    expect(env.FHEVM_CARGO_PROFILE).toBe("debug");
  });

  test("uses first override profile found", () => {
    const env = resolvedComposeEnv(
      stubState({
        overrides: [
          { group: "coprocessor", profile: "custom" },
          { group: "test-suite", profile: "other" },
        ],
      }),
    );
    expect(env.FHEVM_CARGO_PROFILE).toBe("custom");
  });

  test("falls back to release when override has no profile", () => {
    const env = resolvedComposeEnv(stubState({ overrides: [{ group: "coprocessor" }] }));
    expect(env.FHEVM_CARGO_PROFILE).toBe("release");
  });
});

describe("serviceNameList", () => {
  const state = stubState();

  test("returns empty for non-coprocessor components", () => {
    expect(serviceNameList(state, "relayer")).toEqual([]);
    expect(serviceNameList(state, "database")).toEqual([]);
    expect(serviceNameList(state, "minio")).toEqual([]);
  });

  test("returns single-instance service names for count=1", () => {
    const names = serviceNameList(state, "coprocessor");
    expect(names).toEqual([
      "coprocessor-db-migration",
      "coprocessor-host-listener",
      "coprocessor-host-listener-poller",
      "coprocessor-gw-listener",
      "coprocessor-tfhe-worker",
      "coprocessor-zkproof-worker",
      "coprocessor-sns-worker",
      "coprocessor-transaction-sender",
    ]);
  });

  test("returns multi-instance service names for count=2", () => {
    const names = serviceNameList(stubState({ count: 2 }), "coprocessor");
    expect(names).toContain("coprocessor-db-migration");
    expect(names).toContain("coprocessor-host-listener");
    expect(names).toContain("coprocessor1-db-migration");
    expect(names).toContain("coprocessor1-host-listener");
    expect(names).toContain("coprocessor1-tfhe-worker");
    expect(names.length).toBe(16);
  });

  test("returns multi-instance service names for count=3", () => {
    const names = serviceNameList(stubState({ count: 3 }), "coprocessor");
    expect(names).toContain("coprocessor-db-migration");
    expect(names).toContain("coprocessor1-db-migration");
    expect(names).toContain("coprocessor2-db-migration");
    expect(names.length).toBe(24);
  });
});
