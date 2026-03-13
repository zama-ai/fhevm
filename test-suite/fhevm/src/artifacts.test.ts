import fs from "node:fs/promises";
import path from "node:path";

import { describe, expect, test } from "bun:test";
import YAML from "yaml";

import { composeUp, regen, resolvedComposeEnv, rewriteRelayerConfig, serviceNameList } from "./artifacts";
import { composePath, envPath, TEMPLATE_COMPOSE_DIR } from "./layout";
import { stubState } from "./test-helpers";
import { readEnvFile } from "./utils";

describe("resolvedComposeEnv", () => {
  test("includes version env and COMPOSE_IGNORE_ORPHANS", () => {
    const env = resolvedComposeEnv(stubState());
    expect(env.GATEWAY_VERSION).toBe("v0.11.0");
    expect(env.CORE_VERSION).toBe("v0.13.0");
    expect(env.COMPOSE_IGNORE_ORPHANS).toBe("true");
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

describe("compose templates", () => {
  test("transaction sender does not hardcode host chain url", async () => {
    const template = await fs.readFile(
      path.join(TEMPLATE_COMPOSE_DIR, "coprocessor-docker-compose.yml"),
      "utf8",
    );
    expect(template.includes("--host-chain-url")).toBe(false);
  });

  test("coprocessor local builds are serialized", async () => {
    const file = composePath("coprocessor");
    await fs.mkdir(path.dirname(file), { recursive: true });
    await fs.writeFile(
      file,
      [
        "services:",
        "  coprocessor-host-listener:",
        "    image: ghcr.io/zama-ai/fhevm/coprocessor/host-listener:fhevm-local",
        "  coprocessor-gw-listener:",
        "    image: ghcr.io/zama-ai/fhevm/coprocessor/gw-listener:fhevm-local",
      ].join("\n"),
    );
    const calls: string[][] = [];
    try {
      await composeUp(
        "coprocessor",
        stubState({ overrides: [{ group: "coprocessor" }] }),
        {
          runner: async () => ({ stdout: "", stderr: "", code: 1 }),
          liveRunner: async (argv) => {
            calls.push(argv);
            return 0;
          },
        },
        async () => {},
        () => {},
      );
    } finally {
      await fs.rm(file, { force: true });
    }
    expect(calls.filter((argv) => argv.includes("build"))).toEqual([
      [
        "docker",
        "compose",
        "-p",
        "fhevm",
        "--env-file",
        path.join(path.dirname(path.dirname(file)), "env", "versions.env"),
        "-f",
        file,
        "build",
        "coprocessor-host-listener",
      ],
      [
        "docker",
        "compose",
        "-p",
        "fhevm",
        "--env-file",
        path.join(path.dirname(path.dirname(file)), "env", "versions.env"),
        "-f",
        file,
        "build",
        "coprocessor-gw-listener",
      ],
    ]);
  });

  test("legacy relayer config keeps top-level readiness retry", () => {
    const config = rewriteRelayerConfig(
      {
        gateway: {
          readiness_checker: {
            host_acl_check: { retry: { max_attempts: 3, retry_interval_ms: 1000 } },
            gw_ciphertext_check: { retry: { max_attempts: 15, retry_interval_ms: 2000 } },
            public_decrypt: { capacity: 1 },
            user_decrypt: { capacity: 1 },
          },
        },
      },
      stubState(),
    ) as { gateway: { readiness_checker: Record<string, unknown> } };
    expect(config.gateway.readiness_checker.retry).toEqual({
      max_attempts: 15,
      retry_interval_ms: 2000,
    });
    expect(config.gateway.readiness_checker.host_acl_check).toBeUndefined();
  });

  test("modern relayer config stays unchanged", () => {
    const input = {
      gateway: {
        readiness_checker: {
          gw_ciphertext_check: { retry: { max_attempts: 15, retry_interval_ms: 2000 } },
          host_acl_check: { retry: { max_attempts: 3, retry_interval_ms: 1000 } },
        },
      },
    };
    expect(
      rewriteRelayerConfig(
        structuredClone(input),
        stubState({
          envOverrides: {
            RELAYER_VERSION: "v0.10.0",
            RELAYER_MIGRATE_VERSION: "v0.10.0",
          },
        }),
      ),
    ).toEqual(input);
  });

  test("gateway mocked payment env uses discovered protocol payment address", async () => {
    const file = envPath("gateway-mocked-payment");
    const previous = await fs.readFile(file, "utf8").catch(() => "");
    const hadPrevious = previous.length > 0;
    try {
      await regen(
        stubState({
          discovery: {
            gateway: {
              PROTOCOL_PAYMENT_ADDRESS: "0x1111111111111111111111111111111111111111",
              INPUT_VERIFICATION_ADDRESS: "0x2222222222222222222222222222222222222222",
              DECRYPTION_ADDRESS: "0x3333333333333333333333333333333333333333",
              GATEWAY_CONFIG_ADDRESS: "0x4444444444444444444444444444444444444444",
              KMS_GENERATION_ADDRESS: "0x5555555555555555555555555555555555555555",
              CIPHERTEXT_COMMITS_ADDRESS: "0x6666666666666666666666666666666666666666",
            },
            host: {
              ACL_CONTRACT_ADDRESS: "0x7777777777777777777777777777777777777777",
              FHEVM_EXECUTOR_CONTRACT_ADDRESS: "0x8888888888888888888888888888888888888888",
              INPUT_VERIFIER_CONTRACT_ADDRESS: "0x9999999999999999999999999999999999999999",
              KMS_VERIFIER_CONTRACT_ADDRESS: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
              PAUSER_SET_CONTRACT_ADDRESS: "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            },
            kmsSigner: "0xcccccccccccccccccccccccccccccccccccccccc",
            fheKeyId: "fhe-key-id",
            crsKeyId: "crs-key-id",
            endpoints: {
              gatewayHttp: "http://gateway-node:8546",
              gatewayWs: "ws://gateway-node:8546",
              hostHttp: "http://host-node:8545",
              hostWs: "ws://host-node:8545",
              minioInternal: "http://minio:9000",
              minioExternal: "http://127.0.0.1:9000",
            },
          },
        }),
        {
          runner: async () => ({ stdout: "", stderr: "", code: 0 }),
        },
      );
      const env = await readEnvFile(file);
      expect(env.PROTOCOL_PAYMENT_ADDRESS).toBe("0x1111111111111111111111111111111111111111");
    } finally {
      if (hadPrevious) {
        await fs.writeFile(file, previous);
      } else {
        await fs.rm(file, { force: true });
      }
    }
  });

  test("legacy tx-sender compat injects literal retry flags", async () => {
    const state = stubState({
      discovery: {
        gateway: {
          PROTOCOL_PAYMENT_ADDRESS: "0x1111111111111111111111111111111111111111",
          INPUT_VERIFICATION_ADDRESS: "0x2222222222222222222222222222222222222222",
          DECRYPTION_ADDRESS: "0x3333333333333333333333333333333333333333",
          GATEWAY_CONFIG_ADDRESS: "0x4444444444444444444444444444444444444444",
          KMS_GENERATION_ADDRESS: "0x5555555555555555555555555555555555555555",
          CIPHERTEXT_COMMITS_ADDRESS: "0x6666666666666666666666666666666666666666",
          MULTICHAIN_ACL_ADDRESS: "0x7777777777777777777777777777777777777777",
        },
        host: {
          ACL_CONTRACT_ADDRESS: "0x8888888888888888888888888888888888888888",
          FHEVM_EXECUTOR_CONTRACT_ADDRESS: "0x9999999999999999999999999999999999999999",
          INPUT_VERIFIER_CONTRACT_ADDRESS: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
          KMS_VERIFIER_CONTRACT_ADDRESS: "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
          PAUSER_SET_CONTRACT_ADDRESS: "0xcccccccccccccccccccccccccccccccccccccccc",
        },
        kmsSigner: "0xdddddddddddddddddddddddddddddddddddddddd",
        fheKeyId: "fhe-key-id",
        crsKeyId: "crs-key-id",
        endpoints: {
          gatewayHttp: "http://gateway-node:8546",
          gatewayWs: "ws://gateway-node:8546",
          hostHttp: "http://host-node:8545",
          hostWs: "ws://host-node:8545",
          minioInternal: "http://minio:9000",
          minioExternal: "http://127.0.0.1:9000",
        },
      },
    });
    await regen(state, {
      runner: async () => ({ stdout: "", stderr: "", code: 0 }),
    });
    const compose = YAML.parse(await fs.readFile(composePath("coprocessor"), "utf8")) as {
      services: { "coprocessor-transaction-sender": { command: string[] } };
    };
    expect(compose.services["coprocessor-transaction-sender"].command).toContain("--delegation-fallback-polling");
    expect(compose.services["coprocessor-transaction-sender"].command).toContain("30");
    expect(compose.services["coprocessor-transaction-sender"].command).toContain("--delegation-max-retry");
    expect(compose.services["coprocessor-transaction-sender"].command).toContain("100000");
    expect(compose.services["coprocessor-transaction-sender"].command).toContain("--retry-immediately-on-nonce-error");
    expect(compose.services["coprocessor-transaction-sender"].command).toContain("2");
  });

});
