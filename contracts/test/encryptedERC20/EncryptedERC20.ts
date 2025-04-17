import { assert, expect } from 'chai';
import hre from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { deployEncryptedERC20Fixture } from './EncryptedERC20.fixture';

describe('EncryptedERC20', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployEncryptedERC20Fixture();
    this.contractAddress = await contract.getAddress();
    this.erc20 = contract;
    this.instances = await createInstances(this.signers);
  });

  it('should mint the contract', async function () {
    const transaction = await this.erc20.mint(1000);
    await transaction.wait();

    // Reencrypt Alice's balance
    const balanceHandleAlice = await this.erc20.balanceOf(this.signers.alice);

    // Balance handle is deterministic so we can verify the last bytes of the handle
    // Byte 21 was set to 0xff.
    expect(balanceHandleAlice.slice(44, 46)).to.eq('ff');
    // Bytes 22-29 must be the chainId
    const chainId = process.env.SOLIDITY_COVERAGE === 'true' ? 31337 : hre.network.config.chainId;
    assert(chainId, 'Host chainId not set');
    expect(balanceHandleAlice.slice(46, 62)).to.eq(chainId.toString(16).padStart(16, '0'));
    // Byte30: type is euint64 (so position 5 in the FheType enum)
    expect(balanceHandleAlice.slice(62, 64)).to.eq('05');
    // Byte31: handle version is 0
    expect(balanceHandleAlice.slice(64, 66)).to.eq('00');

    const { publicKey: publicKeyAlice, privateKey: privateKeyAlice } = this.instances.alice.generateKeypair();
    const eip712 = this.instances.alice.createEIP712(publicKeyAlice, this.contractAddress);
    const signatureAlice = await this.signers.alice.signTypedData(
      eip712.domain,
      { Reencrypt: eip712.types.Reencrypt },
      eip712.message,
    );

    const balanceAlice = await this.instances.alice.reencrypt(
      balanceHandleAlice,
      privateKeyAlice,
      publicKeyAlice,
      signatureAlice.replace('0x', ''),
      this.contractAddress,
      this.signers.alice.address,
    );

    expect(balanceAlice).to.equal(1000);

    const totalSupply = await this.erc20.totalSupply();
    expect(totalSupply).to.equal(1000);
  });

  it('should transfer tokens between two users', async function () {
    const transaction = await this.erc20.mint(10000);
    const t1 = await transaction.wait();
    expect(t1?.status).to.eq(1);

    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add64(1337);

    const encryptedTransferAmount = await input.encrypt();

    const tx = await this.erc20['transfer(address,bytes32,bytes)'](
      this.signers.bob.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );
    const t2 = await tx.wait();
    expect(t2?.status).to.eq(1);

    // Reencrypt Alice's balance
    const balanceHandleAlice = await this.erc20.balanceOf(this.signers.alice);
    const { publicKey: publicKeyAlice, privateKey: privateKeyAlice } = this.instances.alice.generateKeypair();
    const eip712 = this.instances.alice.createEIP712(publicKeyAlice, this.contractAddress);
    const signatureAlice = await this.signers.alice.signTypedData(
      eip712.domain,
      { Reencrypt: eip712.types.Reencrypt },
      eip712.message,
    );
    const balanceAlice = await this.instances.alice.reencrypt(
      balanceHandleAlice,
      privateKeyAlice,
      publicKeyAlice,
      signatureAlice.replace('0x', ''),
      this.contractAddress,
      this.signers.alice.address,
    );

    expect(balanceAlice).to.equal(10000 - 1337);

    // Reencrypt Bob's balance
    const balanceHandleBob = await this.erc20.balanceOf(this.signers.bob);

    const { publicKey: publicKeyBob, privateKey: privateKeyBob } = this.instances.bob.generateKeypair();
    const eip712Bob = this.instances.bob.createEIP712(publicKeyBob, this.contractAddress);
    const signatureBob = await this.signers.bob.signTypedData(
      eip712Bob.domain,
      { Reencrypt: eip712Bob.types.Reencrypt },
      eip712Bob.message,
    );
    const balanceBob = await this.instances.bob.reencrypt(
      balanceHandleBob,
      privateKeyBob,
      publicKeyBob,
      signatureBob.replace('0x', ''),
      this.contractAddress,
      this.signers.bob.address,
    );

    expect(balanceBob).to.equal(1337);

    // on the other hand, Bob should be unable to read Alice's balance
    try {
      await this.instances.bob.reencrypt(
        balanceHandleAlice,
        privateKeyBob,
        publicKeyBob,
        signatureBob.replace('0x', ''),
        this.contractAddress,
        this.signers.bob.address,
      );
      expect.fail('Expected an error to be thrown - Bob should not be able to reencrypt Alice balance');
    } catch (error) {
      expect(error.message).to.equal('User is not authorized to reencrypt this handle!');
    }

    // and should be impossible to call reencrypt if contractAddress === userAddress
    try {
      const eip712b = this.instances.alice.createEIP712(publicKeyAlice, this.signers.alice.address);
      const signatureAliceb = await this.signers.alice.signTypedData(
        eip712b.domain,
        { Reencrypt: eip712b.types.Reencrypt },
        eip712b.message,
      );
      await this.instances.alice.reencrypt(
        balanceHandleAlice,
        privateKeyAlice,
        publicKeyAlice,
        signatureAliceb.replace('0x', ''),
        this.signers.alice.address,
        this.signers.alice.address,
      );
      expect.fail('Expected an error to be thrown - userAddress and contractAddress cannot be equal');
    } catch (error) {
      expect(error.message).to.equal(
        'userAddress should not be equal to contractAddress when requesting reencryption!',
      );
    }
  });

  it('should not transfer tokens between two users', async function () {
    const transaction = await this.erc20.mint(1000);
    await transaction.wait();

    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add64(1337);
    const encryptedTransferAmount = await input.encrypt();
    const tx = await this.erc20['transfer(address,bytes32,bytes)'](
      this.signers.bob.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );
    await tx.wait();

    const balanceHandleAlice = await this.erc20.balanceOf(this.signers.alice);
    const { publicKey: publicKeyAlice, privateKey: privateKeyAlice } = this.instances.alice.generateKeypair();
    const eip712 = this.instances.alice.createEIP712(publicKeyAlice, this.contractAddress);
    const signatureAlice = await this.signers.alice.signTypedData(
      eip712.domain,
      { Reencrypt: eip712.types.Reencrypt },
      eip712.message,
    );
    const balanceAlice = await this.instances.alice.reencrypt(
      balanceHandleAlice,
      privateKeyAlice,
      publicKeyAlice,
      signatureAlice.replace('0x', ''),
      this.contractAddress,
      this.signers.alice.address,
    );

    expect(balanceAlice).to.equal(1000);

    // Reencrypt Bob's balance
    const balanceHandleBob = await this.erc20.balanceOf(this.signers.bob);

    const { publicKey: publicKeyBob, privateKey: privateKeyBob } = this.instances.bob.generateKeypair();
    const eip712Bob = this.instances.bob.createEIP712(publicKeyBob, this.contractAddress);
    const signatureBob = await this.signers.bob.signTypedData(
      eip712Bob.domain,
      { Reencrypt: eip712Bob.types.Reencrypt },
      eip712Bob.message,
    );
    const balanceBob = await this.instances.bob.reencrypt(
      balanceHandleBob,
      privateKeyBob,
      publicKeyBob,
      signatureBob.replace('0x', ''),
      this.contractAddress,
      this.signers.bob.address,
    );

    expect(balanceBob).to.equal(0);
  });

  it('should be able to transferFrom only if allowance is sufficient', async function () {
    const transaction = await this.erc20.mint(10000);
    await transaction.wait();

    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.add64(1337);
    const encryptedAllowanceAmount = await inputAlice.encrypt();
    const tx = await this.erc20['approve(address,bytes32,bytes)'](
      this.signers.bob.address,
      encryptedAllowanceAmount.handles[0],
      encryptedAllowanceAmount.inputProof,
    );
    await tx.wait();

    const bobErc20 = this.erc20.connect(this.signers.bob);
    const inputBob1 = this.instances.bob.createEncryptedInput(this.contractAddress, this.signers.bob.address);
    inputBob1.add64(1338); // above allowance so next tx should actually not send any token
    const encryptedTransferAmount = await inputBob1.encrypt();
    const tx2 = await bobErc20['transferFrom(address,address,bytes32,bytes)'](
      this.signers.alice.address,
      this.signers.bob.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );
    await tx2.wait();

    // Decrypt Alice's balance
    const balanceHandleAlice = await this.erc20.balanceOf(this.signers.alice);
    const { publicKey: publicKeyAlice, privateKey: privateKeyAlice } = this.instances.alice.generateKeypair();
    const eip712 = this.instances.alice.createEIP712(publicKeyAlice, this.contractAddress);
    const signatureAlice = await this.signers.alice.signTypedData(
      eip712.domain,
      { Reencrypt: eip712.types.Reencrypt },
      eip712.message,
    );
    const balanceAlice = await this.instances.alice.reencrypt(
      balanceHandleAlice,
      privateKeyAlice,
      publicKeyAlice,
      signatureAlice.replace('0x', ''),
      this.contractAddress,
      this.signers.alice.address,
    );
    expect(balanceAlice).to.equal(10000); // check that transfer did not happen, as expected

    // Decrypt Bob's balance
    const balanceHandleBob = await this.erc20.balanceOf(this.signers.bob);
    const { publicKey: publicKeyBob, privateKey: privateKeyBob } = this.instances.bob.generateKeypair();
    const eip712Bob = this.instances.bob.createEIP712(publicKeyBob, this.contractAddress);
    const signatureBob = await this.signers.bob.signTypedData(
      eip712Bob.domain,
      { Reencrypt: eip712Bob.types.Reencrypt },
      eip712Bob.message,
    );
    const balanceBob = await this.instances.bob.reencrypt(
      balanceHandleBob,
      privateKeyBob,
      publicKeyBob,
      signatureBob.replace('0x', ''),
      this.contractAddress,
      this.signers.bob.address,
    );
    expect(balanceBob).to.equal(0); // check that transfer did not happen, as expected

    const inputBob2 = this.instances.bob.createEncryptedInput(this.contractAddress, this.signers.bob.address);
    inputBob2.add64(1337); // below allowance so next tx should send token
    const encryptedTransferAmount2 = await inputBob2.encrypt();
    const tx3 = await bobErc20['transferFrom(address,address,bytes32,bytes)'](
      this.signers.alice.address,
      this.signers.bob.address,
      encryptedTransferAmount2.handles[0],
      encryptedTransferAmount2.inputProof,
    );
    await tx3.wait();

    // Decrypt Alice's balance
    const balanceHandleAlice2 = await this.erc20.balanceOf(this.signers.alice);
    const balanceAlice2 = await this.instances.alice.reencrypt(
      balanceHandleAlice2,
      privateKeyAlice,
      publicKeyAlice,
      signatureAlice.replace('0x', ''),
      this.contractAddress,
      this.signers.alice.address,
    );
    expect(balanceAlice2).to.equal(10000 - 1337); // check that transfer did happen this time

    // Decrypt Bob's balance
    const balanceHandleBob2 = await this.erc20.balanceOf(this.signers.bob);
    const balanceBob2 = await this.instances.bob.reencrypt(
      balanceHandleBob2,
      privateKeyBob,
      publicKeyBob,
      signatureBob.replace('0x', ''),
      this.contractAddress,
      this.signers.bob.address,
    );
    expect(balanceBob2).to.equal(1337); // check that transfer did happen this time
  });
});
