import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { userDecryptSingleHandle } from '../utils';

describe('Reencryption', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    const contractFactory = await ethers.getContractFactory('Reencrypt');

    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    await this.contract.waitForDeployment();
    this.contractAddress = await this.contract.getAddress();
    this.instances = await createInstances(this.signers);
  });

  it('test reencrypt ebool', async function () {
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
    expect(decryptedValue).to.equal(1n);

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
      expect.fail('Expected an error to be thrown - Bob should not be able to reencrypt Alice balance');
    } catch (error) {
      expect(error.message).to.equal('User is not authorized to reencrypt this handle!');
    }

    // and should be impossible to call reencrypt if contractAddress is in list of userAddresses
    try {
      const ctHandleContractPairs = [
        {
          ctHandle: handle,
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
        { UserDecryptRequestVerification: eip712.types.UserDecryptRequestVerification },
        eip712.message,
      );

      await this.instances.alice.userDecrypt(
        ctHandleContractPairs,
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
        'userAddress should not be equal to contractAddress when requesting reencryption!',
      );
    }
  });

  it('test reencrypt euint8', async function () {
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

  it('test reencrypt euint16', async function () {
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

  it('test reencrypt euint32', async function () {
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

  it('test reencrypt euint64', async function () {
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

  it('test reencrypt euint128', async function () {
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

  it('test reencrypt eaddress', async function () {
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
    expect(decryptedValue).to.equal(BigInt('0x8ba1f109551bD432803012645Ac136ddd64DBA72'));
  });

  it('test reencrypt euint256', async function () {
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

  it('test reencrypt ebytes64', async function () {
    const handle = await this.contract.yBytes64();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const decryptedValue = await userDecryptSingleHandle(
      handle,
      this.contractAddress,
      this.instances.alice,
      this.signers.alice,
      privateKey,
      publicKey,
    );
    expect(decryptedValue).to.equal(
      BigInt(
        '0x19d179e0cc7e816dc944582ed4f5652f5951900098fc2e0a15a7ea4dc8cfa4e3b6c54beea5ee95e56b728762f659347ce1d4aa1b05fcc5',
      ),
    );
  });

  it('test reencrypt ebytes128', async function () {
    const handle = await this.contract.yBytes128();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const decryptedValue = await userDecryptSingleHandle(
      handle,
      this.contractAddress,
      this.instances.alice,
      this.signers.alice,
      privateKey,
      publicKey,
    );
    expect(decryptedValue).to.equal(
      BigInt(
        '0x13e7819123de6e2870c7e83bb764508e22d7c3ab8a5aee6bdfb26355ef0d3f1977d651b83bf5f78634fa360aa14debdc3daa6a587b5c2fb1710ab4d6677e62a8577f2d9fecc190ad8b11c9f0a5ec3138b27da1f055437af8c90a9495dad230',
      ),
    );
  });

  it('test reencrypt ebytes256', async function () {
    const handle = await this.contract.yBytes256();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const decryptedValue = await userDecryptSingleHandle(
      handle,
      this.contractAddress,
      this.instances.alice,
      this.signers.alice,
      privateKey,
      publicKey,
    );
    expect(decryptedValue).to.equal(
      BigInt(
        '0xd179e0cc7e816dc944582ed4f5652f5951900098fc2e0a15a7ea4dc8cfa4e3b6c54beea5ee95e56b728762f659347ce1d4aa1b05fcc513e7819123de6e2870c7e83bb764508e22d7c3ab8a5aee6bdfb26355ef0d3f1977d651b83bf5f78634fa360aa14debdc3daa6a587b5c2fb1710ab4d6677e62a8577f2d9fecc190ad8b11c9f0a5ec3138b27da1f055437af8c90a9495dad230',
      ),
    );
  });
});
