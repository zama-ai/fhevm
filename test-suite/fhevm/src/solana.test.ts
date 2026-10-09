import { describe, expect, test } from "bun:test";

import { serializeKmsHostChains, solanaMerkleProofUrl } from "./generate/solana";

const PROOF_SERVER_SIGNER = "0x6254A198F67ad40290a2E7B48aDB2d19B71f67BD";

describe("solana", () => {
  test("gives a Solana host chain the Merkle proof server the connector requires", () => {
    const parsed = JSON.parse(
      serializeKmsHostChains(
        [
          {
            url: "http://host.docker.internal:8899",
            chainId: "72057594037940281",
            kind: "solana",
            solanaProgramId: "SoLaNaProgram111",
          },
        ],
        PROOF_SERVER_SIGNER,
      ),
    ) as Array<Record<string, unknown>>;

    // The kms-worker refuses to load a Solana chain without a proof server, and exits before
    // serving anything. It signs each request for the server's signer address.
    expect(parsed[0]?.solana_proof_servers).toEqual([
      { url: solanaMerkleProofUrl(), signer_address: PROOF_SERVER_SIGNER },
    ]);
    expect(parsed[0]?.solana_host_program_id).toBe("SoLaNaProgram111");
    expect(parsed[0]?.acl_address).toBeUndefined();
  });

  test("keeps the proof fields off an EVM host chain", () => {
    const parsed = JSON.parse(
      serializeKmsHostChains(
        [{ url: "http://host-node:9650", chainId: "9650", kind: "evm", aclAddress: "0xalpha" }],
        PROOF_SERVER_SIGNER,
      ),
    ) as Array<Record<string, unknown>>;

    // The same check refuses an EVM chain that sets a Solana field.
    expect(parsed[0]?.solana_proof_servers).toBeUndefined();
    expect(parsed[0]?.acl_address).toBe("0xalpha");
  });

  test("carries a Solana chain id as a raw integer literal, losslessly", () => {
    // RFC-021 ids exceed Number.MAX_SAFE_INTEGER, so a `JSON.stringify(Number(id))` round trip
    // would silently corrupt the id. Read the literal out of the text rather than through
    // JSON.parse, which would itself go through a double.
    const serialized = serializeKmsHostChains(
      [{ url: "http://host.docker.internal:8899", chainId: "72057594037940281", kind: "solana" }],
      PROOF_SERVER_SIGNER,
    );
    expect(serialized).toContain('"chain_id":72057594037940281');
  });
});
