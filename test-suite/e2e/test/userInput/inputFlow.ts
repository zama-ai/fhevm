import { assert, expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

const CIPHERTEXT_DRIFT_FORBIDDEN_NETWORKS = new Set(['sepolia', 'mainnet', 'zwsDev']);
const activeNetwork = () => process.env.NETWORK ?? process.env.HARDHAT_NETWORK ?? '';

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
    if (CIPHERTEXT_DRIFT_FORBIDDEN_NETWORKS.has(activeNetwork())) {
      this.skip();
    }

    const encryptedAmount = await this.instances.alice.encryptUint64({
      value: 18446744073709550042n,
      contractAddress: this.contractAddress,
      userAddress: this.signers.alice.address,
    });

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
    const encryptedInput = await this.instances.alice.encryptUint64({
      value: 7n,
      contractAddress: this.contractAddress,
      userAddress: this.signers.alice.address,
    });

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
    const decryptedValue = await this.instances.alice.userDecryptSingleHandle({
      handle,
      contractAddress: this.contractAddress,
      signer: this.signers.alice,
    });
    expect(decryptedValue).to.equal(49n);

    // Public decrypt the result - should be 49.
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 49n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
