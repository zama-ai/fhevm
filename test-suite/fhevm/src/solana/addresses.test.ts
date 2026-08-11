import { afterEach, describe, expect, test } from "bun:test";
import { mkdtemp, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";

import {
  evmAddressBytes,
  readGatewayBootstrapInputs,
  SOLANA_HOST_CHAIN_ID,
  SOLANA_HOST_CHAIN_ID_I64,
} from "./addresses";

const ADDRESS_A = "0x000000000000000000000000000000000000aaaa";
const ADDRESS_B = "0x1111111111111111111111111111111111111111";

const word = (hex: string): string => hex.replace(/^0x/, "").padStart(64, "0");
const addressArrayReturnData = (addresses: readonly string[]): string =>
  `0x${word("0x20")}${word(`0x${addresses.length.toString(16)}`)}${addresses.map(word).join("")}`;

describe("solana host chain id constants", () => {
  test("i64 form is the same 64-bit pattern as the u64 form", () => {
    expect(SOLANA_HOST_CHAIN_ID).toBe(9223372036854788153n);
    expect(SOLANA_HOST_CHAIN_ID_I64).toBe(-9223372036854763463n);
    expect(BigInt.asUintN(64, SOLANA_HOST_CHAIN_ID_I64)).toBe(SOLANA_HOST_CHAIN_ID);
  });
});

describe("evmAddressBytes", () => {
  test("decodes a 0x-prefixed 20-byte address", () => {
    const bytes = evmAddressBytes(ADDRESS_B);
    expect(bytes).toHaveLength(20);
    expect(bytes.every((byte) => byte === 0x11)).toBe(true);
  });

  test("rejects short, long, and non-hex inputs", () => {
    expect(() => evmAddressBytes("0x1234")).toThrow("20-byte EVM address");
    expect(() => evmAddressBytes(`${ADDRESS_B}00`)).toThrow("20-byte EVM address");
    expect(() => evmAddressBytes("0xzz11111111111111111111111111111111111111")).toThrow("20-byte EVM address");
  });
});

describe("readGatewayBootstrapInputs", () => {
  const originalFetch = globalThis.fetch;
  afterEach(() => {
    globalThis.fetch = originalFetch;
  });

  const writeAddressArtifact = async (contents: string): Promise<string> => {
    const dir = await mkdtemp(path.join(tmpdir(), "gateway-addresses-"));
    const file = path.join(dir, ".env.gateway");
    await writeFile(file, contents);
    return file;
  };

  test("combines the address artifact with live chain id and signer sets", async () => {
    const addressesPath = await writeAddressArtifact(
      [
        "GATEWAY_CONFIG_ADDRESS=0x2222222222222222222222222222222222222222",
        `INPUT_VERIFICATION_ADDRESS=${ADDRESS_A}`,
        `DECRYPTION_ADDRESS=${ADDRESS_B}`,
      ].join("\n"),
    );
    const calls: { method: string; data?: string }[] = [];
    globalThis.fetch = (async (_url: string | URL | Request, options?: RequestInit) => {
      const request = JSON.parse(String(options?.body)) as {
        id: number;
        method: string;
        params: [{ data?: string }?];
      };
      calls.push({ method: request.method, data: request.params?.[0]?.data });
      // 0x9164d0ae / 0x7eaac8f2 are viem's derived selectors for getCoprocessorSigners() /
      // getKmsSigners() — the same ones the retired bash pinned for `cast call`.
      const result =
        request.method === "eth_chainId"
          ? "0xd903"
          : addressArrayReturnData(request.params?.[0]?.data === "0x9164d0ae" ? [ADDRESS_A] : [ADDRESS_A, ADDRESS_B]);
      return new Response(JSON.stringify({ jsonrpc: "2.0", id: request.id, result }));
    }) as typeof fetch;

    const inputs = await readGatewayBootstrapInputs({ gatewayRpcUrl: "http://127.0.0.1:8546", addressesPath });
    expect(inputs.gatewayChainId).toBe(55555n);
    expect(Buffer.from(inputs.inputVerificationContract).toString("hex")).toBe(ADDRESS_A.slice(2));
    expect(Buffer.from(inputs.decryptionContract).toString("hex")).toBe(ADDRESS_B.slice(2));
    expect(inputs.coprocessorSigners).toHaveLength(1);
    expect(inputs.kmsSigners).toHaveLength(2);
    expect(calls.map((call) => call.method).sort()).toEqual(["eth_call", "eth_call", "eth_chainId"]);
  });

  test("fails on a missing artifact key and on an RPC error", async () => {
    const addressesPath = await writeAddressArtifact(`INPUT_VERIFICATION_ADDRESS=${ADDRESS_A}`);
    await expect(readGatewayBootstrapInputs({ gatewayRpcUrl: "http://unused", addressesPath })).rejects.toThrow(
      "missing GATEWAY_CONFIG_ADDRESS",
    );

    const complete = await writeAddressArtifact(
      [
        "GATEWAY_CONFIG_ADDRESS=0x2222222222222222222222222222222222222222",
        `INPUT_VERIFICATION_ADDRESS=${ADDRESS_A}`,
        `DECRYPTION_ADDRESS=${ADDRESS_B}`,
      ].join("\n"),
    );
    globalThis.fetch = (async () =>
      new Response(JSON.stringify({ jsonrpc: "2.0", id: 1, error: { message: "boom" } }))) as unknown as typeof fetch;
    await expect(readGatewayBootstrapInputs({ gatewayRpcUrl: "http://unused", addressesPath: complete })).rejects.toThrow(
      "boom",
    );
  });
});
