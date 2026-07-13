import { describe, expect, it } from "vitest";

import { resolveNetworkConfig } from "../src/config/networks";

describe("custom FHEVM network definitions", () => {
  it("configures the Ethereum devnet protocol contract", () => {
    expect(
      resolveNetworkConfig("devnet").fhevmChain.fhevm.contracts.protocolConfig
        ?.address,
    ).toBe("0x51f9AFBc89Ea792e1a21a12AB802ab58D4dbee83");
  });

  it("configures the Polygon Amoy devnet protocol contract", () => {
    expect(
      resolveNetworkConfig("devnet-amoy").fhevmChain.fhevm.contracts
        .protocolConfig?.address,
    ).toBe("0x4CcF009Aba90D04f52b31fc7aDdE240578aFe10F");
  });
});
