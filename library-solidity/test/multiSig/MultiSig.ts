import { expect } from 'chai';
import dotenv from 'dotenv';
import fs from 'fs';
import hre from 'hardhat';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { HOST_ADDRESSES_ENV_PATH, userDecryptSingleEuint } from '../utils';
import { deploySimpleMultiSigFixture } from './MultiSig.fixture';
import type { EncryptedSetter, MultiSigHelper, SimpleMultiSig } from '../../typechain-types';
import type { Signers } from '../signers';
import type { FhevmInstances } from '../types';

describe('MultiSig', function () {
  let multiSig: SimpleMultiSig;
  let helper: MultiSigHelper;
  let signers: Signers;
  let setter: EncryptedSetter;
  let instances: FhevmInstances;

  before(async function () {
    await initSigners(3);
    signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deploySimpleMultiSigFixture();
    this.contractAddress = await contract.getAddress();
    multiSig = contract;
    const helperFactory = await hre.ethers.getContractFactory('MultiSigHelper');
    helper = await helperFactory.deploy(this.contractAddress);
    instances = await createInstances(signers);
    const setterFactory = await hre.ethers.getContractFactory('EncryptedSetter');
    setter = await setterFactory.deploy();
  });

  it('should deploy SimpleMultiSig contract', async function () {
    const owners = await multiSig.getOwners();
    expect(owners).to.deep.equal([signers.alice.address, signers.bob.address, signers.carol.address]);
  });

  it('should use helper to make input readable by owners, then allow setter, then use handle in setter via multisig, then allow result to owners to make it readable by owners', async function () {
    const helperAddress = await helper.getAddress();
    const input = instances.alice.createEncryptedInput(helperAddress, signers.alice.address);
    const clearValue = 133799;
    input.add64(clearValue);
    const encryptedValue = await input.encrypt();
    const tx = await helper.allowForMultiSig(encryptedValue.handles[0], encryptedValue.inputProof);
    await tx.wait();

    // now check that all 3 owners can user-decrypt the encryptedValue:
    const { publicKey: publicKeyAlice, privateKey: privateKeyAlice } = instances.alice.generateKeypair();
    const aliceDecrypted = await userDecryptSingleEuint(
      encryptedValue.handles[0],
      this.contractAddress,
      instances.alice,
      signers.alice,
      privateKeyAlice,
      publicKeyAlice,
    );
    expect(aliceDecrypted).to.equal(clearValue);
    const { publicKey: publicKeyBob, privateKey: privateKeyBob } = instances.bob.generateKeypair();
    const bobDecrypted = await userDecryptSingleEuint(
      encryptedValue.handles[0],
      this.contractAddress,
      instances.bob,
      signers.bob,
      privateKeyBob,
      publicKeyBob,
    );
    expect(bobDecrypted).to.equal(clearValue);
    const { publicKey: publicKeyCarol, privateKey: privateKeyCarol } = instances.carol.generateKeypair();
    const carolDecrypted = await userDecryptSingleEuint(
      encryptedValue.handles[0],
      this.contractAddress,
      instances.carol,
      signers.carol,
      privateKeyCarol,
      publicKeyCarol,
    );
    expect(carolDecrypted).to.equal(clearValue);

    // now either any allowed owner OR the multisig (via a proposal) should allow handle to EncryptedSetter (an owner allowing is simpler):
    const ifaceACL = new hre.ethers.Interface([
      'function allow(bytes32 handle, address account)',
      'function multicall(bytes[] calldata data)',
    ]);
    const aclAddress = dotenv.parse(fs.readFileSync(HOST_ADDRESSES_ENV_PATH)).ACL_CONTRACT_ADDRESS;
    const acl = new hre.ethers.Contract(aclAddress, ifaceACL, signers.alice);
    await acl.allow(encryptedValue.handles[0], setter);

    // now the multisig can finally use this handle and send it to EncryptedSetter contract;
    const ifaceSetter = new hre.ethers.Interface(['function setEncryptedValue(bytes32 inputHandle, bytes inputProof)']);
    const calldata2 = ifaceSetter.encodeFunctionData('setEncryptedValue', [
      encryptedValue.handles[0],
      '0x', // use an empty bytes array for inputProof, because handle has already been verified and allowed to multiSig
    ]);
    await multiSig.proposeTx(await setter.getAddress(), calldata2); // alice propose tx
    await multiSig.connect(signers.bob).approveTx(1); // bob approves
    await multiSig.connect(signers.carol).approveTx(1); // carol approves
    await multiSig.executeTx(1); // anyone can execute it finally

    // to make the resulting handle readable by owners, we still need to allow it to them via the multiSig:
    const handleResult = await setter.encryptedResult();
    const multicalldata1 = ifaceACL.encodeFunctionData('allow', [handleResult, signers.alice.address]);
    const multicalldata2 = ifaceACL.encodeFunctionData('allow', [handleResult, signers.bob.address]);
    const multicalldata3 = ifaceACL.encodeFunctionData('allow', [handleResult, signers.carol.address]);
    const multicalldataAll = ifaceACL.encodeFunctionData('multicall', [
      [multicalldata1, multicalldata2, multicalldata3],
    ]);
    await multiSig.proposeTx(aclAddress, multicalldataAll); // alice propose tx
    await multiSig.connect(signers.bob).approveTx(2); // bob approves
    await multiSig.connect(signers.carol).approveTx(2); // carol approves
    await multiSig.executeTx(2); // anyone can execute it finally

    // finally all owners can user-decrypt the result:
    const aliceDecrypted2 = await userDecryptSingleEuint(
      handleResult,
      await setter.getAddress(),
      instances.alice,
      signers.alice,
      privateKeyAlice,
      publicKeyAlice,
    );
    expect(aliceDecrypted2).to.equal(clearValue + 42); // because the setter adds 42 to the encrypted input value
    const bobDecrypted2 = await userDecryptSingleEuint(
      handleResult,
      await setter.getAddress(),
      instances.bob,
      signers.bob,
      privateKeyBob,
      publicKeyBob,
    );
    expect(bobDecrypted2).to.equal(clearValue + 42); // because the setter adds 42 to the encrypted input value
    const carolDecrypted2 = await userDecryptSingleEuint(
      handleResult,
      await setter.getAddress(),
      instances.carol,
      signers.carol,
      privateKeyCarol,
      publicKeyCarol,
    );
    expect(carolDecrypted2).to.equal(clearValue + 42); // because the setter adds 42 to the encrypted input value
  });

  it('should be able to use an uninitialized handle in the setter', async function () {
    const setterFactory = await hre.ethers.getContractFactory('EncryptedSetter');
    const setter2 = (await setterFactory.deploy()) as unknown as EncryptedSetter;
    await multiSig.executeSpecialTx(await setter2.getAddress());

    const aclAddress = dotenv.parse(fs.readFileSync(HOST_ADDRESSES_ENV_PATH)).ACL_CONTRACT_ADDRESS;
    const ifaceACL = new hre.ethers.Interface([
      'function allow(bytes32 handle, address account)',
      'function multicall(bytes[] calldata data)',
    ]);

    const handleResult = await setter2.encryptedResult();
    const multicalldata1 = ifaceACL.encodeFunctionData('allow', [handleResult, signers.alice.address]);
    const multicalldata2 = ifaceACL.encodeFunctionData('allow', [handleResult, signers.bob.address]);
    const multicalldata3 = ifaceACL.encodeFunctionData('allow', [handleResult, signers.carol.address]);
    const multicalldataAll = ifaceACL.encodeFunctionData('multicall', [
      [multicalldata1, multicalldata2, multicalldata3],
    ]);
    await multiSig.proposeTx(aclAddress, multicalldataAll); // alice propose tx
    await multiSig.connect(signers.bob).approveTx(1); // bob approves
    await multiSig.connect(signers.carol).approveTx(1); // carol approves
    await multiSig.executeTx(1); // anyone can execute it finally

    // finally all owners can user-decrypt the result:
    const { publicKey: publicKeyAlice, privateKey: privateKeyAlice } = instances.alice.generateKeypair();
    const aliceDecrypted = await userDecryptSingleEuint(
      handleResult,
      await setter2.getAddress(),
      instances.alice,
      signers.alice,
      privateKeyAlice,
      publicKeyAlice,
    );
    expect(aliceDecrypted).to.equal(42); // because the setter adds 42 to 0 (the uninitialized input)

    const { publicKey: publicKeyBob, privateKey: privateKeyBob } = instances.bob.generateKeypair();
    const bobDecrypted = await userDecryptSingleEuint(
      handleResult,
      await setter2.getAddress(),
      instances.bob,
      signers.bob,
      privateKeyBob,
      publicKeyBob,
    );
    expect(bobDecrypted).to.equal(42); // because the setter adds 42 to 0 (the uninitialized input)

    const { publicKey: publicKeyCarol, privateKey: privateKeyCarol } = instances.carol.generateKeypair();
    const carolDecrypted = await userDecryptSingleEuint(
      handleResult,
      await setter2.getAddress(),
      instances.carol,
      signers.carol,
      privateKeyCarol,
      publicKeyCarol,
    );
    expect(carolDecrypted).to.equal(42); // because the setter adds 42 to 0 (the uninitialized input)
  });
});
