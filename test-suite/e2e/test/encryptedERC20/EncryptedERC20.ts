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

    // User decryption of Alice's balance
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

    const balanceAlice = await this.instances.alice.userDecryptSingleHandle({
      handle: balanceHandleAlice,
      contractAddress: this.contractAddress,
      signer: this.signers.alice,
    });

    expect(balanceAlice).to.equal(1000n);

    const totalSupply = await this.erc20.totalSupply();
    expect(totalSupply).to.equal(1000n);
  });

  it('should transfer tokens between two users.', async function () {
    const transaction = await this.erc20.mint(10000);
    const t1 = await transaction.wait();
    expect(t1?.status).to.eq(1);

    const encryptedTransferAmount = await this.instances.alice.encryptUint64({
      value: 1337,
      contractAddress: this.contractAddress,
      userAddress: this.signers.alice.address,
    });

    const tx = await this.erc20['transfer(address,bytes32,bytes)'](
      this.signers.bob.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );
    const t2 = await tx.wait();
    expect(t2?.status).to.eq(1);

    // User decryption of Alice's balance
    const balanceHandleAlice = await this.erc20.balanceOf(this.signers.alice);

    const balanceAlice = await this.instances.alice.userDecryptSingleHandle({
      handle: balanceHandleAlice,
      contractAddress: this.contractAddress,
      signer: this.signers.alice,
    });

    expect(balanceAlice).to.equal(10000 - 1337);

    // User decryption of Bob's balance
    const balanceHandleBob = await this.erc20.balanceOf(this.signers.bob);

    const balanceBob = await this.instances.bob.userDecryptSingleHandle({
      handle: balanceHandleBob,
      contractAddress: this.contractAddress,
      signer: this.signers.bob,
    });

    expect(balanceBob).to.equal(1337);
  });

  it('should not transfer tokens between two users', async function () {
    const transaction = await this.erc20.mint(1000);
    await transaction.wait();

    const encryptedTransferAmount = await this.instances.alice.encryptUint64({
      value: 1337,
      contractAddress: this.contractAddress,
      userAddress: this.signers.alice.address,
    });

    const tx = await this.erc20['transfer(address,bytes32,bytes)'](
      this.signers.bob.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );
    await tx.wait();

    const balanceHandleAlice = await this.erc20.balanceOf(this.signers.alice);

    const balanceAlice = await this.instances.alice.userDecryptSingleHandle({
      handle: balanceHandleAlice,
      contractAddress: this.contractAddress,
      signer: this.signers.alice,
    });

    expect(balanceAlice).to.equal(1000n);

    // User decryption of Bob's balance
    const balanceHandleBob = await this.erc20.balanceOf(this.signers.bob);

    const balanceBob = await this.instances.bob.userDecryptSingleHandle({
      handle: balanceHandleBob,
      contractAddress: this.contractAddress,
      signer: this.signers.bob,
    });

    expect(balanceBob).to.equal(0);
  });

  it('should be able to transferFrom only if allowance is sufficient', async function () {
    const transaction = await this.erc20.mint(10000);
    await transaction.wait();

    const encryptedAllowanceAmount = await this.instances.alice.encryptUint64({
      value: 1337,
      contractAddress: this.contractAddress,
      userAddress: this.signers.alice.address,
    });

    const tx = await this.erc20['approve(address,bytes32,bytes)'](
      this.signers.bob.address,
      encryptedAllowanceAmount.handles[0],
      encryptedAllowanceAmount.inputProof,
    );
    await tx.wait();

    const bobErc20 = this.erc20.connect(this.signers.bob);

    const encryptedTransferAmount = await this.instances.bob.encryptUint64({
      value: 1338,
      contractAddress: this.contractAddress,
      userAddress: this.signers.bob.address,
    });

    const tx2 = await bobErc20['transferFrom(address,address,bytes32,bytes)'](
      this.signers.alice.address,
      this.signers.bob.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );
    await tx2.wait();

    // Decrypt Alice's balance
    const balanceHandleAlice = await this.erc20.balanceOf(this.signers.alice);
    const transportKeypairAlice = await this.instances.alice.generateKeypair();
    const balanceAlice = await this.instances.alice.userDecryptSingleHandle({
      handle: balanceHandleAlice,
      contractAddress: this.contractAddress,
      signer: this.signers.alice,
      transportKeypair: transportKeypairAlice,
    });
    expect(balanceAlice).to.equal(10000); // check that transfer did not happen, as expected

    // Decrypt Bob's balance
    const balanceHandleBob = await this.erc20.balanceOf(this.signers.bob);
    const transportKeypairBob = await this.instances.bob.generateKeypair();
    const balanceBob = await this.instances.bob.userDecryptSingleHandle({
      handle: balanceHandleBob,
      contractAddress: this.contractAddress,
      signer: this.signers.bob,
      transportKeypair: transportKeypairBob,
    });
    expect(balanceBob).to.equal(0); // check that transfer did not happen, as expected

    const encryptedTransferAmount2 = await this.instances.bob.encryptUint64({
      value: 1337,
      contractAddress: this.contractAddress,
      userAddress: this.signers.bob.address,
    });

    const tx3 = await bobErc20['transferFrom(address,address,bytes32,bytes)'](
      this.signers.alice.address,
      this.signers.bob.address,
      encryptedTransferAmount2.handles[0],
      encryptedTransferAmount2.inputProof,
    );
    await tx3.wait();

    // Decrypt Alice's balance
    const balanceHandleAlice2 = await this.erc20.balanceOf(this.signers.alice);
    const balanceAlice2 = await this.instances.alice.userDecryptSingleHandle({
      handle: balanceHandleAlice2,
      contractAddress: this.contractAddress,
      signer: this.signers.alice,
      transportKeypair: transportKeypairAlice,
    });
    expect(balanceAlice2).to.equal(10000 - 1337); // check that transfer did happen this time

    // Decrypt Bob's balance
    const balanceHandleBob2 = await this.erc20.balanceOf(this.signers.bob);
    const balanceBob2 = await this.instances.bob.userDecryptSingleHandle({
      handle: balanceHandleBob2,
      contractAddress: this.contractAddress,
      signer: this.signers.bob,
      transportKeypair: transportKeypairBob,
    });
    expect(balanceBob2).to.equal(1337); // check that transfer did happen this time
  });

  describe('negative-acl', function () {
    it('should reject when user is not allowed for handle', async function () {
      const transaction = await this.erc20.mint(10000);
      await transaction.wait();

      const encryptedTransferAmount = await this.instances.alice.encryptUint64({
        value: 1337,
        contractAddress: this.contractAddress,
        userAddress: this.signers.alice.address,
      });

      const tx = await this.erc20['transfer(address,bytes32,bytes)'](
        this.signers.bob.address,
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
      );
      await tx.wait();

      const balanceHandleAlice = await this.erc20.balanceOf(this.signers.alice);

      try {
        await this.instances.bob.userDecryptSingleHandle({
          handle: balanceHandleAlice,
          contractAddress: this.contractAddress,
          signer: this.signers.bob,
        });
        expect.fail('Expected an error to be thrown - Bob should not be able to user decrypt Alice balance');
      } catch (error) {
        expect((error as { message: string }).message).to.contain(
          this.instances.bob.getUserDecryptErrorMessage({
            type: 'user-unauthorized',
            signer: this.signers.bob,
            handle: balanceHandleAlice,
          }),
        );
      }
    });
  });
});
