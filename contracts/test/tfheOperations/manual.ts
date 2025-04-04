import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHEManualTestSuite } from '../../types/contracts/tests/TFHEManualTestSuite';
import {
  createInstances,
  decrypt8,
  decrypt16,
  decrypt32,
  decrypt64,
  decrypt128,
  decrypt256,
  decryptAddress,
  decryptBool,
  decryptEbytes64,
  decryptEbytes128,
  decryptEbytes256,
} from '../instance';
import { getSigners, initSigners } from '../signers';
import { bigIntToBytes256 } from '../utils';

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

  it('Select ebool', async function () {
    const tx = await this.contract.test_select_ebool(true, false, true);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
    const tx2 = await this.contract.test_select_ebool(false, false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);
  });

  it('Select ebytes64', async function () {
    const tx = await this.contract.test_select_ebytes64(
      true,
      '0x6798aa6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
      '0x11',
    );
    await tx.wait();
    const res = await decryptEbytes64(await this.contract.resB64());
    expect(res).to.equal(
      ethers.toBeHex(BigInt('0x6798aa6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb'), 64),
    );
    const tx2 = await this.contract.test_select_ebytes64(false, '0x42', '0xaaaaaaaa');
    await tx2.wait();
    const res2 = await decryptEbytes64(await this.contract.resB64());
    expect(res2).to.equal(ethers.toBeHex(BigInt('0xaaaaaaaa'), 64));
  });

  it('Select ebytes128', async function () {
    const tx = await this.contract.test_select_ebytes128(
      true,
      '0x6798aa6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
      '0x11',
    );
    await tx.wait();
    const res = await decryptEbytes128(await this.contract.resB128());
    expect(res).to.equal(
      ethers.toBeHex(BigInt('0x6798aa6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb'), 128),
    );
    const tx2 = await this.contract.test_select_ebytes128(false, '0x42', '0xaaaaaaaa');
    await tx2.wait();
    const res2 = await decryptEbytes128(await this.contract.resB128());
    expect(res2).to.equal(ethers.toBeHex(BigInt('0xaaaaaaaa'), 128));
  });

  it('Select ebytes256', async function () {
    const tx = await this.contract.test_select_ebytes256(
      true,
      '0x6798aa6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
      '0x11',
    );
    await tx.wait();
    const res = await decryptEbytes256(await this.contract.resB256());
    expect(res).to.equal(
      ethers.toBeHex(BigInt('0x6798aa6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb'), 256),
    );
    const tx2 = await this.contract.test_select_ebytes256(false, '0x428899', '0xaaaaaabb');
    await tx2.wait();
    const res2 = await decryptEbytes256(await this.contract.resB256());
    expect(res2).to.equal(ethers.toBeHex(BigInt('0xaaaaaabb'), 256));
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

  it('ebool eq ebool', async function () {
    const tx = await this.contract.eqEbool(true, true);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
    const tx2 = await this.contract.eqEbool(false, false);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);
    const tx3 = await this.contract.eqEbool(false, true);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(false);
    const tx4 = await this.contract.eqEbool(true, false);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(false);
  });

  it('ebool eq ebool - ScalarL', async function () {
    const tx = await this.contract.eqEboolScalarL(true, true);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
    const tx2 = await this.contract.eqEboolScalarL(false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(false);
    const tx3 = await this.contract.eqEboolScalarL(false, false);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(true);
    const tx4 = await this.contract.eqEboolScalarL(true, false);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(false);
  });

  it('ebool eq ebool - ScalarR', async function () {
    const tx = await this.contract.eqEboolScalarL(false, false);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
    const tx2 = await this.contract.eqEboolScalarL(false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(false);
    const tx3 = await this.contract.eqEboolScalarL(true, true);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(true);
    const tx4 = await this.contract.eqEboolScalarL(true, false);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(false);
  });

  it('ebool ne ebool', async function () {
    const tx = await this.contract.neEbool(true, true);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
    const tx2 = await this.contract.neEbool(false, false);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(false);
    const tx3 = await this.contract.neEbool(false, true);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(true);
    const tx4 = await this.contract.neEbool(true, false);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(true);
  });

  it('ebool ne ebool - ScalarL', async function () {
    const tx = await this.contract.neEboolScalarL(true, true);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
    const tx2 = await this.contract.neEboolScalarL(false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);
    const tx3 = await this.contract.neEboolScalarL(false, false);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(false);
    const tx4 = await this.contract.neEboolScalarL(true, false);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(true);
  });

  it('ebool ne ebool - ScalarR', async function () {
    const tx = await this.contract.neEboolScalarL(false, false);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
    const tx2 = await this.contract.neEboolScalarL(false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);
    const tx3 = await this.contract.neEboolScalarL(true, true);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(false);
    const tx4 = await this.contract.neEboolScalarL(true, false);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(true);
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

  it('ebool to euint128 casting works with true', async function () {
    const tx = await this.contract.test_ebool_to_euint128_cast(true);
    await tx.wait();
    const res = await decrypt128(await this.contract.res128());
    expect(res).to.equal(1);
  });

  it('ebool to euint128 casting works with false', async function () {
    const tx = await this.contract.test_ebool_to_euint128_cast(false);
    await tx.wait();
    const res = await decrypt128(await this.contract.res128());
    expect(res).to.equal(0);
  });

  it('ebool to euint256 casting works with true', async function () {
    const tx = await this.contract.test_ebool_to_euint256_cast(true);
    await tx.wait();
    const res = await decrypt256(await this.contract.res256());
    expect(res).to.equal(1);
  });

  it('ebool to euint256 casting works with false', async function () {
    const tx = await this.contract.test_ebool_to_euint256_cast(false);
    await tx.wait();
    const res = await decrypt256(await this.contract.res256());
    expect(res).to.equal(0);
  });

  it('euint128 to euint8 casting works', async function () {
    const tx = await this.contract.test_euint128_to_euint8_cast(7668756464674969496544n);
    await tx.wait();
    const res = await decrypt8(await this.contract.res8());
    expect(res).to.equal(224n);
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

  it('ebool xor scalarL', async function () {
    const tx = await this.contract.test_ebool_xor_scalarL(false, false);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);

    const tx2 = await this.contract.test_ebool_xor_scalarL(false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);

    const tx3 = await this.contract.test_ebool_xor_scalarL(true, false);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(true);

    const tx4 = await this.contract.test_ebool_xor_scalarL(true, true);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(false);
  });

  it('ebool xor scalarR', async function () {
    const tx = await this.contract.test_ebool_xor_scalarR(false, false);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);

    const tx2 = await this.contract.test_ebool_xor_scalarR(false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);

    const tx3 = await this.contract.test_ebool_xor_scalarR(true, false);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(true);

    const tx4 = await this.contract.test_ebool_xor_scalarR(true, true);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(false);
  });

  it('ebool or scalarL', async function () {
    const tx = await this.contract.test_ebool_or_scalarL(false, false);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);

    const tx2 = await this.contract.test_ebool_or_scalarL(false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);

    const tx3 = await this.contract.test_ebool_or_scalarL(true, false);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(true);

    const tx4 = await this.contract.test_ebool_or_scalarL(true, true);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(true);
  });

  it('ebool or scalarR', async function () {
    const tx = await this.contract.test_ebool_or_scalarR(false, false);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);

    const tx2 = await this.contract.test_ebool_or_scalarR(false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);

    const tx3 = await this.contract.test_ebool_or_scalarR(true, false);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(true);

    const tx4 = await this.contract.test_ebool_or_scalarR(true, true);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(true);
  });

  it('ebool and scalarL', async function () {
    const tx = await this.contract.test_ebool_and_scalarL(false, false);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);

    const tx2 = await this.contract.test_ebool_and_scalarL(false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(false);

    const tx3 = await this.contract.test_ebool_and_scalarL(true, false);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(false);

    const tx4 = await this.contract.test_ebool_and_scalarL(true, true);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(true);
  });

  it('ebool and scalarR', async function () {
    const tx = await this.contract.test_ebool_and_scalarR(false, false);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);

    const tx2 = await this.contract.test_ebool_and_scalarR(false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(false);

    const tx3 = await this.contract.test_ebool_and_scalarR(true, false);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(false);

    const tx4 = await this.contract.test_ebool_and_scalarR(true, true);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(true);
  });

  it('eq ebytes256,ebytes256 true', async function () {
    const inputAliceA = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAliceA.addBytes256(bigIntToBytes256(18446744073709550022n));
    const encryptedAmountA = await inputAliceA.encrypt();

    const inputAliceB = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAliceB.addBytes256(bigIntToBytes256(18446744073709550022n));
    const encryptedAmountB = await inputAliceB.encrypt();

    const tx = await this.contract.eqEbytes256(
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
    inputAliceA.addBytes256(bigIntToBytes256(18446744073709550022n));
    const encryptedAmountA = await inputAliceA.encrypt();

    const inputAliceB = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAliceB.addBytes256(bigIntToBytes256(18446744073709550021n));
    const encryptedAmountB = await inputAliceB.encrypt();

    const tx = await this.contract.eqEbytes256(
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
    inputAliceA.addBytes256(bigIntToBytes256(18446744073709550022n));
    const encryptedAmountA = await inputAliceA.encrypt();

    const inputAliceB = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAliceB.addBytes256(bigIntToBytes256(18446744073709550021n));
    const encryptedAmountB = await inputAliceB.encrypt();

    const tx = await this.contract.neEbytes256(
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
    inputAliceA.addBytes256(bigIntToBytes256(184467440184467440184467440184467440n));
    const encryptedAmountA = await inputAliceA.encrypt();

    const inputAliceB = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAliceB.addBytes256(bigIntToBytes256(184467440184467440184467440184467440n));
    const encryptedAmountB = await inputAliceB.encrypt();

    const tx = await this.contract.neEbytes256(
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

  it('ebytes64 eq ebytes64', async function () {
    const tx = await this.contract.eqEbytes64(
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
    const tx2 = await this.contract.eqEbytes64('0x1100', '0x0011');
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(false);
  });

  it('ebytes64 eq ebytes64 - scalarL', async function () {
    const tx = await this.contract.eqEbytes64ScalarL(
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbaaa',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
    const tx2 = await this.contract.eqEbytes64ScalarL('0x1100', '0x1100');
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);
  });

  it('ebytes64 eq ebytes64 - scalarR', async function () {
    const tx = await this.contract.eqEbytes64ScalarR(
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbaaa',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
    const tx2 = await this.contract.eqEbytes64ScalarR('0x1100', '0x1100');
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);
  });

  it('ebytes64 ne ebytes64', async function () {
    const tx = await this.contract.neEbytes64(
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
    const tx2 = await this.contract.neEbytes64('0x1100', '0x0011');
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);
  });

  it('ebytes64 ne ebytes64 - scalarL', async function () {
    const tx = await this.contract.neEbytes64ScalarL(
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbaaa',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
    const tx2 = await this.contract.neEbytes64ScalarL('0x1100', '0x1100');
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(false);
  });

  it('ebytes64 ne ebytes64 - scalarR', async function () {
    const tx = await this.contract.neEbytes64ScalarR(
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbaaa',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
    const tx2 = await this.contract.neEbytes64ScalarR('0x1100', '0x1100');
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(false);
  });

  it('ebytes128 eq ebytes128', async function () {
    const tx = await this.contract.eqEbytes128(
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabbd4fdd06bd752b24ffb9f307805c4e698bf10aed0a47a103e5c1ade64bd31eb73',
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabbd4fdd06bd752b24ffb9f307805c4e698bf10aed0a47a103e5c1ade64bd31eb73',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
    const tx2 = await this.contract.eqEbytes128(
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabbd4fdd06bd752b24ffb9f307805c4e698bf10aed0a47a103e5c1ade64bd31eb73',
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabbd4fdd06bd752b24ffb9f307805c4e698bf10aed0a47a103e5c1ade64bd31eb71',
    );
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(false);
  });

  it('ebytes128 eq ebytes128 - scalarL', async function () {
    const tx = await this.contract.eqEbytes128ScalarL(
      '0x6d4b2086ba8e3d2104fbf4a8dfe9679d6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
      '0x6d4b2086ba8e3d2104fbf4a8dfe9679d6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbaaa',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
    const tx2 = await this.contract.eqEbytes128ScalarL('0x1100', '0x1100');
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);
  });

  it('ebytes128 eq ebytes128 - scalarR', async function () {
    const tx = await this.contract.eqEbytes128ScalarR(
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbaaa',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
    const tx2 = await this.contract.eqEbytes128ScalarR('0x1100', '0x1100');
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);
  });

  it('ebytes128 ne ebytes128', async function () {
    const tx = await this.contract.neEbytes128(
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabbd4fdd06bd752b24ffb9f307805c4e698bf10aed0a47a103e5c1ade64bd31eb73',
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabbd4fdd06bd752b24ffb9f307805c4e698bf10aed0a47a103e5c1ade64bd31eb73',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
    const tx2 = await this.contract.neEbytes128(
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabbd4fdd06bd752b24ffb9f307805c4e698bf10aed0a47a103e5c1ade64bd31eb73',
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabbd4fdd06bd752b24ffb9f307805c4e698bf10aed0a47a103e5c1ade64bd31eb71',
    );
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);
  });

  it('ebytes128 ne ebytes128 - scalarL', async function () {
    const tx = await this.contract.neEbytes128ScalarL(
      '0x6d4b2086ba8e3d2104fbf4a8dfe9679d6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
      '0x6d4b2086ba8e3d2104fbf4a8dfe9679d6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbaaa',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
    const tx2 = await this.contract.neEbytes128ScalarL('0x1100', '0x1100');
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(false);
  });

  it('ebytes128 ne ebytes128 - scalarR', async function () {
    const tx = await this.contract.neEbytes128ScalarR(
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbaaa',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
    const tx2 = await this.contract.neEbytes128ScalarR('0x1100', '0x1100');
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(false);
  });

  it('ebytes256 eq ebytes256 - scalarL', async function () {
    const tx = await this.contract.eqEbytes256ScalarL(
      '0x6d4b2086ba8e3d2104fbf4a8dfe9679d6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
      '0x6d4b2086ba8e3d2104fbf4a8dfe9679d6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbaaa',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
    const tx2 = await this.contract.eqEbytes256ScalarL('0x1100', '0x1100');
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);
  });

  it('ebytes256 eq ebytes256 - scalarR', async function () {
    const tx = await this.contract.eqEbytes256ScalarR(
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabbaa',
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbaaaaa',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
    const tx2 = await this.contract.eqEbytes256ScalarR('0x1100', '0x1100');
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);
  });

  it('ebytes256 ne ebytes256 - scalarL', async function () {
    const tx = await this.contract.neEbytes256ScalarL(
      '0x6d4b2086ba8e3d2104fbf4a8dfe9679d6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabb',
      '0x6d4b2086ba8e3d2104fbf4a8dfe9679d6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbaaa',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
    const tx2 = await this.contract.neEbytes256ScalarL('0x1100', '0x1100');
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(false);
  });

  it('ebytes256 ne ebytes256 - scalarR', async function () {
    const tx = await this.contract.neEbytes256ScalarR(
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbabbaa',
      '0x6bb8166128b0e7a16f60dc255c953288d03107895b0904ea18f7a242bf335fbaaaaa',
    );
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(true);
    const tx2 = await this.contract.neEbytes256ScalarR('0x1100', '0x1100');
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(false);
  });

  it('ebool ne ebool', async function () {
    const tx = await this.contract.neEbool(true, true);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
    const tx2 = await this.contract.neEbool(false, false);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(false);
    const tx3 = await this.contract.neEbool(false, true);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(true);
    const tx4 = await this.contract.neEbool(true, false);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(true);
  });

  it('ebool ne ebool - ScalarL', async function () {
    const tx = await this.contract.neEboolScalarL(true, true);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
    const tx2 = await this.contract.neEboolScalarL(false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);
    const tx3 = await this.contract.neEboolScalarL(false, false);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(false);
    const tx4 = await this.contract.neEboolScalarL(true, false);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(true);
  });

  it('ebool ne ebool - ScalarR', async function () {
    const tx = await this.contract.neEboolScalarL(false, false);
    await tx.wait();
    const res = await decryptBool(await this.contract.resb());
    expect(res).to.equal(false);
    const tx2 = await this.contract.neEboolScalarL(false, true);
    await tx2.wait();
    const res2 = await decryptBool(await this.contract.resb());
    expect(res2).to.equal(true);
    const tx3 = await this.contract.neEboolScalarL(true, true);
    await tx3.wait();
    const res3 = await decryptBool(await this.contract.resb());
    expect(res3).to.equal(false);
    const tx4 = await this.contract.neEboolScalarL(true, false);
    await tx4.wait();
    const res4 = await decryptBool(await this.contract.resb());
    expect(res4).to.equal(true);
  });
});
