/* eslint-disable no-unexpected-multiline */
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/types';
import { expect } from 'chai';
import { network } from 'hardhat';
import type { LocalAccount } from 'viem';

import type { SmartWalletWithDelegation, TestConfidentialERC20Mintable } from '../../types/ethers-contracts/index.ts';
import { deployConfidentialERC20Fixture } from '../confidentialERC20/ConfidentialERC20.fixture.ts';
import { waitNBlocks } from '../utils/blocks.ts';
import { expectRejectedWith } from '../utils/expect.ts';
import { accountFor, getSigners } from '../utils/signers.ts';

// The v2 delegated-decryption file, on the ConfidentialERC20 balance of a smart wallet (the counter
// version of the same asserts lives in delegatedUserDecryption.ts). The delegate signs its permit as a
// viem account, through the SDK client, exactly as v2 did through ethers.

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;

export async function timestampNowAdjusted(): Promise<number> {
  const blockTimestamp: number | undefined = (await ethers.provider.getBlock('latest'))?.timestamp;
  if (blockTimestamp === undefined) {
    return Math.floor(Date.now() / 1000);
  }
  return blockTimestamp + 100;
}

type Signers = {
  deployer: HardhatEthersSigner;
  alice: HardhatEthersSigner;
  bob: HardhatEthersSigner;
  carol: HardhatEthersSigner;
};

export const delegatedUserDecryptSingleHandle = async (
  handle: Hex,
  contractAddress: Hex,
  delegatorAddress: Hex,
  delegate: LocalAccount,
): Promise<unknown> => {
  const startTimeStamp = Math.floor(Date.now() / 1000);
  const durationDays = 10;
  const contractAddresses = [contractAddress];

  // The delegate signs a permit naming the delegator it is acting for. The transport key pair is
  // per-request, so it is created here rather than passed in.
  const transportKeyPair = await fhevm.client.generateTransportKeyPair();

  const signedPermit = await fhevm.client.signLegacyDecryptionPermit({
    contractAddresses,
    startTimestamp: startTimeStamp,
    durationSeconds: durationDays * 24 * 60 * 60,
    signerAddress: delegate.address,
    signer: delegate,
    delegatorAddress,
    transportKeyPair,
  });

  const [decrypted] = await fhevm.client.decryptValuesFromPairs({
    pairs: [{ encryptedValue: handle, contractAddress }],
    transportKeyPair,
    signedPermit,
  });

  return decrypted?.value;
};

