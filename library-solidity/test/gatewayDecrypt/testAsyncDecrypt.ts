import { expect } from 'chai';
import { ethers, network } from 'hardhat';

import { awaitAllDecryptionResults, initDecryptionOracle } from '../asyncDecrypt';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

describe('TestAsyncDecrypt', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.relayerAddress = '0x97F272ccfef4026A1F3f0e0E879d514627B84E69';
    this.instances = await createInstances(this.signers);
    await initDecryptionOracle();
  });

  beforeEach(async function () {
    const contractFactory = await ethers.getContractFactory('TestAsyncDecrypt');
    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    await this.contract.waitForDeployment();
    this.contractAddress = await this.contract.getAddress();
    this.instances = await createInstances(this.signers);
  });

  it.skip('test async decrypt bool infinite loop', async function () {
    const balanceBeforeR = await ethers.provider.getBalance(this.relayerAddress);
    const balanceBeforeU = await ethers.provider.getBalance(this.signers.carol.address);
    const tx = await this.contract.connect(this.signers.carol).requestBoolInfinite();
    await tx.wait();
    const balanceAfterU = await ethers.provider.getBalance(this.signers.carol.address);
    await awaitAllDecryptionResults();
    const y = await this.contract.yBool();
    console.log(y);
    const balanceAfterR = await ethers.provider.getBalance(this.relayerAddress);
    console.log('gas paid by relayer (fulfil tx) : ', balanceBeforeR - balanceAfterR);
    console.log('gas paid by user (request tx) : ', balanceBeforeU - balanceAfterU);
  });

  it('test async decrypt bool', async function () {
    const balanceBeforeR = await ethers.provider.getBalance(this.relayerAddress);
    const balanceBeforeU = await ethers.provider.getBalance(this.signers.carol.address);
    const tx2 = await this.contract.connect(this.signers.carol).requestBool();
    await tx2.wait();
    const balanceAfterU = await ethers.provider.getBalance(this.signers.carol.address);
    await awaitAllDecryptionResults();
    const y = await this.contract.yBool();
    expect(y).to.equal(true);
    const balanceAfterR = await ethers.provider.getBalance(this.relayerAddress);
    console.log('gas paid by relayer (fulfil tx) : ', balanceBeforeR - balanceAfterR);
    console.log('gas paid by user (request tx) : ', balanceBeforeU - balanceAfterU);
  });

  it.skip('test async decrypt FAKE bool', async function () {
    if (network.name !== 'hardhat') {
      // only in fhevm mode
      const txObject = await this.contract.requestFakeBool.populateTransaction();
      const tx = await this.signers.carol.sendTransaction(txObject);
      let receipt = null;
      let waitTime = 0;
      while (receipt === null && waitTime < 15000) {
        receipt = await ethers.provider.getTransactionReceipt(tx.hash);
        if (receipt === null) {
          console.log('Trying again to fetch txn receipt....');
          await new Promise((resolve) => setTimeout(resolve, 5000)); // Wait for 5 seconds
          waitTime += 5000;
        }
      }
      receipt === null ? expect(waitTime >= 15000).to.be.true : expect(receipt!.status).to.equal(0);
    }
  });

  it('test async decrypt uint8', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint8();
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint8();
    expect(y).to.equal(42);
  });

  it.skip('test async decrypt FAKE uint8', async function () {
    if (network.name !== 'hardhat') {
      // only in fhevm mode
      const txObject = await this.contract.requestFakeUint8.populateTransaction();
      const tx = await this.signers.carol.sendTransaction(txObject);
      let receipt = null;
      let waitTime = 0;
      while (receipt === null && waitTime < 15000) {
        receipt = await ethers.provider.getTransactionReceipt(tx.hash);
        if (receipt === null) {
          console.log('Trying again to fetch txn receipt....');
          await new Promise((resolve) => setTimeout(resolve, 5000)); // Wait for 5 seconds
          waitTime += 5000;
        }
      }
      expect(waitTime >= 15000).to.be.true;
    }
  });

  it('test async decrypt uint16', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint16();
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint16();
    expect(y).to.equal(16);
  });

  it.skip('test async decrypt FAKE uint16', async function () {
    if (network.name !== 'hardhat') {
      // only in fhevm mode
      const txObject = await this.contract.requestFakeUint16.populateTransaction();
      const tx = await this.signers.carol.sendTransaction(txObject);
      let receipt = null;
      let waitTime = 0;
      while (receipt === null && waitTime < 15000) {
        receipt = await ethers.provider.getTransactionReceipt(tx.hash);
        if (receipt === null) {
          console.log('Trying again to fetch txn receipt....');
          await new Promise((resolve) => setTimeout(resolve, 5000)); // Wait for 5 seconds
          waitTime += 5000;
        }
      }
      expect(waitTime >= 15000).to.be.true;
    }
  });

  it('test async decrypt uint32', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint32(5, 15);
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint32();
    expect(y).to.equal(52); // 5+15+32
  });

  it.skip('test async decrypt FAKE uint32', async function () {
    if (network.name !== 'hardhat') {
      // only in fhevm mode
      const txObject = await this.contract.requestFakeUint32.populateTransaction();
      const tx = await this.signers.carol.sendTransaction(txObject);
      let receipt = null;
      let waitTime = 0;
      while (receipt === null && waitTime < 15000) {
        receipt = await ethers.provider.getTransactionReceipt(tx.hash);
        if (receipt === null) {
          console.log('Trying again to fetch txn receipt....');
          await new Promise((resolve) => setTimeout(resolve, 5000)); // Wait for 5 seconds
          waitTime += 5000;
        }
      }
      expect(waitTime >= 15000).to.be.true;
    }
  });

  it('test async decrypt uint64', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint64();
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint64();
    expect(y).to.equal(18446744073709551600n);
  });

  it('test async decrypt uint128', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint128();
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint128();
    expect(y).to.equal(1267650600228229401496703205443n);
  });

  it('test async decrypt uint128 non-trivial', async function () {
    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.add128(184467440737095500429401496n);
    const encryptedAmount = await inputAlice.encrypt();
    const tx = await this.contract.requestUint128NonTrivial(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint128();
    expect(y).to.equal(184467440737095500429401496n);
  });

  it('test async decrypt uint256', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint256();
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint256();
    expect(y).to.equal(27606985387162255149739023449108101809804435888681546220650096895197251n);
  });

  it('test async decrypt uint256 non-trivial', async function () {
    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.add256(6985387162255149739023449108101809804435888681546n);
    const encryptedAmount = await inputAlice.encrypt();
    const tx = await this.contract.requestUint256NonTrivial(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint256();
    expect(y).to.equal(6985387162255149739023449108101809804435888681546n);
  });

  it.skip('test async decrypt FAKE uint64', async function () {
    if (network.name !== 'hardhat') {
      // only in fhevm mode
      const txObject = await this.contract.requestFakeUint64.populateTransaction();
      const tx = await this.signers.carol.sendTransaction(txObject);
      let receipt = null;
      let waitTime = 0;
      while (receipt === null && waitTime < 15000) {
        receipt = await ethers.provider.getTransactionReceipt(tx.hash);
        if (receipt === null) {
          console.log('Trying again to fetch txn receipt....');
          await new Promise((resolve) => setTimeout(resolve, 5000)); // Wait for 5 seconds
          waitTime += 5000;
        }
      }
      expect(waitTime >= 15000).to.be.true;
    }
  });

  it('test async decrypt address', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestAddress();
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yAddress();
    expect(y).to.equal('0x8ba1f109551bD432803012645Ac136ddd64DBA72');
  });

  it('test async decrypt several addresses', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestSeveralAddresses();
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yAddress();
    const y2 = await this.contract.yAddress2();
    expect(y).to.equal('0x8ba1f109551bD432803012645Ac136ddd64DBA72');
    expect(y2).to.equal('0xf48b8840387ba3809DAE990c930F3b4766A86ca3');
  });

  it.skip('test async decrypt FAKE address', async function () {
    if (network.name !== 'hardhat') {
      // only in fhevm mode
      const txObject = await this.contract.requestFakeAddress.populateTransaction();
      const tx = await this.signers.carol.sendTransaction(txObject);
      let receipt = null;
      let waitTime = 0;
      while (receipt === null && waitTime < 15000) {
        receipt = await ethers.provider.getTransactionReceipt(tx.hash);
        if (receipt === null) {
          console.log('Trying again to fetch txn receipt....');
          await new Promise((resolve) => setTimeout(resolve, 5000)); // Wait for 5 seconds
          waitTime += 5000;
        }
      }
      expect(waitTime >= 15000).to.be.true;
    }
  });

  it('test async decrypt uint64 non-trivial', async function () {
    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.add64(18446744073709550042n);
    const encryptedAmount = await inputAlice.encrypt();
    const tx = await this.contract.requestUint64NonTrivial(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint64();
    expect(y).to.equal(18446744073709550042n);
  });

  it('test async decrypt mixed', async function () {
    const uint256Input = BigInt('18446744073709550032');
    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.add256(uint256Input);
    const encryptedAmount = await inputAlice.encrypt();
    const tx = await this.contract.requestMixed(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    await awaitAllDecryptionResults();

    const y = await this.contract.yUint256();
    expect(y).to.equal(uint256Input);
    const y2 = await this.contract.yUint32();
    expect(y2).to.equal(32);
    const yb = await this.contract.yBool();
    expect(yb).to.equal(true);
    const yAdd = await this.contract.yAddress();
    expect(yAdd).to.equal('0x8ba1f109551bD432803012645Ac136ddd64DBA72');
  });
});
