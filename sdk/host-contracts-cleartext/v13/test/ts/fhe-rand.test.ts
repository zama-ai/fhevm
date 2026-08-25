import { deploy } from '@fhevm/host-contracts-cleartext/ts';
import { createPublicClient, createWalletClient, http, parseEventLogs, type Address, type Hex } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { foundry } from 'viem/chains';
import { expect, test } from 'vitest';
import { startAnvil, stopAnvil, waitForAnvil } from './utils/anvil.ts';
import { privateKeyFromMnemonic } from './utils/ethUtils.ts';
import { createViemEthereumAdapters } from './utils/viemEthereumLib.ts';

const MNEMONIC = 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';
const FHE_TYPE_UINT64 = 5;

/**
 * How many draws. Enough that a constant or a per-block seed shows up immediately, and small enough that
 * the run stays a few seconds on anvil (one transaction, and therefore one block, per draw).
 */
const DRAWS = 100;

/** Draws forced into a single block. Small — the point is >1 in one block, not volume. */
const BATCH_DRAWS = 5;

const EXECUTOR_ABI = [
  {
    type: 'function',
    name: 'fheRand',
    stateMutability: 'nonpayable',
    inputs: [{ name: 'randType', type: 'uint8' }],
    outputs: [{ name: 'result', type: 'bytes32' }],
  },
  {
    type: 'event',
    name: 'FheRand',
    inputs: [
      { name: 'caller', type: 'address', indexed: true },
      { name: 'randType', type: 'uint8', indexed: false },
      { name: 'seed', type: 'bytes16', indexed: false },
      { name: 'result', type: 'bytes32', indexed: false },
    ],
  },
] as const;

const CLEARTEXT_DB_ABI = [
  {
    type: 'function',
    name: 'get',
    stateMutability: 'view',
    inputs: [{ name: 'handle', type: 'bytes32' }],
    outputs: [{ type: 'uint256' }],
  },
] as const;

/**
 * Consecutive `fheRand` calls must produce distinct handles.
 *
 * This is a structural guarantee, not a statistical one, which is why it is worth asserting as an
 * equality rather than a distribution. `_generateSeed` mixes a storage counter (`$.counterRand`, bumped
 * on every call) into the seed alongside the block data, and the handle is
 * `keccak256(domain, fheRand, randType, seed)` plus type metadata. So the counter alone makes every seed
 * — and therefore every handle — unique, no matter how many draws land in the same block or share a
 * timestamp.
 *
 * That is the failure this guards. Drop the counter and the seed becomes a function of block data only:
 * two draws in one block collide, produce the SAME handle, and the second silently overwrites the first
 * in `CleartextDB`. On anvil each transaction gets its own block, so a naive test would still pass — the
 * assertion here is on the handles rather than on timing, so it does not depend on that.
 */
