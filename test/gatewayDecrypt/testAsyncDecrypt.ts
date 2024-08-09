import { expect } from 'chai';
import { ethers, network } from 'hardhat';

import { asyncDecrypt, awaitAllDecryptionResults } from '../asyncDecrypt';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { bigIntToBytes, waitNBlocks } from '../utils';

describe('TestAsyncDecrypt', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.relayerAddress = '0x97F272ccfef4026A1F3f0e0E879d514627B84E69';

    // very first request of decryption always fail at the moment due to a gateway bug
    // TODO: remove following 8 lines when the gateway bug will be fixed
    const contractFactory = await ethers.getContractFactory('TestAsyncDecrypt');
    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    await this.contract.waitForDeployment();
    this.contractAddress = await this.contract.getAddress();
    this.instances = await createInstances(this.signers);
    const tx = await this.contract.connect(this.signers.carol).requestUint8({ gasLimit: 5_000_000 });
    await tx.wait(); // this first request is here just to silence the current gateway bug at the moment
    await waitNBlocks(1);

    await asyncDecrypt();
  });

  beforeEach(async function () {
    const contractFactory = await ethers.getContractFactory('TestAsyncDecrypt');
    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    this.contractAddress = await this.contract.getAddress();
    this.instances = await createInstances(this.signers);
  });

  it.skip('test async decrypt bool infinite loop', async function () {
    const balanceBeforeR = await ethers.provider.getBalance(this.relayerAddress);
    const balanceBeforeU = await ethers.provider.getBalance(this.signers.carol.address);
    const tx = await this.contract.connect(this.signers.carol).requestBoolInfinite({ gasLimit: 5_000_000 });
    await tx.wait();
    const balanceAfterU = await ethers.provider.getBalance(this.signers.carol.address);
    await awaitAllDecryptionResults();
    const y = await this.contract.yBool();
    console.log(y);
    const balanceAfterR = await ethers.provider.getBalance(this.relayerAddress);
    console.log('gas paid by relayer (fulfil tx) : ', balanceBeforeR - balanceAfterR);
    console.log('gas paid by user (request tx) : ', balanceBeforeU - balanceAfterU);
  });

  it.skip('test async decrypt bool would fail if maxTimestamp is above 1 day', async function () {
    if (network.name === 'hardhat') {
      // mocked mode
      await expect(this.contract.connect(this.signers.carol).requestBoolAboveDelay()).to.be.revertedWith(
        'maxTimestamp exceeded MAX_DELAY',
      );
    } else {
      // fhevm-mode
      const txObject = await this.contract.requestBoolAboveDelay.populateTransaction({ gasLimit: 5_000_000 });
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

  it('test async decrypt bool', async function () {
    const balanceBeforeR = await ethers.provider.getBalance(this.relayerAddress);
    const balanceBeforeU = await ethers.provider.getBalance(this.signers.carol.address);
    const tx2 = await this.contract.connect(this.signers.carol).requestBool({ gasLimit: 5_000_000 });
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
      const txObject = await this.contract.requestFakeBool.populateTransaction({ gasLimit: 5_000_000 });
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

  it('test async decrypt uint4', async function () {
    const balanceBefore = await ethers.provider.getBalance(this.relayerAddress);
    const tx2 = await this.contract.connect(this.signers.carol).requestUint4({ gasLimit: 5_000_000 });
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint4();
    expect(y).to.equal(4);
    const balanceAfter = await ethers.provider.getBalance(this.relayerAddress);
    console.log(balanceBefore - balanceAfter);
  });

  it.skip('test async decrypt FAKE uint4', async function () {
    if (network.name !== 'hardhat') {
      // only in fhevm mode
      const txObject = await this.contract.requestFakeUint4.populateTransaction({ gasLimit: 5_000_000 });
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

  it('test async decrypt uint8', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint8({ gasLimit: 5_000_000 });
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint8();
    expect(y).to.equal(42);
  });

  it.skip('test async decrypt FAKE uint8', async function () {
    if (network.name !== 'hardhat') {
      // only in fhevm mode
      const txObject = await this.contract.requestFakeUint8.populateTransaction({ gasLimit: 5_000_000 });
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
    const tx2 = await this.contract.connect(this.signers.carol).requestUint16({ gasLimit: 5_000_000 });
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint16();
    expect(y).to.equal(16);
  });

  it.skip('test async decrypt FAKE uint16', async function () {
    if (network.name !== 'hardhat') {
      // only in fhevm mode
      const txObject = await this.contract.requestFakeUint16.populateTransaction({ gasLimit: 5_000_000 });
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
    const tx2 = await this.contract.connect(this.signers.carol).requestUint32(5, 15, { gasLimit: 5_000_000 });
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint32();
    expect(y).to.equal(52); // 5+15+32
  });

  it.skip('test async decrypt FAKE uint32', async function () {
    if (network.name !== 'hardhat') {
      // only in fhevm mode
      const txObject = await this.contract.requestFakeUint32.populateTransaction({ gasLimit: 5_000_000 });
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
    const tx2 = await this.contract.connect(this.signers.carol).requestUint64({ gasLimit: 5_000_000 });
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint64();
    expect(y).to.equal(18446744073709551600n);
  });

  it.skip('test async decrypt FAKE uint64', async function () {
    if (network.name !== 'hardhat') {
      // only in fhevm mode
      const txObject = await this.contract.requestFakeUint64.populateTransaction({ gasLimit: 5_000_000 });
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
    const tx2 = await this.contract.connect(this.signers.carol).requestAddress({ gasLimit: 5_000_000 });
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yAddress();
    expect(y).to.equal('0x8ba1f109551bD432803012645Ac136ddd64DBA72');
  });

  it('test async decrypt several addresses', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestSeveralAddresses({ gasLimit: 5_000_000 });
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
      const txObject = await this.contract.requestFakeAddress.populateTransaction({ gasLimit: 5_000_000 });
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

  it('test async decrypt mixed', async function () {
    const contractFactory = await ethers.getContractFactory('TestAsyncDecrypt');
    const contract2 = await contractFactory.connect(this.signers.alice).deploy();
    const tx2 = await contract2.connect(this.signers.carol).requestMixed(5, 15, { gasLimit: 5_000_000 });
    await tx2.wait();
    await awaitAllDecryptionResults();
    let yB = await contract2.yBool();
    expect(yB).to.equal(true);
    let y = await contract2.yUint4();
    expect(y).to.equal(4);
    y = await contract2.yUint8();
    expect(y).to.equal(42);
    y = await contract2.yUint16();
    expect(y).to.equal(16);
    let yAdd = await contract2.yAddress();
    expect(yAdd).to.equal('0x8ba1f109551bD432803012645Ac136ddd64DBA72');
    y = await contract2.yUint32();
    expect(y).to.equal(52); // 5+15+32
    y = await contract2.yUint64();
    expect(y).to.equal(18446744073709551600n);
  });

  it('test async decrypt uint64 non-trivial', async function () {
    // console.log(this.instances.alice)
    // console.log(this.instances.alice.address)
    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.add64(18446744073709550042n);
    const encryptedAmount = inputAlice.encrypt();
    const tx = await this.contract.requestUint64NonTrivial(encryptedAmount.handles[0], encryptedAmount.inputProof, {
      gasLimit: 5_000_000,
    });
    await tx.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint64();
    expect(y).to.equal(18446744073709550042n);
  });

  it('test async decrypt ebytes256 non-trivial', async function () {
    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.addBytes256(bigIntToBytes(18446744073709550022n));
    const encryptedAmount = inputAlice.encrypt();
    const tx = await await this.contract.requestEbytes256NonTrivial(
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
      { gasLimit: 5_000_000 },
    );
    await tx.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yBytes256();
    expect(y).to.equal(ethers.toBeHex(18446744073709550022n, 256));
  });

  it('test async decrypt ebytes256 non-trivial with snapshot [skip-on-coverage]', async function () {
    if (network.name === 'hardhat') {
      this.snapshotId = await ethers.provider.send('evm_snapshot');
      const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
      inputAlice.addBytes256(bigIntToBytes(18446744073709550022n));
      const encryptedAmount = inputAlice.encrypt();
      const tx = await await this.contract.requestEbytes256NonTrivial(
        encryptedAmount.handles[0],
        encryptedAmount.inputProof,
        { gasLimit: 5_000_000 },
      );
      await tx.wait();
      await awaitAllDecryptionResults();
      const y = await this.contract.yBytes256();
      expect(y).to.equal(ethers.toBeHex(18446744073709550022n, 256));

      await ethers.provider.send('evm_revert', [this.snapshotId]);
      const inputAlice2 = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
      inputAlice2.addBytes256(bigIntToBytes(424242n));
      const encryptedAmount2 = inputAlice2.encrypt();
      const tx2 = await await this.contract.requestEbytes256NonTrivial(
        encryptedAmount2.handles[0],
        encryptedAmount2.inputProof,
        { gasLimit: 5_000_000 },
      );
      await tx2.wait();
      await awaitAllDecryptionResults();
      const y2 = await this.contract.yBytes256();
      expect(y2).to.equal(ethers.toBeHex(424242n, 256));
    }
  });

  it('test async decrypt mixed with ebytes256', async function () {
    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.addBytes256(bigIntToBytes(18446744073709550032n));
    const encryptedAmount = inputAlice.encrypt();
    const tx = await await this.contract.requestMixedBytes256(encryptedAmount.handles[0], encryptedAmount.inputProof, {
      gasLimit: 5_000_000,
    });
    await tx.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yBytes256();
    expect(y).to.equal(ethers.toBeHex(18446744073709550032n, 256));
    const yb = await this.contract.yBool();
    expect(yb).to.equal(true);
    const yAdd = await this.contract.yAddress();
    expect(yAdd).to.equal('0x8ba1f109551bD432803012645Ac136ddd64DBA72');
  });
});
