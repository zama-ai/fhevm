import { assert, expect } from 'chai';

// Encrypts 7, runs TestInput.add42ToInput64, then asserts the result decrypts to 49
// through both user decryption and public decryption. Shared by the standard input
// flow and the priority-coprocessor input flow so the two stay in sync.
export const runAdd42InputAndDecrypt = async function (this: Mocha.Context) {
  const encryptedInput = await this.instances.alice.encryptUint64({
    value: 7n,
    contractAddress: this.contractAddress,
    userAddress: this.signers.alice.address,
  });

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
  assert.deepEqual(res.clearValues, { [handle]: 49n });
};
