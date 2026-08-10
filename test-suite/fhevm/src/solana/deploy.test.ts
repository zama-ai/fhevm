import { describe, expect, test } from "bun:test";

import { generateKeyPairSigner, type Instruction, type TransactionSigner } from "@solana/kit";

import type { GatewayBootstrapInputs } from "./addresses";
import { bootstrapZamaHost, kmsCertificateThresholds } from "./deploy";
import { getDefineKmsContextInstructionDataDecoder } from "./internal/generated/zamaHost/instructions/defineKmsContext";
import { getInitializeHostConfigInstructionDataDecoder } from "./internal/generated/zamaHost/instructions/initializeHostConfig";
import { ZAMA_HOST_PROGRAM_ADDRESS } from "./internal/generated/zamaHost/programAddress.js";
import type { SolanaProvisioningContext } from "./provision";

const address20 = (byte: number): Uint8Array => new Uint8Array(20).fill(byte);

const gateway: GatewayBootstrapInputs = {
  gatewayChainId: 55555n,
  inputVerificationContract: address20(0x0a),
  decryptionContract: address20(0x0b),
  coprocessorSigners: [address20(0x0c)],
  kmsSigners: [address20(0x0d), address20(0x0e), address20(0x0f), address20(0x10)],
};

/** A fake context capturing sent instructions, with a stubbed host-config account read. */
const fakeContext = (hostConfigExists: boolean) => {
  const sent: Instruction[][] = [];
  const context = {
    rpc: {
      getAccountInfo: () => ({
        send: async () => ({ value: hostConfigExists ? { data: ["", "base64"], owner: ZAMA_HOST_PROGRAM_ADDRESS } : null }),
      }),
    },
    async sendTransaction(_payer: TransactionSigner, instructions: readonly Instruction[]) {
      sent.push([...instructions]);
    },
    async airdropSol() {},
  } as unknown as SolanaProvisioningContext;
  return { context, sent };
};

describe("kmsCertificateThresholds", () => {
  test("derives 2t+1 and validates it against the registered signer count", () => {
    expect(kmsCertificateThresholds(0, 1).certificateThreshold).toBe(1);
    expect(kmsCertificateThresholds(1, 4).certificateThreshold).toBe(3);
    expect(() => kmsCertificateThresholds(1, 2)).toThrow("2t+1=3");
  });
});

describe("bootstrapZamaHost", () => {
  test("fresh validator: initializes the host config, then defines KMS context 1", async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = fakeContext(false);
    await bootstrapZamaHost(context, { payer, gateway, kmsCorruptionThreshold: 1 });

    expect(sent).toHaveLength(2);
    const [[initialize], [defineContext]] = sent;
    expect(initialize.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    const initializeData = getInitializeHostConfigInstructionDataDecoder().decode(initialize.data ?? new Uint8Array());
    expect(initializeData.chainId).toBe(9223372036854788153n);
    expect(initializeData.gatewayChainId).toBe(55555n);
    expect(initializeData.coprocessorThreshold).toBe(1);
    expect(initializeData.grantDenyListEnabled).toBe(false);
    expect(Buffer.from(initializeData.inputVerificationContract).toString("hex")).toBe("0a".repeat(20));
    expect(Buffer.from(initializeData.decryptionContract).toString("hex")).toBe("0b".repeat(20));

    expect(defineContext.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    const defineData = getDefineKmsContextInstructionDataDecoder().decode(defineContext.data ?? new Uint8Array());
    expect(defineData.contextId).toBe(1n);
    expect(defineData.signers).toHaveLength(4);
    expect(defineData.thresholds).toEqual({ publicDecryption: 3, userDecryption: 3, kmsGen: 3, mpc: 1 });
  });

  test("configured validator: skips initialize_host_config, still defines the context", async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = fakeContext(true);
    await bootstrapZamaHost(context, { payer, gateway });

    expect(sent).toHaveLength(1);
    const defineData = getDefineKmsContextInstructionDataDecoder().decode(sent[0][0].data ?? new Uint8Array());
    // Centralized default: t=0, so every certificate threshold is 1 and mpc mirrors t.
    expect(defineData.thresholds).toEqual({ publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 0 });
  });

  test("refuses a corruption threshold the registered signer set cannot satisfy", async () => {
    const payer = await generateKeyPairSigner();
    const { context } = fakeContext(true);
    await expect(
      bootstrapZamaHost(context, { payer, gateway: { ...gateway, kmsSigners: [address20(1)] }, kmsCorruptionThreshold: 1 }),
    ).rejects.toThrow("only 1 KMS signers");
  });
});
