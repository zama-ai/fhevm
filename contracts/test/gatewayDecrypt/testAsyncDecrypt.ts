import { expect } from 'chai';
import { ethers, network } from 'hardhat';

import { awaitAllDecryptionResults, initDecryptionOracle } from '../asyncDecrypt';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { bigIntToBytes64, bigIntToBytes128, bigIntToBytes256 } from '../utils';

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

  it('test async decrypt uint4', async function () {
    const balanceBefore = await ethers.provider.getBalance(this.relayerAddress);
    const tx2 = await this.contract.connect(this.signers.carol).requestUint4();
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
      const txObject = await this.contract.requestFakeUint4.populateTransaction();
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

  it('test async decrypt ebytes64 trivial', async function () {
    const tx = await this.contract.requestEbytes64Trivial('0x78685689');
    await tx.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yBytes64();
    expect(y).to.equal(ethers.toBeHex(BigInt('0x78685689'), 64));
  });

  it('test async decrypt ebytes64 non-trivial', async function () {
    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.addBytes64(
      bigIntToBytes64(98870780878070870878787887072921111299111111000000292928818818818818221112111n),
    );
    const encryptedAmount = await inputAlice.encrypt();
    const tx = await this.contract.requestEbytes64NonTrivial(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yBytes64();
    expect(y).to.equal(
      ethers.toBeHex(98870780878070870878787887072921111299111111000000292928818818818818221112111n, 64),
    );
  });

  it('test async decrypt ebytes128 trivial', async function () {
    const tx = await this.contract.requestEbytes128Trivial(
      '0x8701d11594415047dfac2d9cb87e6631df5a735a2f364fba1511fa7b812dfad2972b809b80ff25ec19591a598081af357cba384cf5aa8e085678ff70bc55faee',
    );
    await tx.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yBytes128();
    expect(y).to.equal(
      ethers.toBeHex(
        BigInt(
          '0x8701d11594415047dfac2d9cb87e6631df5a735a2f364fba1511fa7b812dfad2972b809b80ff25ec19591a598081af357cba384cf5aa8e085678ff70bc55faee',
        ),
        128,
      ),
    );
  });

  it('test async decrypt ebytes128 non-trivial', async function () {
    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.addBytes128(
      bigIntToBytes128(
        9887078087807087087878788707292111129911111100000029292881881881881822111211198870780878070870878787887072921111299111111000000292928818818818818221112111n,
      ),
    );
    const encryptedAmount = await inputAlice.encrypt();
    const tx = await this.contract.requestEbytes128NonTrivial(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yBytes128();
    expect(y).to.equal(
      ethers.toBeHex(
        9887078087807087087878788707292111129911111100000029292881881881881822111211198870780878070870878787887072921111299111111000000292928818818818818221112111n,
        128,
      ),
    );
  });

  it('test async decrypt ebytes256 trivial', async function () {
    const tx = await this.contract.requestEbytes256Trivial('0x78685689');
    await tx.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yBytes256();
    expect(y).to.equal(ethers.toBeHex(BigInt('0x78685689'), 256));
  });

  it('test async decrypt ebytes256 non-trivial', async function () {
    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.addBytes256(bigIntToBytes256(18446744073709550022n));
    const encryptedAmount = await inputAlice.encrypt();
    const tx = await this.contract.requestEbytes256NonTrivial(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yBytes256();
    expect(y).to.equal(ethers.toBeHex(18446744073709550022n, 256));
  });

  it('test async decrypt ebytes256 non-trivial with snapshot [skip-on-coverage]', async function () {
    if (network.name === 'hardhat') {
      this.snapshotId = await ethers.provider.send('evm_snapshot');
      const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
      inputAlice.addBytes256(bigIntToBytes256(18446744073709550022n));
      const encryptedAmount = await inputAlice.encrypt();
      const tx = await this.contract.requestEbytes256NonTrivial(encryptedAmount.handles[0], encryptedAmount.inputProof);
      await tx.wait();
      await awaitAllDecryptionResults();
      const y = await this.contract.yBytes256();
      expect(y).to.equal(ethers.toBeHex(18446744073709550022n, 256));

      await ethers.provider.send('evm_revert', [this.snapshotId]);
      const inputAlice2 = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
      inputAlice2.addBytes256(bigIntToBytes256(424242n));
      const encryptedAmount2 = await inputAlice2.encrypt();
      const tx2 = await this.contract.requestEbytes256NonTrivial(
        encryptedAmount2.handles[0],
        encryptedAmount2.inputProof,
      );
      await tx2.wait();
      await awaitAllDecryptionResults();
      const y2 = await this.contract.yBytes256();
      expect(y2).to.equal(ethers.toBeHex(424242n, 256));
    }
  });

  it('test async decrypt mixed with ebytes256', async function () {
    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.addBytes256(bigIntToBytes256(18446744073709550032n));
    const encryptedAmount = await inputAlice.encrypt();
    const tx = await this.contract.requestMixedBytes256(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yBytes256();
    expect(y).to.equal(ethers.toBeHex(18446744073709550032n, 256));
    const y2 = await this.contract.yBytes64();
    expect(y2).to.equal(ethers.toBeHex(BigInt('0xaaff42'), 64));
    const yb = await this.contract.yBool();
    expect(yb).to.equal(true);
    const yAdd = await this.contract.yAddress();
    expect(yAdd).to.equal('0x8ba1f109551bD432803012645Ac136ddd64DBA72');
  });
});
