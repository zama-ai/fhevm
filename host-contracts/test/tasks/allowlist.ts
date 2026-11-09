import { expect } from 'chai';
import { type Contract, Wallet } from 'ethers';
import { ethers, network, run } from 'hardhat';
import { rejects } from 'node:assert/strict';

import { readHostAddress } from './taskHelpers';

function runAllowlistTask(name: string, args: Record<string, unknown>) {
  return run(name, { useInternalProxyAddress: true, ...args });
}

describe('HCULimit allowlist tasks', function () {
  const dapp = '0x0000000000000000000000000000000000001111';
  const otherDapp = '0x0000000000000000000000000000000000002222';
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

  it('prints calldata without broadcasting by default', async function () {
    const output: string[] = [];
    const originalLog = console.log;
    console.log = (...args) => output.push(args.join(' '));
    try {
      await runAllowlistTask('task:addToAllowlist', { accounts: dapp });
    } finally {
      console.log = originalLog;
    }
    expect(output.join('\n')).to.include('Calldata: 0x');
    expect(await hcuLimit.getFunction('isAllowlisted').staticCall(dapp)).to.equal(false);
  });

  it('broadcasts the five administration calls', async function () {
    await runAllowlistTask('task:addToAllowlist', { accounts: dapp, broadcast: true });
    expect(await hcuLimit.getFunction('isAllowlisted').staticCall(dapp)).to.equal(true);

    await runAllowlistTask('task:setAllowlistActivated', { activated: true, requiredAccounts: dapp, broadcast: true });
    expect(await hcuLimit.getFunction('isAllowlistActivated').staticCall()).to.equal(true);

    const deadline = BigInt((await ethers.provider.getBlock('latest'))!.timestamp) + 3600n;
    await runAllowlistTask('task:addAllowlister', { account: dapp, deadline: deadline.toString(), broadcast: true });
    expect(await hcuLimit.getFunction('allowlisterDeadline').staticCall(dapp)).to.equal(deadline);

    await runAllowlistTask('task:removeAllowlister', { account: dapp, broadcast: true });
    expect(await hcuLimit.getFunction('allowlisterDeadline').staticCall(dapp)).to.equal(0n);

    await runAllowlistTask('task:removeFromAllowlist', { accounts: dapp, broadcast: true });
    expect(await hcuLimit.getFunction('isAllowlisted').staticCall(dapp)).to.equal(false);

    await runAllowlistTask('task:setAllowlistActivated', { activated: false, broadcast: true });
    expect(await hcuLimit.getFunction('isAllowlistActivated').staticCall()).to.equal(false);
  });

  it('blocks activation when the required dApp list is empty or incomplete', async function () {
    await rejects(runAllowlistTask('task:setAllowlistActivated', { activated: true }), /nonempty comma-separated list/);
    await runAllowlistTask('task:addToAllowlist', { accounts: dapp, broadcast: true });
    await rejects(
      runAllowlistTask('task:setAllowlistActivated', {
        activated: true,
        requiredAccounts: `${dapp},${otherDapp}`,
        broadcast: true,
      }),
      new RegExp(`Cannot activate: these required dApps are not allowlisted: ${otherDapp}`),
    );
    expect(await hcuLimit.getFunction('isAllowlistActivated').staticCall()).to.equal(false);
  });

  it('rejects duplicate accounts and invalid deadlines before broadcasting', async function () {
    await rejects(
      runAllowlistTask('task:addToAllowlist', { accounts: `${dapp},${dapp}`, broadcast: true }),
      /Duplicate addresses/,
    );
    await rejects(
      runAllowlistTask('task:addAllowlister', { account: dapp, deadline: 'tomorrow', broadcast: true }),
      /decimal Unix timestamp/,
    );
    await rejects(
      runAllowlistTask('task:removeAllowlister', { account: `${dapp},${dapp}`, broadcast: true }),
      /exactly one/,
    );
    expect(await hcuLimit.getFunction('isAllowlisted').staticCall(dapp)).to.equal(false);
  });

  it('lets an active allowlister admit a dApp but reserves owner actions for the ACL owner', async function () {
    const allowlister = Wallet.createRandom();
    const previousKey = process.env.TEST_ALLOWLISTER_PRIVATE_KEY;
    process.env.TEST_ALLOWLISTER_PRIVATE_KEY = allowlister.privateKey;
    try {
      await network.provider.send('hardhat_setBalance', [allowlister.address, '0x1000000000000000000']);
      const deadline = BigInt((await ethers.provider.getBlock('latest'))!.timestamp) + 3600n;
      await runAllowlistTask('task:addAllowlister', {
        account: allowlister.address,
        deadline: deadline.toString(),
        broadcast: true,
      });
      await runAllowlistTask('task:addToAllowlist', {
        accounts: dapp,
        broadcast: true,
        signerKeyEnv: 'TEST_ALLOWLISTER_PRIVATE_KEY',
      });
      expect(await hcuLimit.getFunction('isAllowlisted').staticCall(dapp)).to.equal(true);
      await rejects(
        runAllowlistTask('task:setAllowlistActivated', {
          activated: true,
          requiredAccounts: dapp,
          broadcast: true,
          signerKeyEnv: 'TEST_ALLOWLISTER_PRIVATE_KEY',
        }),
        /is not the ACL owner/,
      );
      expect(await hcuLimit.getFunction('isAllowlistActivated').staticCall()).to.equal(false);
    } finally {
      if (previousKey === undefined) delete process.env.TEST_ALLOWLISTER_PRIVATE_KEY;
      else process.env.TEST_ALLOWLISTER_PRIVATE_KEY = previousKey;
    }
  });
});
