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
