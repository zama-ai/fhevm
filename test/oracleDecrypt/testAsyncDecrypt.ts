import { expect } from 'chai';
import { ethers } from 'hardhat';

import { getSigners, initSigners, requestFaucet } from '../signers';

describe('TestAsyncDecrypt', function () {
  before(async function () {
    await initSigners(3);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const alicePrivateKey = '0x8355bb293b8714a06b972bfe692d1bd9f24235c1f4007ae0be285d398b0bba2f'; // private key corresponding to Alice's original account (derived from the mnemonic in `.env.example`)
    const evePrivateKey = '0x717fd99986df414889fd8b51069d4f90a50af72e542c58ee065f5883779099c6'; // private key corresponding to Eve's original account (derived from the mnemonic in `.env.example`)
    this.alice = new ethers.Wallet(alicePrivateKey).connect(ethers.provider);
    this.eve = new ethers.Wallet(evePrivateKey).connect(ethers.provider);
    const addressToCheck = '0xc8c9303Cd7F337fab769686B593B87DC3403E0ce'; // Should be equal to the hardcoded ORACLE_PREDEPLOY_ADDRESS in the Oracle solidity library
    const codeAtAddress = await ethers.provider.getCode(addressToCheck);
    if (codeAtAddress === '0x') {
      await requestFaucet(this.alice.address);
      await requestFaucet(this.eve.address);
      // OraclePredploy not deployed yet, so Eve (original account from `.env`) should deploy it while her nonce is still null to get the correct address
      const nonce = await ethers.provider.getTransactionCount(this.eve.address);
      if (nonce !== 0) {
        throw new Error(
          `The nonce of Eve's account is not null, could not deploy the Oracle predeploy. Please relaunch a clean instance of the fhEVM`,
        );
      }
      const oracleFactory = await ethers.getContractFactory('OraclePredeployTest');
      this.oracle = await oracleFactory.connect(this.eve).deploy(this.alice.address); // Eve sets Alice (original account from `.env`) as the Oracle Admin
      await this.oracle.waitForDeployment();
      const tx1 = await this.oracle.connect(this.alice).addRelayer(this.signers.bob.address); // Bob is an Oracle Relayer
      await tx1.wait();
    } else {
      // An oracle was already deployed at ORACLE_PREDEPLOY_ADDRESS, check that is indeed the OraclePredploy contract by comparing hashes
      const codeHash = ethers.keccak256(codeAtAddress);
      if (
        codeHash === '0xb8363127c68f104f8a6f383a55b9e00ffb49f8470844c76788e149e4af49756f' ||
        codeHash === '0x808db48789429e5f79adb9849400e0d01c5c0f017663f06c6b2ab48f2eeae10e'
      ) {
        // it is indeed the OraclePredploy because it codeHashes are matching on either hardhat node (mocked mode) or on the fhevm node
        this.oracle = await ethers.getContractAt('OraclePredeployTest', addressToCheck);
        const bobIsRelayer = await this.oracle.isRelayer(this.signers.bob.address);
        if (!bobIsRelayer) {
          const tx1 = await this.oracle.connect(this.alice).addRelayer(this.signers.bob.address); // Bob is an Oracle Relayer
          await tx1.wait();
        }
      } else {
        // another contract was deployed by Alice previously, so you should restart the fhEVM
        throw new Error(
          `The nonce of Eve's account is not null, could not deploy the Oracle predeploy. Please relaunch a clean instance of the fhEVM`,
        );
      }
    }
    const contractFactory = await ethers.getContractFactory('TestAsyncDecrypt');
    this.contract = await contractFactory.connect(this.alice).deploy();
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

    // Here we simulate the threshold decryption by the KMS
    const resultDecryption = await this.oracle.connect(this.signers.bob).decryptTestBool(requestID);

    // Finally the relayer submit a tx to fulfill the decryption request and execute the callback before the timeout
    const tx3 = await this.oracle
      .connect(this.signers.bob)
      .fulfillRequestBool(requestID, [resultDecryption], { value: msgValue });
    await tx3.wait();

    const y = await this.contract.yBool();
    expect(y).to.equal(true);
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

    // Here we simulate the threshold decryption by the KMS
    const resultDecryption = await this.oracle.connect(this.signers.bob).decryptTestUint8(requestID);

    // Finally the relayer submit a tx to fulfill the decryption request and execute the callback before the timeout
    const tx3 = await this.oracle
      .connect(this.signers.bob)
      .fulfillRequestUint8(requestID, [resultDecryption], { value: msgValue });
    await tx3.wait();

    const y = await this.contract.yUint8();
    expect(y).to.equal(8);
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

    // Here we simulate the threshold decryption by the KMS
    const resultDecryption = await this.oracle.connect(this.signers.bob).decryptTestUint16(requestID);

    // Finally the relayer submit a tx to fulfill the decryption request and execute the callback before the timeout
    const tx3 = await this.oracle
      .connect(this.signers.bob)
      .fulfillRequestUint16(requestID, [resultDecryption], { value: msgValue });
    await tx3.wait();

    const y = await this.contract.yUint16();
    expect(y).to.equal(16);
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

    // Here we simulate the threshold decryption by the KMS
    const resultDecryption = await this.oracle.connect(this.signers.bob).decryptTestUint32(requestID);

    // Finally the relayer submit a tx to fulfill the decryption request and execute the callback before the timeout
    const tx3 = await this.oracle
      .connect(this.signers.bob)
      .fulfillRequestUint32(requestID, [resultDecryption], { value: msgValue });
    await tx3.wait();

    const y = await this.contract.yUint32();
    expect(y).to.equal(52); // 5+15+32
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

    // Here we simulate the threshold decryption by the KMS
    const resultDecryption = await this.oracle.connect(this.signers.bob).decryptTestUint64(requestID);

    // Finally the relayer submit a tx to fulfill the decryption request and execute the callback before the timeout
    const tx3 = await this.oracle
      .connect(this.signers.bob)
      .fulfillRequestUint64(requestID, [resultDecryption], { value: msgValue });
    await tx3.wait();

    const y = await this.contract.yUint64();
    expect(y).to.equal(64);
  });
});
