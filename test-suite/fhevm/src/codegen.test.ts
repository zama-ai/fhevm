import path from "node:path";

import { describe, expect, test } from "bun:test";

import {
  applyBuildPolicy,
  appendVolume,
  applyInstanceAdjustments,
  interpolateComposeValue,
  interpolateString,
  LOCAL_BUILD_TAG,
  overriddenServicesForComponent,
  resolvedComposeEnv,
  resolveComposePath,
  retagLocal,
  rewriteComposePaths,
  rewriteCoprocessorDependsOn,
  rewriteVolume,
  serviceNameList,
  type ComposeDoc,
} from "./render-compose";
import { TEMPLATE_COMPOSE_DIR } from "./layout";
import { stubState } from "./test-helpers";

// ---------------------------------------------------------------------------
// resolvedComposeEnv
// ---------------------------------------------------------------------------
describe("resolvedComposeEnv", () => {
  test("includes version env and COMPOSE_IGNORE_ORPHANS", () => {
    const env = resolvedComposeEnv(stubState());
    expect(env.GATEWAY_VERSION).toBe("v0.11.0");
    expect(env.CORE_VERSION).toBe("v0.13.0");
    expect(env.COMPOSE_IGNORE_ORPHANS).toBe("true");
  });

  test("does not mutate the original versions.env", () => {
    const state = stubState();
    const original = { ...state.versions.env };
    resolvedComposeEnv(state);
    expect(state.versions.env).toEqual(original);
    expect(state.versions.env).not.toHaveProperty("COMPOSE_IGNORE_ORPHANS");
  });
});

// ---------------------------------------------------------------------------
// rewriteCoprocessorDependsOn
// ---------------------------------------------------------------------------
describe("rewriteCoprocessorDependsOn", () => {
  test("rewrites cloned service dependencies with prefix", () => {
    const clonedServices = new Set(["coprocessor-db-migration", "coprocessor-tfhe-worker"]);
    const result = rewriteCoprocessorDependsOn(
      {
        "coprocessor-db-migration": { condition: "service_completed_successfully" },
        "coprocessor-tfhe-worker": { condition: "service_started" },
      },
      "coprocessor1-",
      clonedServices,
    );
    expect(result).toEqual({
      "coprocessor1-db-migration": { condition: "service_completed_successfully" },
      "coprocessor1-tfhe-worker": { condition: "service_started" },
    });
  });

  test("leaves non-cloned dependencies unchanged", () => {
    const clonedServices = new Set(["coprocessor-db-migration"]);
    const result = rewriteCoprocessorDependsOn(
      {
        "coprocessor-db-migration": { condition: "service_completed_successfully" },
        "some-external-service": { condition: "service_healthy" },
      },
      "coprocessor1-",
      clonedServices,
    );
    expect(result).toEqual({
      "coprocessor1-db-migration": { condition: "service_completed_successfully" },
      "some-external-service": { condition: "service_healthy" },
    });
  });

  test("handles empty depends_on", () => {
    expect(rewriteCoprocessorDependsOn({}, "coprocessor1-", new Set())).toEqual({});
  });
});

// ---------------------------------------------------------------------------
// applyBuildPolicy
// ---------------------------------------------------------------------------
describe("applyBuildPolicy", () => {
  test("retags image for overridden service", () => {
    const service: Record<string, unknown> = {
      image: "ghcr.io/zama-ai/fhevm/coprocessor/host-listener:v0.11.0",
      build: { context: "./coprocessor" },
    };
    applyBuildPolicy(service, true);
    expect(service.image).toBe(
      `ghcr.io/zama-ai/fhevm/coprocessor/host-listener:${LOCAL_BUILD_TAG}`,
    );
    // build should be kept for overridden services
    expect(service.build).toBeDefined();
  });

  test("removes build for non-overridden service", () => {
    const service: Record<string, unknown> = {
      image: "ghcr.io/zama-ai/fhevm/coprocessor/host-listener:v0.11.0",
      build: { context: "./coprocessor" },
    };
    applyBuildPolicy(service, false);
    expect(service.image).toBe("ghcr.io/zama-ai/fhevm/coprocessor/host-listener:v0.11.0");
    expect(service.build).toBeUndefined();
  });

  test("handles service without image gracefully", () => {
    const service: Record<string, unknown> = { build: { context: "." } };
    applyBuildPolicy(service, true);
    // retagLocal returns non-string as-is
    expect(service.image).toBeUndefined();
  });
});

