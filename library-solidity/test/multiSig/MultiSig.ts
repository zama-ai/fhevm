import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import type { FhevmInstance } from '@zama-fhe/relayer-sdk/node';
import { expect } from 'chai';
import dotenv from 'dotenv';
import fs from 'fs';
import hre from 'hardhat';

import type { EncryptedSetter } from '../../typechain-types';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { userDecryptSingleHandle } from '../utils';
import { deploySimpleMultiSigFixture } from './MultiSig.fixture';

/**
 * Feeds a genuine user input to `setter`, which then stores `encryptedResult64 = clearValue + 42` and
 * allows both itself and `signer` to use that result.
 * @returns the handle of the stored result, stemming from a computation.
 */
async function computeResult64FromUserInput(
  setter: EncryptedSetter,
  instance: FhevmInstance,
  signer: HardhatEthersSigner,
  clearValue: number | bigint,
): Promise<string> {
  const input = instance.createEncryptedInput(await setter.getAddress(), signer.address);
  input.add64(clearValue);
  const encryptedValue = await input.encrypt();
  const tx = await setter.connect(signer).computeResult64(encryptedValue.handles[0], encryptedValue.inputProof);
  await tx.wait();

  return setter.encryptedResult64();
}

/**
 * Feeds a genuine `euint32` user input to `setter`, which stores the verified handle as
 * `encryptedValue32` and grants the requested persistent rights on it.
 * @returns the handle of the verified user input, of type `FheType.Uint32`.
 */
async function setEncryptedValue32FromUserInput(
  setter: EncryptedSetter,
  instance: FhevmInstance,
  signer: HardhatEthersSigner,
  clearValue: number | bigint,
  allowSender: boolean,
  allowContract: boolean,
): Promise<string> {
  const input = instance.createEncryptedInput(await setter.getAddress(), signer.address);
  input.add32(clearValue);
  const encryptedValue = await input.encrypt();
  const tx = await setter
    .connect(signer)
    .setEncryptedValue32(encryptedValue.handles[0], encryptedValue.inputProof, allowSender, allowContract);
  await tx.wait();

  return setter.encryptedValue32();
}

/**
 * Feeds a genuine `euint64` user input to `setter`, which stores the verified handle as
 * `encryptedValue64` and grants the requested persistent rights on it.
 * @returns the handle of the verified user input, of type `FheType.Uint64`.
 */
async function setEncryptedValue64FromUserInput(
  setter: EncryptedSetter,
  instance: FhevmInstance,
  signer: HardhatEthersSigner,
  clearValue: number | bigint,
  allowSender: boolean,
  allowContract: boolean,
): Promise<string> {
  const input = instance.createEncryptedInput(await setter.getAddress(), signer.address);
  input.add64(clearValue);
  const encryptedValue = await input.encrypt();
  const tx = await setter
    .connect(signer)
    .setEncryptedValue64(encryptedValue.handles[0], encryptedValue.inputProof, allowSender, allowContract);
  await tx.wait();

  return setter.encryptedValue64();
}

