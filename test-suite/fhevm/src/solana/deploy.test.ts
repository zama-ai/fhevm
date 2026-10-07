import { type Address, type Instruction, type TransactionSigner, generateKeyPairSigner } from '@solana/kit';
import { describe, expect, test } from 'bun:test';

import { BRINGUP_KMS_CONTEXT_ID, type GatewayBootstrapInputs } from './addresses';
import {
  assertKmsThresholdsMatch,
  bootstrapThresholdsForState,
  bootstrapZamaHost,
  kmsCertificateThreshold,
  lifecycleComposeProject,
} from './deploy';
import {
  findHostConfigPda,
  findKmsContextPda,
  findRandNoncePda,
  getDefineKmsContextInstructionDataDecoder,
  getDefineKmsContextInstructionDataEncoder,
  getHostConfigEncoder,
  getInitializeHostConfigInstructionDataDecoder,
  KMS_CONTEXT_DISCRIMINATOR,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from '@fhevm/solana-zama-host';
import { zamaHostProgramDataAddress } from './provision';
import { loadCoprocessorScenario, resolveScenarioFile } from '../scenario/resolve';
import type { HostDeployContext } from '../../../../solana/deploy/src/send';

const address20 = (byte: number): Uint8Array => new Uint8Array(20).fill(byte);

const kmsCorruptionThreshold = 1;

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
                            chainId: 72057594037940281n,
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
                            paused: { execution: false, verifiedInputs: false, aclWrites: false },
                            grantDenyListEnabled: false,
                            maxHcuPerTx: 1n,
                            maxHcuDepthPerTx: 1n,
                            hcuBlockCapPerApp: 1n,
                            bump: 0,
                          }),
                        ).toString('base64')
                      : Buffer.concat([
                          Buffer.from(KMS_CONTEXT_DISCRIMINATOR),
                          Buffer.from(
                            getDefineKmsContextInstructionDataEncoder().encode({
                              contextId: BRINGUP_KMS_CONTEXT_ID,
                              signers: [...gateway.kmsSigners],
                              thresholds: { publicDecryption: 3, userDecryption: 3, kmsGen: 3, mpc: 1 },
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
    async fundSol() {},
  } as unknown as HostDeployContext;
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
  test('derives 2t+1 and requires a 3t+1 signer committee', () => {
    expect(kmsCertificateThreshold(0, 1)).toBe(1);
    expect(kmsCertificateThreshold(1, 4)).toBe(3);
    expect(kmsCertificateThreshold(4, 13)).toBe(9);
    expect(() => kmsCertificateThreshold(1, 2)).toThrow('3t+1=4');
    // t=0 against a 4-party gateway: the 1-of-4 context the threshold check exists to refuse.
    expect(() => kmsCertificateThreshold(0, 4)).toThrow('3t+1=1');
    expect(() => kmsCertificateThreshold(1, 5)).toThrow('but 5 are registered');
  });
});

describe('host bootstrap thresholds', () => {
  test('come from the scenario the EVM stack was rendered from', async () => {
    const scenario = resolveScenarioFile('/tmp/solana.yaml', await loadCoprocessorScenario('solana'));
    // `solana` omits `kms`, so it runs the default 4-party cluster with t=1.
    expect(bootstrapThresholdsForState({ scenario })).toEqual({ coprocessorThreshold: 1, kmsCorruptionThreshold: 1 });
  });

  test('a Solana KMS context that differs from the EVM one fails', () => {
    const evm = { publicDecryption: 3, userDecryption: 3, kmsGen: 3, mpc: 1 };
    expect(() => assertKmsThresholdsMatch(evm, evm)).not.toThrow();
    expect(() => assertKmsThresholdsMatch({ ...evm, userDecryption: 1 }, evm)).toThrow('userDecryption: solana=1 evm=3');
  });
});

describe('bootstrapZamaHost', () => {
  test('fresh validator: initializes the host config, then defines KMS context 1', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(false, payer.address);
    await bootstrapZamaHost(context, { payer, gateway, kmsCorruptionThreshold });

    expect(sent).toHaveLength(2);
    const [[initialize], [defineContext]] = sent;
    expect(initialize.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    const programData = await zamaHostProgramDataAddress();
    expect(initialize.accounts?.some((account) => account.address === programData)).toBe(true);
    const initializeData = getInitializeHostConfigInstructionDataDecoder().decode(initialize.data ?? new Uint8Array());
    expect(initializeData.chainId).toBe(72057594037940281n);
    expect(initializeData.gatewayChainId).toBe(55555n);
    expect(initializeData.coprocessorThreshold).toBe(1);
    expect(initializeData.grantDenyListEnabled).toBe(false);
    expect(Buffer.from(initializeData.inputVerificationContract).toString('hex')).toBe('0a'.repeat(20));
    expect(Buffer.from(initializeData.decryptionContract).toString('hex')).toBe('0b'.repeat(20));

    expect(defineContext.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    const defineData = getDefineKmsContextInstructionDataDecoder().decode(defineContext.data ?? new Uint8Array());
    expect(Buffer.from(defineData.contextId)).toEqual(Buffer.from(BRINGUP_KMS_CONTEXT_ID));
    expect(defineData.signers).toHaveLength(4);
    expect(defineData.thresholds).toEqual({ publicDecryption: 3, userDecryption: 3, kmsGen: 3, mpc: 1 });
  });

  test('bootstrap derives the randomness account under the given host, not the compiled default', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(false, payer.address);
    const programAddress = (await generateKeyPairSigner()).address;
    await bootstrapZamaHost(context, { payer, gateway, kmsCorruptionThreshold, programAddress });
    const [givenNonce] = await findRandNoncePda({ programAddress });
    const [defaultNonce] = await findRandNoncePda();
    const accounts = sent[0][0].accounts!.map((account) => account.address);
    expect(accounts).toContain(givenNonce);
    expect(accounts).not.toContain(defaultNonce);
  });

  test('configured validator: skips initialize_host_config, still defines the context', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(true, payer.address);
    await bootstrapZamaHost(context, { payer, gateway, kmsCorruptionThreshold });

    expect(sent).toHaveLength(1);
    const defineData = getDefineKmsContextInstructionDataDecoder().decode(sent[0][0].data ?? new Uint8Array());
    expect(defineData.thresholds).toEqual({ publicDecryption: 3, userDecryption: 3, kmsGen: 3, mpc: 1 });
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
    ).rejects.toThrow('but 1 are registered');
  });

  test('refuses a different gateway without submitting transactions', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(true, payer.address);
    await expect(
      bootstrapZamaHost(context, { payer, gateway: { ...gateway, gatewayChainId: 999n }, kmsCorruptionThreshold }),
    ).rejects.toThrow(
      'does not match',
    );
    expect(sent).toHaveLength(0);
  });

  test('already bootstrapped: skips both initialize_host_config and define_kms_context', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(true, payer.address, true);
    await bootstrapZamaHost(context, { payer, gateway, kmsCorruptionThreshold });
    expect(sent).toHaveLength(0);
  });
});

test('first-deploy preflight rejects malformed inputs without sending initialization', async () => {
  const payer = await generateKeyPairSigner();
  const { context, sent } = await fakeContext(false, payer.address);
  for (const invalid of [
    { coprocessorThreshold: 0 },
    { coprocessorThreshold: 2 },
    { coprocessorThreshold: 256 },
    { kmsCorruptionThreshold: -1 },
    { kmsCorruptionThreshold: 1.5 },
    { gateway: { ...gateway, gatewayChainId: 1n << 63n } },
    { gateway: { ...gateway, inputVerificationContract: address20(0) } },
    { gateway: { ...gateway, decryptionContract: address20(0) } },
    { gateway: { ...gateway, kmsSigners: [address20(0)] } },
    { gateway: { ...gateway, coprocessorSigners: [address20(1), address20(1)] } },
  ]) {
    await expect(
      bootstrapZamaHost(context, { payer, gateway, kmsCorruptionThreshold, validateOnly: true, ...invalid }),
    ).rejects.toThrow();
  }
  expect(sent).toHaveLength(0);
});
