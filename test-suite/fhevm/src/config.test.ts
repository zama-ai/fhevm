import { describe, expect, test } from "bun:test";
import YAML from "yaml";

import { renderRelayerConfig } from "./generate/config";

describe("config", () => {
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

  test("boots the Solana MMR proof service when a Solana host chain is declared", () => {
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
        hostChains: [{ key: "solana", type: "solana", chainId: "9223372036854788153", rpcPort: 8899 }],
      },
    );
    const parsed = YAML.parse(rendered) as {
      solana_proof?: { rpc_url: string; program_id: string; leaf_store_path: string };
    };
    expect(parsed.solana_proof).toEqual({
      rpc_url: "http://host.docker.internal:8899",
      program_id: "SoLaNaProgram111",
      leaf_store_path: "/tmp/solana-mmr-leaves.json",
    });
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
});
