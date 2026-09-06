import { describe, expect, it } from "vitest";

import { resolveNetworkConfig } from "../src/config/networks";
import { resolveChain } from "../src/config/resolve";
import { NETWORKS } from "../src/types";

describe("custom FHEVM network definitions", () => {
  it("configures the Ethereum devnet protocol contract", () => {
    expect(
      resolveNetworkConfig("devnet").fhevmChain.fhevm.contracts.protocolConfig
        ?.address,
    ).toBe("0x1aa1E8f03E6aC23EEd65305fF6C89A3Fc55f13a0");
  });

  it("configures the Polygon Amoy devnet protocol contract", () => {
    expect(
      resolveNetworkConfig("devnet-amoy").fhevmChain.fhevm.contracts
        .protocolConfig?.address,
    ).toBe("0x4CcF009Aba90D04f52b31fc7aDdE240578aFe10F");
  });

  it("configures the Polygon Amoy testnet from the SDK preset", () => {
    const config = resolveNetworkConfig("testnet-amoy");

    expect(config.fhevmChain.fhevm.contracts.protocolConfig?.address).toBe(
      "0x4CcF009Aba90D04f52b31fc7aDdE240578aFe10F",
    );
    expect(config.fhevmChain.fhevm.relayerUrl).toBe(
      "https://relayer.testnet.zama.org",
    );
    expect(config.fheTestAddress).toBe(
      "0xa66bCEd74D1Df0736d0eb8E52371b1b1AAA1F0F0",
    );
  });

  it("configures Polygon PoS mainnet host contracts and the shared mainnet gateway", () => {
    const config = resolveNetworkConfig("polygon");

    expect(config.hostChain.id).toBe(137);
    expect(config.envRpcUrl).toBe("POLYGON_RPC_URL");
    expect(config.fhevmChain.fhevm.contracts.acl.address).toBe(
      "0x6737F17e31cf26a1b62fb0362acC5a16CB156F49",
    );
    expect(config.fhevmChain.fhevm.contracts.protocolConfig?.address).toBe(
      "0x17f62Ab3A1Ea519703cD597410147A30Fa1a7f1e",
    );
    expect(config.fhevmChain.fhevm.relayerUrl).toBe(
      "https://relayer.mainnet.zama.org",
    );
    expect(config.fhevmChain.fhevm.gateway.id).toBe(261_131);
    expect(config.fheTestAddress).toBe(
      "0xFb10eda9e9b4f3f7dd928B6F32fBB94E2a20451d",
    );
  });

  it("does not carry per-network runtime version overrides", () => {
    for (const network of NETWORKS) {
      expect(resolveNetworkConfig(network)).not.toHaveProperty("runtime");
    }
  });

  it("keeps custom relayer overrides independent from runtime version policy", () => {
    const chain = resolveChain({
      network: "testnet",
      relayerUrl: "https://candidate-relayer.example/v2",
    });

    expect(chain.fhevm.relayerUrl).toBe("https://candidate-relayer.example");
    expect(resolveNetworkConfig("testnet")).not.toHaveProperty("runtime");
  });
});
