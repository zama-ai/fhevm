import { expect } from 'chai';
import dotenv from 'dotenv';
import fs from 'fs';
import { ethers } from 'hardhat';

import { getSigners, initSigners } from '../signers';

describe('TestAsyncDecrypt', function () {
  before(async function () {
    await initSigners(3);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contractFactory = await ethers.getContractFactory('TestAsyncDecrypt');
    this.contract = await contractFactory.deploy();
    const parsedEnv = dotenv.parse(fs.readFileSync('oracle/.env.oracle'));
    this.oracle = await ethers.getContractAt('OraclePredeploy', parsedEnv.ORACLE_PREDEPLOY_ADDRESS);
    const privKeyRelayer = process.env.PRIVATE_KEY_ORACLE_DEPLOYER;
    this.relayer = new ethers.Wallet(privKeyRelayer!, ethers.provider);
  });

  it('test async decrypt bool', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestBool({ gasLimit: 5_000_000 });
    await tx2.wait();
    const filter = this.oracle.filters.EventDecryptionEBool;

    const events = await this.oracle.queryFilter(filter, -1);
    const event = events[0];
    const args = event.args;
    const requestID = args[0];
    const handlesCTs = args[1];
    // the relayer will call the endpoint getCipherText() on a node to get the ciphertexts bytes arrays from this array of handles
    // + Merkle proof that the event was indeed emitted by the Oracle contract with this handle and send all this data to the KMS
    // then the KMS will check the Merkle Proof of the event + check that indeed hash(ciphertext)=handleCT for each handleCT

    const msgValue = args[4];

    if (process.env.HARDHAT_NETWORK === 'hardhat') {
      // Here we simulate the threshold decryption by the KMS
      const resultDecryption = handlesCTs[0] === 1n;
      // Finally the relayer submit a tx to fulfill the decryption request and execute the callback before the timeout
      const tx3 = await this.oracle
        .connect(this.relayer)
        .fulfillRequestBool(requestID, [resultDecryption], { value: msgValue });
      await tx3.wait();

      const y = await this.contract.yBool();
      expect(y).to.equal(true);
    }
  });

  it('test async decrypt uint8', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint8({ gasLimit: 5_000_000 });
    await tx2.wait();
    const filter = this.oracle.filters.EventDecryptionEUint8;
    const events = await this.oracle.queryFilter(filter, -1);
    const event = events[0];
    const args = event.args;
    const requestID = args[0];
    const handlesCTs = args[1];
    // the relayer will call the endpoint getCipherText() on a node to get the ciphertexts bytes arrays from this array of handles
    // + Merkle proof that the event was indeed emitted by the Oracle contract with this handle and send all this data to the KMS
    // then the KMS will check the Merkle Proof of the event + check that indeed hash(ciphertext)=handleCT for each handleCT

    const msgValue = args[4];

    if (process.env.HARDHAT_NETWORK === 'hardhat') {
      // Here we simulate the threshold decryption by the KMS
      const resultDecryption = handlesCTs[0];
      // Finally the relayer submit a tx to fulfill the decryption request and execute the callback before the timeout
      const tx3 = await this.oracle
        .connect(this.relayer)
        .fulfillRequestUint8(requestID, [resultDecryption], { value: msgValue });
      await tx3.wait();

      const y = await this.contract.yUint8();
      expect(y).to.equal(8);
    }
  });

  it('test async decrypt uint16', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint16({ gasLimit: 5_000_000 });
    await tx2.wait();
    const filter = this.oracle.filters.EventDecryptionEUint16;
    const events = await this.oracle.queryFilter(filter, -1);
    const event = events[0];
    const args = event.args;
    const requestID = args[0];
    const handlesCTs = args[1];
    // the relayer will call the endpoint getCipherText() on a node to get the ciphertexts bytes arrays from this array of handles
    // + Merkle proof that the event was indeed emitted by the Oracle contract with this handle and send all this data to the KMS
    // then the KMS will check the Merkle Proof of the event + check that indeed hash(ciphertext)=handleCT for each handleCT

    const msgValue = args[4];

    if (process.env.HARDHAT_NETWORK === 'hardhat') {
      // Here we simulate the threshold decryption by the KMS
      const resultDecryption = handlesCTs[0];
      // Finally the relayer submit a tx to fulfill the decryption request and execute the callback before the timeout
      const tx3 = await this.oracle
        .connect(this.relayer)
        .fulfillRequestUint16(requestID, [resultDecryption], { value: msgValue });
      await tx3.wait();

      const y = await this.contract.yUint16();
      expect(y).to.equal(16);
    }
  });

  it('test async decrypt uint32', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint32(5, 15, { gasLimit: 5_000_000 });
    await tx2.wait();
    const filter = this.oracle.filters.EventDecryptionEUint32;
    const events = await this.oracle.queryFilter(filter, -1);
    const event = events[0];
    const args = event.args;
    const requestID = args[0];
    const handlesCTs = args[1];
    // the relayer will call the endpoint getCipherText() on a node to get the ciphertexts bytes arrays from this array of handles
    // + Merkle proof that the event was indeed emitted by the Oracle contract with this handle and send all this data to the KMS
    // then the KMS will check the Merkle Proof of the event + check that indeed hash(ciphertext)=handleCT for each handleCT

    const msgValue = args[4];

    if (process.env.HARDHAT_NETWORK === 'hardhat') {
      // Here we simulate the threshold decryption by the KMS
      const resultDecryption = handlesCTs[0];
      // Finally the relayer submit a tx to fulfill the decryption request and execute the callback before the timeout
      const tx3 = await this.oracle
        .connect(this.relayer)
        .fulfillRequestUint32(requestID, [resultDecryption], { value: msgValue });
      await tx3.wait();

      const y = await this.contract.yUint32();
      expect(y).to.equal(52); // 5+15+32
    }
  });

  it('test async decrypt uint64', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint64({ gasLimit: 5_000_000 });
    await tx2.wait();
    const filter = this.oracle.filters.EventDecryptionEUint64;
    const events = await this.oracle.queryFilter(filter, -1);
    const event = events[0];
    const args = event.args;
    const requestID = args[0];
    const handlesCTs = args[1];
    // the relayer will call the endpoint getCipherText() on a node to get the ciphertexts bytes arrays from this array of handles
    // + Merkle proof that the event was indeed emitted by the Oracle contract with this handle and send all this data to the KMS
    // then the KMS will check the Merkle Proof of the event + check that indeed hash(ciphertext)=handleCT for each handleCT

    const msgValue = args[4];

    if (process.env.HARDHAT_NETWORK === 'hardhat') {
      // Here we simulate the threshold decryption by the KMS
      const resultDecryption = handlesCTs[0];
      // Finally the relayer submit a tx to fulfill the decryption request and execute the callback before the timeout
      const tx3 = await this.oracle
        .connect(this.relayer)
        .fulfillRequestUint64(requestID, [resultDecryption], { value: msgValue });
      await tx3.wait();

      const y = await this.contract.yUint64();
      expect(y).to.equal(64);
    }
  });
});
