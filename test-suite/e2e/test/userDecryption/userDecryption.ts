import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

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
    const decryptedValue = await this.instances.alice.userDecryptSingleHandle({
      handle,
      contractAddress: this.contractAddress,
      signer: this.signers.alice,
    });
    expect(decryptedValue).to.equal(true);
  });

  it('test user decrypt euint8', async function () {
    const handle = await this.contract.xUint8();
    const decryptedValue = await this.instances.alice.userDecryptSingleHandle({
      handle,
      contractAddress: this.contractAddress,
      signer: this.signers.alice,
    });
    expect(decryptedValue).to.equal(42n);
  });

  it('test user decrypt euint16', async function () {
    const handle = await this.contract.xUint16();
    const decryptedValue = await this.instances.alice.userDecryptSingleHandle({
      handle,
      contractAddress: this.contractAddress,
      signer: this.signers.alice,
    });
    expect(decryptedValue).to.equal(16n);
  });

  it('test user decrypt euint32', async function () {
    const handle = await this.contract.xUint32();
    const decryptedValue = await this.instances.alice.userDecryptSingleHandle({
      handle,
      contractAddress: this.contractAddress,
      signer: this.signers.alice,
    });
    expect(decryptedValue).to.equal(32n);
  });

  it('test user decrypt euint64', async function () {
    const handle = await this.contract.xUint64();
    const decryptedValue = await this.instances.alice.userDecryptSingleHandle({
      handle,
      contractAddress: this.contractAddress,
      signer: this.signers.alice,
    });
    expect(decryptedValue).to.equal(18446744073709551600n);
  });

  it('test user decrypt euint128', async function () {
    const handle = await this.contract.xUint128();
    const decryptedValue = await this.instances.alice.userDecryptSingleHandle({
      handle,
      contractAddress: this.contractAddress,
      signer: this.signers.alice,
    });
    expect(decryptedValue).to.equal(145275933516363203950142179850024740765n);
  });

  it('test user decrypt eaddress', async function () {
    const handle = await this.contract.xAddress();
    const decryptedValue = await this.instances.alice.userDecryptSingleHandle({
      handle,
      contractAddress: this.contractAddress,
      signer: this.signers.alice,
    });
    expect(decryptedValue).to.equal('0x8ba1f109551bD432803012645Ac136ddd64DBA72');
  });

  it('test user decrypt euint256', async function () {
    const handle = await this.contract.xUint256();
    const decryptedValue = await this.instances.alice.userDecryptSingleHandle({
      handle,
      contractAddress: this.contractAddress,
      signer: this.signers.alice,
    });
    expect(decryptedValue).to.equal(74285495974541385002137713624115238327312291047062397922780925695323480915729n);
  });

  describe('negative-acl', function () {
    it('should reject when user is not allowed for handle', async function () {
      const handle = await this.contract.xBool();

      try {
        await this.instances.bob.userDecryptSingleHandle({
          handle,
          contractAddress: this.contractAddress,
          signer: this.signers.bob,
        });

        expect.fail('Expected an error to be thrown - user is not allowed for handle');
      } catch (error) {
        expect((error as { message: string }).message).to.contain(
          this.instances.bob.getUserDecryptErrorMessage({
            type: 'user-unauthorized',
            signer: this.signers.bob,
            handle,
          }),
        );
      }
    });

    it('should reject when userAddress equals contractAddress', async function () {
      const handle = await this.contract.xBool();
      try {
        await this.instances.alice.userDecryptSingleHandle({
          contractAddress: this.signers.alice.address,
          handle,
          signer: this.signers.alice,
        });
        expect.fail('Expected an error to be thrown - userAddress and contractAddress cannot be equal');
      } catch (error) {
        expect((error as { message: string }).message).to.contain(
          this.instances.alice.getUserDecryptErrorMessage({
            type: 'user-equal-contract',
            signer: this.signers.alice,
          }),
        );
      }
    });

    it('should reject when contract is not allowed for handle', async function () {
      const handle = await this.contract.xBool();
      const factory2 = await ethers.getContractFactory('HTTPPublicDecrypt');
      const contract2 = await factory2.connect(this.signers.alice).deploy();
      await contract2.waitForDeployment();
      const wrongContractAddress = await contract2.getAddress();

      try {
        await this.instances.alice.userDecryptSingleHandle({
          contractAddress: wrongContractAddress,
          handle,
          signer: this.signers.alice,
        });
        expect.fail('Expected an error - contract should not be allowed');
      } catch (error) {
        expect((error as { message: string }).message).to.include(
          this.instances.alice.getUserDecryptErrorMessage({
            type: 'contract-unauthorized',
            signer: this.signers.alice,
            contractAddress: wrongContractAddress,
            handle,
          }),
        );
      }
    });

    it('should reject when request has expired', async function () {
      const handle = await this.contract.xBool();
      try {
        await this.instances.alice.userDecryptSingleHandle({
          contractAddress: this.contractAddress,
          handle,
          signer: this.signers.alice,
          startTimestamp: Number(BigInt(Math.floor(Date.now() / 1000)) - 20n * 86400n),
        });
        expect.fail('Expected an error to be thrown - request should have expired');
      } catch (error) {
        expect((error as { message: string }).message).to.include('request has expired');
      }
    });
  });
});
