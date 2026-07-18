import {
  defineNewKmsContext,
  deploy,
  destroyKmsContext,
  precomputeAddresses,
} from '@fhevm/host-contracts-cleartext/ts';
import { createPublicClient, http, type Address } from 'viem';
import { foundry } from 'viem/chains';
import { expect, test } from 'vitest';
import { startAnvil, stopAnvil, waitForAnvil } from './anvil.ts';
import { privateKeyFromMnemonic, privateKeyToAddress } from './ethUtils.ts';
import { createViemEthereumAdapters } from './viemEthereumLib.ts';

const MNEMONIC = 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';

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
] as const;

test('destroyKmsContext retires a past KMS context through ACLOwner.execute', async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });
  const deployerAddress = privateKeyToAddress({ privateKey: deployerKey });

  const anvil = startAnvil({ port: 8631, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: deployerKey });
    const publicClient = createPublicClient({ chain: foundry, transport: http(anvil.rpcUrl) });

    const { fhevmAddresses, cleartextAddresses, pauserSetAddress } = precomputeAddresses({
      ethUtils: adapters.utils,
      from: deployerAddress,
      startNonce: 0n,
    });

    // Deploy a default v13 stack (ACL owned by the standing ACLOwner, ACLOwner owned by `admin`).
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

    // The deploy seeds the first context; rotate once so it becomes a PAST (non-current) context — the
    // current context cannot be destroyed.
    const firstContextId = await currentContextId();
    expect(await isValid(firstContextId)).toBe(true);

    await defineNewKmsContext({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      admin: adapters.signer,
      aclOwnerAddress: deployed.aclOwnerAddress,
      protocolConfigAddress: deployed.fhevmAddresses.protocolConfigAddress,
    });
    const secondContextId = await currentContextId();
    expect(secondContextId).toBe(firstContextId + 1n);
    expect(await isValid(firstContextId)).toBe(true); // still valid: past but not destroyed

    // Destroy the first (now past) context.
    await destroyKmsContext({
      ethUtils: adapters.utils,
      admin: adapters.signer,
      aclOwnerAddress: deployed.aclOwnerAddress,
      protocolConfigAddress: deployed.fhevmAddresses.protocolConfigAddress,
      kmsContextId: firstContextId,
    });

    // The past context is now invalid; the current context is untouched.
    expect(await isValid(firstContextId)).toBe(false);
    expect(await currentContextId()).toBe(secondContextId);
    expect(await isValid(secondContextId)).toBe(true);
  } finally {
    await stopAnvil(anvil.process);
  }
}, 120_000);
