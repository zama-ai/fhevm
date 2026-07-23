import { deployAt, type BootstrapConfigV14, type FixedAddressesV14 } from '@fhevm/host-contracts-cleartext/ts';
import { createPublicClient, createTestClient, createWalletClient, http, type Address, type Hex } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';

/**
 * Stands the cleartext stack up on an ALREADY RUNNING dev node (anvil or `hardhat node`), at the
 * addresses `ZamaConfig._getLocalConfig()` pins for chainid 31337.
 *
 *   anvil                      # or: npx hardhat node
 *   npm run deploy:local       # this script; RPC_URL overrides http://127.0.0.1:8545
 *
 * This is the package-owned counterpart of what the hardhat plugin does automatically on first use, for
 * consumers that cannot do it themselves — e.g. `forge script --broadcast` against a local node (a forge
 * script's cheatcodes only affect its simulation, never the node), or any workflow that wants a persistent
 * FHEVM-ready chain before tooling connects.
 *
 * The stack is initialized with the STANDARD MOCK IDENTITY — the same signer keys and gateway identity that
 * `@fhevm/hardhat-plugin` (`src/engine/stack/config.ts`) and `forge-fhevm` compile in. That is what lets the
 * plugin ADOPT a chain this script prepared instead of redeploying: its readiness probe checks that
 * `KMSVerifier.getKmsSigners()` contains the mock KMS signer below. Change these values in lockstep or not
 * at all.
 */

const RPC_URL = process.env.RPC_URL ?? 'http://127.0.0.1:8545';

/** anvil's and `hardhat node`'s account #0 ("test test ... junk"); funded on both. Override for custom nodes. */
const ADMIN_PRIVATE_KEY = (process.env.ADMIN_PRIVATE_KEY ??
  '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80') as Hex;

// The shared mock identity (see the note above).
const KMS_SIGNER_PRIVATE_KEY: Hex = '0x388b7680e4e1afa06efbfd45cdd1fe39f3c6af381df6555a19661f283b97de91';
const COPROCESSOR_SIGNER_PRIVATE_KEY: Hex = '0x7ec8ada6642fc4ccfb7729bc29c17cf8d21b61abd5642d1db992c0b8672ab901';
const GATEWAY_CHAIN_ID = 10901n;
const GATEWAY_DECRYPTION_ADDRESS = '0x5ffdaAB0373E62E2ea2944776209aEf29E631A64';
const GATEWAY_INPUT_VERIFICATION_ADDRESS = '0x812b06e1CDCE800494b79fFE4f925A504a9A9810';
const KMS_TX_SENDER = '0x0000000000000000000000000000000000C0FFEE';

/**
 * The first three are pinned by `ZamaConfig._getLocalConfig()` (chainid 31337) — compiled into every
 * contract under test. The rest are the free choices the sibling consumers use, kept identical.
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

const KMS_VERIFIER_ABI = [
  { type: 'function', name: 'getKmsSigners', stateMutability: 'view', inputs: [], outputs: [{ type: 'address[]' }] },
] as const;

function bootstrapConfig(kmsSigner: string, coprocessorSigner: string): BootstrapConfigV14 {
  return {
    kmsVerifier: { verifyingContractSource: GATEWAY_DECRYPTION_ADDRESS, chainIDSource: GATEWAY_CHAIN_ID },
    inputVerifier: {
      verifyingContractSource: GATEWAY_INPUT_VERIFICATION_ADDRESS,
      chainIDSource: GATEWAY_CHAIN_ID,
      initialSigners: [coprocessorSigner],
      initialThreshold: 1n,
    },
    // The block cap is maxed out (2^48 - 1): throttling long dev sessions is a production concern.
    hcuLimit: { hcuCapPerBlock: 281474976710655n, maxHCUDepthPerTx: 5000000n, maxHCUPerTx: 20000000n },
    protocolConfig: {
      initialKmsNodeParams: [
        {
          txSenderAddress: KMS_TX_SENDER,
          signerAddress: kmsSigner,
          ipAddress: '127.0.0.1',
          storageUrl: 'https://kms.local',
          partyId: 1,
          mpcIdentity: 'kms-1',
          caCert: '0x',
          storagePrefix: '',
        },
      ],
      initialThresholds: { publicDecryption: 1n, userDecryption: 1n, kmsGen: 1n, mpc: 1n },
      softwareVersion: '0.0.0-mock',
      pcrValues: [],
    },
  };
}

/**
 * anvil first: it aliases much of the `hardhat_*` namespace for compatibility, so probing hardhat first
 * would misidentify it (and `anvil_*` is what viem's test client sends in 'anvil' mode).
 */
