import { describe, expect, test } from "bun:test";
import YAML from "yaml";

import { renderRelayerConfig } from "./generate/config";

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
    expect(parsed.keyurl).toEqual({
      fhe_public_key: {
        data_id: "fhe-public-key-data-id",
        url: "http://0.0.0.0:3001/publicKey.bin",
      },
      crs: {
        data_id: "crs-data-id",
        url: "http://0.0.0.0:3001/crs2048.bin",
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
});
