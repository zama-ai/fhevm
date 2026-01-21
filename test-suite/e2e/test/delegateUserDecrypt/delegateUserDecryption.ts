import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { delegatedUserDecryptSingleHandle } from '../utils';

const BLOCK_TIME_MS = 1000;

describe('Delegate user decryption', function () {
  const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    const contractFactory = await ethers.getContractFactory('DelegateUserDecryptDelegator');

    this.contractDelegator = await contractFactory.connect(this.signers.alice).deploy();
    await this.contractDelegator.waitForDeployment();
    this.contractDelegatorAddress = await this.contractDelegator.getAddress();

    const contractFactoryDelegate = await ethers.getContractFactory('DelegateUserDecryptDelegate');

    this.contractDelegate = await contractFactoryDelegate.connect(this.signers.alice).deploy();
    await this.contractDelegate.waitForDeployment();
    this.contractDelegateAddress = await this.contractDelegate.getAddress();

    // Delegate user decryption from Alice to Bob
    this.delegateAddress = this.signers.bob.address;
    await this.contractDelegator
      .connect(this.signers.alice)
      .delegateUserDecryption(this.delegateAddress, this.contractDelegateAddress);

    // Wait for 15 seconds to ensure delegation is propagated by coprocessor.
    await sleep(15 * BLOCK_TIME_MS);
  });

  it('test delegated user decrypt ebool', async function () {
    const handle = await this.contractDelegator.xBool();
    const { publicKey, privateKey } = this.instances.bob.generateKeypair();

    const decryptedValue = await delegatedUserDecryptSingleHandle(
      this.instances.bob,
      handle,
      this.contractDelegateAddress,
      this.contractDelegatorAddress,
      this.delegateAddress,
      this.signers.bob,
      privateKey,
      publicKey,
    );

    expect(decryptedValue).to.equal(true);
  });

  it('test delegated user decrypt euint8', async function () {
    const handle = await this.contractDelegator.xUint8();
    const { publicKey, privateKey } = this.instances.bob.generateKeypair();

    const decryptedValue = await delegatedUserDecryptSingleHandle(
      this.instances.bob,
      handle,
      this.contractDelegateAddress,
      this.contractDelegatorAddress,
      this.delegateAddress,
      this.signers.bob,
      privateKey,
      publicKey,
    );

    expect(decryptedValue).to.equal(42n);
  });

  it('test delegated user decrypt euint16', async function () {
    const handle = await this.contractDelegator.xUint16();
    const { publicKey, privateKey } = this.instances.bob.generateKeypair();

    const decryptedValue = await delegatedUserDecryptSingleHandle(
      this.instances.bob,
      handle,
      this.contractDelegateAddress,
      this.contractDelegatorAddress,
      this.delegateAddress,
      this.signers.bob,
      privateKey,
      publicKey,
    );

    expect(decryptedValue).to.equal(16n);
  });

  it('test delegated user decrypt euint32', async function () {
    const handle = await this.contractDelegator.xUint32();
    const { publicKey, privateKey } = this.instances.bob.generateKeypair();

    const decryptedValue = await delegatedUserDecryptSingleHandle(
      this.instances.bob,
      handle,
      this.contractDelegateAddress,
      this.contractDelegatorAddress,
      this.delegateAddress,
      this.signers.bob,
      privateKey,
      publicKey,
    );

    expect(decryptedValue).to.equal(32n);
  });

  it('test delegated user decrypt euint64', async function () {
    const handle = await this.contractDelegator.xUint64();
    const { publicKey, privateKey } = this.instances.bob.generateKeypair();

    const decryptedValue = await delegatedUserDecryptSingleHandle(
      this.instances.bob,
      handle,
      this.contractDelegateAddress,
      this.contractDelegatorAddress,
      this.delegateAddress,
      this.signers.bob,
      privateKey,
      publicKey,
    );

    expect(decryptedValue).to.equal(18446744073709551600n);
  });

  it('test delegated user decrypt euint128', async function () {
    const handle = await this.contractDelegator.xUint128();
    const { publicKey, privateKey } = this.instances.bob.generateKeypair();

    const decryptedValue = await delegatedUserDecryptSingleHandle(
      this.instances.bob,
      handle,
      this.contractDelegateAddress,
      this.contractDelegatorAddress,
      this.delegateAddress,
      this.signers.bob,
      privateKey,
      publicKey,
    );

    expect(decryptedValue).to.equal(145275933516363203950142179850024740765n);
  });

  it('test delegated user decrypt euint256', async function () {
    const handle = await this.contractDelegator.xUint256();
    const { publicKey, privateKey } = this.instances.bob.generateKeypair();

    const decryptedValue = await delegatedUserDecryptSingleHandle(
      this.instances.bob,
      handle,
      this.contractDelegateAddress,
      this.contractDelegatorAddress,
      this.delegateAddress,
      this.signers.bob,
      privateKey,
      publicKey,
    );

    expect(decryptedValue).to.equal(74285495974541385002137713624115238327312291047062397922780925695323480915729n);
  });

  it('test delegated user decrypt eaddress', async function () {
    const handle = await this.contractDelegator.xAddress();
    const { publicKey, privateKey } = this.instances.bob.generateKeypair();

    const decryptedValue = await delegatedUserDecryptSingleHandle(
      this.instances.bob,
      handle,
      this.contractDelegateAddress,
      this.contractDelegatorAddress,
      this.delegateAddress,
      this.signers.bob,
      privateKey,
      publicKey,
    );

    expect(decryptedValue).to.equal('0x8ba1f109551bD432803012645Ac136ddd64DBA72');
  });

  it('test delegated user decrypt revoked', async function () {
    const revoke = await this.contractDelegator
      .connect(this.signers.alice)
      .revokeUserDecryptionDelegation(this.signers.bob.address, this.contractDelegateAddress);
    const revoke_result = await revoke.wait(1);
    expect(revoke_result.status).to.equal(1);

    // Wait for 15 seconds to ensure revocation is propagated by coprocessor.
    await sleep(15 * BLOCK_TIME_MS);

    const handle = await this.contractDelegator.xBool();
    const { publicKey, privateKey } = this.instances.bob.generateKeypair();

    await expect(delegatedUserDecryptSingleHandle(
        this.instances.bob,
        handle,
        this.contractDelegateAddress,
        this.contractDelegatorAddress,
        this.delegateAddress,
        this.signers.bob,
        privateKey,
        publicKey,
      )).to.be.rejectedWith(
      new RegExp('Could not estimate gas'),
    );
  });
});
