import {
  DEFAULT_KMS_SOFTWARE_VERSION,
  DEFAULT_KMS_THRESHOLDS,
  DEFAULT_PCR_VALUES,
  deploy,
  destroyKmsContext,
  generateFromExistingDefaultKmsNodes,
  nextDefaultKmsSignerWindow,
  precomputeAddresses,
} from '@fhevm/host-contracts-cleartext/ts';
import { createPublicClient, http, type Address } from 'viem';
import { foundry } from 'viem/chains';
import { expect, test } from 'vitest';
import { startAnvil, stopAnvil, waitForAnvil } from './anvil.ts';
import { privateKeyFromMnemonic, privateKeyToAddress } from './ethUtils.ts';
import { createViemEthereumAdapters } from './viemEthereumLib.ts';

const MNEMONIC = 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';

const KMS_NODE_PARAMS_COMPONENTS = [
  { name: 'txSenderAddress', type: 'address' },
  { name: 'signerAddress', type: 'address' },
  { name: 'ipAddress', type: 'string' },
  { name: 'storageUrl', type: 'string' },
  { name: 'partyId', type: 'int32' },
  { name: 'mpcIdentity', type: 'string' },
  { name: 'caCert', type: 'bytes' },
  { name: 'storagePrefix', type: 'string' },
] as const;

const PROTOCOL_CONFIG_ABI = [
  {
    type: 'function',
    name: 'getCurrentKmsContextId',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'uint256' }],
  },
  {
    type: 'function',
    name: 'isValidKmsContext',
    stateMutability: 'view',
    inputs: [{ name: 'kmsContextId', type: 'uint256' }],
    outputs: [{ type: 'bool' }],
  },
  {
    type: 'function',
    name: 'getKmsSigners',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'address[]' }],
  },
  {
    type: 'function',
    name: 'defineNewKmsContextAndEpoch',
    stateMutability: 'nonpayable',
    inputs: [
      { name: 'kmsNodeParams', type: 'tuple[]', components: KMS_NODE_PARAMS_COMPONENTS },
      {
        name: 'thresholds',
        type: 'tuple',
        components: [
          { name: 'publicDecryption', type: 'uint256' },
          { name: 'userDecryption', type: 'uint256' },
          { name: 'kmsGen', type: 'uint256' },
          { name: 'mpc', type: 'uint256' },
        ],
      },
      { name: 'softwareVersion', type: 'string' },
      {
        name: 'pcrValues',
        type: 'tuple[]',
        components: [
          { name: 'pcr0', type: 'bytes' },
          { name: 'pcr1', type: 'bytes' },
          { name: 'pcr2', type: 'bytes' },
        ],
      },
    ],
    outputs: [],
  },
] as const;

const ACL_OWNER_ABI = [
  {
    type: 'function',
    name: 'execute',
    stateMutability: 'nonpayable',
    inputs: [
      { name: 'target', type: 'address' },
      { name: 'data', type: 'bytes' },
    ],
    outputs: [{ type: 'bytes' }],
  },
] as const;