// ---------------------------------------------------------------------------
// retagLocal
// ---------------------------------------------------------------------------
describe("retagLocal", () => {
  test("replaces tag in image string", () => {
    expect(retagLocal("ghcr.io/zama-ai/fhevm/host-listener:v0.11.0")).toBe(
      `ghcr.io/zama-ai/fhevm/host-listener:${LOCAL_BUILD_TAG}`,
    );
  });

  test("returns non-string values unchanged", () => {
    expect(retagLocal(42)).toBe(42);
    expect(retagLocal(undefined)).toBeUndefined();
    expect(retagLocal(null)).toBeNull();
  });
});

// ---------------------------------------------------------------------------
// appendVolume
// ---------------------------------------------------------------------------
describe("appendVolume", () => {
  test("adds volume to empty service", () => {
    const service: Record<string, unknown> = {};
    appendVolume(service, "/host/path:/container/path");
    expect(service.volumes).toEqual(["/host/path:/container/path"]);
  });

  test("adds volume to service with existing volumes", () => {
    const service: Record<string, unknown> = { volumes: ["/other:/other"] };
    appendVolume(service, "/host/path:/container/path");
    expect(service.volumes).toEqual(["/other:/other", "/host/path:/container/path"]);
  });

  test("deduplicates by container target path", () => {
    const service: Record<string, unknown> = {
      volumes: ["/old/path:/container/path"],
    };
    appendVolume(service, "/new/path:/container/path");
    expect(service.volumes).toEqual(["/new/path:/container/path"]);
  });

  test("does not add duplicate identical volume", () => {
    const service: Record<string, unknown> = {
      volumes: ["/host/path:/container/path"],
    };
    appendVolume(service, "/host/path:/container/path");
    expect(service.volumes).toEqual(["/host/path:/container/path"]);
  });

  test("handles volumes with multiple colons (read-only)", () => {
    const service: Record<string, unknown> = {
      volumes: ["/old:/container/path:ro"],
    };
    appendVolume(service, "/new:/container/path:ro");
    expect(service.volumes).toEqual(["/new:/container/path:ro"]);
  });
});

// ---------------------------------------------------------------------------
// rewriteComposePaths
// ---------------------------------------------------------------------------
describe("rewriteComposePaths", () => {
  test("resolves relative volume host paths", () => {
    const doc: ComposeDoc = {
      services: {
        svc: {
          volumes: ["./data:/app/data", "named-volume:/app/named"],
        },
      },
    };
    rewriteComposePaths(doc);
    const volumes = doc.services.svc.volumes as string[];
    expect(volumes[0]).toBe(`${path.resolve(TEMPLATE_COMPOSE_DIR, "./data")}:/app/data`);
    // named volumes (no leading dot or slash) stay unchanged
    expect(volumes[1]).toBe("named-volume:/app/named");
  });

  test("resolves relative build context and dockerfile", () => {
    const doc: ComposeDoc = {
      services: {
        svc: {
          build: { context: "./build-ctx", dockerfile: "./Dockerfile" },
        },
      },
    };
    rewriteComposePaths(doc);
    const build = doc.services.svc.build as Record<string, unknown>;
    expect(build.context).toBe(path.resolve(TEMPLATE_COMPOSE_DIR, "./build-ctx"));
    expect(build.dockerfile).toBe(path.resolve(TEMPLATE_COMPOSE_DIR, "./Dockerfile"));
  });

  test("leaves absolute paths unchanged", () => {
    const doc: ComposeDoc = {
      services: {
        svc: {
          volumes: ["/absolute/path:/app/data"],
          build: { context: "/absolute/ctx" },
        },
      },
    };
    rewriteComposePaths(doc);
    expect((doc.services.svc.volumes as string[])[0]).toBe("/absolute/path:/app/data");
    expect((doc.services.svc.build as Record<string, unknown>).context).toBe("/absolute/ctx");
  });

  test("handles non-string volume entries (object mounts)", () => {
    const objMount = { type: "bind", source: "/src", target: "/dest" };
    const doc: ComposeDoc = {
      services: { svc: { volumes: [objMount] } },
    };
    rewriteComposePaths(doc);
    expect((doc.services.svc.volumes as unknown[])[0]).toBe(objMount);
  });
});

