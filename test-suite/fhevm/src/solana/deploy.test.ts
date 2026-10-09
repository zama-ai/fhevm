import { type Address, type Instruction, type TransactionSigner, generateKeyPairSigner } from '@solana/kit';
import { describe, expect, test } from 'bun:test';
import { readFile } from 'node:fs/promises';
import path from 'node:path';

import { hexToBytes } from '@fhevm/sdk/base';

import { BRINGUP_KMS_CONTEXT_ID, BRINGUP_KMS_EPOCH_ID, type GatewayBootstrapInputs, bytes32HexFromId } from './addresses';
import {
  assertActiveKmsPairMatches,
  assertKmsSignersMatch,
  assertKmsThresholdsMatch,
  bootstrapThresholdsForState,
  bootstrapZamaHost,
  kmsCertificateThreshold,
  lifecycleComposeProject,
} from './deploy';
import {
  findEventAuthorityPda,
  findHostConfigPda,
  findKmsContextPda,
  findRandNoncePda,
  getDefineKmsContextInstructionDataDecoder,
  getHostConfigEncoder,
  getInitializeHostConfigInstructionDataDecoder,
  getKmsContextEncoder,
  getSetMaxHcuDepthPerTxInstructionDataDecoder,
  getSetMaxHcuPerTxInstructionDataDecoder,
  type HostConfigArgs,
  SET_MAX_HCU_DEPTH_PER_TX_DISCRIMINATOR,
  SET_MAX_HCU_PER_TX_DISCRIMINATOR,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from '@fhevm/solana-zama-host';
import { zamaHostProgramDataAddress } from './provision';
import { loadCoprocessorScenario, resolveScenarioFile } from '../scenario/resolve';
import { HCU_LIMITS } from '../../../../solana/deploy/src/constants';
import type { HostDeployContext } from '../../../../solana/deploy/src/send';
import { REPO_ROOT } from '../layout';
import { solanaHostChainId } from '../../../../sdk/js-sdk/src/core/chains/hostChainId';

const address20 = (byte: number): Uint8Array => new Uint8Array(20).fill(byte);
// The program's unlimited sentinel, `u64::MAX`.
const unlimited = 2n ** 64n - 1n;

const kmsCorruptionThreshold = 1;

// Not the localnet id, so a test passes only if bootstrap writes the id it is given.
const chainId = solanaHostChainId(777n);

const gateway: GatewayBootstrapInputs = {
  gatewayChainId: 55555n,
  inputVerificationContract: address20(0x0a),
  decryptionContract: address20(0x0b),
  coprocessorSigners: [address20(0x0c)],
  kmsSigners: [address20(0x0d), address20(0x0e), address20(0x0f), address20(0x10)],
};

/** A fake context capturing sent instructions, with stubbed host-config / kms-context reads. */
const fakeContext = async (
  hostConfigExists: boolean,
  payer: Address,
  kmsContextExists = false,
  hcuLimits: Pick<HostConfigArgs, 'maxHcuPerTx' | 'maxHcuDepthPerTx' | 'hcuBlockCapPerApp'> = {
    ...HCU_LIMITS,
    hcuBlockCapPerApp: unlimited,
  },
) => {
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
                            chainId,
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
                            currentKmsEpochId: BRINGUP_KMS_EPOCH_ID,
                            paused: { execution: false, verifiedInputs: false, aclWrites: false },
                            grantDenyListEnabled: false,
                            ...hcuLimits,
                            bump: 0,
                          }),
                        ).toString('base64')
                      : Buffer.from(
                          getKmsContextEncoder().encode({
                            contextId: BRINGUP_KMS_CONTEXT_ID,
                            signers: [...gateway.kmsSigners],
                            thresholds: { publicDecryption: 3, userDecryption: 3, kmsGen: 3, mpc: 1 },
                            destroyed: false,
                            bump: 0,
                          }),
                        ).toString('base64'),
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
    expect(() => kmsCertificateThreshold(1, 5)).toThrow('the gateway has 5 registered');
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

  test("zama-host's active pair must be the EVM host's: mirroring the aborted epoch fails", () => {
    const context = 0x07n << 248n;
    const epoch = 0x08n << 248n;
    const id = (value: bigint) => hexToBytes(bytes32HexFromId(value));
    // Step 4b: the EVM host's active pair is the recovery epoch …254; the aborted epoch is …253.
    const evm = { kmsContextId: context + 3n, kmsEpochId: epoch + 254n };
    const recovery = { currentKmsContextId: id(context + 3n), currentKmsEpochId: id(epoch + 254n) };
    const aborted = { currentKmsContextId: id(context + 3n), currentKmsEpochId: id(epoch + 253n) };
    expect(() => assertActiveKmsPairMatches(recovery, evm)).not.toThrow();
    expect(() => assertActiveKmsPairMatches(aborted, evm)).toThrow(
      "differs from the EVM ProtocolConfig's",
    );
  });

  test('a Solana KMS context with the EVM signers in another order fails', () => {
    const evm = [1, 2, 3, 4].map((party) => new Uint8Array(20).fill(party));
    expect(() => assertKmsSignersMatch([...evm], evm)).not.toThrow();
    expect(() => assertKmsSignersMatch([evm[1]!, evm[0]!, evm[2]!, evm[3]!], evm)).toThrow(
      'Solana KMS context signers differ from the EVM ProtocolConfig',
    );
  });
});

