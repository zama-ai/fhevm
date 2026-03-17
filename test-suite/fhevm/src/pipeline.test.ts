import { describe, expect, test } from "bun:test";
import { Effect, Either } from "effect";

import {
  assertSchemaCompatibility,
  coprocessorHealthContainers,
  coprocessorServicesForOverrides,
  overrideWarnings,
  preflight,
  resolveUpgradePlan,
  shellEscape,
  stateStepIndex,
  validateDiscovery,
} from "./pipeline";
import { depsToLayer, fakeRunner, portCheckResponses } from "./test-helpers";
import { stubBundle } from "./test-helpers";
import { defaultCoprocessorScenario } from "./scenario";
import { predictedCrsId, predictedKeyId } from "./utils";
import type { Discovery, LocalOverride, State } from "./types";
import { STEP_NAMES } from "./types";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

const readyDiscovery = (): Discovery => ({
  gateway: {
    GATEWAY_CONFIG_ADDRESS: "0x1",
    KMS_GENERATION_ADDRESS: "0x2",
    DECRYPTION_ADDRESS: "0x3",
    INPUT_VERIFICATION_ADDRESS: "0x4",
    CIPHERTEXT_COMMITS_ADDRESS: "0x5",
  },
  host: {
    ACL_CONTRACT_ADDRESS: "0xa",
    FHEVM_EXECUTOR_CONTRACT_ADDRESS: "0xb",
    KMS_VERIFIER_CONTRACT_ADDRESS: "0xc",
    INPUT_VERIFIER_CONTRACT_ADDRESS: "0xd",
    PAUSER_SET_CONTRACT_ADDRESS: "0xe",
  },
  kmsSigner: "0xabc",
  fheKeyId: predictedKeyId(),
  crsKeyId: predictedCrsId(),
  endpoints: {
    gatewayHttp: "http://gateway-node:8546",
    gatewayWs: "ws://gateway-node:8546",
    hostHttp: "http://host-node:8545",
    hostWs: "ws://host-node:8545",
    minioInternal: "http://minio:9000",
    minioExternal: "http://172.17.0.2:9000",
  },
});

const stubState = (
  overrides: Partial<State> = {},
): Pick<State, "overrides" | "topology"> & State => ({
  target: "latest-main",
  lockPath: "/tmp/fake.json",
  versions: {
    target: "latest-main",
    lockName: "latest-main.json",
    env: {
      COPROCESSOR_TX_SENDER_VERSION: "v0.12.0",
      COPROCESSOR_HOST_LISTENER_VERSION: "v0.12.0",
      CONNECTOR_GW_LISTENER_VERSION: "v0.11.0",
    },
    sources: [],
  },
  overrides: [],
  topology: { count: 1, threshold: 1 },
  scenario: defaultCoprocessorScenario(),
  completedSteps: [],
  updatedAt: "2024-01-01T00:00:00.000Z",
  ...overrides,
});

// ---------------------------------------------------------------------------
// stateStepIndex
// ---------------------------------------------------------------------------

describe("stateStepIndex", () => {
  test("returns correct index for each step", () => {
    expect(stateStepIndex("preflight")).toBe(0);
    expect(stateStepIndex("resolve")).toBe(1);
    expect(stateStepIndex("test-suite")).toBe(STEP_NAMES.length - 1);
  });
});

// ---------------------------------------------------------------------------
// overrideWarnings
// ---------------------------------------------------------------------------