// ---------------------------------------------------------------------------
// resolveComposePath
// ---------------------------------------------------------------------------
describe("resolveComposePath", () => {
  test("resolves relative paths against TEMPLATE_COMPOSE_DIR", () => {
    expect(resolveComposePath("./foo")).toBe(path.resolve(TEMPLATE_COMPOSE_DIR, "./foo"));
  });

  test("leaves non-relative paths unchanged", () => {
    expect(resolveComposePath("/absolute")).toBe("/absolute");
    expect(resolveComposePath("named")).toBe("named");
  });
});

// ---------------------------------------------------------------------------
// rewriteVolume
// ---------------------------------------------------------------------------
describe("rewriteVolume", () => {
  test("resolves relative host path", () => {
    expect(rewriteVolume("./data:/app")).toBe(
      `${path.resolve(TEMPLATE_COMPOSE_DIR, "./data")}:/app`,
    );
  });

  test("leaves absolute host path unchanged", () => {
    expect(rewriteVolume("/abs:/app")).toBe("/abs:/app");
  });

  test("leaves named volumes unchanged", () => {
    expect(rewriteVolume("my-vol:/app")).toBe("my-vol:/app");
  });

  test("returns non-string values unchanged", () => {
    const obj = { type: "bind" };
    expect(rewriteVolume(obj)).toBe(obj);
  });
});

// ---------------------------------------------------------------------------
// interpolateComposeValue
// ---------------------------------------------------------------------------
describe("interpolateComposeValue", () => {
  const vars = { FOO: "bar", PORT: "8080" };

  test("interpolates a string", () => {
    expect(interpolateComposeValue("${FOO}:${PORT}", vars)).toBe("bar:8080");
  });

  test("interpolates strings in arrays", () => {
    expect(interpolateComposeValue(["${FOO}", "literal", "${PORT}"], vars)).toEqual([
      "bar",
      "literal",
      "8080",
    ]);
  });

  test("interpolates strings in nested objects", () => {
    expect(
      interpolateComposeValue({ key: "${FOO}", nested: { port: "${PORT}" } }, vars),
    ).toEqual({ key: "bar", nested: { port: "8080" } });
  });

  test("leaves unresolved placeholders unchanged", () => {
    expect(interpolateComposeValue("${MISSING}", vars)).toBe("${MISSING}");
  });

  test("leaves escaped dollar signs unchanged", () => {
    expect(interpolateComposeValue("$${FOO}", vars)).toBe("$${FOO}");
  });

  test("returns non-string/non-object values unchanged", () => {
    expect(interpolateComposeValue(42, vars)).toBe(42);
    expect(interpolateComposeValue(null, vars)).toBeNull();
    expect(interpolateComposeValue(true, vars)).toBe(true);
  });
});

// ---------------------------------------------------------------------------
// interpolateString
// ---------------------------------------------------------------------------
describe("interpolateString", () => {
  test("replaces known variables", () => {
    expect(interpolateString("hello ${NAME}", { NAME: "world" })).toBe("hello world");
  });

  test("leaves unknown variables unchanged", () => {
    expect(interpolateString("${UNKNOWN}", {})).toBe("${UNKNOWN}");
  });

  test("does not replace escaped placeholders", () => {
    expect(interpolateString("$${ESCAPED}", { ESCAPED: "nope" })).toBe("$${ESCAPED}");
  });
});

