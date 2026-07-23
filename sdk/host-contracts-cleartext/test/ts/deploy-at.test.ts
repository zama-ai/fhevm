import { deployAt, type BootstrapConfigV14, type FixedAddressesV14 } from '@fhevm/host-contracts-cleartext/ts';
import { createPublicClient, createWalletClient, http, parseEventLogs, type Address, type Hex } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { foundry } from 'viem/chains';
import { expect, test } from 'vitest';
import { startAnvil, stopAnvil, waitForAnvil } from './anvil.ts';
import { privateKeyFromMnemonic, privateKeyToAddress } from './ethUtils.ts';
import { createViemEthereumAdapters } from './viemEthereumLib.ts';

const MNEMONIC = 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';
const FHE_TYPE_UINT64 = 5;

/**
 * The addresses a Hardhat/Foundry harness actually needs: the ones `ZamaConfig._getLocalConfig()` compiles
 * into a contract under test. They are arbitrary as far as the stack is concerned, and impossible for
 * `deploy` to reach — which is the whole point of `deployAt`.
 */
const FIXED: FixedAddressesV14 = {
  fhevmAddresses: {
    aclAddress: '0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D',
    fhevmExecutorAddress: '0xe3a9105a3a932253A70F126eb1E3b589C643dD24',
    kmsVerifierAddress: '0x901F8942346f7AB3a01F6D7613119Bca447Bb030',
    inputVerifierAddress: '0x36772142b74871f255CbD7A3e89B401d3e45825f',
    hcuLimitAddress: '0x233ff88A48c172d29F675403e6A8e302b0F032D9',
    protocolConfigAddress: '0x44aA028fd264C76BF4A8f8B4d8A5272f6AE25CAc',
    kmsGenerationAddress: '0x216be43148dB537BeddBC268163deb1a802b5553',
  },
  cleartextAddresses: {
    cleartextArithmeticAddress: '0x7071727374757677787980818283848586878889',
    cleartextDbAddress: '0x8081828384858687888990919293949596979899',
  },
  pauserSetAddress: '0xded0D2a71268DC12622BdD1b55d68a1CB5662327',
};

const EXECUTOR_ABI = [
  {
    type: 'function',
    name: 'trivialEncrypt',
    stateMutability: 'nonpayable',
    inputs: [
      { name: 'pt', type: 'uint256' },
      { name: 'toType', type: 'uint8' },
    ],
    outputs: [{ name: 'result', type: 'bytes32' }],
  },
  {
    type: 'function',
    name: 'plaintexts',
    stateMutability: 'view',
    inputs: [{ name: 'handle', type: 'bytes32' }],
    outputs: [{ type: 'uint256' }],
  },
  {
    type: 'event',
    name: 'TrivialEncrypt',
    inputs: [
      { name: 'caller', type: 'address', indexed: true },
      { name: 'pt', type: 'uint256', indexed: false },
      { name: 'toType', type: 'uint8', indexed: false },
      { name: 'result', type: 'bytes32', indexed: false },
    ],
  },
] as const;

const KMS_VERIFIER_ABI = [
  {
    type: 'function',
    name: 'getKmsSigners',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'address[]' }],
  },
] as const;

function bootstrapConfig(adminAddress: string, kmsSigner: string): BootstrapConfigV14 {
  return {
    kmsVerifier: { verifyingContractSource: adminAddress, chainIDSource: 1n },
    inputVerifier: {
      verifyingContractSource: adminAddress,
      chainIDSource: 1n,
      initialSigners: [adminAddress],
      initialThreshold: 1n,
    },
    hcuLimit: { hcuCapPerBlock: 281474976710655n, maxHCUDepthPerTx: 5000000n, maxHCUPerTx: 20000000n },
    protocolConfig: {
      initialKmsNodeParams: [
        {
          txSenderAddress: adminAddress,
          signerAddress: kmsSigner,
          ipAddress: '127.0.0.1',
          storageUrl: 'https://kms.example',
          partyId: 1,
          mpcIdentity: 'kms-1',
          caCert: '0x',
          storagePrefix: '',
        },
      ],
      initialThresholds: { publicDecryption: 1n, userDecryption: 1n, kmsGen: 1n, mpc: 1n },
      softwareVersion: '0.0.0-test',
      pcrValues: [],
    },
  };
}

test('deployAt places the stack at fixed addresses and the cleartext layer round-trips', async () => {
  const adminKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });
  const adminAddress = privateKeyToAddress({ privateKey: adminKey });
  const kmsSigner = privateKeyToAddress({
    privateKey: privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 8 }),
  });

  const anvil = startAnvil({ port: 8611, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: adminKey });
    const publicClient = createPublicClient({ chain: foundry, transport: http(anvil.rpcUrl) });
    const wallet = createWalletClient({
      account: privateKeyToAccount(adminKey),
      chain: foundry,
      transport: http(anvil.rpcUrl),
    });

    await deployAt({
      ethProvider: adapters.provider,
      admin: adapters.signer,
      addresses: FIXED,
      config: bootstrapConfig(adminAddress, kmsSigner),
    });

    // (a) Every contract landed at the address we ASKED for — not one derived from a nonce.
    for (const address of [
      ...Object.values(FIXED.fhevmAddresses),
      ...Object.values(FIXED.cleartextAddresses),
      FIXED.pauserSetAddress,
    ]) {
      const code = await publicClient.getCode({ address: address as Address });
      expect(code, `code at ${address}`).toBeDefined();
      expect((code ?? '0x').length, `code at ${address}`).toBeGreaterThan(2);
    }

    // (b) The initializers really ran: the KMS signer set is readable through ProtocolConfig.
    const signers = await publicClient.readContract({
      address: FIXED.fhevmAddresses.kmsVerifierAddress as Address,
      abi: KMS_VERIFIER_ABI,
      functionName: 'getKmsSigners',
    });
    expect(signers.map((s) => s.toLowerCase())).toContain(kmsSigner.toLowerCase());

    // (c) Functional round-trip through the patched cross-references: the executor must reach
    //     CleartextArithmetic, which must be an authorized writer on CleartextDB. If any address had been
    //     patched wrongly, this reverts or records nothing.
    const executor = FIXED.fhevmAddresses.fhevmExecutorAddress as Address;
    const hash = await wallet.writeContract({
      address: executor,
      abi: EXECUTOR_ABI,
      functionName: 'trivialEncrypt',
      args: [42n, FHE_TYPE_UINT64],
    });
    const receipt = await publicClient.waitForTransactionReceipt({ hash });

    const events = parseEventLogs({ abi: EXECUTOR_ABI, eventName: 'TrivialEncrypt', logs: receipt.logs });
    const handle: Hex | undefined = events[0]?.args.result;
    expect(handle).toBeDefined();

    if (handle === undefined) {
      throw new Error('TrivialEncrypt event not found in receipt');
    }

    const stored = await publicClient.readContract({
      address: executor,
      abi: EXECUTOR_ABI,
      functionName: 'plaintexts',
      args: [handle],
    });
    expect(stored).toBe(42n);
  } finally {
    await stopAnvil(anvil.process);
  }
}, 120_000);
