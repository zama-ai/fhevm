// Exercises the reference ethers adapter in test/ts/utils/ethersEthereumLib.ts.
//
// The adapter is meant to be copy-pasted by consumers, so it needs proof rather than review. Two
// tests, covering the two ways the nonce sequence can break:
//
//   1. A full stack deployed through it, with every address compared against `precomputeAddresses`.
//      Since each address is CREATE(deployer, startNonce + k), matching addresses IS the proof that
//      the ~26 sends occupied a contiguous nonce range — the exact thing an adapter that lets ethers
//      choose gets wrong.
//   2. A failed broadcast must not consume a nonce. Driven with a stub signer, because making a real
//      node reject one send mid-deploy is not reproducible.
import { deploy, precomputeAddresses } from '../../pkg/ts/index.ts';
import { ethers as EthersT } from 'ethers';
import { expect, test } from 'vitest';
import { startAnvil, stopAnvil, waitForAnvil, MNEMONIC, DEPLOYER_ADDRESS_INDEX } from '@fhevm/sdk-common-dev';
import { createEthersEthereumAdapters, createEthersEthereumSigner } from '@fhevm/sdk-vendored-dev/ethersEthereumLib.ts';

const DEPLOYER_INDEX = DEPLOYER_ADDRESS_INDEX;

////////////////////////////////////////////////////////////////////////////////

test('the reference ethers adapter deploys a stack at the precomputed addresses', async () => {
  const anvil = startAnvil({ port: 8640, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const wallet = EthersT.HDNodeWallet.fromPhrase(MNEMONIC, undefined, `m/44'/60'/0'/0/${String(DEPLOYER_INDEX)}`);
    const { provider, signer, utils } = createEthersEthereumAdapters({
      rpcUrl: anvil.rpcUrl,
      privateKey: wallet.privateKey,
    });

    const deployerAddress = await signer.getAddress();
    expect(deployerAddress.toLowerCase()).toBe(wallet.address.toLowerCase());

    // What the addresses must come out as. `precomputeAddresses` is synchronous and takes the start
    // nonce; reading it through the adapter rather than assuming 0n also exercises the provider.
    const startNonce = BigInt(await provider.getTransactionCount({ address: deployerAddress }));
    const expected = precomputeAddresses({
      ethUtils: utils,
      from: deployerAddress as `0x${string}`,
      startNonce,
    });

    // One adapter for both roles: the counter is per adapter, so two adapters over the same account
    // would each start their own and collide.
    const deployed = await deploy({ ethProvider: provider, ethUtils: utils, deployer: signer, admin: signer });

    expect(deployed.fhevmAddresses).toStrictEqual(expected.fhevmAddresses);
    expect(deployed.cleartextAddresses).toStrictEqual(expected.cleartextAddresses);
    expect(deployed.pauserSetAddress).toBe(expected.pauserSetAddress);

    // Every proxy actually carries code, so the addresses are not merely arithmetic agreement.
    for (const address of Object.values(deployed.fhevmAddresses)) {
      expect(await provider.getCodeAt({ address })).not.toBe('0x');
    }
  } finally {
    await stopAnvil(anvil.process);
  }
}, 120_000);

////////////////////////////////////////////////////////////////////////////////

/** Minimal ABI for the stub calls below — ethers refuses to encode a function it cannot find. */
const NOOP_ABI = [{ type: 'function', name: 'noop', inputs: [], outputs: [], stateMutability: 'nonpayable' }] as const;

/** A signer that records the nonces it was asked to broadcast, and fails the first attempt. */
function createFlakySigner(parameters: { readonly failFirst: boolean }): {
  readonly signer: EthersT.Signer;
  readonly attemptedNonces: number[];
} {
  const attemptedNonces: number[] = [];
  let calls = 0;

  const receipt = { status: 1, hash: '0xreceipt', contractAddress: null };
  const stub = {
    getAddress: () => Promise.resolve('0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4'),
    provider: {
      // The one node read the adapter makes. Deliberately not 0, so an adapter that ignored it and
      // started from zero would be caught.
      getTransactionCount: () => Promise.resolve(7),
    },
    sendTransaction: (request: { nonce: number }) => {
      attemptedNonces.push(request.nonce);
      calls += 1;
      if (parameters.failFirst && calls === 1) {
        return Promise.reject(new Error('broadcast rejected by the node'));
      }
      return Promise.resolve({ hash: '0xtx', wait: () => Promise.resolve(receipt) });
    },
  };

  return { signer: stub as unknown as EthersT.Signer, attemptedNonces };
}

test('a failed broadcast does not consume a nonce', async () => {
  const { signer, attemptedNonces } = createFlakySigner({ failFirst: true });
  const adapter = createEthersEthereumSigner(signer);
  const call = { address: '0x0000000000000000000000000000000000000001', abi: NOOP_ABI, functionName: 'noop' };

  // The adapter must start from the count it read, not from 0.
  await expect(adapter.writeContract({ ...call, args: [] })).rejects.toThrow('broadcast rejected by the node');
  expect(attemptedNonces).toStrictEqual([7]);

  // The nonce the node never accepted must be handed out again. Advancing it here would leave a
  // permanent gap, and every address derived after this point would be wrong.
  await adapter.writeContract({ ...call, args: [] });
  expect(attemptedNonces).toStrictEqual([7, 7]);

  // And from there the sequence continues contiguously.
  await adapter.writeContract({ ...call, args: [] });
  expect(attemptedNonces).toStrictEqual([7, 7, 8]);
});

////////////////////////////////////////////////////////////////////////////////

test('sends are serialized, so unawaited calls cannot interleave nonces', async () => {
  const { signer, attemptedNonces } = createFlakySigner({ failFirst: false });
  const adapter = createEthersEthereumSigner(signer);
  const call = { address: '0x0000000000000000000000000000000000000001', abi: NOOP_ABI, functionName: 'noop', args: [] };

  // Fired without awaiting in between — the queue is what keeps these in order and gap-free.
  await Promise.all([adapter.writeContract(call), adapter.writeContract(call), adapter.writeContract(call)]);

  expect(attemptedNonces).toStrictEqual([7, 8, 9]);
});