describe("overrideWarnings", () => {
  test("returns empty for full-group overrides", () => {
    const overrides: LocalOverride[] = [{ group: "coprocessor" }];
    expect(overrideWarnings(overrides)).toEqual([]);
  });

  test("warns on per-service override for schema-coupled group", () => {
    const overrides: LocalOverride[] = [
      {
        group: "coprocessor",
        services: ["coprocessor-host-listener"],
      },
    ];
    const warnings = overrideWarnings(overrides);
    expect(warnings.length).toBe(1);
    expect(warnings[0]).toContain("per-service override");
  });

  test("warns on network target with overrides", () => {
    const overrides: LocalOverride[] = [{ group: "coprocessor" }];
    const warnings = overrideWarnings(overrides, "devnet");
    expect(warnings.length).toBe(1);
    expect(warnings[0]).toContain("Overriding on network target");
  });

  test("no network warning for latest-main", () => {
    const overrides: LocalOverride[] = [{ group: "coprocessor" }];
    const warnings = overrideWarnings(overrides, "latest-main");
    expect(warnings).toEqual([]);
  });

  test("empty overrides yields no warnings", () => {
    expect(overrideWarnings([])).toEqual([]);
    expect(overrideWarnings([], "devnet")).toEqual([]);
  });

  test("per-service kms-connector override warns", () => {
    const overrides: LocalOverride[] = [
      {
        group: "kms-connector",
        services: ["kms-connector-gw-listener"],
      },
    ];
    const warnings = overrideWarnings(overrides);
    expect(warnings.length).toBe(1);
    expect(warnings[0]).toContain("kms-connector");
  });

  test("per-service test-suite override does not warn (not schema-coupled)", () => {
    const overrides: LocalOverride[] = [
      {
        group: "test-suite",
        services: ["test-suite-e2e-debug"],
      },
    ];
    expect(overrideWarnings(overrides)).toEqual([]);
  });
});

describe("preflight", () => {
  test("fails early when cast is unavailable", async () => {
    const error = await Effect.runPromise(
      preflight(stubState(), true, false).pipe(
        Effect.provide(
          depsToLayer({
            runner: fakeRunner({
              "which bun": "",
              "which docker": "",
              "which cast": { stdout: "", stderr: "", code: 1 },
            }),
          }),
        ),
        Effect.flip,
      ),
    );
    expect(error.message).toContain('Required command "cast" not found');
  });

  test("fails early when docker is unavailable", async () => {
    const error = await Effect.runPromise(
      preflight(stubState(), true, false).pipe(
        Effect.provide(
          depsToLayer({
            runner: fakeRunner({
              "which bun": "",
              "which docker": "",
              "which cast": "",
              "docker ps --filter label=com.docker.compose.project=fhevm --format {{.Ports}}": {
                stdout: "",
                stderr: "docker daemon unavailable",
                code: 1,
              },
              ...portCheckResponses,
            }),
          }),
        ),
        Effect.flip,
      ),
    );
    expect(error.message).toContain("docker daemon unavailable");
  });
});

describe("assertSchemaCompatibility", () => {
  test("treats local coprocessor services from scenarios like partial overrides", async () => {
    const error = await Effect.runPromise(
      assertSchemaCompatibility(
        stubBundle(),
        [],
        {
          version: 1,
          kind: "coprocessor-consensus",
          origin: "file",
          topology: { count: 2, threshold: 2 },
          instances: [
            { index: 0, source: { mode: "inherit" }, env: {}, args: {} },
            {
              index: 1,
              source: { mode: "local" },
              env: {},
              args: {},
              localServices: ["coprocessor-host-listener"],
            },
          ],
        },
        false,
      ).pipe(
        Effect.provide(
          depsToLayer({
            runner: fakeRunner({
              "git rev-parse -q --verify v0.11.0^{commit}": "",
              "git ls-files --others --exclude-standard -- coprocessor/fhevm-engine/db-migration/migrations": "",
              "git diff --quiet --exit-code v0.11.0 -- coprocessor/fhevm-engine/db-migration/migrations": {
                stdout: "",
                stderr: "",
                code: 1,
              },
            }),
          }),
        ),
        Effect.flip,
      ),
    );
    expect(error.message).toContain("local DB migrations diverge");
  });
});

// ---------------------------------------------------------------------------
// resolveUpgradePlan
// ---------------------------------------------------------------------------

