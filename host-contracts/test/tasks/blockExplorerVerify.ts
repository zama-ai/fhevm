import { expect } from 'chai';

import { verifyContract } from '../../tasks/blockExplorerVerify';
import type { RunTaskFunction } from 'hardhat/types';

describe('verifyContract', function () {
  it('forwards the address to the verify:verify subtask on success', async function () {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const calls: Array<{ name: string; args: any }> = [];
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const fakeRun = async (name: string, args: any) => {
      calls.push({ name, args });
    };

    await verifyContract(fakeRun as RunTaskFunction, '0xdeadbeef');

    expect(calls).to.have.length(1);
    expect(calls[0].name).to.eq('verify:verify');
    expect(calls[0].args.address).to.eq('0xdeadbeef');
    expect(calls[0].args.constructorArguments).to.deep.eq([]);
  });

  it('swallows the proxy "Already Verified" error (hardhat-verify casing)', async function () {
    const fakeRun = async () => {
      throw new Error('Failed to verify ERC1967Proxy contract at 0x1dD3c231: Already Verified');
    };

    // Should resolve without throwing.
    await verifyContract(fakeRun as RunTaskFunction, '0x1dD3c231');
  });

  it('swallows a lowercase "already verified" error (implementation re-verify)', async function () {
    const fakeRun = async () => {
      throw new Error('Contract source code already verified');
    };

    await verifyContract(fakeRun as RunTaskFunction, '0xabc');
  });

  it('rethrows a genuine verification failure', async function () {
    const fakeRun = async () => {
      throw new Error('Invalid API Key');
    };

    let threw = false;
    try {
      await verifyContract(fakeRun as RunTaskFunction, '0xabc');
    } catch (error) {
      threw = true;
      expect(String(error)).to.include('Invalid API Key');
    }
    expect(threw, 'expected a genuine failure to be rethrown').to.eq(true);
  });
});
