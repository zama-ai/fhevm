import { fail } from 'assert';
import { expect } from 'chai';
import { writeFile } from 'fs';
import { ethers } from 'hardhat';

import type { TFHEManualTestSuite } from '../../types/contracts/tests/TFHEManualTestSuite';
import { OPTIMISTIC_REQUIRES_ENABLED } from '../generated';
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

  it('Select works returning if false', async function () {
    const res = await this.contract.test_select(
      this.instances.alice.encryptBool(false),
      this.instances.alice.encrypt32(3),
      this.instances.alice.encrypt32(4),
    );
    expect(res).to.equal(4);
  });

  it('Select works returning if true', async function () {
    const res = await this.contract.test_select(
      this.instances.alice.encryptBool(true),
      this.instances.alice.encrypt32(4),
    );
    expect(res).to.equal(3);
  });

  it('eaddress dec', async function () {
    const input = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const res = await this.contract.test_eaddress_decrypt(input);
    expect(res).to.equal('0x8ba1f109551bD432803012645Ac136ddd64DBA72');
  });

  it('eaddress eq eaddress,eaddress true', async function () {
    const a = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const res = await this.contract.test_eq_eaddress_eaddress(a, b);
    expect(res).to.equal(true);
  });

  it('eaddress eq eaddress,eaddress false', async function () {
    const a = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = this.instances.alice.encryptAddress('0x9ba1f109551bd432803012645ac136ddd64dba72');
    const res = await this.contract.test_eq_eaddress_eaddress(a, b);
    expect(res).to.equal(false);
  });

  it('eaddress eq scalar eaddress,address true', async function () {
    const a = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x8ba1f109551bd432803012645ac136ddd64dba72';
    const res = await this.contract.eq_eaddress_address(a, b);
    expect(res).to.equal(true);
  });

  it('eaddress eq scalar eaddress,address false', async function () {
    const a = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x9aa1f109551bd432803012645ac136ddd64dba72';
    const res = await this.contract.eq_eaddress_address(a, b);
    expect(res).to.equal(false);
  });

  it('eaddress eq scalar address,eaddress true', async function () {
    const a = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x8ba1f109551bd432803012645ac136ddd64dba72';
    const res = await this.contract.eq_address_eaddress(b, a);
    expect(res).to.equal(true);
  });

  it('eaddress eq scalar address,eaddress false', async function () {
    const a = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x9aa1f109551bd432803012645ac136ddd64dba72';
    const res = await this.contract.eq_address_eaddress(b, a);
    expect(res).to.equal(false);
  });

  it('eaddress ne eaddress,eaddress false', async function () {
    const a = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const res = await this.contract.test_ne_eaddress_eaddress(a, b);
    expect(res).to.equal(false);
  });

  it('eaddress ne eaddress,eaddress true', async function () {
    const a = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = this.instances.alice.encryptAddress('0x9ba1f109551bd432803012645ac136ddd64dba72');
    const res = await this.contract.test_ne_eaddress_eaddress(a, b);
    expect(res).to.equal(true);
  });

  it('eaddress ne scalar eaddress,address true', async function () {
    const a = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x9aa1f109551bd432803012645ac136ddd64dba72';
    const res = await this.contract.ne_eaddress_address(a, b);
    expect(res).to.equal(true);
  });

  it('eaddress ne scalar eaddress,address false', async function () {
    const a = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x8ba1f109551bd432803012645ac136ddd64dba72';
    const res = await this.contract.ne_eaddress_address(a, b);
    expect(res).to.equal(false);
  });

  it('eaddress ne scalar address,eaddress false', async function () {
    const a = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x8ba1f109551bd432803012645ac136ddd64dba72';
    const res = await this.contract.ne_address_eaddress(b, a);
    expect(res).to.equal(false);
  });

  it('eaddress ne scalar address,eaddress true', async function () {
    const a = this.instances.alice.encryptAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x9aa1f109551bd432803012645ac136ddd64dba72';
    const res = await this.contract.ne_address_eaddress(b, a);
    expect(res).to.equal(true);
  });

  it('ebool to euint4 casting works with true', async function () {
    const res = await this.contract.test_ebool_to_euint4_cast(true);
    expect(res).to.equal(1);
  });

  it('ebool to euint4 casting works with false', async function () {
    const res = await this.contract.test_ebool_to_euint4_cast(false);
    expect(res).to.equal(0);
  });

  it('ebool to euint8 casting works with true', async function () {
    const res = await this.contract.test_ebool_to_euint8_cast(true);
    expect(res).to.equal(1);
  });

  it('ebool to euint8 casting works with false', async function () {
    const res = await this.contract.test_ebool_to_euint8_cast(false);
    expect(res).to.equal(0);
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

  it('ebool to euint64 casting works with true', async function () {
    const res = await this.contract.test_ebool_to_euint64_cast(true);
    expect(res).to.equal(1);
  });

  it('ebool to euint64 casting works with false', async function () {
    const res = await this.contract.test_ebool_to_euint64_cast(false);
    expect(res).to.equal(0);
  });

  it('ebool not for false is true', async function () {
    const res = await this.contract.test_ebool_not(false);
    expect(res).to.equal(true);
  });

  it('ebool not for true is false', async function () {
    const res = await this.contract.test_ebool_not(true);
    expect(res).to.equal(false);
  });

  it('ebool and', async function () {
    expect(await this.contract.test_ebool_and(false, false)).to.equal(false);
    expect(await this.contract.test_ebool_and(false, true)).to.equal(false);
    expect(await this.contract.test_ebool_and(true, false)).to.equal(false);
    expect(await this.contract.test_ebool_and(true, true)).to.equal(true);
  });

  it('ebool or', async function () {
    expect(await this.contract.test_ebool_or(false, false)).to.equal(false);
    expect(await this.contract.test_ebool_or(false, true)).to.equal(true);
    expect(await this.contract.test_ebool_or(true, false)).to.equal(true);
    expect(await this.contract.test_ebool_or(true, true)).to.equal(true);
  });

  it('ebool xor', async function () {
    expect(await this.contract.test_ebool_xor(false, false)).to.equal(false);
    expect(await this.contract.test_ebool_xor(false, true)).to.equal(true);
    expect(await this.contract.test_ebool_xor(true, false)).to.equal(true);
    expect(await this.contract.test_ebool_xor(true, true)).to.equal(false);
  });
});
