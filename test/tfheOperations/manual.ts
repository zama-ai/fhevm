import { fail } from 'assert';
import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHEManualTestSuite } from '../../types/contracts/tests/TFHEManualTestSuite';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

async function deployTfheManualTestFixture(): Promise<TFHEManualTestSuite> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHEManualTestSuite');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('TFHE manual operations', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contract = await deployTfheManualTestFixture();
    this.contractAddress = await contract.getAddress();
    this.contract = contract;
    const instances = await createInstances(this.contractAddress, ethers, this.signers);
    this.instances = instances;
  });

  it('Cmux works returning if false', async function () {
    const res = await this.contract.test_cmux(
      this.instances.alice.encrypt8(0),
      this.instances.alice.encrypt32(3),
      this.instances.alice.encrypt32(4),
    );
    expect(res).to.equal(4);
  });

  it('Cmux works returning if true', async function () {
    const res = await this.contract.test_cmux(
      this.instances.alice.encrypt8(1),
      this.instances.alice.encrypt32(3),
      this.instances.alice.encrypt32(4),
    );
    expect(res).to.equal(3);
  });

  it('ebool to euint16 casting works with true', async function () {
    const res = await this.contract.test_ebool_to_euint16_cast(true);
    expect(res).to.equal(1);
  });

  it('ebool to euint16 casting works with false', async function () {
    const res = await this.contract.test_ebool_to_euint16_cast(false);
    expect(res).to.equal(0);
  });

  it('ebool to euint32 casting works with true', async function () {
    const res = await this.contract.test_ebool_to_euint32_cast(true);
    expect(res).to.equal(1);
  });

  it('ebool to euint32 casting works with false', async function () {
    const res = await this.contract.test_ebool_to_euint32_cast(false);
    expect(res).to.equal(0);
  });

  it('optimistic require with true succeeds', async function () {
    await this.contract.test_opt_req(true);
  });

  it('optimistic require with false fails', async function () {
    try {
      await this.contract.test_opt_req(false);
      fail('This should fail');
    } catch (e: any) {
      expect(e.message).to.contain('execution reverted');
    }
  });

  it('stateful optimistic require with true succeeds', async function () {
    const res = await this.contract.test_opt_req_stateful(true);
    const receipt = await res.wait();
    expect(receipt.status).to.equal(1);
  });

  it('stateful optimistic require with false fails', async function () {
    const res = await this.contract.test_opt_req_stateful(false);
    const receipt = await res.wait();
    expect(receipt.status).to.not.equal(1);
  });
});
