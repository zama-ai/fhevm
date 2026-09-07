import { type Address, type Instruction, type TransactionSigner, generateKeyPairSigner } from '@solana/kit';
import { describe, expect, test } from 'bun:test';
import { createHash } from 'node:crypto';

import { BRINGUP_KMS_CONTEXT_ID, type GatewayBootstrapInputs } from './addresses';
import { bootstrapZamaHost, kmsCertificateThreshold, lifecycleComposeProject } from './deploy';
import { PREVIEW_ENV_ZAMA_HOST_PROGRAM_ADDRESS } from './host-deploy/program-profile';
import { getHostConfigEncoder } from './internal/generated/zamaHost/accounts/hostConfig';
import {
  getDefineKmsContextInstructionDataDecoder,
  getDefineKmsContextInstructionDataEncoder,
} from './internal/generated/zamaHost/instructions/defineKmsContext';
import { getInitializeHostConfigInstructionDataDecoder } from './internal/generated/zamaHost/instructions/initializeHostConfig';
import { findHostConfigPda, findKmsContextPda, findRandNoncePda } from './internal/generated/zamaHost/pdas/index.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from './internal/generated/zamaHost/programAddress.js';
import { zamaHostProgramDataAddress } from './provision';
import type { SolanaProvisioningContext } from './provision';

const address20 = (byte: number): Uint8Array => new Uint8Array(20).fill(byte);

const gateway: GatewayBootstrapInputs = {
  gatewayChainId: 55555n,
  inputVerificationContract: address20(0x0a),
  decryptionContract: address20(0x0b),
  coprocessorSigners: [address20(0x0c)],
  kmsSigners: [address20(0x0d), address20(0x0e), address20(0x0f), address20(0x10)],
};

/** A fake context capturing sent instructions, with stubbed host-config / kms-context reads. */
const fakeContext = async (hostConfigExists: boolean, payer: Address, kmsContextExists = false) => {
  const [hostConfig] = await findHostConfigPda();
  const [kmsContext] = await findKmsContextPda({ contextId: BRINGUP_KMS_CONTEXT_ID });
  const sent: Instruction[][] = [];
  const context = {
    rpc: {
      getAccountInfo: (address: string) => ({
        send: async () => {
          const key = String(address);
          const exists = key === hostConfig ? hostConfigExists : key === kmsContext ? kmsContextExists : false;
          return {
            value: exists
              ? {
                  data: [
                    key === hostConfig
                      ? Buffer.from(
                          getHostConfigEncoder().encode({
                            admin: payer,
                            chainId: 9223372036854788153n,
                            gatewayChainId: gateway.gatewayChainId,
                            inputVerificationContract: gateway.inputVerificationContract,
                            decryptionContract: gateway.decryptionContract,
                            coprocessorSigners: [
                              ...gateway.coprocessorSigners,
                              ...Array.from({ length: 7 }, () => address20(0)),
                            ],
                            coprocessorSignerCount: 1,
                            coprocessorThreshold: 1,
                            currentKmsContextId: BRINGUP_KMS_CONTEXT_ID,
                            paused: false,
                            grantDenyListEnabled: false,
                            maxHcuPerTx: 1n,
                            maxHcuDepthPerTx: 1n,
                            hcuBlockCapPerApp: 1n,
                            updatedSlot: 0n,
                            bump: 0,
                          }),
                        ).toString('base64')
                      : Buffer.concat([
                          createHash('sha256').update('account:KmsContext').digest().subarray(0, 8),
                          Buffer.from(
                            getDefineKmsContextInstructionDataEncoder().encode({
                              contextId: BRINGUP_KMS_CONTEXT_ID,
                              signers: [...gateway.kmsSigners],
                              thresholds: { publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 0 },
                            }),
                          ).subarray(8),
                          Buffer.from([0, 0]),
                        ]).toString('base64'),
                    'base64',
                  ] as const,
                  owner: ZAMA_HOST_PROGRAM_ADDRESS,
                  executable: false,
                  lamports: 1n,
                  space: 0n,
                }
              : null,
          };
        },
      }),
    },
    async sendTransaction(_payer: TransactionSigner, instructions: readonly Instruction[]) {
      sent.push([...instructions]);
    },
    async airdropSol() {},
  } as unknown as SolanaProvisioningContext;
  return { context, sent };
};

