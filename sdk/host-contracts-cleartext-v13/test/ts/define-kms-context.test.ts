import { deploy, defineNewKmsContext } from '@fhevm/host-contracts-cleartext/ts';
import { createPublicClient, http, type Address } from 'viem';
import { mnemonicToAccount } from 'viem/accounts';
import { foundry } from 'viem/chains';
import { expect, test } from 'vitest';
import { startAnvil, stopAnvil, waitForAnvil } from './anvil.ts';
import { privateKeyFromMnemonic } from './ethUtils.ts';
import { createViemEthereumAdapters } from './viemEthereumLib.ts';

const MNEMONIC = 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';
// The default KMS signer pool is derived from the FHEVM test mnemonic at m/44'/60'/0'/3/<i>.
const FHEVM_MNEMONIC = 'test test test test test test test future home engine virtual motion';
const defaultKmsSigner = (index: number): string =>
  mnemonicToAccount(FHEVM_MNEMONIC, { changeIndex: 3, addressIndex: index }).address.toLowerCase();

const PROTOCOL_CONFIG_ABI = [
  { type: 'function', name: 'getKmsSigners', stateMutability: 'view', inputs: [], outputs: [{ type: 'address[]' }] },
  {
    type: 'function',
    name: 'getCurrentKmsContextId',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'uint256' }],
  },
] as const;

test('defineNewKmsContext rotates the KMS signer window through ACLOwner.execute', async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });

  const anvil = startAnvil({ port: 8630, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: deployerKey });
    const publicClient = createPublicClient({ chain: foundry, transport: http(anvil.rpcUrl) });

    // Deploy a default v13 stack with no config AND no precomputed addresses — deploy derives them from
    // the deployer's live nonce. (no config → default KMS pool[0..3]; ACL owned by the standing
    // ACLOwner, ACLOwner owned by `admin` — exactly the topology defineNewKmsContext targets.)
    const deployed = await deploy({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
    });

    const protocolConfig = deployed.fhevmAddresses.protocolConfigAddress as Address;
    const readSigners = async (): Promise<string[]> =>
      [
        ...(await publicClient.readContract({
          address: protocolConfig,
          abi: PROTOCOL_CONFIG_ABI,
          functionName: 'getKmsSigners',
        })),
      ].map((s) => s.toLowerCase());
    const readContextId = (): Promise<bigint> =>
      publicClient.readContract({
        address: protocolConfig,
        abi: PROTOCOL_CONFIG_ABI,
        functionName: 'getCurrentKmsContextId',
      });

    // Initial context: the default pool window [0, 4).
    expect(await readSigners()).toEqual([0, 1, 2, 3].map(defaultKmsSigner));
    const initialContextId = await readContextId();

    // Rotate to the next window [4, 8), routed through ACLOwner.execute (admin owns the ACLOwner).
    const { signers } = await defineNewKmsContext({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      admin: adapters.signer,
      aclOwnerAddress: deployed.aclOwnerAddress,
      protocolConfigAddress: deployed.fhevmAddresses.protocolConfigAddress,
    });

    const nextWindow = [4, 5, 6, 7].map(defaultKmsSigner);
    expect(signers.map((s) => s.toLowerCase())).toEqual(nextWindow);
    // The on-chain context was actually replaced: new signer set + a fresh context id.
    expect(await readSigners()).toEqual(nextWindow);
    expect(await readContextId()).toBe(initialContextId + 1n);
  } finally {
    await stopAnvil(anvil.process);
  }
}, 120_000);
