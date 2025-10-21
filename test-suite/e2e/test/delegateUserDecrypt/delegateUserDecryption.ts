import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { userDecryptSingleHandle } from '../utils';

describe('Delegate user decryption', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    const contractFactory = await ethers.getContractFactory('DelegateUserDecrypt');

    // TODO: update to delegate to a second contract and call it to user decrypt

    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    await this.contract.waitForDeployment();
    this.contractAddress = await this.contract.getAddress();
    this.instances = await createInstances(this.signers);
  });

  it('test delegate user decrypt ebool', async function () {
    const handle = await this.contract.xBool();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const decryptedValue = await userDecryptSingleHandle(
      handle,
      this.contractAddress,
      this.instances.alice,
      this.signers.alice,
      privateKey,
      publicKey,
    );
    expect(decryptedValue).to.equal(true);

    // on the other hand, Bob should be unable to read Alice's handle
    try {
      const { publicKey: publicKeyBob, privateKey: privateKeyBob } = this.instances.bob.generateKeypair();
      await userDecryptSingleHandle(
        handle,
        this.contractAddress,
        this.instances.bob,
        this.signers.bob,
        privateKeyBob,
        publicKeyBob,
      );
      expect.fail('Expected an error to be thrown - Bob should not be able to user decrypt Alice balance');
    } catch (error) {
      expect(error.message).to.equal(
        `User ${this.signers.bob.address} is not authorized to user decrypt handle ${handle}!`,
      );
    }

    // and should be impossible to call userDecrypt if contractAddress is in list of userAddresses
    try {
      const handleContractPairs = [
        {
          handle: handle,
          contractAddress: this.signers.alice.address, // this should be impossible, as expected by this test
        },
      ];
      const startTimeStamp = Math.floor(Date.now() / 1000).toString();
      const durationDays = '10'; // String for consistency
      const contractAddresses = [this.signers.alice.address]; // this should be impossible, as expected by this test

      // Use the new createEIP712 function
      const eip712 = this.instances.alice.createEIP712(publicKey, contractAddresses, startTimeStamp, durationDays);

      // Update the signing to match the new primaryType
      const signature = await this.signers.alice.signTypedData(
        eip712.domain,
        {
          UserDecryptRequestVerification: eip712.types.UserDecryptRequestVerification,
        },
        eip712.message,
      );

      await this.instances.alice.userDecrypt(
        handleContractPairs,
        privateKey,
        publicKey,
        signature.replace('0x', ''),
        contractAddresses,
        this.signers.alice.address,
        startTimeStamp,
        durationDays,
      );

      expect.fail('Expected an error to be thrown - userAddress and contractAddress cannot be equal');
    } catch (error) {
      expect(error.message).to.equal(
        `userAddress ${this.signers.alice.address} should not be equal to contractAddress when requesting user decryption!`,
      );
    }
  });