describe('lifecycleComposeProject', () => {
  const originalProject = process.env.FHEVM_COMPOSE_PROJECT;
  const restore = () => {
    if (originalProject === undefined) delete process.env.FHEVM_COMPOSE_PROJECT;
    else process.env.FHEVM_COMPOSE_PROJECT = originalProject;
  };

  test('standalone mode uses the default project without consulting the env', () => {
    process.env.FHEVM_COMPOSE_PROJECT = 'not-a-valid-project';
    try {
      expect(lifecycleComposeProject(undefined)).toBe('fhevm');
    } finally {
      restore();
    }
  });

  test('lifecycle mode requires the per-boot project shape', () => {
    try {
      process.env.FHEVM_COMPOSE_PROJECT = 'fhevm-demo-12345678-1234-4123-8123-123456789abc';
      expect(lifecycleComposeProject('/tmp/fhevm-demo-1/x')).toBe('fhevm-demo-12345678-1234-4123-8123-123456789abc');
      process.env.FHEVM_COMPOSE_PROJECT = 'fhevm';
      expect(() => lifecycleComposeProject('/tmp/fhevm-demo-1/x')).toThrow('invalid lifecycle Compose project');
      delete process.env.FHEVM_COMPOSE_PROJECT;
      expect(() => lifecycleComposeProject('/tmp/fhevm-demo-1/x')).toThrow('invalid lifecycle Compose project');
    } finally {
      restore();
    }
  });
});

describe('kmsCertificateThreshold', () => {
  test('derives 2t+1 and validates it against the registered signer count', () => {
    expect(kmsCertificateThreshold(0, 1)).toBe(1);
    expect(kmsCertificateThreshold(1, 4)).toBe(3);
    expect(() => kmsCertificateThreshold(1, 2)).toThrow('2t+1=3');
  });
});

describe('bootstrapZamaHost', () => {
  test('fresh validator: initializes the host config, then defines KMS context 1', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(false, payer.address);
    await bootstrapZamaHost(context, { payer, gateway, kmsCorruptionThreshold: 1 });

    expect(sent).toHaveLength(2);
    const [[initialize], [defineContext]] = sent;
    expect(initialize.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    const programData = await zamaHostProgramDataAddress();
    expect(initialize.accounts?.some((account) => account.address === programData)).toBe(true);
    const initializeData = getInitializeHostConfigInstructionDataDecoder().decode(initialize.data ?? new Uint8Array());
    expect(initializeData.chainId).toBe(9223372036854788153n);
    expect(initializeData.gatewayChainId).toBe(55555n);
    expect(initializeData.coprocessorThreshold).toBe(1);
    expect(initializeData.grantDenyListEnabled).toBe(false);
    expect(Buffer.from(initializeData.inputVerificationContract).toString('hex')).toBe('0a'.repeat(20));
    expect(Buffer.from(initializeData.decryptionContract).toString('hex')).toBe('0b'.repeat(20));

    expect(defineContext.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    const defineData = getDefineKmsContextInstructionDataDecoder().decode(defineContext.data ?? new Uint8Array());
    expect(Buffer.from(defineData.contextId)).toEqual(Buffer.from(BRINGUP_KMS_CONTEXT_ID));
    expect(Buffer.from(defineData.contextId).toString('hex')).toBe(
      '0700000000000000000000000000000000000000000000000000000000000001',
    );
    expect(defineData.signers).toHaveLength(4);
    expect(defineData.thresholds).toEqual({ publicDecryption: 3, userDecryption: 3, kmsGen: 3, mpc: 1 });
  });

  test('preview bootstrap derives the randomness account under the preview host', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(false, payer.address);
    const programAddress = PREVIEW_ENV_ZAMA_HOST_PROGRAM_ADDRESS;
    await bootstrapZamaHost(context, { payer, gateway, programAddress });
    const [previewNonce] = await findRandNoncePda({ programAddress });
    const [localNonce] = await findRandNoncePda();
    const accounts = sent[0][0].accounts!.map((account) => account.address);
    expect(accounts).toContain(previewNonce);
    expect(accounts).not.toContain(localNonce);
  });

  test('configured validator: skips initialize_host_config, still defines the context', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(true, payer.address);
    await bootstrapZamaHost(context, { payer, gateway });

    expect(sent).toHaveLength(1);
    const defineData = getDefineKmsContextInstructionDataDecoder().decode(sent[0][0].data ?? new Uint8Array());
    // Centralized default: t=0, so every certificate threshold is 1 and mpc mirrors t.
    expect(defineData.thresholds).toEqual({ publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 0 });
  });

  test('refuses a corruption threshold the registered signer set cannot satisfy', async () => {
    const payer = await generateKeyPairSigner();
    const { context } = await fakeContext(true, payer.address);
    await expect(
      bootstrapZamaHost(context, {
        payer,
        gateway: { ...gateway, kmsSigners: [address20(1)] },
        kmsCorruptionThreshold: 1,
      }),
    ).rejects.toThrow('only 1 KMS signers');
  });

  test('refuses a different gateway without submitting transactions', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(true, payer.address);
    await expect(bootstrapZamaHost(context, { payer, gateway: { ...gateway, gatewayChainId: 999n } })).rejects.toThrow(
      'does not match',
    );
    expect(sent).toHaveLength(0);
  });

  test('already bootstrapped: skips both initialize_host_config and define_kms_context', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(true, payer.address, true);
    await bootstrapZamaHost(context, { payer, gateway });
    expect(sent).toHaveLength(0);
  });
});
