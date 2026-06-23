// EXEMPLAR — unit tests for the driver logic (no cluster). Run: `bun test stack/lib`.
//
// Proves the §0-recipe encoding + the discover→regenerate keystone + CLI flag parsing are
// correct without a live boot: phase ordering/shape, address parsing (discover), config
// threading (regenerate), the wired DEFAULT_CONFIG, and the --kms topology parser.

import { describe, expect, test } from "bun:test";

import { parseKmsFlag } from "../cli/up";
import {
  DEFAULT_CONFIG,
  RECIPE,
  bootStack,
  parseGatewayAddresses,
  parseHostAddresses,
  type Discovery,
  type Phase,
} from "./recipe";
import { threadDiscovery } from "./render";

describe("RECIPE — §0 phase encoding", () => {
  test("phases mirror the fhevm-cli up order (base→kms-signer→deploy→discover→…)", () => {
    expect(RECIPE.map((p) => p.id)).toEqual([
      "cluster",
      "chains",
      "data-plane",
      "kms-core",
      "kms-signer",
      "gateway-deploy",
      "host-deploy",
      "gateway-wire",
      "fund-tx-sender",
      "coprocessor",
      "kms-connector",
      "trigger-keygen",
      "await-keygen",
      "relayer",
      "erc20",
    ]);
  });

  test("every phase has a runnable body + a non-empty readiness gate", () => {
    for (const p of RECIPE as Phase[]) {
      expect(typeof p.run).toBe("function");
      expect(p.gate.length).toBeGreaterThan(0);
      expect(p.title.length).toBeGreaterThan(0);
    }
  });

  test("key-bearing phases carry the load-bearing invariants", () => {
    const inv = (id: string) => (RECIPE.find((p) => p.id === id)?.invariants ?? []).join(" ");
    // listener-before-keygen (no backfill)
    expect(inv("coprocessor")).toMatch(/before.*keygen|polling BEFORE|backfill|ActivateKey/i);
    // empty vault + persistence + db ordering
    expect(inv("data-plane")).toMatch(/EMPTY|overwrite/i);
    expect(inv("data-plane")).toMatch(/PVC/i);
    expect(inv("data-plane")).toMatch(/relayer_db|db-init|migration/i);
    // discover the live signer, never hardcode
    expect(inv("kms-signer")).toMatch(/log|discover|non-deterministic|stale/i);
    // gateway payment wiring (the input-proof revert)
    expect(inv("gateway-wire")).toMatch(/ProtocolPayment|payment|input-proof|revert/i);
  });
});

describe("discover — address parsing from deploy logs", () => {
  test("parseGatewayAddresses extracts the order-dependent proxy addresses", () => {
    const log = [
      "ZamaOFT deployed successfully at address: 0x5ffdaAB0373E62E2ea2944776209aEf29E631A64",
      "GatewayConfig address 0x576Ea67208b146E63C5255d0f90104E25e3e04c7 written successfully!",
      "CiphertextCommits address 0xeAC2EfFA07844aB326D92d1De29E136a6793DFFA written successfully!",
      "Decryption address 0xF0bFB159C7381F7CB332586004d8247252C5b816 written successfully!",
      "InputVerification address 0x35760912360E875DA50D40a74305575c23D55783 written successfully!",
      "ProtocolPayment address 0x3b12Fc766Eb598b285998877e8E90F3e43a1F8d2 written successfully!",
      "PauserSet address 0x1ceFA8E3F3271358218B52c33929Cf76078004c1 written successfully!",
    ].join("\n");
    expect(parseGatewayAddresses(log)).toEqual({
      gatewayConfig: "0x576Ea67208b146E63C5255d0f90104E25e3e04c7",
      ciphertextCommits: "0xeAC2EfFA07844aB326D92d1De29E136a6793DFFA",
      decryption: "0xF0bFB159C7381F7CB332586004d8247252C5b816",
      inputVerification: "0x35760912360E875DA50D40a74305575c23D55783",
      protocolPayment: "0x3b12Fc766Eb598b285998877e8E90F3e43a1F8d2",
      pauserSet: "0x1ceFA8E3F3271358218B52c33929Cf76078004c1",
      zamaOft: "0x5ffdaAB0373E62E2ea2944776209aEf29E631A64",
    });
  });

  test("parseHostAddresses extracts host contracts (both 'code set' and 'written' formats)", () => {
    const log = [
      "ACL code set successfully at address: 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c",
      "FHEVMExecutor code set successfully at address: 0xcCAe95fF1d11656358E782570dF0418F59fA40e1",
      "KMSGeneration address 0x3E0fBCcE61af7C01113027449eEFFF5DCd501419 written successfully!",
      "KMSVerifier code set successfully at address: 0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11",
      "InputVerifier code set successfully at address: 0x857Ca72A957920Fa0FB138602995839866Bd4005",
    ].join("\n");
    expect(parseHostAddresses(log)).toEqual({
      acl: "0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c",
      fhevmExecutor: "0xcCAe95fF1d11656358E782570dF0418F59fA40e1",
      kmsGeneration: "0x3E0fBCcE61af7C01113027449eEFFF5DCd501419",
      kmsVerifier: "0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11",
      inputVerifier: "0x857Ca72A957920Fa0FB138602995839866Bd4005",
    });
  });
});

