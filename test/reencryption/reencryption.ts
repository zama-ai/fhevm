import { expect } from 'chai';
import { ethers, network } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { bigIntToBytes } from '../utils';

describe('Reencryption', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    const contractFactory = await ethers.getContractFactory('Reencrypt');

    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    this.contractAddress = await this.contract.getAddress();
    this.instances = await createInstances(this.signers);

    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.addBytes256(
      bigIntToBytes(184467440737095500228978978978978970980978908978978907890778907089780970897890n),
    );
    const encryptedAmount = inputAlice.encrypt();
    const tx = await this.contract.setEBytes256(encryptedAmount.handles[0], encryptedAmount.inputProof, {
      gasLimit: 5_000_000,
    });
    await tx.wait();
  });

  it('test reencrypt ebool', async function () {
    const handle = await this.contract.xBool();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const eip712 = this.instances.alice.createEIP712(publicKey, this.contractAddress);
    const signature = await this.signers.alice.signTypedData(
      eip712.domain,
      { Reencrypt: eip712.types.Reencrypt },
      eip712.message,
    );
    const decryptedValue = await this.instances.alice.reencrypt(
      handle,
      privateKey,
      publicKey,
      signature.replace('0x', ''),
      this.contractAddress,
      this.signers.alice.address,
    );
    expect(decryptedValue).to.equal(1);
  });

  it('test reencrypt euint4', async function () {
    const handle = await this.contract.xUint4();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const eip712 = this.instances.alice.createEIP712(publicKey, this.contractAddress);
    const signature = await this.signers.alice.signTypedData(
      eip712.domain,
      { Reencrypt: eip712.types.Reencrypt },
      eip712.message,
    );
    const decryptedValue = await this.instances.alice.reencrypt(
      handle,
      privateKey,
      publicKey,
      signature.replace('0x', ''),
      this.contractAddress,
      this.signers.alice.address,
    );
    expect(decryptedValue).to.equal(4);
  });

  it('test reencrypt euint8', async function () {
    const handle = await this.contract.xUint8();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const eip712 = this.instances.alice.createEIP712(publicKey, this.contractAddress);
    const signature = await this.signers.alice.signTypedData(
      eip712.domain,
      { Reencrypt: eip712.types.Reencrypt },
      eip712.message,
    );
    const decryptedValue = await this.instances.alice.reencrypt(
      handle,
      privateKey,
      publicKey,
      signature.replace('0x', ''),
      this.contractAddress,
      this.signers.alice.address,
    );
    expect(decryptedValue).to.equal(42);
  });

  it('test reencrypt euint16', async function () {
    const handle = await this.contract.xUint16();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const eip712 = this.instances.alice.createEIP712(publicKey, this.contractAddress);
    const signature = await this.signers.alice.signTypedData(
      eip712.domain,
      { Reencrypt: eip712.types.Reencrypt },
      eip712.message,
    );
    const decryptedValue = await this.instances.alice.reencrypt(
      handle,
      privateKey,
      publicKey,
      signature.replace('0x', ''),
      this.contractAddress,
      this.signers.alice.address,
    );
    expect(decryptedValue).to.equal(16);
  });

  it('test reencrypt euint32', async function () {
    const handle = await this.contract.xUint32();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const eip712 = this.instances.alice.createEIP712(publicKey, this.contractAddress);
    const signature = await this.signers.alice.signTypedData(
      eip712.domain,
      { Reencrypt: eip712.types.Reencrypt },
      eip712.message,
    );
    const decryptedValue = await this.instances.alice.reencrypt(
      handle,
      privateKey,
      publicKey,
      signature.replace('0x', ''),
      this.contractAddress,
      this.signers.alice.address,
    );
    expect(decryptedValue).to.equal(32);
  });

  it('test reencrypt euint64', async function () {
    const handle = await this.contract.xUint64();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const eip712 = this.instances.alice.createEIP712(publicKey, this.contractAddress);
    const signature = await this.signers.alice.signTypedData(
      eip712.domain,
      { Reencrypt: eip712.types.Reencrypt },
      eip712.message,
    );
    const decryptedValue = await this.instances.alice.reencrypt(
      handle,
      privateKey,
      publicKey,
      signature.replace('0x', ''),
      this.contractAddress,
      this.signers.alice.address,
    );
    expect(decryptedValue).to.equal(18446744073709551600n);
  });

  it('test reencrypt eaddress', async function () {
    const handle = await this.contract.xAddress();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const eip712 = this.instances.alice.createEIP712(publicKey, this.contractAddress);
    const signature = await this.signers.alice.signTypedData(
      eip712.domain,
      { Reencrypt: eip712.types.Reencrypt },
      eip712.message,
    );
    const decryptedValue = await this.instances.alice.reencrypt(
      handle,
      privateKey,
      publicKey,
      signature.replace('0x', ''),
      this.contractAddress,
      this.signers.alice.address,
    );
    expect(decryptedValue).to.equal(BigInt('0x8ba1f109551bD432803012645Ac136ddd64DBA72'));
  });

  it('test reencrypt ebytes256', async function () {
    const handle = await this.contract.yBytes256();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const eip712 = this.instances.alice.createEIP712(publicKey, this.contractAddress);
    const signature = await this.signers.alice.signTypedData(
      eip712.domain,
      { Reencrypt: eip712.types.Reencrypt },
      eip712.message,
    );
    const decryptedValue = await this.instances.alice.reencrypt(
      handle,
      privateKey,
      publicKey,
      signature.replace('0x', ''),
      this.contractAddress,
      this.signers.alice.address,
    );
    expect(decryptedValue).to.equal(184467440737095500228978978978978970980978908978978907890778907089780970897890n);
  });
});
