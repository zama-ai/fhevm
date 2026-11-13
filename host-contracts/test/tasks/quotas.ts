import { expect } from 'chai';
import { type Contract, Wallet } from 'ethers';
import { ethers, network, run } from 'hardhat';
import { rejects } from 'node:assert/strict';

import { readHostAddress } from './taskHelpers';

function runQuotaTask(name: string, args: Record<string, unknown>) {
  return run(name, { useInternalProxyAddress: true, ...args });
}

describe('HCULimit quota tasks', function () {
  const dapp = '0x0000000000000000000000000000000000001111';
  const otherDapp = '0x0000000000000000000000000000000000002222';
  const quotaId = '7';
  let hcuLimit: Contract;
  let snapshot: string;

  before(async function () {
    hcuLimit = (await ethers.getContractAt(
      'HCULimit',
      readHostAddress('HCU_LIMIT_CONTRACT_ADDRESS'),
    )) as unknown as Contract;
  });

  beforeEach(async function () {
    snapshot = await network.provider.send('evm_snapshot');
  });

  afterEach(async function () {
    await network.provider.send('evm_revert', [snapshot]);
  });

  async function withSigner(fn: (signer: Wallet) => Promise<void>) {
    const signer = Wallet.createRandom();
    const previousKey = process.env.TEST_QUOTA_SIGNER_PRIVATE_KEY;
    process.env.TEST_QUOTA_SIGNER_PRIVATE_KEY = signer.privateKey;
    try {
      await network.provider.send('hardhat_setBalance', [signer.address, '0x1000000000000000000']);
      await fn(new Wallet(signer.privateKey));
    } finally {
      if (previousKey === undefined) delete process.env.TEST_QUOTA_SIGNER_PRIVATE_KEY;
      else process.env.TEST_QUOTA_SIGNER_PRIVATE_KEY = previousKey;
    }
  }

  it('prints calldata without broadcasting by default', async function () {
    const output: string[] = [];
    const originalLog = console.log;
    console.log = (...args) => output.push(args.join(' '));
    try {
      await runQuotaTask('task:setQuota', { quotaId, maxHcu: '1000', decayPerBlock: '1000' });
    } finally {
      console.log = originalLog;
    }
    expect(output.join('\n')).to.include('Calldata: 0x');
    expect((await hcuLimit.getFunction('getQuota').staticCall(quotaId))[0]).to.equal(0n);
  });

  it('broadcasts the owner calls', async function () {
    await runQuotaTask('task:setQuota', { quotaId, maxHcu: '1000', decayPerBlock: '10', broadcast: true });
    const [maxHCU, decayPerBlock] = await hcuLimit.getFunction('getQuota').staticCall(quotaId);
    expect([maxHCU, decayPerBlock]).to.deep.equal([1000n, 10n]);

    await runQuotaTask('task:assignQuota', { quotaId, account: dapp, broadcast: true });
    expect(await hcuLimit.getFunction('getContractQuota').staticCall(dapp)).to.equal(7n);

    await runQuotaTask('task:setQuotaAdmin', { quotaId, admin: otherDapp, broadcast: true });
    expect(await hcuLimit.getFunction('getQuotaAdmin').staticCall(quotaId)).to.equal(otherDapp);

    await runQuotaTask('task:pauseQuota', { quotaId, broadcast: true });
    expect(await hcuLimit.getFunction('isQuotaPaused').staticCall(quotaId)).to.equal(true);

    await runQuotaTask('task:unpauseQuota', { quotaId, broadcast: true });
    expect(await hcuLimit.getFunction('isQuotaPaused').staticCall(quotaId)).to.equal(false);
  });

  it('blocks stopping the default quota when the required dApp list is empty or incomplete', async function () {
    await rejects(
      runQuotaTask('task:setQuota', { quotaId: '0', maxHcu: '0', decayPerBlock: '0' }),
      /nonempty comma-separated list/,
    );
    await runQuotaTask('task:setQuota', { quotaId, maxHcu: '1000', decayPerBlock: '1000', broadcast: true });
    await runQuotaTask('task:assignQuota', { quotaId, account: dapp, broadcast: true });
    await rejects(
      runQuotaTask('task:setQuota', {
        quotaId: '0',
        maxHcu: '0',
        decayPerBlock: '0',
        requiredAccounts: `${dapp},${otherDapp}`,
        broadcast: true,
      }),
      new RegExp(`Cannot stop quota 0: these required dApps compute in it: ${otherDapp}`),
    );
    expect((await hcuLimit.getFunction('getQuota').staticCall(0))[0]).to.not.equal(0n);

    await runQuotaTask('task:setQuota', {
      quotaId: '0',
      maxHcu: '0',
      decayPerBlock: '0',
      requiredAccounts: dapp,
      broadcast: true,
    });
    expect((await hcuLimit.getFunction('getQuota').staticCall(0))[0]).to.equal(0n);
  });

  it('requires the dApp list before stopping a quota that replaced the allowlist', async function () {
    await rejects(
      runQuotaTask('task:setQuota', { quotaId: '1', maxHcu: '0', decayPerBlock: '0' }),
      /nonempty comma-separated list/,
    );
    await runQuotaTask('task:setQuota', {
      quotaId: '2',
      maxHcu: '0',
      decayPerBlock: '0',
      requiredAccounts: dapp,
      broadcast: true,
    });
  });

  it('rejects malformed arguments before broadcasting', async function () {
    await rejects(
      runQuotaTask('task:setQuota', { quotaId: '65536', maxHcu: '1', decayPerBlock: '1', broadcast: true }),
      /Quota id must be at most 65535/,
    );
    await rejects(
      runQuotaTask('task:setQuota', { quotaId, maxHcu: 'lots', decayPerBlock: '1', broadcast: true }),
      /Maximum HCU must be a decimal integer/,
    );
    await rejects(
      runQuotaTask('task:assignQuota', { quotaId, account: `${dapp},${dapp}`, broadcast: true }),
      /exactly one/,
    );
    expect(await hcuLimit.getFunction('getContractQuota').staticCall(dapp)).to.equal(0n);
  });

  it('lets a quota admin grant and revoke, and reserves owner calls for the ACL owner', async function () {
    await withSigner(async (admin) => {
      await runQuotaTask('task:setQuota', { quotaId, maxHcu: '1000', decayPerBlock: '1000', broadcast: true });
      await runQuotaTask('task:setQuotaAdmin', { quotaId, admin: admin.address, broadcast: true });
      // An admin only takes contracts from a stopped default quota.
      await runQuotaTask('task:assignQuota', { quotaId, account: otherDapp, broadcast: true });
      await runQuotaTask('task:setQuota', {
        quotaId: '0',
        maxHcu: '0',
        decayPerBlock: '0',
        requiredAccounts: otherDapp,
        broadcast: true,
      });

      const asAdmin = { broadcast: true, signerKeyEnv: 'TEST_QUOTA_SIGNER_PRIVATE_KEY' };
      await runQuotaTask('task:grantQuota', { quotaId, account: dapp, ...asAdmin });
      expect(await hcuLimit.getFunction('getContractQuota').staticCall(dapp)).to.equal(7n);
      await runQuotaTask('task:revokeQuota', { quotaId, account: dapp, ...asAdmin });
      expect(await hcuLimit.getFunction('getContractQuota').staticCall(dapp)).to.equal(0n);

      await rejects(runQuotaTask('task:assignQuota', { quotaId, account: dapp, ...asAdmin }), /is not the ACL owner/);
      await rejects(runQuotaTask('task:pauseQuota', { quotaId, ...asAdmin }), /neither the ACL owner nor a pauser/);
      await rejects(runQuotaTask('task:grantQuota', { quotaId, account: dapp, broadcast: true }), /not the admin/);
    });
  });
});