// The v13 sibling test destroys a PAST context after a one-call rotation. v14 has no rotation helper
// yet (context activation is a multi-party ceremony — see `ts/kmsContext.ts`), so this test destroys a
// PENDING context instead: `defineNewKmsContextAndEpoch` (sent raw through `ACLOwner.execute`) leaves
// the new context Pending, which is live — and therefore destroyable — without ever being current.
// Once the rotation helper lands, extend this with the past-context path.
test('destroyKmsContext retires a pending KMS context through ACLOwner.execute', async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });
  const deployerAddress = privateKeyToAddress({ privateKey: deployerKey });

  const anvil = startAnvil({ port: 8632, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: deployerKey });
    const publicClient = createPublicClient({ chain: foundry, transport: http(anvil.rpcUrl) });

    const { fhevmAddresses, cleartextAddresses, pauserSetAddress } = precomputeAddresses({
      ethUtils: adapters.utils,
      from: deployerAddress,
      startNonce: 0n,
    });

    // Deploy a default v14 stack (ACL owned by the standing ACLOwner, ACLOwner owned by `admin`).
    const deployed = await deploy({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
      precomputed: { fhevmAddresses, cleartextAddresses, pauserSetAddress },
    });

    const protocolConfig = deployed.fhevmAddresses.protocolConfigAddress as Address;
    const currentContextId = (): Promise<bigint> =>
      publicClient.readContract({
        address: protocolConfig,
        abi: PROTOCOL_CONFIG_ABI,
        functionName: 'getCurrentKmsContextId',
      });
    const isValid = (kmsContextId: bigint): Promise<boolean> =>
      publicClient.readContract({
        address: protocolConfig,
        abi: PROTOCOL_CONFIG_ABI,
        functionName: 'isValidKmsContext',
        args: [kmsContextId],
      });

    const firstContextId = await currentContextId();
    expect(await isValid(firstContextId)).toBe(true);

    // The current context cannot be destroyed (CurrentKmsContextCannotBeDestroyed bubbles up through
    // ACLOwner.execute).
    await expect(
      destroyKmsContext({
        ethUtils: adapters.utils,
        admin: adapters.signer,
        aclOwnerAddress: deployed.aclOwnerAddress,
        protocolConfigAddress: deployed.fhevmAddresses.protocolConfigAddress,
        kmsContextId: firstContextId,
      }),
    ).rejects.toThrow();

    // Define a new context on the next default-pool signer window. It stays Pending — never confirmed
    // by the KMS quorum — so the current context is untouched (and the pending one is not yet "valid").
    const liveSigners = (await publicClient.readContract({
      address: protocolConfig,
      abi: PROTOCOL_CONFIG_ABI,
      functionName: 'getKmsSigners',
    })) as readonly string[];
    const nextNodeParams = generateFromExistingDefaultKmsNodes(nextDefaultKmsSignerWindow(liveSigners));
    const defineCallData = await adapters.utils.encodeCall({
      abi: PROTOCOL_CONFIG_ABI,
      functionName: 'defineNewKmsContextAndEpoch',
      args: [nextNodeParams, DEFAULT_KMS_THRESHOLDS, DEFAULT_KMS_SOFTWARE_VERSION, DEFAULT_PCR_VALUES],
    });
    await adapters.signer.writeContract({
      address: deployed.aclOwnerAddress,
      abi: ACL_OWNER_ABI,
      functionName: 'execute',
      args: [deployed.fhevmAddresses.protocolConfigAddress, defineCallData],
    });

    const pendingContextId = firstContextId + 1n;
    expect(await currentContextId()).toBe(firstContextId);
    expect(await isValid(pendingContextId)).toBe(false); // Pending, not Active

    // Destroy the pending context: succeeds while it is live...
    await destroyKmsContext({
      ethUtils: adapters.utils,
      admin: adapters.signer,
      aclOwnerAddress: deployed.aclOwnerAddress,
      protocolConfigAddress: deployed.fhevmAddresses.protocolConfigAddress,
      kmsContextId: pendingContextId,
    });

    // ...and is rejected once destroyed (InvalidKmsContext) — the observable that the destroy landed.
    await expect(
      destroyKmsContext({
        ethUtils: adapters.utils,
        admin: adapters.signer,
        aclOwnerAddress: deployed.aclOwnerAddress,
        protocolConfigAddress: deployed.fhevmAddresses.protocolConfigAddress,
        kmsContextId: pendingContextId,
      }),
    ).rejects.toThrow();

    // The current context is untouched throughout.
    expect(await currentContextId()).toBe(firstContextId);
    expect(await isValid(firstContextId)).toBe(true);
  } finally {
    await stopAnvil(anvil.process);
  }
}, 120_000);