describe('bootstrapZamaHost', () => {
  test('fresh validator: initializes the host config with the HCU limits, then defines KMS context 1', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(false, payer.address);
    await bootstrapZamaHost(context, { payer, chainId, gateway, kmsCorruptionThreshold });

    expect(sent).toHaveLength(2);
    expect(sent[0]).toHaveLength(3);
    const [[initialize, setDepth, setTotal], [defineContext]] = sent;
    expect(Buffer.from(setDepth.data!.subarray(0, 8))).toEqual(Buffer.from(SET_MAX_HCU_DEPTH_PER_TX_DISCRIMINATOR));
    expect(Buffer.from(setTotal.data!.subarray(0, 8))).toEqual(Buffer.from(SET_MAX_HCU_PER_TX_DISCRIMINATOR));
    expect(getSetMaxHcuDepthPerTxInstructionDataDecoder().decode(setDepth.data!).value).toBe(
      HCU_LIMITS.maxHcuDepthPerTx,
    );
    expect(getSetMaxHcuPerTxInstructionDataDecoder().decode(setTotal.data!).value).toBe(HCU_LIMITS.maxHcuPerTx);
    expect(initialize.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    const programData = await zamaHostProgramDataAddress();
    expect(initialize.accounts?.some((account) => account.address === programData)).toBe(true);
    const initializeData = getInitializeHostConfigInstructionDataDecoder().decode(initialize.data ?? new Uint8Array());
    expect(initializeData.chainId).toBe(chainId);
    expect(initializeData.gatewayChainId).toBe(55555n);
    expect(initializeData.coprocessorThreshold).toBe(1);
    expect(initializeData.grantDenyListEnabled).toBe(false);
    expect(Buffer.from(initializeData.inputVerificationContract).toString('hex')).toBe('0a'.repeat(20));
    expect(Buffer.from(initializeData.decryptionContract).toString('hex')).toBe('0b'.repeat(20));

    expect(defineContext.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    const defineData = getDefineKmsContextInstructionDataDecoder().decode(defineContext.data ?? new Uint8Array());
    expect(Buffer.from(defineData.contextId)).toEqual(Buffer.from(BRINGUP_KMS_CONTEXT_ID));
    expect(Buffer.from(defineData.epochId)).toEqual(Buffer.from(BRINGUP_KMS_EPOCH_ID));
    expect(defineData.signers).toHaveLength(4);
    expect(defineData.thresholds).toEqual({ publicDecryption: 3, userDecryption: 3, kmsGen: 3, mpc: 1 });
  });

  test('bootstrap targets the given host and its randomness account, not the compiled default', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(false, payer.address);
    const programAddress = (await generateKeyPairSigner()).address;
    await bootstrapZamaHost(context, { payer, chainId, gateway, kmsCorruptionThreshold, programAddress });
    const [givenNonce] = await findRandNoncePda({ programAddress });
    const [defaultNonce] = await findRandNoncePda();
    const accounts = sent[0][0].accounts!.map((account) => account.address);
    expect(accounts).toContain(givenNonce);
    expect(accounts).not.toContain(defaultNonce);
    const [hostConfig] = await findHostConfigPda({ programAddress });
    const [eventAuthority] = await findEventAuthorityPda({ programAddress });
    for (const instruction of sent[0]) {
      expect(instruction.programAddress).toBe(programAddress);
      const addresses = instruction.accounts!.map((account) => account.address);
      expect(addresses).toContain(hostConfig);
      expect(addresses).toContain(eventAuthority);
    }
  });

  test('configured validator: skips initialize_host_config, still defines the context', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(true, payer.address);
    await bootstrapZamaHost(context, { payer, chainId, gateway, kmsCorruptionThreshold });

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
        chainId,
        gateway: { ...gateway, kmsSigners: [address20(1)] },
        kmsCorruptionThreshold: 1,
      }),
    ).rejects.toThrow('the gateway has 1 registered');
  });

  test('refuses a different chain or gateway without submitting transactions', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(true, payer.address);
    for (const mismatch of [
      { chainId: solanaHostChainId(778n), gateway },
      { chainId, gateway: { ...gateway, gatewayChainId: 999n } },
    ]) {
      await expect(bootstrapZamaHost(context, { payer, ...mismatch, kmsCorruptionThreshold })).rejects.toThrow(
        'does not match',
      );
    }
    expect(sent).toHaveLength(0);
  });

  test('accepts a host whose admin tuned its HCU limits, and leaves them as they are', async () => {
    const payer = await generateKeyPairSigner();
    for (const hcuLimits of [
      { maxHcuPerTx: 30_000_000n, maxHcuDepthPerTx: 6_000_000n, hcuBlockCapPerApp: 40_000_000n },
      { maxHcuPerTx: unlimited, maxHcuDepthPerTx: 5_000_000n, hcuBlockCapPerApp: unlimited },
    ]) {
      const { context, sent } = await fakeContext(true, payer.address, true, hcuLimits);
      await bootstrapZamaHost(context, { payer, chainId, gateway, kmsCorruptionThreshold });
      expect(sent).toHaveLength(0);
    }
  });

  test('refuses a host whose HCU limits are still unlimited without submitting transactions', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(true, payer.address, true, {
      maxHcuPerTx: unlimited,
      maxHcuDepthPerTx: unlimited,
      hcuBlockCapPerApp: unlimited,
    });
    await expect(bootstrapZamaHost(context, { payer, chainId, gateway, kmsCorruptionThreshold })).rejects.toThrow(
      'still has unlimited HCU limits (never bootstrapped)',
    );
    expect(sent).toHaveLength(0);
  });

  test('already bootstrapped: skips both initialize_host_config and define_kms_context', async () => {
    const payer = await generateKeyPairSigner();
    const { context, sent } = await fakeContext(true, payer.address, true);
    await bootstrapZamaHost(context, { payer, chainId, gateway, kmsCorruptionThreshold });
    expect(sent).toHaveLength(0);
  });
});

