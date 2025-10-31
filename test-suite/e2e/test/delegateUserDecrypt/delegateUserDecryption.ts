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
    const contractFactory = await ethers.getContractFactory('DelegateUserDecryptDelegator');

    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    await this.contract.waitForDeployment();
    this.contractAddress = await this.contract.getAddress();

    const contractFactoryDelegate = await ethers.getContractFactory('DelegateUserDecryptDelegate');

    this.contractDelegate = await contractFactoryDelegate.connect(this.signers.alice).deploy();
    await this.contractDelegate.waitForDeployment();
    this.contractDelegateAddress = await this.contractDelegate.getAddress();

    this.instances = await createInstances(this.signers);
  });

  const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
  it('test delegate user decrypt ebool', async function () {
    const block_time = 1000; // ms
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    const handle1 = await this.contract.xBool();
    const tx_use_bool = await this.contractDelegate.useBool(handle1);
    await tx_use_bool.wait(1); // wait for 1 block
    const tx_handle2 = await this.contractDelegate.getResult();
    console.log("Handle2:", tx_handle2);
    const handle2 = tx_handle2;

    const delegate = await this.contract.delegate(this.contractDelegateAddress);
    const delegate_result = await delegate.wait(1);
    expect(delegate_result.status).to.equal(1);

    await sleep( 15 * block_time); // wait for 15 seconds to ensure delegation is active

    const revoke = await this.contract.revoke(this.contractDelegateAddress);
    const revoke_result = await revoke.wait(1);
    expect(revoke_result.status).to.equal(1);
    await sleep( 15 * block_time); // wait for 15 seconds to ensure delegation is revoked

  });
});
