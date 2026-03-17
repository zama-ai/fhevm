import { describe, expect, test } from "bun:test";

import {
  renderGatewayAddressesEnv,
  renderGatewayAddressesSolidity,
  renderHostAddressesEnv,
  renderHostAddressesSolidity,
  renderPaymentBridgingAddressesSolidity,
} from "./render-addresses";
import { stubState } from "./test-helpers";

describe("render-addresses", () => {
  test("renders gateway address env and solidity artifacts from discovery", () => {
    const state = stubState({
      discovery: {
        gateway: {
          GATEWAY_CONFIG_ADDRESS: "0x1",
          INPUT_VERIFICATION_ADDRESS: "0x2",
          KMS_GENERATION_ADDRESS: "0x3",
          CIPHERTEXT_COMMITS_ADDRESS: "0x4",
          DECRYPTION_ADDRESS: "0x5",
          PROTOCOL_PAYMENT_ADDRESS: "0x6",
          PAUSER_SET_ADDRESS: "0x7",
          MULTICHAIN_ACL_ADDRESS: "0x8",
        },
        host: {},
        kmsSigner: "",
        fheKeyId: "",
        crsKeyId: "",
        endpoints: {
          gatewayHttp: "",
          gatewayWs: "",
          hostHttp: "",
          hostWs: "",
          minioInternal: "",
          minioExternal: "",
        },
      },
    });

    expect(renderGatewayAddressesEnv(state)).toContain("GATEWAY_CONFIG_ADDRESS=0x1");
    expect(renderGatewayAddressesEnv(state)).toContain("MULTICHAIN_ACL_ADDRESS=0x8");
    expect(renderGatewayAddressesSolidity(state)).toContain("address constant gatewayConfigAddress = 0x1;");
    expect(renderGatewayAddressesSolidity(state)).toContain("address constant multichainACLAddress = 0x8;");
    expect(renderGatewayAddressesSolidity(state)).toContain("address constant protocolPaymentAddress = 0x6;");
  });

  test("renders host address env and solidity artifacts from discovery", () => {
    const state = stubState({
      discovery: {
        gateway: {},
        host: {
          ACL_CONTRACT_ADDRESS: "0xa",
          FHEVM_EXECUTOR_CONTRACT_ADDRESS: "0xb",
          KMS_VERIFIER_CONTRACT_ADDRESS: "0xc",
          INPUT_VERIFIER_CONTRACT_ADDRESS: "0xd",
          HCU_LIMIT_CONTRACT_ADDRESS: "0xe",
          PAUSER_SET_CONTRACT_ADDRESS: "0xf",
        },
        kmsSigner: "",
        fheKeyId: "",
        crsKeyId: "",
        endpoints: {
          gatewayHttp: "",
          gatewayWs: "",
          hostHttp: "",
          hostWs: "",
          minioInternal: "",
          minioExternal: "",
        },
      },
    });

    expect(renderHostAddressesEnv(state)).toContain("ACL_CONTRACT_ADDRESS=0xa");
    expect(renderHostAddressesSolidity(state)).toContain("address constant aclAdd = 0xa;");
    expect(renderHostAddressesSolidity(state)).toContain("address constant pauserSetAdd = 0xf;");
  });

  test("renders payment bridging solidity constants from rendered gateway env", () => {
    const output = renderPaymentBridgingAddressesSolidity({
      ZAMA_OFT_ADDRESS: "0xabc",
      FEES_SENDER_TO_BURNER_ADDRESS: "0xdef",
    });

    expect(output).toContain("address constant zamaOFTAddress = 0xabc;");
    expect(output).toContain("address constant feesSenderToBurnerAddress = 0xdef;");
  });
});