describe('MultiSig', function () {
  before(async function () {
    await initSigners(3);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deploySimpleMultiSigFixture();
    this.contractAddress = await contract.getAddress();
    this.multiSig = contract;
    const helperFactory = await hre.ethers.getContractFactory('MultiSigHelper');
    this.helper = await helperFactory.deploy(this.contractAddress);
    this.instances = await createInstances(this.signers);
    const setterFactory = await hre.ethers.getContractFactory('EncryptedSetter');
    this.setter = await setterFactory.deploy();
  });

  it('should deploy SimpleMultiSig contract', async function () {
    const owners = await this.multiSig.getOwners();
    expect(owners).to.deep.equal([this.signers.alice.address, this.signers.bob.address, this.signers.carol.address]);
  });

  it('should use helper to make input readable by owners, then allow setter, then use handle in setter via multisig, then allow result to owners to make it readable by owners', async function () {
    const helperAddress = await this.helper.getAddress();
    const input = this.instances.alice.createEncryptedInput(helperAddress, this.signers.alice.address);
    const clearValue = 133799;
    input.add64(clearValue);
    const encryptedValue = await input.encrypt();
    const tx = await this.helper.allowForMultiSig(encryptedValue.handles[0], encryptedValue.inputProof);
    await tx.wait();

    // now check that all 3 owners can user-decrypt the encryptedValue:
    const { publicKey: publicKeyAlice, privateKey: privateKeyAlice } = this.instances.alice.generateKeypair();
    const aliceDecrypted = await userDecryptSingleHandle(
      encryptedValue.handles[0],
      this.contractAddress,
      this.instances.alice,
      this.signers.alice,
      privateKeyAlice,
      publicKeyAlice,
    );
    expect(aliceDecrypted).to.equal(clearValue);
    const { publicKey: publicKeyBob, privateKey: privateKeyBob } = this.instances.bob.generateKeypair();
    const bobDecrypted = await userDecryptSingleHandle(
      encryptedValue.handles[0],
      this.contractAddress,
      this.instances.bob,
      this.signers.bob,
      privateKeyBob,
      publicKeyBob,
    );
    expect(bobDecrypted).to.equal(clearValue);
    const { publicKey: publicKeyCarol, privateKey: privateKeyCarol } = this.instances.carol.generateKeypair();
    const carolDecrypted = await userDecryptSingleHandle(
      encryptedValue.handles[0],
      this.contractAddress,
      this.instances.carol,
      this.signers.carol,
      privateKeyCarol,
      publicKeyCarol,
    );
    expect(carolDecrypted).to.equal(clearValue);

    // now either any allowed owner OR the multisig (via a proposal) should allow handle to EncryptedSetter (an owner allowing is simpler):
    const ifaceACL = new hre.ethers.Interface([
      'function allow(bytes32 handle, address account)',
      'function multicall(bytes[] calldata data)',
    ]);
    const aclAddress = dotenv.parse(fs.readFileSync('fhevmTemp/addresses/.env.host')).ACL_CONTRACT_ADDRESS;
    const acl = new hre.ethers.Contract(aclAddress, ifaceACL, this.signers.alice);
    await acl.allow(encryptedValue.handles[0], this.setter);

    // now the multisig can finally use this handle and send it to EncryptedSetter contract;
    const ifaceSetter = new hre.ethers.Interface(['function computeResult64(bytes32 inputHandle, bytes inputProof)']);
    const calldata2 = ifaceSetter.encodeFunctionData('computeResult64', [
      encryptedValue.handles[0],
      '0x', // use an empty bytes array for inputProof, because handle has already been verified and allowed to multiSig
    ]);
    await this.multiSig.proposeTx(await this.setter.getAddress(), calldata2); // alice propose tx
    await this.multiSig.connect(this.signers.bob).approveTx(1); // bob approves
    await this.multiSig.connect(this.signers.carol).approveTx(1); // carol approves
    await this.multiSig.executeTx(1); // anyone can execute it finally

    // to make the resulting handle readable by owners, we still need to allow it to them via the multiSig:
    const handleResult = await this.setter.encryptedResult64();
    const multicalldata1 = ifaceACL.encodeFunctionData('allow', [handleResult, this.signers.alice.address]);
    const multicalldata2 = ifaceACL.encodeFunctionData('allow', [handleResult, this.signers.bob.address]);
    const multicalldata3 = ifaceACL.encodeFunctionData('allow', [handleResult, this.signers.carol.address]);
    const multicalldataAll = ifaceACL.encodeFunctionData('multicall', [
      [multicalldata1, multicalldata2, multicalldata3],
    ]);
    await this.multiSig.proposeTx(aclAddress, multicalldataAll); // alice propose tx
    await this.multiSig.connect(this.signers.bob).approveTx(2); // bob approves
    await this.multiSig.connect(this.signers.carol).approveTx(2); // carol approves
    await this.multiSig.executeTx(2); // anyone can execute it finally

    // finally all owners can user-decrypt the result:
    const aliceDecrypted2 = await userDecryptSingleHandle(
      handleResult,
      await this.setter.getAddress(),
      this.instances.alice,
      this.signers.alice,
      privateKeyAlice,
      publicKeyAlice,
    );
    expect(aliceDecrypted2).to.equal(clearValue + 42); // because the setter adds 42 to the encrypted input value
    const bobDecrypted2 = await userDecryptSingleHandle(
      handleResult,
      await this.setter.getAddress(),
      this.instances.bob,
      this.signers.bob,
      privateKeyBob,
      publicKeyBob,
    );
    expect(bobDecrypted2).to.equal(clearValue + 42); // because the setter adds 42 to the encrypted input value
    const carolDecrypted2 = await userDecryptSingleHandle(
      handleResult,
      await this.setter.getAddress(),
      this.instances.carol,
      this.signers.carol,
      privateKeyCarol,
      publicKeyCarol,
    );
    expect(carolDecrypted2).to.equal(clearValue + 42); // because the setter adds 42 to the encrypted input value
  });

  it('should be able to use an uninitialized handle in the setter', async function () {
    const setterFactory = await hre.ethers.getContractFactory('EncryptedSetter');
    const setter2 = await setterFactory.deploy();
    await this.multiSig.executeSpecialTx(await setter2.getAddress());

    const aclAddress = dotenv.parse(fs.readFileSync('fhevmTemp/addresses/.env.host')).ACL_CONTRACT_ADDRESS;
    const ifaceACL = new hre.ethers.Interface([
      'function allow(bytes32 handle, address account)',
      'function multicall(bytes[] calldata data)',
    ]);

    const handleResult = await setter2.encryptedResult64();
    const multicalldata1 = ifaceACL.encodeFunctionData('allow', [handleResult, this.signers.alice.address]);
    const multicalldata2 = ifaceACL.encodeFunctionData('allow', [handleResult, this.signers.bob.address]);
    const multicalldata3 = ifaceACL.encodeFunctionData('allow', [handleResult, this.signers.carol.address]);
    const multicalldataAll = ifaceACL.encodeFunctionData('multicall', [
      [multicalldata1, multicalldata2, multicalldata3],
    ]);
    await this.multiSig.proposeTx(aclAddress, multicalldataAll); // alice propose tx
    await this.multiSig.connect(this.signers.bob).approveTx(1); // bob approves
    await this.multiSig.connect(this.signers.carol).approveTx(1); // carol approves
    await this.multiSig.executeTx(1); // anyone can execute it finally

    // finally all owners can user-decrypt the result:
    const { publicKey: publicKeyAlice, privateKey: privateKeyAlice } = this.instances.alice.generateKeypair();
    const aliceDecrypted = await userDecryptSingleHandle(
      handleResult,
      await setter2.getAddress(),
      this.instances.alice,
      this.signers.alice,
      privateKeyAlice,
      publicKeyAlice,
    );
    expect(aliceDecrypted).to.equal(42); // because the setter adds 42 to 0 (the uninitialized input)

    const { publicKey: publicKeyBob, privateKey: privateKeyBob } = this.instances.bob.generateKeypair();
    const bobDecrypted = await userDecryptSingleHandle(
      handleResult,
      await setter2.getAddress(),
      this.instances.bob,
      this.signers.bob,
      privateKeyBob,
      publicKeyBob,
    );
    expect(bobDecrypted).to.equal(42); // because the setter adds 42 to 0 (the uninitialized input)

    const { publicKey: publicKeyCarol, privateKey: privateKeyCarol } = this.instances.carol.generateKeypair();
    const carolDecrypted = await userDecryptSingleHandle(
      handleResult,
      await setter2.getAddress(),
      this.instances.carol,
      this.signers.carol,
      privateKeyCarol,
      publicKeyCarol,
    );
    expect(carolDecrypted).to.equal(42); // because the setter adds 42 to 0 (the uninitialized input)
  });

  it('should revert when the sender is allowed to use the handle but it is not a valid external handle', async function () {
    // Alice feeds a genuine user input to the setter, hence she is allowed to use the result.
    const computedHandle64 = await computeResult64FromUserInput(
      this.setter,
      this.instances.alice,
      this.signers.alice,
      1,
    );

    // The result is a handle stemming from a computation, hence its input index (byte21) is the
    // reserved `0xff` value, which never identifies a user input.
    const handleBytes = hre.ethers.getBytes(computedHandle64);
    expect(handleBytes[21]).to.equal(0xff);
    expect(handleBytes[30]).to.equal(5);

    // Alice is allowed to use it and the setter is allowed to compute on it, so the ACL checks all
    // pass: `_checkExternalHandle` is the only thing rejecting the call. Without it, `fromExternal`
    // would launder a computed handle into a fresh, unverified user input.
    await expect(this.setter.computeResult64(computedHandle64, '0x'))
      .to.be.revertedWithCustomError(
        { interface: new hre.ethers.Interface(['error InvalidExternalHandle(bytes32)']) },
        'InvalidExternalHandle',
      )
      .withArgs(computedHandle64);
  });

  it('should revert when the external handle type does not match the expected one', async function () {
    // Alice feeds a genuine euint32 user input to the setter, which keeps the verified handle and
    // allows her only!
    const userInputHandle32 = await setEncryptedValue32FromUserInput(
      this.setter,
      this.instances.alice,
      this.signers.alice,
      1,
      true, // allowSender
      false, // allowContract
    );

    // Being a user input, its input index (byte21) is a valid one, and Alice is
    // allowed to use it. Hence its type (byte30, `FheType.Uint32` = 4) is the only thing that does
    // not match the `externalEuint64` expected by `computeResult64`.
    const handleBytes = hre.ethers.getBytes(userInputHandle32);
    expect(handleBytes[21]).to.equal(0);
    expect(handleBytes[30]).to.equal(4);

    // Without `_checkExternalHandle`, `fromExternal` would reinterpret this euint32 handle as an
    // euint64 one, and the call would go through.
    await expect(this.setter.computeResult64(userInputHandle32, '0x'))
      .to.be.revertedWithCustomError(
        { interface: new hre.ethers.Interface(['error InvalidExternalHandle(bytes32)']) },
        'InvalidExternalHandle',
      )
      .withArgs(userInputHandle32);
  });

  it('should revert when the external handle is valid but the sender is not allowed to use it', async function () {
    // Alice feeds a genuine euint64 user input to the setter, which keeps the verified handle but
    // grants the rights to itself only.
    const userInputHandle64 = await setEncryptedValue64FromUserInput(
      this.setter,
      this.instances.alice,
      this.signers.alice,
      1,
      false, // allowSender
      true, // allowContract
    );

    // Its type (byte30, `FheType.Uint64` = 5) and its input index (byte21) both match what
    // `computeResult64` expects, so it clears `_checkExternalHandle` entirely.
    const handleBytes = hre.ethers.getBytes(userInputHandle64);
    expect(handleBytes[21]).to.equal(0);
    expect(handleBytes[30]).to.equal(5);

    // Only the ACL check is left to reject the call.
    await expect(this.setter.computeResult64(userInputHandle64, '0x'))
      .to.be.revertedWithCustomError(
        { interface: new hre.ethers.Interface(['error SenderNotAllowedToUseHandle(bytes32,address)']) },
        'SenderNotAllowedToUseHandle',
      )
      .withArgs(userInputHandle64, this.signers.alice.address);
  });
});