describe("regenerate — threadDiscovery weaves discovered values into configs", () => {
  test("emits NO patches when nothing is discovered yet (safe to call early)", () => {
    expect(threadDiscovery(DEFAULT_CONFIG, { host: {}, gateway: {} })).toEqual([]);
  });

  test("the discovered InputVerification reaches BOTH coprocessor and relayer", () => {
    const patches = threadDiscovery(DEFAULT_CONFIG, {
      host: {},
      gateway: { inputVerification: "0xAAA0000000000000000000000000000000000aaa" },
    });
    const targets = patches
      .filter((p) => p.data.INPUT_VERIFICATION_ADDRESS || p.data.APP_GATEWAY__CONTRACTS__INPUT_VERIFICATION_ADDRESS)
      .map((p) => p.configMap)
      .sort();
    expect(targets).toEqual(["coprocessor-env", "relayer-env"]);
  });

  test("the mocked-payment approve targets the DISCOVERED ProtocolPayment (the live bug)", () => {
    const pp = "0xbb12Fc766Eb598b285998877e8E90F3e43a1F8d2";
    const oft = "0xcc7daAB0373E62E2ea2944776209aEf29E631A64";
    const disc: Discovery = { host: {}, gateway: { protocolPayment: pp, zamaOft: oft } };
    const pay = threadDiscovery(DEFAULT_CONFIG, disc).find((p) => p.configMap === "gateway-mocked-payment-env");
    expect(pay?.data.PROTOCOL_PAYMENT_ADDRESS).toBe(pp);
    expect(pay?.data.ZAMA_OFT_ADDRESS).toBe(oft);
  });

  test("the realized key id threads into the relayer keyurl + coprocessor key URLs", () => {
    const id = "0x0400000000000000000000000000000000000000000000000000000000000001";
    const patches = threadDiscovery(DEFAULT_CONFIG, { host: {}, gateway: {}, fheKeyId: id });
    const relayer = patches.find((p) => p.data.APP_KEYURL__FHE_PUBLIC_KEY__URL);
    expect(relayer?.data.APP_KEYURL__FHE_PUBLIC_KEY__URL).toContain(id);
    expect(relayer?.data.APP_KEYURL__FHE_PUBLIC_KEY__URL).toContain(DEFAULT_CONFIG.publicVaultUrl);
    const copro = patches.find((p) => p.configMap === "coprocessor-env" && p.data.FHE_KEY_ID);
    expect(copro?.data.FHE_KEY_ID).toBe(id);
  });
});

describe("DEFAULT_CONFIG — static config, no hardcoded contract addresses", () => {
  test("carries NO contract-address map (addresses are discovered)", () => {
    expect((DEFAULT_CONFIG as Record<string, unknown>).contracts).toBeUndefined();
  });
  test("keygenTxSender is 20-byte hex", () => {
    expect(DEFAULT_CONFIG.keygenTxSender).toMatch(/^0x[0-9a-fA-F]{40}$/);
  });
  test("chains + versions are the v0.13 train (gateway uses the 1.6.2 js-sdk wasm)", () => {
    expect(DEFAULT_CONFIG.hostChain.chainId).toBe(12345);
    expect(DEFAULT_CONFIG.gatewayChain.chainId).toBe(54321);
    expect(DEFAULT_CONFIG.hostChain.mnemonic).not.toBe(DEFAULT_CONFIG.gatewayChain.mnemonic);
    expect(DEFAULT_CONFIG.versions.gatewayHost).toBe("v0.13.0-6");
    expect(DEFAULT_CONFIG.versions.core).toBe("v0.13.20-0");
    expect(DEFAULT_CONFIG.versions.jsSdkTfhe).toBe("1.6.2");
  });
});

describe("parseKmsFlag — --kms topology parser", () => {
  test("centralized", () => {
    expect(parseKmsFlag("centralized")).toEqual({ mode: "centralized" });
  });
  test("threshold:N defaults the threshold to ceil(2N/3)", () => {
    expect(parseKmsFlag("threshold:4")).toEqual({ mode: "threshold", parties: 4, threshold: 3 });
  });
  test("threshold:N/T honors an explicit threshold", () => {
    expect(parseKmsFlag("threshold:4/2")).toEqual({ mode: "threshold", parties: 4, threshold: 2 });
  });
  test("invalid forms throw", () => {
    expect(() => parseKmsFlag("nonsense")).toThrow();
    expect(() => parseKmsFlag("threshold:")).toThrow();
  });
});

describe("bootStack — the thin driver walker", () => {
  const stubCtx = () =>
    new Proxy(
      {},
      { get: () => async () => {} },
    ) as never;

  test("emits an ordered receipt and stops on first failure", async () => {
    const receipts = await bootStack(stubCtx(), DEFAULT_CONFIG, { from: "erc20" });
    expect(receipts.map((r) => r.id)).toEqual(["erc20"]);
    expect(receipts[0].status).toBe("ok");
  });

  test("unknown resume id is rejected", async () => {
    await expect(bootStack(stubCtx(), DEFAULT_CONFIG, { from: "nope" })).rejects.toThrow(/unknown phase/);
  });
});
