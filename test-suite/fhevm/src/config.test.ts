import { describe, expect, test } from "bun:test";
import YAML from "yaml";
import { readFileSync } from "node:fs";
import path from "node:path";

import { renderRelayerConfig, renderRelayerHttpConfig } from "./generate/config";
import { RELAYER_HTTP_CONFIG_BASE, RELAYER_HTTP_PORT } from "./layout";
import { predictedCrsId, predictedKeyId } from "./utils/fs";

describe("config", () => {
  test("renders legacy static keyurl config for released relayer images", () => {
    const rendered = renderRelayerConfig(
      {
        versions: { env: { RELAYER_VERSION: "v0.13.0-2" } } as never,
        discovery: undefined,
      },
      `keyurl:
  kms_generation_address: "0x3E0fBCcE61af7C01113027449eEFFF5DCd501419"
  poll_interval_ms: 12000
`,
    );
    const parsed = YAML.parse(rendered) as {
      keyurl: {
        fhe_public_key?: { data_id: string; url: string };
        crs?: { data_id: string; url: string };
        kms_generation_address?: string;
      };
    };
    const fheKeyId = predictedKeyId();
    const crsKeyId = predictedCrsId();
    expect(parsed.keyurl).toEqual({
      fhe_public_key: {
        data_id: fheKeyId,
        url: `http://localhost:9000/kms-public/PUB/PublicKey/${fheKeyId}`,
      },
      crs: {
        data_id: crsKeyId,
        url: `http://localhost:9000/kms-public/PUB/CRS/${crsKeyId}`,
      },
    });
  });

  test("renders legacy keyurl config from discovered material ids", () => {
    const rendered = renderRelayerConfig(
      {
        versions: { env: { RELAYER_VERSION: "v0.11.0" } } as never,
        discovery: {
          objectStoreKeyPrefix: "PUB-p1",
          fheKeyId: "f".repeat(64),
          crsKeyId: "c".repeat(64),
          actualFheKeyId: "a".repeat(64),
          actualCrsKeyId: "b".repeat(64),
          endpoints: {
            objectStoreInternal: "http://object-store:9000",
            objectStoreExternal: "http://172.18.0.10:9000",
          },
        } as never,
      },
      `keyurl:
  kms_generation_address: "0x3E0fBCcE61af7C01113027449eEFFF5DCd501419"
  poll_interval_ms: 12000
`,
    );
    const parsed = YAML.parse(rendered) as {
      keyurl: {
        fhe_public_key?: { data_id: string; url: string };
        crs?: { data_id: string; url: string };
      };
    };
    expect(parsed.keyurl).toEqual({
      fhe_public_key: {
        data_id: "a".repeat(64),
        url: `http://localhost:9000/kms-public/PUB-p1/PublicKey/${"a".repeat(64)}`,
      },
      crs: {
        data_id: "b".repeat(64),
        url: `http://localhost:9000/kms-public/PUB-p1/CRS/${"b".repeat(64)}`,
      },
    });
  });

  test("keeps host-chain-poller keyurl config for modern relayer builds", () => {
    const rendered = renderRelayerConfig(
      {
        versions: { env: { RELAYER_VERSION: "LOCAL BUILD" } } as never,
        discovery: undefined,
      },
      `keyurl:
  kms_generation_address: "0x3E0fBCcE61af7C01113027449eEFFF5DCd501419"
  poll_interval_ms: 12000
`,
    );
    const parsed = YAML.parse(rendered) as {
      keyurl: {
        kms_generation_address?: string;
        poll_interval_ms?: number;
        fhe_public_key?: unknown;
      };
    };
    expect(parsed.keyurl).toEqual({
      kms_generation_address: "0x3E0fBCcE61af7C01113027449eEFFF5DCd501419",
      poll_interval_ms: 12000,
    });
  });

  test("rewrites relayer host chains from the active topology, including the default chain", () => {
    const rendered = renderRelayerConfig(
      {
        versions: { env: { RELAYER_VERSION: "v0.11.0" } } as never,
        discovery: {
          hosts: {
            alpha: { ACL_CONTRACT_ADDRESS: "0xalpha" },
            beta: { ACL_CONTRACT_ADDRESS: "0xbeta" },
          },
        } as never,
      },
      `host_chains:
  - chain_id: 12345
    url: "http://host-node:8545"
    acl_address: "0xtemplate"
`,
      {
        hostChains: [
          { key: "alpha", chainId: "9650", rpcPort: 9650 },
          { key: "beta", chainId: "9750", rpcPort: 9750 },
        ],
      },
    );
    const parsed = YAML.parse(rendered) as {
      host_chains: Array<{ chain_id: number; url: string; acl_address: string }>;
    };
    expect(parsed.host_chains).toEqual([
      { chain_id: 9650, url: "http://host-node:9650", acl_address: "0xalpha" },
      { chain_id: 9750, url: "http://host-node-beta:9750", acl_address: "0xbeta" },
    ]);
  });

  test("omits relayer solana_proof even when a Solana host chain is declared", () => {
    const rendered = renderRelayerConfig(
      {
        versions: { env: { RELAYER_VERSION: "v0.11.0" } } as never,
        discovery: { hosts: { solana: { ACL_CONTRACT_ADDRESS: "SoLaNaProgram111" } } } as never,
      },
      `host_chains:
  - chain_id: 12345
    url: "http://host-node:8545"
    acl_address: "0xtemplate"
`,
      {
        hostChains: [{ key: "solana", type: "solana", chainId: "72057594037940281", rpcPort: 8899 }],
      },
    );
    const parsed = YAML.parse(rendered) as { solana_proof?: unknown };
    expect(parsed.solana_proof).toBeUndefined();
  });

  test("omits solana_proof for an EVM-only topology", () => {
    const rendered = renderRelayerConfig(
      {
        versions: { env: { RELAYER_VERSION: "v0.11.0" } } as never,
        discovery: { hosts: { alpha: { ACL_CONTRACT_ADDRESS: "0xalpha" } } } as never,
      },
      `host_chains:
  - chain_id: 12345
    url: "http://host-node:8545"
    acl_address: "0xtemplate"
`,
      { hostChains: [{ key: "alpha", chainId: "9650", rpcPort: 9650 }] },
    );
    const parsed = YAML.parse(rendered) as { solana_proof?: unknown };
    expect(parsed.solana_proof).toBeUndefined();
  });

  test("renders relayer-http with every host chain and every party's proxy, spare included", () => {
    const rendered = renderRelayerHttpConfig(
      readFileSync(RELAYER_HTTP_CONFIG_BASE, "utf8"),
      {
        hostChains: [
          { key: "host", chainId: "12345", rpcPort: 8545 },
          { key: "solana", type: "solana", chainId: "130140237723663404", rpcPort: 8899 },
        ],
        kms: { parties: 5, threshold: 1, committeeSize: 4 } as never,
      },
      {
        "gateway-sc": {
          PUBLIC_DECRYPTION_THRESHOLD: "3",
          USER_DECRYPTION_THRESHOLD: "3",
          ...Object.fromEntries([0, 1, 2, 3, 4].map((i) => [`KMS_TX_SENDER_ADDRESS_${i}`, `0xsender${i + 1}`])),
        },
        "kms-connector": {
          KMS_CONNECTOR_PROXY_BIND_ADDRESS: "0.0.0.0:8443",
          KMS_CONNECTOR_ETHEREUM_CHAIN_ID: "12345",
          KMS_CONNECTOR_PROTOCOL_CONFIG_CONTRACT__ADDRESS: "0xprotocolconfig",
        },
      },
    );
    const parsed = YAML.parse(rendered);
    expect(parsed.log.format).toBe("json");
    expect(parsed.http.endpoint).toBe(`0.0.0.0:${RELAYER_HTTP_PORT}`);
    expect(parsed.http.supported_chain_ids).toEqual(["12345", "130140237723663404"]);
    expect(parsed.kms_aggregator.public_decrypt.threshold).toBe(3);
    expect(parsed.kms_aggregator.user_decrypt.threshold).toBe(3);
    expect(parsed.kms_aggregator.auth).toMatchObject({ chain_id: 12345, protocol_config: "0xprotocolconfig" });
    // The spare (party 5) is listed, so the node swap reaches it without a new config.
    expect(parsed.kms_aggregator.endpoints).toEqual([
      { name: "kms_1", url: "https://kms-connector-proxy:8443", audience: "0xsender1" },
      { name: "kms_2", url: "https://kms-connector-2-proxy:8443", audience: "0xsender2" },
      { name: "kms_3", url: "https://kms-connector-3-proxy:8443", audience: "0xsender3" },
      { name: "kms_4", url: "https://kms-connector-4-proxy:8443", audience: "0xsender4" },
      { name: "kms_5", url: "https://kms-connector-5-proxy:8443", audience: "0xsender5" },
    ]);
  });
});


test("local relayer configurations retain a polling path for responses emitted during restart", () => {
  for (const file of ["templates/config/relayer.yaml", "config/relayer/local.yaml"]) {
    const template = readFileSync(path.resolve(import.meta.dir, "..", file), "utf8");
    const config = YAML.parse(renderRelayerConfig({ versions: { env: {} } as never }, template));
    expect(config.gateway.listener_pool.listeners).toContainEqual({
      type: "polling", url: "http://gateway-node:8546",
    });
  }
});
