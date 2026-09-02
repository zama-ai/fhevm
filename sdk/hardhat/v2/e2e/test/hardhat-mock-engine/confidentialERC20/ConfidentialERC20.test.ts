/* eslint-disable no-unexpected-multiline */
import { timestampNow } from '@fhevm/hardhat-plugin';
import { expect } from 'chai';
import * as hre from 'hardhat';
// test using 'fhevm' environment extension via `import { fhevm } from "hardhat";`
import { fhevm } from 'hardhat';

import type { TestConfidentialERC20Mintable } from '../../../typechain-types';
import { type Signers, getSigners, initSigners } from '../signers';
import { deployConfidentialERC20Fixture, userDecryptAllowance, userDecryptBalance } from './ConfidentialERC20.fixture';

describe('ConfidentialERC20', function () {
  // @dev The placeholder is type(uint256).max --> 2**256 - 1.
  const PLACEHOLDER = 2n ** 256n - 1n;
  let signers: Signers;
  let confidentialERC20: TestConfidentialERC20Mintable;
  let confidentialERC20Address: string;

  before(async function () {
    await initSigners();
    signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployConfidentialERC20Fixture(
      signers.alice,
      'Naraggara',
      'NARA',
      await signers.alice.getAddress(),
    );
    confidentialERC20Address = await contract.getAddress();
    confidentialERC20 = contract;
  });

  it('post-deployment state', async function () {
    expect(await confidentialERC20.totalSupply()).to.equal(0);
    expect(await confidentialERC20.name()).to.equal('Naraggara');
    expect(await confidentialERC20.symbol()).to.equal('NARA');
    expect(await confidentialERC20.decimals()).to.be.eq(BigInt(6));

    // test using 'fhevm' environment extension via `import { fhevm } from "hardhat";`
    await fhevm.assertCoprocessorInitialized(confidentialERC20, 'TestConfidentialERC20Mintable');
  });

  it('should mint the contract', async function () {
    //await helpers.getStorageAt(address, storageSlot);
    /*
When you encouter this kind of error:
        CoprocessorConfig storage $ = getCoprocessorConfig(); return "undefined" => FHE.setCoprocessor was not called!
        We must ADD a test!!!!!! for the user!!

     Error: Transaction reverted: function returned an unexpected amount of data
    at TestConfidentialERC20Mintable.trivialEncrypt (@fhevm/solidity/lib/Impl.sol:686)
    at TestConfidentialERC20Mintable.asEuint64 (@fhevm/solidity/lib/FHE.sol:8559)
    at TestConfidentialERC20Mintable.add (@fhevm/solidity/lib/FHE.sol:6615)
    at TestConfidentialERC20Mintable._unsafeMintNoEvent (contracts/token/ERC20/ConfidentialERC20.sol:178)
    at TestConfidentialERC20Mintable._unsafeMint (contracts/token/ERC20/ConfidentialERC20.sol:169)
    at TestConfidentialERC20Mintable.mint (contracts/token/ERC20/extensions/ConfidentialERC20Mintable.sol:36)
    at EdrProviderWrapper.request (node_modules/hardhat/src/internal/hardhat-network/provider/provider.ts:359:41)
    at async HardhatEthersSigner.sendTransaction (node_modules/@nomicfoundation/hardhat-ethers/src/signers.ts:125:18)
    at async send (node_modules/ethers/src.ts/contract/contract.ts:313:20)
    at async Proxy.mint (node_modules/ethers/src.ts/contract/contract.ts:352:16)
    at async Context.<anonymous> (test/hardhat-mock-engine/confidentialERC20/ConfidentialERC20.test.ts:48:16)
*/
    const mintAmount = 1000;
    const tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, mintAmount);
    await tx.wait();
    //await expect(tx).to.emit(confidentialERC20, "Mint").withArgs(signers.alice, mintAmount);

    expect(await userDecryptBalance(signers.alice, confidentialERC20, confidentialERC20Address)).to.equal(mintAmount);

    expect(await confidentialERC20.totalSupply()).to.equal(mintAmount);
  });

  it('should transfer tokens between two users', async function () {
    const mintAmount = 10_000;
    const transferAmount = 1337;

    let tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, mintAmount);
    await tx.wait();

    const balanceHandle = await confidentialERC20.balanceOf(signers.alice);
    console.log('balanceHandle=' + balanceHandle);

    expect(await userDecryptBalance(signers.alice, confidentialERC20, confidentialERC20Address)).to.equal(mintAmount);

    const input = hre.fhevm.createEncryptedInput(confidentialERC20Address, signers.alice.address);
    input.add64(transferAmount);
    const encryptedTransferAmount = await input.encrypt();

    tx = await confidentialERC20
      .connect(signers.alice)
      ['transfer(address,bytes32,bytes)'](
        signers.bob.address,
        encryptedTransferAmount.handles[0],
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

    const input = hre.fhevm.createEncryptedInput(confidentialERC20Address, signers.alice.address);
    input.add64(transferAmount);
    const encryptedTransferAmount = await input.encrypt();

    tx = await confidentialERC20['transfer(address,bytes32,bytes)'](
      signers.bob.address,
      encryptedTransferAmount.handles[0],
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
    // @dev There is no transfer done since the mint amount is smaller than the transfer
    //      amount.
    const mintAmount = 10_000;
    const transferAmount = 1337;

    let tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, mintAmount);
    await tx.wait();

    const inputAlice = hre.fhevm.createEncryptedInput(confidentialERC20Address, signers.alice.address);
    inputAlice.add64(transferAmount);
    const encryptedAllowanceAmount = await inputAlice.encrypt();

    tx = await confidentialERC20['approve(address,bytes32,bytes)'](
      signers.bob.address,
      encryptedAllowanceAmount.handles[0],
      encryptedAllowanceAmount.inputProof,
    );
    await tx.wait();

    await expect(tx).to.emit(confidentialERC20, 'Approval').withArgs(signers.alice, signers.bob, PLACEHOLDER);

    // @dev The allowance amount is set to be equal to the transfer amount.
    expect(
      await userDecryptAllowance(signers.alice, signers.bob, confidentialERC20, confidentialERC20Address),
    ).to.equal(transferAmount);

    const bobErc20 = confidentialERC20.connect(signers.bob);
    const inputBob1 = hre.fhevm.createEncryptedInput(confidentialERC20Address, signers.bob.address);
    inputBob1.add64(transferAmount + 1); // above allowance so next tx should actually not send any token
    const encryptedTransferAmount = await inputBob1.encrypt();

    const tx2 = await bobErc20['transferFrom(address,address,bytes32,bytes)'](
      signers.alice.address,
      signers.bob.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );
    await tx2.wait();

    await expect(tx2).to.emit(confidentialERC20, 'Transfer').withArgs(signers.alice, signers.bob, PLACEHOLDER);

    // Decrypt Alice's balance
    expect(await userDecryptBalance(signers.alice, confidentialERC20, confidentialERC20Address)).to.equal(mintAmount); // check that transfer did not happen, as expected

    // Decrypt Bob's balance
    expect(await userDecryptBalance(signers.bob, confidentialERC20, confidentialERC20Address)).to.equal(0); // check that transfer did not happen, as expected

    const inputBob2 = hre.fhevm.createEncryptedInput(confidentialERC20Address, signers.bob.address);
    inputBob2.add64(transferAmount); // below allowance so next tx should send token
    const encryptedTransferAmount2 = await inputBob2.encrypt();

    const tx3 = await bobErc20['transferFrom(address,address,bytes32,bytes)'](
      signers.alice.address,
      signers.bob.address,
      encryptedTransferAmount2.handles[0],
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

    const inputAlice = hre.fhevm.createEncryptedInput(confidentialERC20Address, signers.alice.address);
    inputAlice.add64(amount);
    const encryptedAllowanceAmount = await inputAlice.encrypt();

    const tx = await confidentialERC20
      .connect(signers.alice)
      ['approve(address,bytes32,bytes)'](
        signers.bob.address,
        encryptedAllowanceAmount.handles[0],
        encryptedAllowanceAmount.inputProof,
      );

    await tx.wait();

    const allowanceHandleAlice: string = await confidentialERC20.allowance(signers.alice, signers.bob);

    const transportKeyPairCarol = await hre.fhevm.client.generateTransportKeyPair();

    const startTimestamp = timestampNow();
    const durationDays = 365;

    const signedPermitCarol = await hre.fhevm.client.signLegacyDecryptionPermit({
      contractAddresses: [confidentialERC20Address],
      startTimestamp,
      // The legacy API measured validity in days; `@fhevm/sdk` takes seconds.
      durationSeconds: durationDays * 24 * 60 * 60,
      signerAddress: signers.carol.address,
      signer: signers.carol,
      transportKeyPair: transportKeyPairCarol,
    });

    await expect(
      hre.fhevm.client.decryptValuesFromPairs({
        pairs: [{ encryptedValue: allowanceHandleAlice, contractAddress: confidentialERC20Address }],
        transportKeyPair: transportKeyPairCarol,
        signedPermit: signedPermitCarol,
      }),
    ).to.be.rejectedWith(`User ${signers.carol.address} is not authorized to decrypt handle ${allowanceHandleAlice}!`);
  });

  it('should not be able to read the balance if not user after initialization', async function () {
    // Mint is used to initialize the balanceOf(alice)
    const amount = 10_000;
    const tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, amount);
    await tx.wait();

    const balanceHandleAlice: string = await confidentialERC20.balanceOf(signers.alice);

    const transportKeyPairBob = await hre.fhevm.client.generateTransportKeyPair();

    const startTimestamp = timestampNow();
    const durationDays = 365;

    const signedPermitBob = await hre.fhevm.client.signLegacyDecryptionPermit({
      contractAddresses: [confidentialERC20Address],
      startTimestamp,
      // The legacy API measured validity in days; `@fhevm/sdk` takes seconds.
      durationSeconds: durationDays * 24 * 60 * 60,
      signerAddress: signers.bob.address,
      signer: signers.bob,
      transportKeyPair: transportKeyPairBob,
    });

    await expect(
      hre.fhevm.client.decryptValuesFromPairs({
        pairs: [{ encryptedValue: balanceHandleAlice, contractAddress: confidentialERC20Address }],
        transportKeyPair: transportKeyPairBob,
        signedPermit: signedPermitBob,
      }),
    ).to.be.rejectedWith(`User ${signers.bob.address} is not authorized to decrypt handle ${balanceHandleAlice}!`);
  });

  it('receiver cannot be null address', async function () {
    const NULL_ADDRESS = '0x0000000000000000000000000000000000000000';
    const mintAmount = 100_000;
    const transferAmount = 50_000;
    const tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, mintAmount);
    await tx.wait();

    const input = hre.fhevm.createEncryptedInput(confidentialERC20Address, signers.alice.address);
    input.add64(transferAmount);
    const encryptedTransferAmount = await input.encrypt();

    await expect(
      confidentialERC20
        .connect(signers.alice)
        ['transfer(address,bytes32,bytes)'](
          NULL_ADDRESS,
          encryptedTransferAmount.handles[0],
          encryptedTransferAmount.inputProof,
        ),
    ).to.be.revertedWithCustomError(confidentialERC20, 'ERC20InvalidReceiver');
  });

  it('sender who is not allowed cannot transfer using a handle from another account', async function () {
    const mintAmount = 100_000;
    const transferAmount = 50_000;
    let tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, mintAmount);
    await tx.wait();

    const input = hre.fhevm.createEncryptedInput(confidentialERC20Address, signers.alice.address);
    input.add64(transferAmount);
    const encryptedTransferAmount = await input.encrypt();

    tx = await confidentialERC20
      .connect(signers.alice)
      ['transfer(address,bytes32,bytes)'](
        signers.carol.address,
        encryptedTransferAmount.handles[0],
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

    let input = hre.fhevm.createEncryptedInput(confidentialERC20Address, signers.alice.address);
    input.add64(mintAmount);
    const encryptedAllowanceAmount = await input.encrypt();

    tx = await confidentialERC20
      .connect(signers.alice)
      ['approve(address,bytes32,bytes)'](
        signers.carol.address,
        encryptedAllowanceAmount.handles[0],
        encryptedAllowanceAmount.inputProof,
      );
    await tx.wait();

    input = hre.fhevm.createEncryptedInput(confidentialERC20Address, signers.carol.address);
    input.add64(transferAmount);
    const encryptedTransferAmount = await input.encrypt();

    tx = await confidentialERC20
      .connect(signers.carol)
      ['transferFrom(address,address,bytes32,bytes)'](
        signers.alice.address,
        signers.carol.address,
        encryptedTransferAmount.handles[0],
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
    const input = hre.fhevm.createEncryptedInput(confidentialERC20Address, signers.alice.address);
    input.add64(amount);
    const encryptedAllowanceAmount = await input.encrypt();

    const tx = await confidentialERC20
      .connect(signers.alice)
      ['approve(address,bytes32,bytes)'](
        signers.carol.address,
        encryptedAllowanceAmount.handles[0],
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