async function detectNodeMode(request: (args: { method: string }) => Promise<unknown>): Promise<'anvil' | 'hardhat'> {
  try {
    await request({ method: 'anvil_nodeInfo' });
    return 'anvil';
  } catch {
    /* not anvil */
  }
  try {
    await request({ method: 'hardhat_metadata' });
    return 'hardhat';
  } catch {
    /* not hardhat */
  }
  throw new Error(
    `The node at ${RPC_URL} answers neither anvil_nodeInfo nor hardhat_metadata. ` +
      'deploy:local needs the setCode/setStorageAt cheat codes of anvil or `hardhat node`.',
  );
}

async function main(): Promise<void> {
  const account = privateKeyToAccount(ADMIN_PRIVATE_KEY);
  const transport = http(RPC_URL);
  const publicClient = createPublicClient({ transport });

  const chainId = await publicClient.getChainId();
  if (chainId !== 31337) {
    throw new Error(
      `The node at ${RPC_URL} has chainid ${chainId}, but ZamaConfig pins the local stack addresses for 31337.`,
    );
  }

  const mode = await detectNodeMode((args) => publicClient.request(args as never));
  const testClient = createTestClient({ mode, transport });
  const walletClient = createWalletClient({ account, transport });

  // No overwrite path: `setCodeAt` replaces code but NOT storage, so re-initializing over a used stack
  // trips on surviving state (e.g. ProtocolConfig's KMS context counter). A fresh stack means a fresh node.
  const executor = FIXED.fhevmAddresses.fhevmExecutorAddress as Address;
  const existing = (await publicClient.getCode({ address: executor })) ?? '0x';
  if (existing.length > 2) {
    console.log(`FHEVM cleartext stack already present on ${RPC_URL} (${mode}); nothing to do.`);
    console.log('For a fresh stack, restart the node and run this again.');
    return;
  }

  console.log(`Deploying the FHEVM cleartext stack to ${RPC_URL} (${mode})...`);

  await deployAt({
    ethProvider: {
      async setCodeAt(parameters) {
        await testClient.setCode({ address: parameters.address as Address, bytecode: parameters.bytecode as Hex });
      },
      async setStorageAt(parameters) {
        await testClient.setStorageAt({
          address: parameters.address as Address,
          index: parameters.slot as Hex,
          value: parameters.value as Hex,
        });
      },
      async getCodeAt(parameters) {
        return (await publicClient.getCode({ address: parameters.address as Address })) ?? '0x';
      },
    },
    admin: {
      getAddress: () => Promise.resolve(account.address),
      deploy: () => {
        throw new Error('deployAt places code with setCodeAt; it never deploys via CREATE.');
      },
      async writeContract(parameters) {
        const call = parameters as {
          readonly address: string;
          readonly abi: readonly unknown[];
          readonly functionName: string;
          readonly args?: readonly unknown[];
        };
        const hash = await walletClient.writeContract({
          address: call.address as Address,
          abi: call.abi as never,
          functionName: call.functionName,
          args: call.args as unknown as never,
          chain: null,
        });
        await publicClient.waitForTransactionReceipt({ hash });
        return hash;
      },
    },
    addresses: FIXED,
    config: bootstrapConfig(
      privateKeyToAccount(KMS_SIGNER_PRIVATE_KEY).address,
      privateKeyToAccount(COPROCESSOR_SIGNER_PRIVATE_KEY).address,
    ),
  });

  // Read back through the stack itself, so a silent misconfiguration fails here and not in a user's test.
  const kmsSigners = await publicClient.readContract({
    address: FIXED.fhevmAddresses.kmsVerifierAddress as Address,
    abi: KMS_VERIFIER_ABI,
    functionName: 'getKmsSigners',
  });
  const expected = privateKeyToAccount(KMS_SIGNER_PRIVATE_KEY).address.toLowerCase();
  if (!kmsSigners.some((signer) => signer.toLowerCase() === expected)) {
    throw new Error('Post-deploy check failed: KMSVerifier.getKmsSigners() does not contain the mock KMS signer.');
  }

  console.log('Done. Stack addresses (pinned by ZamaConfig for chainid 31337):');
  console.log(`  ACL              ${FIXED.fhevmAddresses.aclAddress}`);
  console.log(`  FHEVMExecutor    ${FIXED.fhevmAddresses.fhevmExecutorAddress}`);
  console.log(`  KMSVerifier      ${FIXED.fhevmAddresses.kmsVerifierAddress}`);
}

main().catch((error: unknown) => {
  console.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
});
