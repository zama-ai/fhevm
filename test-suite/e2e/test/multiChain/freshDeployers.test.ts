import { strict as assert } from 'node:assert';
import { getCreateAddress } from 'ethers';
import { createFundedDeployers } from './freshDeployers';

type Funder = Parameters<typeof createFundedDeployers>[0][number];

function chain(nonce: number, options: { status?: number; balance?: bigint; pending?: number; fail?: boolean } = {}) {
  const transfers: string[] = [];
  const provider = {
    getBalance: async () => options.balance ?? 10n ** 18n,
    getTransactionCount: async () => options.pending ?? 0,
  };
  const funder = {
    provider,
    sendTransaction: async ({ to }: { to: string }) => {
      if (options.fail) throw new Error('funding transport failed');
      nonce += 1;
      transfers.push(to);
      return { wait: async () => ({ status: options.status ?? 1 }) };
    },
  } as unknown as Funder;
  return { funder, transfers, nonce: () => nonce };
}

describe('Multi-Chain State Isolation: fresh deployers', () => {
  it('fresh paired CREATE addresses survive different prior funder nonces and repeated suites', async () => {
    const a = chain(31),
      b = chain(32);
    const first = await createFundedDeployers([a.funder, b.funder]);
    assert.equal(first[0].address, first[1].address);
    assert.notEqual(first[0], first[1]);
    assert.deepEqual(a.transfers, [first[0].address]);
    assert.deepEqual(b.transfers, [first[1].address]);
    assert.deepEqual([a.nonce(), b.nonce()], [32, 33]);
    assert.deepEqual(
      getCreateAddress({ from: first[0].address, nonce: await first[0].getNonce('pending') }),
      getCreateAddress({ from: first[1].address, nonce: await first[1].getNonce('pending') }),
    );
    first[0].increment();
    assert.equal(await first[0].getNonce('pending'), 1);
    assert.equal(await first[1].getNonce('pending'), 0);
    first[0].reset();
    assert.equal(await first[0].getNonce('pending'), 0);
    const second = await createFundedDeployers([a.funder, b.funder]);
    assert.equal(second[0].address, second[1].address);
    assert.notEqual(second[0].address, first[0].address);
    assert.equal(await second[0].getNonce('pending'), 0);
    assert.equal(await second[1].getNonce('pending'), 0);
  });

  it('failed funding on either chain prevents returning deployers', async () => {
    for (const failing of [0, 1]) {
      const funders = [chain(3, { status: failing === 0 ? 0 : 1 }), chain(9, { status: failing === 1 ? 0 : 1 })];
      await assert.rejects(createFundedDeployers(funders.map((item) => item.funder)), /funding was not confirmed/);
    }
  });

  it('a successful receipt without the funded balance is rejected', async () => {
    await assert.rejects(
      createFundedDeployers([chain(0).funder, chain(0, { balance: 0n }).funder]),
      /funding was not confirmed/,
    );
  });

  it('funding transport failure remains a setup failure', async () => {
    await assert.rejects(
      createFundedDeployers([chain(0).funder, chain(0, { fail: true }).funder]),
      /funding transport failed/,
    );
  });

  it('unexpected fresh-account history is rejected before deployment', async () => {
    await assert.rejects(
      createFundedDeployers([chain(0).funder, chain(0, { pending: 1 }).funder]),
      /already has transaction history/,
    );
  });

  it('disconnected or incomplete funding configuration fails before a transfer', async () => {
    const a = chain(0);
    await assert.rejects(createFundedDeployers([a.funder]), /connected funders/);
    await assert.rejects(
      createFundedDeployers([a.funder, { provider: null } as unknown as Funder]),
      /connected funders/,
    );
    await assert.rejects(createFundedDeployers([a.funder, a.funder], 0n), /positive funding/);
    assert.deepEqual(a.transfers, []);
  });
});
