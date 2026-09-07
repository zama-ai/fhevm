import { describe, expect, test } from "bun:test";

import {
  serializeKmsHostChains,
  solanaHostChainIdI64,
  solanaLeafProofUrl,
  SOLANA_LEAF_PROOF_API_KEY,
} from "./generate/solana";

describe("solana", () => {
  test("gives a Solana host chain the leaf-proof endpoints and key the connector requires", () => {
    const parsed = JSON.parse(
      serializeKmsHostChains([
        {
          url: "http://host.docker.internal:8899",
          chainId: "9223372036854788153",
          kind: "solana",
          solanaProgramId: "SoLaNaProgram111",
        },
      ]),
    ) as Array<Record<string, unknown>>;

    // `validate_host_chain_configs` in the kms-worker refuses a Solana chain whose
    // `solana_proof_endpoints` is empty or whose `solana_proof_api_key` is absent, and the
    // worker exits before serving anything.
    expect(parsed[0]?.solana_proof_endpoints).toEqual([solanaLeafProofUrl()]);
    expect(parsed[0]?.solana_proof_api_key).toBe(SOLANA_LEAF_PROOF_API_KEY);
    expect(parsed[0]?.solana_host_program_id).toBe("SoLaNaProgram111");
    expect(parsed[0]?.acl_address).toBeUndefined();
  });

  test("keeps the proof fields off an EVM host chain", () => {
    const parsed = JSON.parse(
      serializeKmsHostChains([
        { url: "http://host-node:9650", chainId: "9650", kind: "evm", aclAddress: "0xalpha" },
      ]),
    ) as Array<Record<string, unknown>>;

    // The same validator rejects an EVM chain that sets either field.
    expect(parsed[0]?.solana_proof_endpoints).toBeUndefined();
    expect(parsed[0]?.solana_proof_api_key).toBeUndefined();
    expect(parsed[0]?.acl_address).toBe("0xalpha");
  });

  test("carries a Solana chain id as a raw integer literal, losslessly", () => {
    // RFC-021 ids exceed Number.MAX_SAFE_INTEGER, so a `JSON.stringify(Number(id))` round trip
    // would silently corrupt the id. Read the literal out of the text rather than through
    // JSON.parse, which would itself go through a double.
    const serialized = serializeKmsHostChains([
      { url: "http://host.docker.internal:8899", chainId: "9223372036854788153", kind: "solana" },
    ]);
    expect(serialized).toContain('"chain_id":9223372036854788153');
  });

  test("maps a u64 host chain id to the two's-complement i64 the coprocessor DB stores", () => {
    expect(solanaHostChainIdI64("9223372036854788153")).toBe("-9223372036854763463");
    expect(solanaHostChainIdI64("9650")).toBe("9650");
  });
});