// ---------------------------------------------------------------------------
// applyInstanceAdjustments
// ---------------------------------------------------------------------------
describe("applyInstanceAdjustments", () => {
  test("injects env_file", () => {
    const service = { container_name: "coprocessor-tfhe-worker", command: ["run"] };
    const result = applyInstanceAdjustments("coprocessor-tfhe-worker", service, "/path/to/env", {});
    expect(result.env_file).toEqual(["/path/to/env"]);
  });

  test("replaces --key-cache-size with --tenant-key-cache-size", () => {
    const service = {
      container_name: "coprocessor-tfhe-worker",
      command: ["run", "--key-cache-size", "100"],
    };
    const result = applyInstanceAdjustments("coprocessor-tfhe-worker", service, "/env", {});
    expect(result.command).toContain("--tenant-key-cache-size");
    expect(result.command).not.toContain("--key-cache-size");
  });

  test("merges override env into environment", () => {
    const service = {
      container_name: "coprocessor-tfhe-worker",
      environment: { EXISTING: "value" },
    };
    const override = { env: { EXTRA: "extra-value" }, args: {} };
    const result = applyInstanceAdjustments("coprocessor-tfhe-worker", service, "/env", {}, override);
    expect(result.environment).toEqual({ EXISTING: "value", EXTRA: "extra-value" });
  });

  test("does not set environment when override has empty env", () => {
    const service = { container_name: "coprocessor-tfhe-worker" };
    const override = { env: {}, args: {} };
    const result = applyInstanceAdjustments("coprocessor-tfhe-worker", service, "/env", {}, override);
    expect(result.environment).toBeUndefined();
  });

  test("injects compat args from policy", () => {
    const service = {
      container_name: "coprocessor-host-listener",
      command: ["run"],
    };
    const compatArgs = {
      "host-listener": [
        ["--coprocessor-api-key", { env: "COPROCESSOR_API_KEY" }],
      ] as Array<readonly [string, { env: string } | { value: string }]>,
    };
    const envVars = { COPROCESSOR_API_KEY: "my-key" };
    const result = applyInstanceAdjustments("coprocessor-host-listener", service, "/env", envVars, undefined, compatArgs);
    expect(result.command).toContain("--coprocessor-api-key");
    expect(result.command).toContain("my-key");
  });

  test("drops unsupported compat flags for old remote images", () => {
    const service = {
      container_name: "coprocessor-gw-listener",
      command: [
        "run",
        "--ciphertext-commits-address=${CIPHERTEXT_COMMITS_ADDRESS}",
        "--gateway-config-address=${GATEWAY_CONFIG_ADDRESS}",
        "--kms-generation-address=${KMS_GENERATION_ADDRESS}",
      ],
    };
    const result = applyInstanceAdjustments(
      "coprocessor-gw-listener",
      service,
      "/env",
      {
        CIPHERTEXT_COMMITS_ADDRESS: "0x1",
        GATEWAY_CONFIG_ADDRESS: "0x2",
        KMS_GENERATION_ADDRESS: "0x3",
      },
      undefined,
      {},
      {
        "gw-listener": ["--ciphertext-commits-address", "--gateway-config-address"],
      },
    );
    expect(result.command).toEqual([
      "run",
      "--kms-generation-address=0x3",
    ]);
  });

  test("injects literal compat args", () => {
    const service = {
      container_name: "coprocessor-transaction-sender",
      command: ["run"],
    };
    const compatArgs = {
      "transaction-sender": [
        ["--delegation-fallback-polling", { value: "30" }],
      ] as Array<readonly [string, { env: string } | { value: string }]>,
    };
    const result = applyInstanceAdjustments("coprocessor-transaction-sender", service, "/env", {}, undefined, compatArgs);
    expect(result.command).toContain("--delegation-fallback-polling");
    expect(result.command).toContain("30");
  });

  test("skips env-based compat args when env var is missing", () => {
    const service = {
      container_name: "coprocessor-host-listener",
      command: ["run"],
    };
    const compatArgs = {
      "host-listener": [
        ["--coprocessor-api-key", { env: "MISSING_KEY" }],
      ] as Array<readonly [string, { env: string } | { value: string }]>,
    };
    const result = applyInstanceAdjustments("coprocessor-host-listener", service, "/env", {}, undefined, compatArgs);
    expect(result.command).not.toContain("--coprocessor-api-key");
  });

  test("applies override args", () => {
    const service = {
      container_name: "coprocessor-tfhe-worker",
      command: ["run", "--existing"],
    };
    const override = { env: {}, args: { "tfhe-worker": ["--extra-flag", "val"] } };
    const result = applyInstanceAdjustments("coprocessor-tfhe-worker", service, "/env", {}, override);
    expect(result.command).toContain("--extra-flag");
    expect(result.command).toContain("val");
  });

  test("applies wildcard override args", () => {
    const service = {
      container_name: "coprocessor-tfhe-worker",
      command: ["run"],
    };
    const override = { env: {}, args: { "*": ["--global-flag"] } };
    const result = applyInstanceAdjustments("coprocessor-tfhe-worker", service, "/env", {}, override);
    expect(result.command).toContain("--global-flag");
  });

  test("interpolates env vars in service values", () => {
    const service = {
      container_name: "coprocessor-tfhe-worker",
      image: "ghcr.io/example:${MY_VERSION}",
    };
    const result = applyInstanceAdjustments("coprocessor-tfhe-worker", service, "/env", { MY_VERSION: "v1.2.3" });
    expect(result.image).toBe("ghcr.io/example:v1.2.3");
  });

  test("does not mutate original service object", () => {
    const service = {
      container_name: "coprocessor-tfhe-worker",
      command: ["run"],
    };
    const original = structuredClone(service);
    applyInstanceAdjustments("coprocessor-tfhe-worker", service, "/env", {});
    expect(service).toEqual(original);
  });
});