describe('Delegated user decryption (ConfidentialERC20)', function () {
  let signers: Signers;
  let token: TestConfidentialERC20Mintable;
  let tokenAddress: Hex;
  let smartWallet: SmartWalletWithDelegation;
  let smartWalletAddress: Hex;

  before(async function () {
    const suiteSigners = await getSigners(connection);
    signers = {
      deployer: suiteSigners.alice,
      alice: suiteSigners.bob,
      bob: suiteSigners.carol,
      carol: suiteSigners.dave,
    };
  });

  beforeEach(async function () {
    // Deploy token
    token = await deployConfidentialERC20Fixture(
      signers.alice,
      'Zama Confidential Token',
      'ZAMA',
      await signers.alice.getAddress(),
    );
    tokenAddress = (await token.getAddress()) as Hex;

    // Deploy SmartWalletWithDelegation with Bob as the owner.
    const smartWalletFactory = await ethers.getContractFactory('SmartWalletWithDelegation');
    smartWallet = await smartWalletFactory.connect(signers.bob).deploy(signers.bob.address);
    await smartWallet.waitForDeployment();
    smartWalletAddress = (await smartWallet.getAddress()) as Hex;

    // Alice mints tokens to herself.
    const mintAmount = 1000000n;
    const mintTx = await token.connect(signers.alice).mint(signers.alice, mintAmount);
    await mintTx.wait();

    // Alice transfers some tokens to the smartWallet contract.
    const transferAmount = 500000n;
    const encryptedTransferAmount = await fhevm
      .createEncryptedInput(tokenAddress, signers.alice.address as Hex)
      .add64(transferAmount)
      .encrypt();
    const [transferHandle] = encryptedTransferAmount.handles;
    if (transferHandle === undefined) throw new Error('encrypt() returned no handle');

    const transferTx = await token
      .connect(signers.alice)
      ['transfer(address,bytes32,bytes)'](smartWalletAddress, transferHandle, encryptedTransferAmount.inputProof);
    await transferTx.wait();
  });

  it('test delegated user decryption - smartWallet owner delegates his own EOA to decrypt the smartWallet balance', async function () {
    // Bob (smartWallet owner) delegates decryption rights to his own EOA.
    const expirationTimestamp = (await timestampNowAdjusted()) + 86400; // 24 hours from now
    const delegateTx = await smartWallet
      .connect(signers.bob)
      .delegateUserDecryption(signers.bob.address, tokenAddress, expirationTimestamp);
    await delegateTx.wait();

    // Get the encrypted balance handle of the smartWallet.
    const balanceHandle = (await token.balanceOf(smartWalletAddress)) as Hex;

    // Bob's EOA can now decrypt the smartWallet's confidential balance.
    const decryptedBalance = await delegatedUserDecryptSingleHandle(
      balanceHandle,
      tokenAddress,
      smartWalletAddress, // delegatorAddress
      accountFor(signers.bob), // delegate
    );

    // Verify the decrypted balance matches what was transferred.
    expect(decryptedBalance).to.equal(500000n);
  });

  it('test delegated user decryption - smartWallet owner delegates a third EOA to decrypt the smartWallet balance', async function () {
    // Bob (smartWallet owner) delegates decryption rights to Carol's EOA.
    const expirationTimestamp = (await timestampNowAdjusted()) + 86400; // 24 hours from now
    const delegateTx = await smartWallet
      .connect(signers.bob)
      .delegateUserDecryption(signers.carol.address, tokenAddress, expirationTimestamp);
    await delegateTx.wait();

    // Get the encrypted balance handle of the smartWallet.
    const balanceHandle = (await token.balanceOf(smartWalletAddress)) as Hex;

    // Carol's EOA can now decrypt the smartWallet's confidential balance.
    const decryptedBalance = await delegatedUserDecryptSingleHandle(
      balanceHandle,
      tokenAddress,
      smartWalletAddress, // delegatorAddress
      accountFor(signers.carol), // delegate
    );

    // Verify the decrypted balance matches what was transferred.
    expect(decryptedBalance).to.equal(500000n);
  });

  it('test delegated user decryption - smartWallet can execute transference of funds to a third EOA', async function () {
    // First, Bob needs to delegate so the smartWallet can initiate transfers.
    const expirationTimestamp = (await timestampNowAdjusted()) + 86400; // 24 hours from now
    const delegateTx = await smartWallet
      .connect(signers.bob)
      .delegateUserDecryption(signers.bob.address, tokenAddress, expirationTimestamp);
    await delegateTx.wait();

    // Get the current smartWallet balance before transfer
    const smartWalletBalanceBefore = (await token.balanceOf(smartWalletAddress)) as Hex;

    const decryptedBalanceBefore = await delegatedUserDecryptSingleHandle(
      smartWalletBalanceBefore,
      tokenAddress,
      smartWalletAddress,
      accountFor(signers.bob),
    );

    // Bob proposes a transaction from the smartWallet to transfer tokens to Carol.
    // The encrypted input must be created for the smartWallet address since it will be the msg.sender.
    const transferAmount = 100000n;
    const input = fhevm.createEncryptedInput(tokenAddress, smartWalletAddress);
    input.add64(transferAmount);
    const encryptedTransferAmount = await input.encrypt();
    const [transferHandle] = encryptedTransferAmount.handles;
    if (transferHandle === undefined) throw new Error('encrypt() returned no handle');

    // Encode the transfer function call with full signature to avoid ambiguity.
    const transferData = token.interface.encodeFunctionData('transfer(address,bytes32,bytes)', [
      signers.carol.address,
      transferHandle,
      encryptedTransferAmount.inputProof,
    ]);

    // Propose the transaction.
    const proposeTx = await smartWallet.connect(signers.bob).proposeTx(tokenAddress, transferData);
    await proposeTx.wait();

    // Get the transaction ID.
    const txId = await smartWallet.txCounter();

    // Execute the transaction.
    const executeTx = await smartWallet.connect(signers.bob).executeTx(txId);
    await executeTx.wait();

    // Verify the smartWallet balance decreased.
    const smartWalletBalanceAfter = (await token.balanceOf(smartWalletAddress)) as Hex;
    const decryptedBalanceAfter = await delegatedUserDecryptSingleHandle(
      smartWalletBalanceAfter,
      tokenAddress,
      smartWalletAddress,
      accountFor(signers.bob),
    );

    // The smartWallet balance should have decreased by the transfer amount.
    expect(Number(decryptedBalanceBefore) - Number(decryptedBalanceAfter)).to.equal(Number(transferAmount));
  });

  it('test delegated user decryption - smartWallet revokes the delegation of user decryption to an EOA', async function () {
    // First, ensure Bob has delegation.
    const expirationTimestamp = (await timestampNowAdjusted()) + 86400; // 24 hours from now
    const delegateTx = await smartWallet
      .connect(signers.bob)
      .delegateUserDecryption(signers.bob.address, tokenAddress, expirationTimestamp);
    await delegateTx.wait();

    // Revoke the delegation for Bob's EOA.
    const revokeTx = await smartWallet
      .connect(signers.bob)
      .revokeUserDecryptionDelegation(signers.bob.address, tokenAddress);
    await revokeTx.wait();

    // Wait for 15 blocks to ensure revocation is propagated by the coprocessor.
    await waitNBlocks(connection, 15);

    // Try to decrypt the smartWallet balance with Bob's EOA, which should now fail.
    const balanceHandle = (await token.balanceOf(smartWalletAddress)) as Hex;

    await expectRejectedWith(
      delegatedUserDecryptSingleHandle(balanceHandle, tokenAddress, smartWalletAddress, accountFor(signers.bob)),
      /^Delegate (.+) is not delegated by (.+) to user decrypt handle (.+) on contract (.+)!/,
    );
  });
});
