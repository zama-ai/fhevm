import { expect } from 'chai';
import { ethers } from 'hardhat';

import { getSigners, initSigners } from '../signers';

describe('TestAsyncDecrypt', function () {
  before(async function () {
    await initSigners(3);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const evePrivateKey = '0x717fd99986df414889fd8b51069d4f90a50af72e542c58ee065f5883779099c6'; // private key corresponding to Eve's original account (derived from the mnemonic in `.env.example`)
    this.eve = new ethers.Wallet(evePrivateKey).connect(ethers.provider);
    const tx0 = await this.signers.alice.sendTransaction({ to: this.eve.address, value: ethers.parseEther('0.1') });
    await tx0.wait();

    const addressToCheck = '0xc8c9303Cd7F337fab769686B593B87DC3403E0ce'; // Should be equal to the hardcoded ORACLE_PREDEPLOY_ADDRESS in the Oracle solidity library
    const codeAtAddress = await ethers.provider.getCode(addressToCheck);
    if (codeAtAddress === '0x') {
      // OraclePredploy not deployed yet, so Eve (original's account from `.env`) should deploy it while her nonce is still null to get the correct address
      const nonce = await ethers.provider.getTransactionCount(this.eve);
      if (nonce !== 0) {
        throw new Error(
          `The nonce of Eve's account is not null, could not deploy the Oracle predeploy. Please relaunch a clean instance of the fhEVM`,
        );
      }
      const oracleFactory = await ethers.getContractFactory('OraclePredeploy');
      this.oracle = await oracleFactory.connect(this.eve).deploy(); // Eve is the Oracle Admin
      await this.oracle.waitForDeployment();
      const tx1 = await this.oracle.connect(this.eve).addRelayer(this.signers.bob.address); // Bob is an Oracle Relayer
      await tx1.wait();
    } else {
      // An oracle was already deployed at ORACLE_PREDEPLOY_ADDRESS, check that is indeed the OraclePredploy contract by comparing hashes
      const codeHash = ethers.keccak256(codeAtAddress);
      if (
        codeHash === '0xabf5180b65c12aae6fb23e1c971550809bb33088d0409d25f55ef8d9dbdb2d1f' ||
        codeHash === '0x236016a40c97d82055188e770d4bd2eaa72160b1bcfd4542018bdf5725816ddd'
      ) {
        // it is indeed the OraclePredploy because it codeHashes are matching on either hardhat node (mocked mode) or on the fhevm node
        this.oracle = await ethers.getContractAt('OraclePredeploy', addressToCheck);
        const bobIsRelayer = await this.oracle.isRelayer(this.signers.bob.address);
        if (!bobIsRelayer) {
          const tx1 = await this.oracle.connect(this.eve).addRelayer(this.signers.bob.address); // Bob is an Oracle Relayer
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
    this.contract = await contractFactory.connect(this.signers.alice).deploy();
  });

  it('test async decrypt', async function () {
    const tx2 = await this.contract.connect(this.signers.carol).request(5, 15, { gasLimit: 5_000_000 });
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
    const resultDecryption = await this.oracle.connect(this.signers.bob).decryptTEST(requestID);

    // Finally the relayer submit a tx to fulfill the decryption request and execute the callback before the timeout
    const tx3 = await this.oracle
      .connect(this.signers.bob)
      .fulfillRequestUint32(requestID, [resultDecryption], { value: msgValue });
    await tx3.wait();

    const y = await this.contract.y();
    expect(y).to.equal(52); // 5+15+32
  });
});
