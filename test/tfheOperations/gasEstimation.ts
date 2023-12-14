import { fail } from 'assert';
import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { GasEstimation } from '../../types/examples/GasEstimation';
import { OPTIMISTIC_REQUIRES_ENABLED } from '../generated';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

async function deployGasEstimation(): Promise<GasEstimation> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('GasEstimation');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('Gas estimation', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contract = await deployGasEstimation();
    this.contractAddress = await contract.getAddress();
    this.contract = contract;
    const instances = await createInstances(this.contractAddress, ethers, this.signers);
    this.instances = instances;
  });

  it('estimate gas', async function () {
    const empty = await this.contract.empty.estimateGas();
    console.log('empty', empty);
    await [
      'add',
      'sub',
      'mul',
      'div',
      'rem',
      'and',
      'or',
      'xor',
      'shr',
      'shl',
      'eq',
      'ne',
      'ge',
      'gt',
      'le',
      'lt',
      'min',
      'max',
      'not',
      'neg',
      'cmux',
      'decrypt',
      'randEuint',
    ].reduce(async (p, method) => {
      await p;
      for (let i = 8; i <= 32; i *= 2) {
        const methodName = `${method}${i}`;
        const res = await this.contract[methodName].estimateGas();
        // console.log(methodName, res, res - empty);
        console.log(methodName, Math.round((Number(res) - Number(empty)) / 1000) * 1000);
      }
    }, Promise.resolve());
  });
});