describe("resolveUpgradePlan", () => {
  test("rejects missing group", () => {
    expect(() =>
      resolveUpgradePlan(stubState(), undefined),
    ).toThrow("upgrade expects one of");
  });

  test("rejects unsupported group", () => {
    expect(() =>
      resolveUpgradePlan(stubState(), "gateway-contracts"),
    ).toThrow("upgrade expects one of");
  });

  test("rejects when no matching override", () => {
    expect(() =>
      resolveUpgradePlan(stubState(), "coprocessor"),
    ).toThrow("upgrade requires an active local coprocessor instance");
  });

  test("resolves coprocessor plan", () => {
    const state = stubState({
      scenario: {
        version: 1,
        kind: "coprocessor-consensus",
        origin: "override-shorthand",
        topology: { count: 1, threshold: 1 },
        instances: [
          {
            index: 0,
            source: { mode: "local" },
            env: {},
            args: {},
          },
        ],
      },
      overrides: [{ group: "coprocessor" }],
    });
    const plan = resolveUpgradePlan(state, "coprocessor");
    expect(plan.group).toBe("coprocessor");
    expect(plan.component).toBe("coprocessor");
    expect(plan.step).toBe("coprocessor");
    expect(plan.services.length).toBeGreaterThan(0);
    // Should not include db-migration
    expect(
      plan.services.some((s) => s.endsWith("-db-migration")),
    ).toBe(false);
  });

  test("resolves kms-connector plan", () => {
    const state = stubState({
      overrides: [{ group: "kms-connector" }],
    });
    const plan = resolveUpgradePlan(state, "kms-connector");
    expect(plan.group).toBe("kms-connector");
    expect(plan.component).toBe("kms-connector");
    expect(plan.step).toBe("kms-connector");
    expect(plan.services.length).toBeGreaterThan(0);
    expect(
      plan.services.some((s) => s.endsWith("-db-migration")),
    ).toBe(false);
  });

  test("resolves test-suite plan", () => {
    const state = stubState({
      overrides: [{ group: "test-suite" }],
    });
    const plan = resolveUpgradePlan(state, "test-suite");
    expect(plan.group).toBe("test-suite");
    expect(plan.component).toBe("test-suite");
    expect(plan.step).toBe("test-suite");
    expect(plan.services).toEqual(["test-suite-e2e-debug"]);
  });

  test("resolves coprocessor plan with per-service overrides", () => {
    const state = stubState({
      scenario: {
        version: 1,
        kind: "coprocessor-consensus",
        origin: "override-shorthand",
        topology: { count: 1, threshold: 1 },
        instances: [
          {
            index: 0,
            source: { mode: "local" },
            env: {},
            args: {},
            localServices: [
              "coprocessor-host-listener",
              "coprocessor-host-listener-poller",
            ],
          },
        ],
      },
      overrides: [
        {
          group: "coprocessor",
          services: ["coprocessor-host-listener", "coprocessor-host-listener-poller"],
        },
      ],
    });
    const plan = resolveUpgradePlan(state, "coprocessor");
    expect(plan.services).toEqual([
      "coprocessor-host-listener",
      "coprocessor-host-listener-poller",
    ]);
  });

  test("multi-coprocessor expands per-service overrides", () => {
    const state = stubState({
      scenario: {
        version: 1,
        kind: "coprocessor-consensus",
        origin: "file",
        topology: { count: 2, threshold: 2 },
        instances: [
          {
            index: 0,
            source: { mode: "local" },
            env: {},
            args: {},
            localServices: ["coprocessor-host-listener"],
          },
          {
            index: 1,
            source: { mode: "local" },
            env: {},
            args: {},
            localServices: ["coprocessor-host-listener"],
          },
        ],
      },
      topology: { count: 2, threshold: 2 },
      overrides: [
        {
          group: "coprocessor",
          services: ["coprocessor-host-listener"],
        },
      ],
    });
    const plan = resolveUpgradePlan(state, "coprocessor");
    expect(plan.services).toContain("coprocessor-host-listener");
    expect(plan.services).toContain("coprocessor1-host-listener");
  });
});

// ---------------------------------------------------------------------------
// validateDiscovery
// ---------------------------------------------------------------------------

