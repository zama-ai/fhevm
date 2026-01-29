import { expect, assert } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { userDecryptSingleHandle } from '../utils';

describe('Input Flow', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const envContractAddress = process.env.TEST_INPUT_CONTRACT_ADDRESS;
    const contractFactory = await ethers.getContractFactory('TestInput');
    if (!envContractAddress) {
      this.contract = await contractFactory.connect(this.signers.alice).deploy();
      this.contractAddress = await this.contract.getAddress();
      await this.contract.waitForDeployment();
    } else {
      this.contractAddress = envContractAddress;
      this.contract = contractFactory.connect(this.signers.alice).attach(this.contractAddress);
    }
    this.instances = await createInstances(this.signers);
  });

  it('test user input uint64 (non-trivial)', async function () {
    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.add64(18446744073709550042n);
    const encryptedAmount = await inputAlice.encrypt();
    encryptedAmount.handles.forEach((handle: any, index: any) => {
      // Assuming handle is a Uint8Array or Buffer
      console.log(`  Handle ${index}: 0x${Buffer.from(handle).toString('hex')}`);
    });
    console.log('InputProof: 0x' + Buffer.from(encryptedAmount.inputProof).toString('hex'));
    const tx = await this.contract.requestUint64NonTrivial(encryptedAmount.handles[0], encryptedAmount.inputProof);
    const receipt = await tx.wait();
    expect(receipt.status).to.equal(1);
  });

  it('test add 42 to uint64 input and decrypt', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add64(7n);
    const encryptedInput = await input.encrypt();
    encryptedInput.handles.forEach((handle: any, index: any) => {
      // Assuming handle is a Uint8Array or Buffer
      console.log(`  Handle ${index}: 0x${Buffer.from(handle).toString('hex')}`);
    });
    console.log('InputProof: 0x' + Buffer.from(encryptedInput.inputProof).toString('hex'));
    const tx = await this.contract.add42ToInput64(encryptedInput.handles[0], encryptedInput.inputProof);
    const receipt = await tx.wait();
    expect(receipt.status).to.equal(1);

    const handle = await this.contract.resUint64();

    // User decrypt the result - should be 7 + 42 = 49.
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const decryptedValue = await userDecryptSingleHandle(
      handle,
      this.contractAddress,
      this.instances.alice,
      this.signers.alice,
      privateKey,
      publicKey,
    );
    expect(decryptedValue).to.equal(49n);

    // Public decrypt the result - should be 49.
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 49n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
