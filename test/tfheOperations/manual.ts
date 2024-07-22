import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHEManualTestSuite } from '../../types/contracts/tests/TFHEManualTestSuite';
import {
  createInstances,
  decrypt4,
  decrypt8,
  decrypt16,
  decrypt32,
  decrypt64,
  decryptAddress,
  decryptBool,
} from '../instance';
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

describe('TFHE manual operations', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contract = await deployTfheManualTestFixture();
    this.contractAddress = await contract.getAddress();
    this.contract = contract;
    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('Select works returning if false', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addBool(false);
    input.add32(3);
    input.add32(4);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_select(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.handles[2],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract.res32());
    expect(res).to.equal(4);
  });

  it('Select works returning if true', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addBool(true);
    input.add32(3);
    input.add32(4);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_select(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.handles[2],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract.res32());
    expect(res).to.equal(3);
  });

  it('Select works for eaddress returning if false', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addBool(false);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    input.addAddress('0x8881f109551bd432803012645ac136ddd64dba72');
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_select_eaddress(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.handles[2],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptAddress(await this.contract.resAdd());
    expect(res).to.equal('0x8881f109551bd432803012645ac136ddd64dba72');
  });

  it('Select works for eaddress returning if true', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addBool(true);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    input.addAddress('0x8881f109551bd432803012645ac136ddd64dba72');
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_select_eaddress(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.handles[2],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptAddress(await this.contract.resAdd());
    expect(res).to.equal('0x8ba1f109551bd432803012645ac136ddd64dba72');
  });

  it('eaddress eq eaddress,eaddress true', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_eq_eaddress_eaddress(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
  });

  it('eaddress eq eaddress,eaddress false', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    input.addAddress('0x9ba1f109551bd432803012645ac136ddd64dba72');
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_eq_eaddress_eaddress(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
  });

  it('eaddress eq scalar eaddress,address true', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x8ba1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_eq_eaddress_address(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
  });

  it('eaddress eq scalar eaddress,address false', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x9aa1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_eq_eaddress_address(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
  });

  it('eaddress eq scalar address,eaddress true', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x8ba1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_eq_address_eaddress(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
  });

  it('eaddress eq scalar address,eaddress false', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x9aa1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_eq_address_eaddress(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
  });

  it('eaddress ne eaddress,eaddress false', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_ne_eaddress_eaddress(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
  });

  it('eaddress ne eaddress,eaddress true', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    input.addAddress('0x9ba1f109551bd432803012645ac136ddd64dba72');
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_ne_eaddress_eaddress(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
  });

  it('eaddress ne scalar eaddress,address false', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x8ba1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_ne_eaddress_address(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
  });

  it('eaddress ne scalar eaddress,address true', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x9aa1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_ne_eaddress_address(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
  });

  it('eaddress ne scalar address,eaddress false', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x8ba1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_ne_address_eaddress(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
  });

  it('eaddress ne scalar address,eaddress true', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x9aa1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_ne_address_eaddress(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
  });

  it('ebool to euint4 casting works with true', async function () {
    const tx = await this.contract.test_ebool_to_euint4_cast(true);
    await tx.wait();
    const res = await decrypt4(await this.contract.res4());
    expect(res).to.equal(1);
  });

  it('ebool to euint4 casting works with false', async function () {
    const tx = await this.contract.test_ebool_to_euint4_cast(false);
    await tx.wait();
    const res = await decrypt4(await this.contract.res4());
    expect(res).to.equal(0);
  });

  it('ebool to euint8 casting works with true', async function () {
    const tx = await this.contract.test_ebool_to_euint8_cast(true);
    await tx.wait();
    const res = await decrypt8(await this.contract.res8());
    expect(res).to.equal(1);
  });

  it('ebool to euint8 casting works with false', async function () {
    const tx = await this.contract.test_ebool_to_euint8_cast(false);
    await tx.wait();
    const res = await decrypt8(await this.contract.res8());
    expect(res).to.equal(0);
  });

  it('ebool to euint16 casting works with true', async function () {
    const tx = await this.contract.test_ebool_to_euint16_cast(true);
    await tx.wait();
    const res = await decrypt16(await this.contract.res16());
    expect(res).to.equal(1);
  });

  it('ebool to euint16 casting works with false', async function () {
    const tx = await this.contract.test_ebool_to_euint16_cast(false);
    await tx.wait();
    const res = await decrypt16(await this.contract.res16());
    expect(res).to.equal(0);
  });

  it('ebool to euint32 casting works with true', async function () {
    const tx = await this.contract.test_ebool_to_euint32_cast(true);
    await tx.wait();
    const res = await decrypt32(await this.contract.res32());
    expect(res).to.equal(1);
  });

  it('ebool to euint32 casting works with false', async function () {
    const tx = await this.contract.test_ebool_to_euint32_cast(false);
    await tx.wait();
    const res = await decrypt32(await this.contract.res32());
    expect(res).to.equal(0);
  });

  it('ebool to euint64 casting works with true', async function () {
    const tx = await this.contract.test_ebool_to_euint64_cast(true);
    await tx.wait();
    const res = await decrypt64(await this.contract.res64());
    expect(res).to.equal(1);
  });

  it('ebool to euint64 casting works with false', async function () {
    const tx = await this.contract.test_ebool_to_euint64_cast(false);
    await tx.wait();
    const res = await decrypt64(await this.contract.res64());
    expect(res).to.equal(0);
  });

  it('ebool not for false is true', async function () {
    const tx = await this.contract.test_ebool_not(false);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
  });

  it('ebool not for true is false', async function () {
    const tx = await this.contract.test_ebool_not(true);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
  });

  it('ebool and', async function () {
    const tx = await this.contract.test_ebool_and(false, false);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);

    const tx2 = await this.contract.test_ebool_and(false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(false);

    const tx3 = await this.contract.test_ebool_and(true, false);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(false);

    const tx4 = await this.contract.test_ebool_and(true, true);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(true);
  });

  it('ebool or', async function () {
    const tx = await this.contract.test_ebool_or(false, false);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);

    const tx2 = await this.contract.test_ebool_or(false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);

    const tx3 = await this.contract.test_ebool_or(true, false);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(true);

    const tx4 = await this.contract.test_ebool_or(true, true);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(true);
  });

  it('ebool xor', async function () {
    const tx = await this.contract.test_ebool_xor(false, false);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);

    const tx2 = await this.contract.test_ebool_xor(false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);

    const tx3 = await this.contract.test_ebool_xor(true, false);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(true);

    const tx4 = await this.contract.test_ebool_xor(true, true);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(false);
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

    const res = await this.contract.resb();
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

    const res = await this.contract.resb();
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

    const res = await this.contract.resb();
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

    const res = await this.contract.resb();
    const decRes = await decryptBool(res);
    expect(decRes).to.equal(false);
  });
});
