import { describe, expect, test } from "bun:test";

import {
  serializeKmsHostChains,
  solanaLeafProofUrl,
  SOLANA_LEAF_PROOF_API_KEY,
} from "./generate/solana";

describe("solana", () => {
  test("gives a Solana host chain the leaf-proof route the connector requires", () => {
    const parsed = JSON.parse(
      serializeKmsHostChains([
        {
          url: "http://host.docker.internal:8899",
          chainId: "72057594037940281",
          kind: "solana",
          solanaProgramId: "SoLaNaProgram111",
        },
      ]),
    ) as Array<Record<string, unknown>>;

    // The kms-worker refuses to load a Solana chain without a proof route, and exits before
    // serving anything.
    expect(parsed[0]?.solana_proof_routes).toEqual([
      { url: solanaLeafProofUrl(), api_key: SOLANA_LEAF_PROOF_API_KEY },
    ]);
    expect(parsed[0]?.chain_kind).toBeUndefined();
    expect(parsed[0]?.solana_host_program_id).toBe("SoLaNaProgram111");
    expect(parsed[0]?.acl_address).toBeUndefined();
  });

  test("keeps the proof fields off an EVM host chain", () => {
    const parsed = JSON.parse(
      serializeKmsHostChains([
        { url: "http://host-node:9650", chainId: "9650", kind: "evm", aclAddress: "0xalpha" },
      ]),
    ) as Array<Record<string, unknown>>;

    // The same check refuses an EVM chain that sets a Solana field.
    expect(parsed[0]?.solana_proof_routes).toBeUndefined();
    expect(parsed[0]?.acl_address).toBe("0xalpha");
  });

  test("carries a Solana chain id as a raw integer literal, losslessly", () => {
    // RFC-021 ids exceed Number.MAX_SAFE_INTEGER, so a `JSON.stringify(Number(id))` round trip
    // would silently corrupt the id. Read the literal out of the text rather than through
    // JSON.parse, which would itself go through a double.
    const serialized = serializeKmsHostChains([
      { url: "http://host.docker.internal:8899", chainId: "72057594037940281", kind: "solana" },
    ]);
    expect(serialized).toContain('"chain_id":72057594037940281');
  });
});
