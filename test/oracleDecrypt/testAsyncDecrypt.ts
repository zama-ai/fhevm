import { expect } from 'chai';
import { ethers } from 'hardhat';

import { asyncDecrypt, awaitAllDecryptionResults } from '../asyncDecrypt';
import { getSigners, initSigners } from '../signers';

describe('TestAsyncDecrypt', function () {
  before(async function () {
    await initSigners(3);
    this.signers = await getSigners();
    await asyncDecrypt();
  });

  beforeEach(async function () {
    const contractFactory = await ethers.getContractFactory('TestAsyncDecrypt');
    this.contract = await contractFactory.connect(this.signers.alice).deploy();
  });

  it('test async decrypt bool', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestBool({ gasLimit: 500_000 });
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yBool();
    expect(y).to.equal(true);
  });

  it('test async decrypt uint4', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint4({ gasLimit: 500_000 });
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint4();
    expect(y).to.equal(4);
  });

  it('test async decrypt uint8', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint8({ gasLimit: 500_000 });
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint8();
    expect(y).to.equal(8);
  });

  it('test async decrypt uint16', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint16({ gasLimit: 500_000 });
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint16();
    expect(y).to.equal(16);
  });

  it('test async decrypt uint32', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint32(5, 15, { gasLimit: 500_000 });
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint32();
    expect(y).to.equal(52); // 5+15+32
  });

  it('test async decrypt uint64', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint64({ gasLimit: 500_000 });
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint64();
    expect(y).to.equal(64);
  });
});
