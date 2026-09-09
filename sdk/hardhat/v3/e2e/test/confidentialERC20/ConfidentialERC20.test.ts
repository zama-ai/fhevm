/* eslint-disable no-unexpected-multiline */
import { timestampNow } from '@fhevm/hardhat-plugin-v3';
import { expect } from 'chai';
import { network } from 'hardhat';

import type { TestConfidentialERC20Mintable } from '../../types/ethers-contracts/index.ts';
import { expectRejectedWith } from '../utils/expect.ts';
import { type Signers, accountFor, getSigners } from '../utils/signers.ts';
import {
  deployConfidentialERC20Fixture,
  userDecryptAllowance,
  userDecryptBalance,
} from './ConfidentialERC20.fixture.ts';

const connection = await network.getOrCreate();
const { fhevm } = connection;

type Hex = `0x${string}`;

function escapeRegExp(text: string): string {
  return text.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

describe('ConfidentialERC20', function () {
  // @dev The placeholder is type(uint256).max --> 2**256 - 1.
  const PLACEHOLDER = 2n ** 256n - 1n;
  let signers: Signers;
  let confidentialERC20: TestConfidentialERC20Mintable;
  let confidentialERC20Address: Hex;

  // One euint64 input for `user`; `handles` is `Hex[]`, so the single handle is narrowed here once.
  async function encrypt64(user: Hex, value: number | bigint): Promise<{ handle: Hex; inputProof: Hex }> {
    const encrypted = await fhevm.createEncryptedInput(confidentialERC20Address, user).add64(value).encrypt();
    const [handle] = encrypted.handles;
    if (handle === undefined) throw new Error('encrypt() returned no handle');
    return { handle, inputProof: encrypted.inputProof };
  }

  before(async function () {
    signers = await getSigners(connection);
  });

  beforeEach(async function () {
    const contract = await deployConfidentialERC20Fixture(
      signers.alice,
      'Naraggara',
      'NARA',
      await signers.alice.getAddress(),
    );
    confidentialERC20Address = (await contract.getAddress()) as Hex;
    confidentialERC20 = contract;
  });

  it('post-deployment state', async function () {
    expect(await confidentialERC20.totalSupply()).to.equal(0);
    expect(await confidentialERC20.name()).to.equal('Naraggara');
    expect(await confidentialERC20.symbol()).to.equal('NARA');
    expect(await confidentialERC20.decimals()).to.be.eq(BigInt(6));

    await fhevm.assertCoprocessorInitialized(confidentialERC20, 'TestConfidentialERC20Mintable');
  });

  it('should mint the contract', async function () {
    const mintAmount = 1000;
    const tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, mintAmount);
    await tx.wait();

    expect(await userDecryptBalance(signers.alice, confidentialERC20, confidentialERC20Address)).to.equal(mintAmount);

    expect(await confidentialERC20.totalSupply()).to.equal(mintAmount);
  });

  it('should transfer tokens between two users', async function () {
    const mintAmount = 10_000;
    const transferAmount = 1337;

    let tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, mintAmount);
    await tx.wait();

    expect(await userDecryptBalance(signers.alice, confidentialERC20, confidentialERC20Address)).to.equal(mintAmount);

    const encryptedTransferAmount = await encrypt64(signers.alice.address as Hex, transferAmount);

    tx = await confidentialERC20
      .connect(signers.alice)
      ['transfer(address,bytes32,bytes)'](
        signers.bob.address,
        encryptedTransferAmount.handle,
        encryptedTransferAmount.inputProof,
      );
    await tx.wait();
    await expect(tx).to.emit(confidentialERC20, 'Transfer').withArgs(signers.alice, signers.bob, PLACEHOLDER);

    // Decrypt Alice's balance
    expect(await userDecryptBalance(signers.alice, confidentialERC20, confidentialERC20Address)).to.equal(
      mintAmount - transferAmount,
    );
    // Decrypt Bob's balance
    expect(await userDecryptBalance(signers.bob, confidentialERC20, confidentialERC20Address)).to.equal(transferAmount);
  });

  it('should not transfer tokens between two users if transfer amount is higher than balance', async function () {
    // @dev There is no transfer done since the mint amount is smaller than the transfer
    //      amount.
    const mintAmount = 1000;
    const transferAmount = 1337;

    let tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, mintAmount);
    await tx.wait();

    const encryptedTransferAmount = await encrypt64(signers.alice.address as Hex, transferAmount);

    tx = await confidentialERC20['transfer(address,bytes32,bytes)'](
      signers.bob.address,
      encryptedTransferAmount.handle,
      encryptedTransferAmount.inputProof,
    );
    await tx.wait();

    // @dev There is no error-handling in this version of ConfidentialERC20.
    await expect(tx).to.emit(confidentialERC20, 'Transfer').withArgs(signers.alice, signers.bob, PLACEHOLDER);

    // Decrypt Alice's balance
    expect(await userDecryptBalance(signers.alice, confidentialERC20, confidentialERC20Address)).to.equal(mintAmount);

    // Decrypt Bob's balance
    expect(await userDecryptBalance(signers.bob, confidentialERC20, confidentialERC20Address)).to.equal(0);
  });

  it('should be able to transferFrom only if allowance is sufficient', async function () {
    const mintAmount = 10_000;
    const transferAmount = 1337;

    let tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, mintAmount);
    await tx.wait();

    const encryptedAllowanceAmount = await encrypt64(signers.alice.address as Hex, transferAmount);

    tx = await confidentialERC20['approve(address,bytes32,bytes)'](
      signers.bob.address,
      encryptedAllowanceAmount.handle,
      encryptedAllowanceAmount.inputProof,
    );
    await tx.wait();

    await expect(tx).to.emit(confidentialERC20, 'Approval').withArgs(signers.alice, signers.bob, PLACEHOLDER);

    // @dev The allowance amount is set to be equal to the transfer amount.
    expect(
      await userDecryptAllowance(signers.alice, signers.bob, confidentialERC20, confidentialERC20Address),
    ).to.equal(transferAmount);

    const bobErc20 = confidentialERC20.connect(signers.bob);
    // above allowance so next tx should actually not send any token
    const encryptedTransferAmount = await encrypt64(signers.bob.address as Hex, transferAmount + 1);

    const tx2 = await bobErc20['transferFrom(address,address,bytes32,bytes)'](
      signers.alice.address,
      signers.bob.address,
      encryptedTransferAmount.handle,
      encryptedTransferAmount.inputProof,
    );
    await tx2.wait();

    await expect(tx2).to.emit(confidentialERC20, 'Transfer').withArgs(signers.alice, signers.bob, PLACEHOLDER);

    // Decrypt Alice's balance
    expect(await userDecryptBalance(signers.alice, confidentialERC20, confidentialERC20Address)).to.equal(mintAmount); // check that transfer did not happen, as expected

    // Decrypt Bob's balance
    expect(await userDecryptBalance(signers.bob, confidentialERC20, confidentialERC20Address)).to.equal(0); // check that transfer did not happen, as expected

    // below allowance so next tx should send token
    const encryptedTransferAmount2 = await encrypt64(signers.bob.address as Hex, transferAmount);

    const tx3 = await bobErc20['transferFrom(address,address,bytes32,bytes)'](
      signers.alice.address,
      signers.bob.address,
      encryptedTransferAmount2.handle,
      encryptedTransferAmount2.inputProof,
    );
    await tx3.wait();

    // Decrypt Alice's balance
    expect(await userDecryptBalance(signers.alice, confidentialERC20, confidentialERC20Address)).to.equal(
      mintAmount - transferAmount,
    ); // check that transfer did happen this time

    // Decrypt Bob's balance
    expect(await userDecryptBalance(signers.bob, confidentialERC20, confidentialERC20Address)).to.equal(transferAmount); // check that transfer did happen this time

    // Verify Alice's allowance is 0
    expect(
      await userDecryptAllowance(signers.alice, signers.bob, confidentialERC20, confidentialERC20Address),
    ).to.equal(0);
  });

  it('should not be able to read the allowance if not spender/owner after initialization', async function () {
    const amount = 10_000;

    const encryptedAllowanceAmount = await encrypt64(signers.alice.address as Hex, amount);

    const tx = await confidentialERC20
      .connect(signers.alice)
      ['approve(address,bytes32,bytes)'](
        signers.bob.address,
        encryptedAllowanceAmount.handle,
        encryptedAllowanceAmount.inputProof,
      );

    await tx.wait();

    const allowanceHandleAlice = (await confidentialERC20.allowance(signers.alice, signers.bob)) as Hex;

    const carol = accountFor(signers.carol);
    const transportKeyPairCarol = await fhevm.client.generateTransportKeyPair();

    const startTimestamp = timestampNow();
    const durationDays = 365;

    const signedPermitCarol = await fhevm.client.signLegacyDecryptionPermit({
      contractAddresses: [confidentialERC20Address],
      startTimestamp,
      // The legacy API measured validity in days; `@fhevm/sdk` takes seconds.
      durationSeconds: durationDays * 24 * 60 * 60,
      signerAddress: carol.address,
      signer: carol,
      transportKeyPair: transportKeyPairCarol,
    });

    await expectRejectedWith(
      fhevm.client.decryptValuesFromPairs({
        pairs: [{ encryptedValue: allowanceHandleAlice, contractAddress: confidentialERC20Address }],
        transportKeyPair: transportKeyPairCarol,
        signedPermit: signedPermitCarol,
      }),
      new RegExp(
        escapeRegExp(`User ${signers.carol.address} is not authorized to decrypt handle ${allowanceHandleAlice}!`),
      ),
    );
  });

  it('should not be able to read the balance if not user after initialization', async function () {
    // Mint is used to initialize the balanceOf(alice)
    const amount = 10_000;
    const tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, amount);
    await tx.wait();

    const balanceHandleAlice = (await confidentialERC20.balanceOf(signers.alice)) as Hex;

    const bob = accountFor(signers.bob);
    const transportKeyPairBob = await fhevm.client.generateTransportKeyPair();

    const startTimestamp = timestampNow();
    const durationDays = 365;

    const signedPermitBob = await fhevm.client.signLegacyDecryptionPermit({
      contractAddresses: [confidentialERC20Address],
      startTimestamp,
      // The legacy API measured validity in days; `@fhevm/sdk` takes seconds.
      durationSeconds: durationDays * 24 * 60 * 60,
      signerAddress: bob.address,
      signer: bob,
      transportKeyPair: transportKeyPairBob,
    });

    await expectRejectedWith(
      fhevm.client.decryptValuesFromPairs({
        pairs: [{ encryptedValue: balanceHandleAlice, contractAddress: confidentialERC20Address }],
        transportKeyPair: transportKeyPairBob,
        signedPermit: signedPermitBob,
      }),
      new RegExp(
        escapeRegExp(`User ${signers.bob.address} is not authorized to decrypt handle ${balanceHandleAlice}!`),
      ),
    );
  });

  it('receiver cannot be null address', async function () {
    const NULL_ADDRESS = '0x0000000000000000000000000000000000000000';
    const mintAmount = 100_000;
    const transferAmount = 50_000;
    const tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, mintAmount);
    await tx.wait();

    const encryptedTransferAmount = await encrypt64(signers.alice.address as Hex, transferAmount);

    await expect(
      confidentialERC20
        .connect(signers.alice)
        ['transfer(address,bytes32,bytes)'](
          NULL_ADDRESS,
          encryptedTransferAmount.handle,
          encryptedTransferAmount.inputProof,
        ),
    ).to.be.revertedWithCustomError(confidentialERC20, 'ERC20InvalidReceiver');
  });

  it('sender who is not allowed cannot transfer using a handle from another account', async function () {
    const mintAmount = 100_000;
    const transferAmount = 50_000;
    let tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, mintAmount);
    await tx.wait();

    const encryptedTransferAmount = await encrypt64(signers.alice.address as Hex, transferAmount);

    tx = await confidentialERC20
      .connect(signers.alice)
      ['transfer(address,bytes32,bytes)'](
        signers.carol.address,
        encryptedTransferAmount.handle,
        encryptedTransferAmount.inputProof,
      );

    await tx.wait();

    const balanceHandleAlice = await confidentialERC20.balanceOf(signers.alice.address);

    await expect(
      confidentialERC20.connect(signers.bob)['transfer(address,bytes32)'](signers.carol.address, balanceHandleAlice),
    ).to.be.revertedWithCustomError(confidentialERC20, 'FHESenderNotAllowed');
  });

  it('sender who is not allowed cannot transferFrom using a handle from another account', async function () {
    const mintAmount = 100_000;
    const transferAmount = 50_000;

    let tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, mintAmount);
    await tx.wait();

    const encryptedAllowanceAmount = await encrypt64(signers.alice.address as Hex, mintAmount);

    tx = await confidentialERC20
      .connect(signers.alice)
      ['approve(address,bytes32,bytes)'](
        signers.carol.address,
        encryptedAllowanceAmount.handle,
        encryptedAllowanceAmount.inputProof,
      );
    await tx.wait();

    const encryptedTransferAmount = await encrypt64(signers.carol.address as Hex, transferAmount);

    tx = await confidentialERC20
      .connect(signers.carol)
      ['transferFrom(address,address,bytes32,bytes)'](
        signers.alice.address,
        signers.carol.address,
        encryptedTransferAmount.handle,
        encryptedTransferAmount.inputProof,
      );
    await tx.wait();

    const allowanceHandleAlice = await confidentialERC20.allowance(signers.alice.address, signers.carol.address);

    await expect(
      confidentialERC20
        .connect(signers.bob)
        ['transferFrom(address,address,bytes32)'](signers.alice.address, signers.bob.address, allowanceHandleAlice),
    ).to.be.revertedWithCustomError(confidentialERC20, 'FHESenderNotAllowed');
  });

  it('sender who is not allowed cannot approve using a handle from another account', async function () {
    const amount = 100_000;
    const encryptedAllowanceAmount = await encrypt64(signers.alice.address as Hex, amount);

    const tx = await confidentialERC20
      .connect(signers.alice)
      ['approve(address,bytes32,bytes)'](
        signers.carol.address,
        encryptedAllowanceAmount.handle,
        encryptedAllowanceAmount.inputProof,
      );

    await tx.wait();

    const allowanceHandleAlice = await confidentialERC20.allowance(signers.alice.address, signers.carol.address);

    await expect(
      confidentialERC20.connect(signers.bob)['approve(address,bytes32)'](signers.carol.address, allowanceHandleAlice),
    ).to.be.revertedWithCustomError(confidentialERC20, 'FHESenderNotAllowed');
  });

  it('ConfidentialERC20Mintable - only owner can mint', async function () {
    await expect(confidentialERC20.connect(signers.bob).mint(signers.bob, 1)).to.be.revertedWithCustomError(
      confidentialERC20,
      'OwnableUnauthorizedAccount',
    );
  });
});