// ---------------------------------------------------------------------------
// serviceNameList
// ---------------------------------------------------------------------------
describe("serviceNameList", () => {
  test("returns empty for non-coprocessor components", () => {
    const state = stubState();
    expect(serviceNameList(state, "relayer")).toEqual([]);
    expect(serviceNameList(state, "database")).toEqual([]);
    expect(serviceNameList(state, "minio")).toEqual([]);
  });

  test("returns single-instance service names for count=1", () => {
    const state = stubState();
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

// ---------------------------------------------------------------------------
// overriddenServicesForComponent
// ---------------------------------------------------------------------------
describe("overriddenServicesForComponent", () => {
  test("returns empty set when no overrides", () => {
    const state = stubState();
    const result = overriddenServicesForComponent(state, "coprocessor");
    expect(result.size).toBe(0);
  });

  test("ignores coprocessor overrides because coprocessor builds are scenario-driven", () => {
    const state = stubState({ overrides: [{ group: "coprocessor" }] });
    const result = overriddenServicesForComponent(state, "coprocessor");
    expect(result.size).toBe(0);
  });

  test("ignores per-service coprocessor overrides for the same reason", () => {
    const state = stubState({
      overrides: [{ group: "coprocessor", services: ["coprocessor-tfhe-worker"] }],
    });
    const result = overriddenServicesForComponent(state, "coprocessor");
    expect(result.size).toBe(0);
  });

  test("returns empty set for unrelated component", () => {
    const state = stubState({ overrides: [{ group: "coprocessor" }] });
    const result = overriddenServicesForComponent(state, "relayer");
    expect(result.size).toBe(0);
  });

  test("combines services from multiple overrides", () => {
    const state = stubState({
      overrides: [
        { group: "gateway-contracts" },
        { group: "host-contracts" },
      ],
    });
    const gatewayResult = overriddenServicesForComponent(state, "gateway-sc");
    expect(gatewayResult.has("gateway-sc-deploy")).toBe(true);
    expect(gatewayResult.has("gateway-sc-add-network")).toBe(true);

    const hostResult = overriddenServicesForComponent(state, "host-sc");
    expect(hostResult.has("host-sc-deploy")).toBe(true);
    expect(hostResult.has("host-sc-add-pausers")).toBe(true);
  });
});