describe("validateDiscovery", () => {
  test("passes with complete discovery", () => {
    const state = stubState({ discovery: readyDiscovery() });
    const result = Effect.runSync(Effect.either(validateDiscovery(state)));
    expect(Either.isRight(result)).toBe(true);
  });

  test("fails when no discovery", () => {
    const state = stubState({ discovery: undefined });
    const result = Effect.runSync(Effect.either(validateDiscovery(state)));
    expect(Either.isLeft(result)).toBe(true);
    if (Either.isLeft(result)) {
      expect(result.left.message).toBe("Missing discovery state");
    }
  });

  test("fails when gateway key missing", () => {
    const discovery = readyDiscovery();
    delete (discovery.gateway as Record<string, string>)[
      "GATEWAY_CONFIG_ADDRESS"
    ];
    const state = stubState({ discovery });
    const result = Effect.runSync(Effect.either(validateDiscovery(state)));
    expect(Either.isLeft(result)).toBe(true);
    if (Either.isLeft(result)) {
      expect(result.left.message).toBe(
        "Missing gateway discovery value GATEWAY_CONFIG_ADDRESS",
      );
    }
  });

  test("fails when host key missing", () => {
    const discovery = readyDiscovery();
    delete (discovery.host as Record<string, string>)[
      "ACL_CONTRACT_ADDRESS"
    ];
    const state = stubState({ discovery });
    const result = Effect.runSync(Effect.either(validateDiscovery(state)));
    expect(Either.isLeft(result)).toBe(true);
    if (Either.isLeft(result)) {
      expect(result.left.message).toBe(
        "Missing host discovery value ACL_CONTRACT_ADDRESS",
      );
    }
  });

  test("fails when kmsSigner empty", () => {
    const discovery = readyDiscovery();
    discovery.kmsSigner = "";
    const state = stubState({ discovery });
    const result = Effect.runSync(Effect.either(validateDiscovery(state)));
    expect(Either.isLeft(result)).toBe(true);
    if (Either.isLeft(result)) {
      expect(result.left.message).toBe("Missing KMS signer discovery");
    }
  });

  test("fails when fheKeyId empty", () => {
    const discovery = readyDiscovery();
    discovery.fheKeyId = "";
    const state = stubState({ discovery });
    const result = Effect.runSync(Effect.either(validateDiscovery(state)));
    expect(Either.isLeft(result)).toBe(true);
    if (Either.isLeft(result)) {
      expect(result.left.message).toBe("Missing predicted key ids");
    }
  });
});

// ---------------------------------------------------------------------------
// coprocessorServicesForOverrides
// ---------------------------------------------------------------------------

describe("coprocessorServicesForOverrides", () => {
  test("returns all services for empty services list", () => {
    const state = stubState();
    const services = coprocessorServicesForOverrides(state);
    expect(services.length).toBeGreaterThan(0);
    expect(services).toContain("coprocessor-db-migration");
  });

  test("returns specific services for given list", () => {
    const state = stubState();
    const services = coprocessorServicesForOverrides(state, [
      "coprocessor-host-listener",
    ]);
    expect(services).toEqual(["coprocessor-host-listener"]);
  });

  test("expands to multiple instances", () => {
    const state = stubState({
      topology: { count: 3, threshold: 3 },
    });
    const services = coprocessorServicesForOverrides(state, [
      "coprocessor-host-listener",
    ]);
    expect(services).toEqual([
      "coprocessor-host-listener",
      "coprocessor1-host-listener",
      "coprocessor2-host-listener",
    ]);
  });
});

// ---------------------------------------------------------------------------
// coprocessorHealthContainers
// ---------------------------------------------------------------------------

describe("coprocessorHealthContainers", () => {
  test("returns containers for single coprocessor", () => {
    const names = coprocessorHealthContainers({
      topology: { count: 1, threshold: 1 },
    });
    expect(names.length).toBeGreaterThan(0);
    // Should not include migration
    expect(names.every((n) => !n.includes("migration"))).toBe(true);
  });

  test("returns containers for multiple coprocessors", () => {
    const names = coprocessorHealthContainers({
      topology: { count: 2, threshold: 2 },
    });
    const hasInstance1 = names.some((n) => n.startsWith("coprocessor1-"));
    expect(hasInstance1).toBe(true);
  });
});

// ---------------------------------------------------------------------------
// shellEscape
// ---------------------------------------------------------------------------

describe("shellEscape", () => {
  test("wraps in single quotes", () => {
    expect(shellEscape("hello world")).toBe("'hello world'");
  });

  test("escapes single quotes", () => {
    expect(shellEscape("it's")).toBe("'it'\\''s'");
  });

  test("handles empty string", () => {
    expect(shellEscape("")).toBe("''");
  });
});
