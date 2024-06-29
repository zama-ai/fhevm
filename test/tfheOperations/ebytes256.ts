import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHEManualTestSuite } from '../../types/contracts/tests/TFHEManualTestSuite';
import { createInstances, decryptBool } from '../instance';
import { getSigners, initSigners } from '../signers';
import { bigIntToBytes } from '../utils';

async function deployTfheManualTestFixture(): Promise<TFHEManualTestSuite> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHEManualTestSuite');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('Ebytes256 operations', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contract = await deployTfheManualTestFixture();
    this.contractAddress = await contract.getAddress();
    this.contract = contract;
    this.instances = await createInstances(this.signers);
  });

  it('eq ebytes256,ebytes256 true', async function () {
    const inputAliceA = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAliceA.addBytes256(bigIntToBytes(18446744073709550022n));
    const encryptedAmountA = inputAliceA.encrypt();

    const inputAliceB = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAliceB.addBytes256(bigIntToBytes(18446744073709550022n));
    const encryptedAmountB = inputAliceB.encrypt();

    const tx = await await this.contract.eqEbytes256(
      encryptedAmountA.handles[0],
      encryptedAmountA.inputProof,
      encryptedAmountB.handles[0],
      encryptedAmountB.inputProof,
    );
    await tx.wait();

    const res = await this.contract.res();
    const decRes = await decryptBool(res);
    expect(decRes).to.equal(true);
  });

  it('eq ebytes256,ebytes256 false', async function () {
    const inputAliceA = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAliceA.addBytes256(bigIntToBytes(18446744073709550022n));
    const encryptedAmountA = inputAliceA.encrypt();

    const inputAliceB = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAliceB.addBytes256(bigIntToBytes(18446744073709550021n));
    const encryptedAmountB = inputAliceB.encrypt();

    const tx = await await this.contract.eqEbytes256(
      encryptedAmountA.handles[0],
      encryptedAmountA.inputProof,
      encryptedAmountB.handles[0],
      encryptedAmountB.inputProof,
    );
    await tx.wait();

    const res = await this.contract.res();
    const decRes = await decryptBool(res);
    expect(decRes).to.equal(false);
  });

  it('ne ebytes256,ebytes256 true', async function () {
    const inputAliceA = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAliceA.addBytes256(bigIntToBytes(18446744073709550022n));
    const encryptedAmountA = inputAliceA.encrypt();

    const inputAliceB = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAliceB.addBytes256(bigIntToBytes(18446744073709550021n));
    const encryptedAmountB = inputAliceB.encrypt();

    const tx = await await this.contract.neEbytes256(
      encryptedAmountA.handles[0],
      encryptedAmountA.inputProof,
      encryptedAmountB.handles[0],
      encryptedAmountB.inputProof,
    );
    await tx.wait();

    const res = await this.contract.res();
    const decRes = await decryptBool(res);
    expect(decRes).to.equal(true);
  });

  it('ne ebytes256,ebytes256 false', async function () {
    const inputAliceA = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAliceA.addBytes256(bigIntToBytes(184467440184467440184467440184467440n));
    const encryptedAmountA = inputAliceA.encrypt();

    const inputAliceB = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAliceB.addBytes256(bigIntToBytes(184467440184467440184467440184467440n));
    const encryptedAmountB = inputAliceB.encrypt();

    const tx = await await this.contract.neEbytes256(
      encryptedAmountA.handles[0],
      encryptedAmountA.inputProof,
      encryptedAmountB.handles[0],
      encryptedAmountB.inputProof,
    );
    await tx.wait();

    const res = await this.contract.res();
    const decRes = await decryptBool(res);
    expect(decRes).to.equal(false);
  });
});
