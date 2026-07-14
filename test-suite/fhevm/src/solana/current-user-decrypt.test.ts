import { describe, expect, test } from "bun:test";

import { runSolanaCurrentUserDecrypt, type CurrentUserDecryptDependencies } from "./current-user-decrypt";

const hex32 = (byte: string) => `0x${byte.repeat(64)}`;
const validEnvironment = (): Record<string, string> => ({
  UD_RELAYER_URL: "http://127.0.0.1:3000",
  UD_CONTRACTS_CHAIN_ID: "9223372036854788153",
  UD_HANDLE: hex32("1"),
  UD_SECRET_KEY: hex32("2"),
  UD_CONTEXT_ID: hex32("3"),
  UD_ALLOWED_DOMAIN_KEYS: hex32("4"),
  UD_ACL_VALUE_KEY: hex32("5"),
  UD_EXPECTED: "42",
});

const sdkReturning = (result: unknown): CurrentUserDecryptDependencies => ({
  userDecrypt: async () => result as never,
});

describe("solana-current-user-decrypt", () => {
  test("requires every explicit current-decrypt input", async () => {
    for (const name of Object.keys(validEnvironment())) {
      const environment: Record<string, string | undefined> = validEnvironment();
      delete environment[name];
      await expect(runSolanaCurrentUserDecrypt(environment, sdkReturning([{ value: 42n }]))).rejects.toThrow(
        `missing env ${name}`,
      );
    }
  });

  test("passes the current handle and ACL value key to the SDK and returns the expected plaintext", async () => {
    let received: unknown;
    const value = await runSolanaCurrentUserDecrypt(validEnvironment(), {
      userDecrypt: async (input) => {
        received = input.request;
        return [{ value: 42n }];
      },
    });

    expect(value).toBe(42n);
    expect(received).toMatchObject({
      handles: [hex32("1")],
      allowedAclDomainKeys: [hex32("4")],
      aclValueKey: Uint8Array.from(Buffer.from("5".repeat(64), "hex")),
    });
    expect(received).not.toHaveProperty("mmrProof");
  });

  test("rejects an empty SDK result", async () => {
    await expect(runSolanaCurrentUserDecrypt(validEnvironment(), sdkReturning([]))).rejects.toThrow(
      "returned 0 clear values; expected exactly 1",
    );
  });

  test("rejects more than one SDK result", async () => {
    await expect(
      runSolanaCurrentUserDecrypt(validEnvironment(), sdkReturning([{ value: 42n }, { value: 42n }])),
    ).rejects.toThrow("returned 2 clear values; expected exactly 1");
  });

  test("rejects a mismatched plaintext", async () => {
    await expect(runSolanaCurrentUserDecrypt(validEnvironment(), sdkReturning([{ value: 41n }]))).rejects.toThrow(
      "cleartext 41 != expected 42",
    );
  });

  test("passes SDK queued, malformed, and terminal failures through unchanged", async () => {
    const errors = [
      Object.assign(new Error("request still queued"), { status: "queued" }),
      Object.assign(new Error("malformed response"), { label: "malformed_json" }),
      Object.assign(new Error("request terminated"), { status: "failed" }),
    ];
    for (const error of errors) {
      let thrown: unknown;
      try {
        await runSolanaCurrentUserDecrypt(validEnvironment(), {
          userDecrypt: async () => Promise.reject(error),
        });
      } catch (cause) {
        thrown = cause;
      }
      expect(thrown).toBe(error);
    }
  });
});
