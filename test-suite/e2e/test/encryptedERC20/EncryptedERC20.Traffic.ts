import fs from 'node:fs';

import { expect } from 'chai';

import type { EncryptedERC20 } from '../../types';
import { checkConsensus } from '../consensusWatchdog';
import { createInstances } from '../instance';
import { type Signers, getSigners, initSigners } from '../signers';
import { deployEncryptedERC20Fixture } from './EncryptedERC20.fixture';

// Runs TRAFFIC_TRANSFERS transfers, or, when TRAFFIC_STOP_FILE is set, until that file exists: a test that
// upgrades the stack runs it in the background and creates the file once the upgrade is over.
const STOP_FILE = process.env.TRAFFIC_STOP_FILE;
const MAX_TRANSFERS = STOP_FILE ? Infinity : Number(process.env.TRAFFIC_TRANSFERS ?? 20);
const PAUSE_MS = Number(process.env.TRAFFIC_PAUSE_MS ?? 1000);
const SEED = Number(process.env.TRAFFIC_SEED ?? Date.now() % 2 ** 32);
const MINT_AMOUNT = 1_000n;

/** mulberry32: small and seeded, so TRAFFIC_SEED replays a failing run transfer for transfer. */
const seededRandom = (seed: number) => () => {
  seed = (seed + 0x6d2b79f5) | 0;
  let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
  t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
  return ((t ^ (t >>> 14)) >>> 0) / 2 ** 32;
};

describe('EncryptedERC20 traffic', function () {
  before(async function () {
    await initSigners(5);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
  });

  it('[erc20-traffic] keeps five wallets on the expected balances', async function () {
    this.timeout(0);
    const wallets = Object.keys(this.signers) as (keyof Signers)[];
    const random = seededRandom(SEED);
    const pick = <T>(items: T[]) => items[Math.floor(random() * items.length)]!;
    const contract: EncryptedERC20 = await deployEncryptedERC20Fixture();
    const contractAddress = await contract.getAddress();
    expect((await (await contract.mint(MINT_AMOUNT)).wait())?.status).to.eq(1);
    const expected = Object.fromEntries(wallets.map((wallet) => [wallet, 0n])) as Record<keyof Signers, bigint>;
    expected.alice = MINT_AMOUNT;
    const decryptBalance = async (wallet: keyof Signers) =>
      this.instances[wallet].userDecryptSingleHandle({
        handle: await contract.balanceOf(this.signers[wallet]),
        contractAddress,
        signer: this.signers[wallet],
      });
    console.log(`[erc20-traffic] seed=${SEED} token=${contractAddress}`);

    const startedAt = Date.now();
    let count = 0;
    while (count < MAX_TRANSFERS && !(STOP_FILE && fs.existsSync(STOP_FILE))) {
      count += 1;
      const from = pick(wallets.filter((wallet) => expected[wallet] > 0n));
      const to = pick(wallets.filter((wallet) => wallet !== from));
      // One transfer in ten asks for more than the sender holds; the contract then moves nothing.
      const asked = random() < 0.1 ? expected[from] + 1n : 1n + BigInt(Math.floor(random() * Number(expected[from])));
      const moved = asked <= expected[from] ? asked : 0n;

      const input = await this.instances[from].encryptUint64({
        value: asked,
        contractAddress,
        userAddress: this.signers[from].address,
      });
      const tx = await contract
        .connect(this.signers[from])
        ['transfer(address,bytes32,bytes)'](this.signers[to].address, input.handles[0], input.inputProof);
      expect((await tx.wait())?.status, `transfer #${count}`).to.eq(1);
      expected[from] -= moved;
      expected[to] += moved;

      expect(await decryptBalance(from), `${from} after transfer #${count}`).to.eq(expected[from]);
      expect(await decryptBalance(to), `${to} after transfer #${count}`).to.eq(expected[to]);
      await checkConsensus();
      console.log(`[erc20-traffic #${count}] ${from} -> ${to}: asked ${asked}, moved ${moved}`);
      await new Promise((resolve) => setTimeout(resolve, PAUSE_MS));
    }
    const minutes = (Date.now() - startedAt) / 60_000;
    console.log(`[erc20-traffic] ${count} transfers in ${minutes.toFixed(1)} min, seed=${SEED}`);
  });
});
