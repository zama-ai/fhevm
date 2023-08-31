import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners } from '../signers';
import { deployTfheTestFixture } from './tfheOperations.fixture';

describe('TFHE operations', function () {
  before(async function () {
    this.signers = await getSigners();

    const contract = await deployTfheTestFixture();
    this.contractAddress = await contract.getAddress();
    this.contract = contract;
    const instances = await createInstances(this.contractAddress, ethers, this.signers);
    this.instances = instances;
  });

  it('should work for addition', async function () {
    const res = await this.contract.addUint8(3, 4);
    expect(res).to.equal(7);
  });

  it('should work for multiplication', async function () {
    const res = await this.contract.mulUint8(3, 4);
    expect(res).to.equal(12);
  });
});