test(`${String(DRAWS)} consecutive fheRand calls all produce distinct handles`, async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });

  const anvil = startAnvil({ port: 8634, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: deployerKey });
    const publicClient = createPublicClient({ chain: foundry, transport: http(anvil.rpcUrl) });
    const wallet = createWalletClient({
      account: privateKeyToAccount(deployerKey),
      chain: foundry,
      transport: http(anvil.rpcUrl),
    });

    const deployed = await deploy({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
    });

    const executor = deployed.fhevmAddresses.fhevmExecutorAddress as Address;
    const cleartextDb = deployed.cleartextAddresses.cleartextDbAddress as Address;

    const fheRand = async (): Promise<{ handle: Hex; seed: Hex }> => {
      const hash = await wallet.writeContract({
        address: executor,
        abi: EXECUTOR_ABI,
        functionName: 'fheRand',
        args: [FHE_TYPE_UINT64],
      });
      const receipt = await publicClient.waitForTransactionReceipt({ hash });
      const events = parseEventLogs({ abi: EXECUTOR_ABI, eventName: 'FheRand', logs: receipt.logs });
      const event = events[0];
      if (event === undefined) {
        throw new Error('FheRand event not found');
      }
      return { handle: event.args.result, seed: event.args.seed };
    };

    const handles: Hex[] = [];
    const seeds: Hex[] = [];
    for (let i = 0; i < DRAWS; i++) {
      // Sequential on purpose: the point is CONSECUTIVE calls, so each must be mined before the next is
      // sent. Promise.all would submit them concurrently and prove nothing about ordering.
      const { handle, seed } = await fheRand();
      handles.push(handle);
      seeds.push(seed);
    }

    expect(handles).toHaveLength(DRAWS);

    // The assertion, stated so a failure names the duplicate rather than just the count.
    const seenAt = new Map<Hex, number>();
    const duplicates: string[] = [];
    handles.forEach((handle, index) => {
      const first = seenAt.get(handle);
      if (first === undefined) {
        seenAt.set(handle, index);
      } else {
        duplicates.push(`draw ${String(index)} repeats draw ${String(first)}: ${handle}`);
      }
    });
    expect(duplicates, `fheRand returned a repeated handle:\n  ${duplicates.join('\n  ')}`).toEqual([]);
    expect(seenAt.size).toBe(DRAWS);

    // The seeds are distinct for the same reason, checked separately: if handles ever collide, this says
    // whether the seed or the hashing was at fault.
    expect(new Set(seeds).size).toBe(DRAWS);

    // Every draw actually landed in the DB. Distinct handles that do not resolve would mean the
    // executor emitted an event without the arithmetic contract recording anything.
    for (const handle of handles) {
      const value = await publicClient.readContract({
        address: cleartextDb,
        abi: CLEARTEXT_DB_ABI,
        functionName: 'get',
        args: [handle],
      });
      // 0 is a legitimate draw, so presence is all that can be asserted — not a non-zero value.
      expect(typeof value).toBe('bigint');
    }

    // Not asserted: that the VALUES are all distinct. With uint64 that is overwhelmingly likely but not
    // guaranteed, and a birthday collision would be a flake rather than a defect. That the values are
    // not all identical is covered by the distinct seeds above, which is what feeds them.

    // --- The case that makes the above non-trivial: several draws in ONE block. ---
    //
    // Everything so far had one transaction per block, because anvil automines. Block data therefore
    // differed on every draw, so a seed built from block data ALONE would still have produced distinct
    // handles and this test would have passed while the counter did nothing. Batching draws into a
    // single block removes that: same timestamp, same blockhash, same everything except `$.counterRand`.
    await publicClient.request({ method: 'evm_setAutomine', params: [false] } as never);
    const batched: Hex[] = [];
    try {
      const hashes: Hex[] = [];
      for (let i = 0; i < BATCH_DRAWS; i++) {
        hashes.push(
          await wallet.writeContract({
            address: executor,
            abi: EXECUTOR_ABI,
            functionName: 'fheRand',
            args: [FHE_TYPE_UINT64],
          }),
        );
      }
      await publicClient.request({ method: 'evm_mine', params: [] } as never);

      const blockNumbers = new Set<bigint>();
      for (const hash of hashes) {
        const receipt = await publicClient.waitForTransactionReceipt({ hash });
        blockNumbers.add(receipt.blockNumber);
        const events = parseEventLogs({ abi: EXECUTOR_ABI, eventName: 'FheRand', logs: receipt.logs });
        const event = events[0];
        if (event === undefined) {
          throw new Error('FheRand event not found in the batched block');
        }
        batched.push(event.args.result);
      }

      // The premise of this section. If anvil split them across blocks the assertion below would be the
      // same trivial one as above, so it is checked rather than assumed.
      expect(blockNumbers.size, `expected all ${String(BATCH_DRAWS)} draws in ONE block`).toBe(1);
    } finally {
      await publicClient.request({ method: 'evm_setAutomine', params: [true] } as never);
    }

    expect(new Set(batched).size, 'draws within a single block must still produce distinct handles').toBe(BATCH_DRAWS);
    // And they do not collide with the earlier ones either — the counter is global, not per block.
    expect(new Set([...handles, ...batched]).size).toBe(DRAWS + BATCH_DRAWS);
  } finally {
    await stopAnvil(anvil.process);
  }
}, 180_000);