test('first-deploy preflight rejects malformed inputs without sending initialization', async () => {
  const payer = await generateKeyPairSigner();
  const { context, sent } = await fakeContext(false, payer.address);
  for (const invalid of [
    { chainId: 12345n },
    { chainId: solanaHostChainId(12345n) | (1n << 64n) },
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
      bootstrapZamaHost(context, { payer, chainId, gateway, kmsCorruptionThreshold, validateOnly: true, ...invalid }),
    ).rejects.toThrow();
  }
  expect(sent).toHaveLength(0);
});

test('HCU limits match the values the EVM deployment initializes HCULimit with', async () => {
  const tasks = await readFile(path.join(REPO_ROOT, 'host-contracts/tasks/taskDeploy.ts'), 'utf8');
  const task = tasks.slice(tasks.indexOf("task('task:deployHCULimit')"));
  const args = task.match(/fn: 'initializeFromEmptyProxy', args: \[([^\]]*)\]/)?.[1];
  // HCULimit.initializeFromEmptyProxy(hcuCapPerBlock, maxHCUDepthPerTx, maxHCUPerTx).
  const [, depth, total] = [...(args ?? '').matchAll(/BigInt\('(\d+)'\)/g)].map((match) => BigInt(match[1]!));
  expect({ maxHcuDepthPerTx: depth, maxHcuPerTx: total }).toEqual(HCU_LIMITS);
});
