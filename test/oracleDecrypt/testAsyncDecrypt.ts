import { expect } from 'chai';
import { ethers } from 'hardhat';

import { asyncDecrypt, awaitAllDecryptionResults } from '../asyncDecrypt';
import { getSigners, initSigners } from '../signers';
import { createInstances } from '../instance';

describe('TestAsyncDecrypt', function () {
  before(async function () {
    await asyncDecrypt();
    await initSigners(3);
    this.signers = await getSigners();
    const contractFactory = await ethers.getContractFactory('TestAsyncDecrypt');
    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    this.contractAddress = await this.contract.getAddress();
    const instances = await createInstances(this.contractAddress, ethers, this.signers);
    this.instances = instances;
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
    const tx2 = await this.contract.connect(this.signers.carol).requestUint8({ gasLimit: 5_000_000 });
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint8();
    expect(y).to.equal(42);
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

  it('test async decrypt uint160', async function () {
    const input = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const tx2 = await this.contract.connect(this.signers.carol).requestUint160(input, { gasLimit: 500_000 });
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint160();
    expect(y).to.equal(797161134358056856230896843146392277790002887282n);
  });
});
