// EXEMPLAR — unit tests for the driver logic (no cluster). Run: `bun test stack/lib`.
//
// Proves the §0-recipe encoding + the discover→regenerate keystone + CLI flag parsing are
// correct without a live boot: phase ordering/shape, address parsing (discover), config
// threading (regenerate), the wired DEFAULT_CONFIG, and the --kms topology parser.

import { describe, expect, mock, test } from "bun:test";

// The recipe phases shell out via the module-level `kubectlApply` (not through ctx), so the
// hermetic driver tests below must stub it — otherwise `bun test` runs real `kubectl apply`
// against the developer's live kubeconfig. ctx-level calls are stubbed by the Proxy in stubCtx.
import * as kubectl from "./kubectl";

mock.module("./kubectl", () => ({ ...kubectl, kubectlApply: async () => "" }));

import { parseKmsFlag } from "../cli/up";
import {
  DEFAULT_CONFIG,
  RECIPE,
  STATIC_GATEWAY_ADDRESSES,
  STATIC_HOST_ADDRESSES,
  bootStack,
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

describe("static addresses — deterministic, no log-scraping", () => {
  const isAddr = (v: string | undefined) => typeof v === "string" && /^0x[0-9a-fA-F]{40}$/.test(v);

  test("gateway table is complete and well-formed", () => {
    const a = STATIC_GATEWAY_ADDRESSES;
    expect(
      [a.gatewayConfig, a.inputVerification, a.ciphertextCommits, a.decryption, a.protocolPayment, a.pauserSet, a.zamaOft].every(isAddr),
    ).toBe(true);
    // The gateway InputVerification — its address is threaded into host-sc-env as the
    // cross-chain EIP712 verifyingContractSource; a stale value here causes InvalidSigner.
    expect(a.inputVerification).toBe("0x35760912360E875DA50D40a74305575c23D55783");
  });

  test("host table is complete and well-formed", () => {
    const h = STATIC_HOST_ADDRESSES;
    expect([h.acl, h.fhevmExecutor, h.kmsGeneration, h.kmsVerifier, h.inputVerifier, h.hcuLimit].every(isAddr)).toBe(true);
  });
});

describe("regenerate — threadDiscovery weaves discovered values into configs", () => {
  test("emits NO patches when nothing is discovered yet (safe to call early)", () => {
    expect(threadDiscovery(DEFAULT_CONFIG, { host: {}, gateway: {} })).toEqual([]);
  });

  test("the discovered InputVerification reaches every consumer (incl. host-sc-env — the InvalidSigner fix)", () => {
    const patches = threadDiscovery(DEFAULT_CONFIG, {
      host: {},
      gateway: { inputVerification: "0xAAA0000000000000000000000000000000000aaa" },
    });
    const targets = patches
      .filter((p) => p.data.INPUT_VERIFICATION_ADDRESS || p.data.APP_GATEWAY__CONTRACTS__INPUT_VERIFICATION_ADDRESS)
      .map((p) => p.configMap)
      .sort();
    // host-sc-env: the host InputVerifier's EIP712 verifyingContractSource = THIS gateway address;
    // a stale value here is the InvalidSigner bug. erc20-env: lets `up` drive the e2e self-contained.
    expect(targets).toEqual(["coprocessor-env", "erc20-env", "host-sc-env", "relayer-env"]);
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
