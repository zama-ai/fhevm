import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { userDecryptSingleHandle } from '../utils';

describe('User decryption', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    const contractFactory = await ethers.getContractFactory('UserDecrypt');

    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    await this.contract.waitForDeployment();
    this.contractAddress = await this.contract.getAddress();
    this.instances = await createInstances(this.signers);
  });

  it('test user decrypt ebool', async function () {
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
  });

  it('test user decrypt euint8', async function () {
    const handle = await this.contract.xUint8();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const decryptedValue = await userDecryptSingleHandle(
      handle,
      this.contractAddress,
      this.instances.alice,
      this.signers.alice,
      privateKey,
      publicKey,
    );
    expect(decryptedValue).to.equal(42n);
  });

  it('test user decrypt euint16', async function () {
    const handle = await this.contract.xUint16();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const decryptedValue = await userDecryptSingleHandle(
      handle,
      this.contractAddress,
      this.instances.alice,
      this.signers.alice,
      privateKey,
      publicKey,
    );
    expect(decryptedValue).to.equal(16n);
  });

  it('test user decrypt euint32', async function () {
    const handle = await this.contract.xUint32();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const decryptedValue = await userDecryptSingleHandle(
      handle,
      this.contractAddress,
      this.instances.alice,
      this.signers.alice,
      privateKey,
      publicKey,
    );
    expect(decryptedValue).to.equal(32n);
  });

  it('test user decrypt euint64', async function () {
    const handle = await this.contract.xUint64();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const decryptedValue = await userDecryptSingleHandle(
      handle,
      this.contractAddress,
      this.instances.alice,
      this.signers.alice,
      privateKey,
      publicKey,
    );
    expect(decryptedValue).to.equal(18446744073709551600n);
  });

  it('test user decrypt euint128', async function () {
    const handle = await this.contract.xUint128();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const decryptedValue = await userDecryptSingleHandle(
      handle,
      this.contractAddress,
      this.instances.alice,
      this.signers.alice,
      privateKey,
      publicKey,
    );
    expect(decryptedValue).to.equal(145275933516363203950142179850024740765n);
  });

  it('test user decrypt eaddress', async function () {
    const handle = await this.contract.xAddress();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const decryptedValue = await userDecryptSingleHandle(
      handle,
      this.contractAddress,
      this.instances.alice,
      this.signers.alice,
      privateKey,
      publicKey,
    );
    expect(decryptedValue).to.equal('0x8ba1f109551bD432803012645Ac136ddd64DBA72');
  });

  it('test user decrypt euint256', async function () {
    const handle = await this.contract.xUint256();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const decryptedValue = await userDecryptSingleHandle(
      handle,
      this.contractAddress,
      this.instances.alice,
      this.signers.alice,
      privateKey,
      publicKey,
    );
    expect(decryptedValue).to.equal(74285495974541385002137713624115238327312291047062397922780925695323480915729n);
  });

  describe('negative-acl', function () {
    it('should reject when user is not allowed for handle', async function () {
      const handle = await this.contract.xBool();
      const { publicKey: publicKeyBob, privateKey: privateKeyBob } = this.instances.bob.generateKeypair();

      try {
        await userDecryptSingleHandle(
          handle,
          this.contractAddress,
          this.instances.bob,
          this.signers.bob,
          privateKeyBob,
          publicKeyBob,
        );
        expect.fail('Expected an error to be thrown - user is not allowed for handle');
      } catch (error) {
        expect(error.message).to.equal(
          `User address ${this.signers.bob.address} is not authorized to user decrypt handle ${handle}!`,
        );
      }
    });

    it('should reject when userAddress equals contractAddress', async function () {
      const handle = await this.contract.xBool();
      const { publicKey, privateKey } = this.instances.alice.generateKeypair();

      try {
        const handleContractPairs = [
          {
            handle: handle,
            contractAddress: this.signers.alice.address,
          },
        ];
        const startTimeStamp = Math.floor(Date.now() / 1000);
        const durationDays = 10;
        const contractAddresses = [this.signers.alice.address];

        const eip712 = this.instances.alice.createEIP712(publicKey, contractAddresses, startTimeStamp, durationDays);

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
          `User address ${this.signers.alice.address} should not be equal to contract address when requesting user decryption!`,
        );
      }
    });

    it('should reject when contract is not allowed for handle', async function () {
      const handle = await this.contract.xBool();
      const factory2 = await ethers.getContractFactory('HTTPPublicDecrypt');
      const contract2 = await factory2.connect(this.signers.alice).deploy();
      await contract2.waitForDeployment();
      const wrongContractAddress = await contract2.getAddress();

      const { publicKey, privateKey } = this.instances.alice.generateKeypair();
      const handleContractPairs = [{ handle, contractAddress: wrongContractAddress }];
      const startTimeStamp = Math.floor(Date.now() / 1000);
      const durationDays = 10;
      const contractAddresses = [wrongContractAddress];
      const eip712 = this.instances.alice.createEIP712(publicKey, contractAddresses, startTimeStamp, durationDays);
      const signature = await this.signers.alice.signTypedData(
        eip712.domain,
        { UserDecryptRequestVerification: eip712.types.UserDecryptRequestVerification },
        eip712.message,
      );

      try {
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
        expect.fail('Expected an error - contract should not be allowed');
      } catch (error) {
        expect(error.message).to.include('is not authorized to user decrypt handle');
      }
    });

    it('should reject when request has expired', async function () {
      const handle = await this.contract.xBool();
      const handleContractPairs = [
        {
          handle: handle,
          contractAddress: this.contractAddress,
        },
      ];
      const { publicKey, privateKey } = this.instances.alice.generateKeypair();
      const startTimeStamp = Number(BigInt(Math.floor(Date.now() / 1000)) - 20n * 86400n);
      const durationDays = 10;
      const contractAddresses = [this.contractAddress];

      const eip712 = this.instances.alice.createEIP712(publicKey, contractAddresses, startTimeStamp, durationDays);

      const signature = await this.signers.alice.signTypedData(
        eip712.domain,
        {
          UserDecryptRequestVerification: eip712.types.UserDecryptRequestVerification,
        },
        eip712.message,
      );

      try {
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
        expect.fail('Expected an error to be thrown - request should have expired');
      } catch (error) {
        expect(error.message).to.equal('User decrypt request has expired');
      }
    });
  });
});
