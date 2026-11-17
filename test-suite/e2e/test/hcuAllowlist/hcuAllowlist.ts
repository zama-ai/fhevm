// E2E coverage of the HCULimit computation allowlist.
//
// While the allowlist is active, only allowlisted dapp contracts may request HCU-charged FHE
// computation on the host chain. The ACL owner controls activation, removals and allowlister roles.
// Allowlisters may only add dapps, until a fixed deadline.
//
// Test cases:
//
//   - computation gate
//       . inactive by default, so computation is unrestricted
//       . every operation family is rejected for a dapp absent from the allowlist (trivial
//         encryption, binary, comparison, scalar, unary, cast, ternary, random, bounded random,
//         sum, isIn, mulDiv), and a rejected transaction consumes no block HCU
//       . the same operations succeed for an allowlisted dapp and their results decrypt
//       . input verification, ACL permissions and decryption stay outside the gate
//       . removal, deactivation and reactivation, with membership preserved across them
//       . an allowlisted dapp stays subject to the depth, block and transaction HCU limits
//       . only the FHEVMExecutor may report a computation to HCULimit
//   - block HCU whitelist interaction
//       . all six combinations of activation, allowlist membership and block HCU whitelist membership
//   - allowlisters
//       . an active allowlister adds dapps and can do nothing else
//       . expiration at the deadline, also while the allowlist is inactive, then a new grant
//       . revocation by the ACL owner and renunciation by the allowlister
//       . an allowlister re-adds a removed dapp until its role is revoked
//       . the ACL owner extends and shortens an active grant
//       . an account with no role is rejected by every administration function
//   - batches
//       . additions skip existing members, removals are strict, invalid entries revert the batch
//       . empty batches are rejected while the allowlist is active
//   - factories and constructors
//       . a dapp deployed by an allowlisted factory must be allowlisted itself
//       . a dapp computing in its constructor needs its CREATE2 address allowlisted before deployment
//
// Not covered here: ACL ownership transfer (it would change the owner of the shared stack) and the
// upgrade path (covered by the host-contracts tests).
//
// Shared state: HCULimit is shared by every suite on the stack, and an active allowlist blocks all
// other dapps. Each test starts with the allowlist inactive, and afterEach restores activation, the
// HCU limits, allowlister grants and block HCU whitelist entries, even when a test fails.
//
// Expiration tests wait on real block time (about 20 seconds each) instead of moving the chain clock.
import { assert, expect } from 'chai';
import type { ContractTransactionResponse, Signer, TransactionReceipt, Wallet } from 'ethers';
import { ethers } from 'hardhat';

import type { HCUAllowlistDapp, HCUAllowlistFactory } from '../../types';
import { createInstances } from '../instance';
import { isLiveNetwork } from '../network';
import { getSigners, initSigners } from '../signers';
import { waitForTransactionReceipt } from '../utils';

// HCULimit is deployed by the host-sc stack but not compiled in the E2E test suite.
const HCU_LIMIT_ABI = [
  'function getBlockMeter() view returns (uint48, uint48)',
  'function getGlobalHCUCapPerBlock() view returns (uint48)',
  'function getMaxHCUPerTx() view returns (uint48)',
  'function getMaxHCUDepthPerTx() view returns (uint48)',
  'function setHCUPerBlock(uint48)',
  'function setMaxHCUPerTx(uint48)',
  'function setMaxHCUDepthPerTx(uint48)',
  'function addToBlockHCUWhitelist(address)',
  'function removeFromBlockHCUWhitelist(address)',
  'function isBlockHCUWhitelisted(address) view returns (bool)',
  'function isAllowlistActivated() view returns (bool)',
  'function isAllowlisted(address) view returns (bool)',
  'function isAllowlister(address) view returns (bool)',
  'function allowlisterDeadline(address) view returns (uint256)',
  'function setAllowlistActivated(bool)',
  'function addToAllowlist(address[])',
  'function removeFromAllowlist(address[])',
  'function addAllowlister(address,uint256)',
  'function removeAllowlister(address)',
  'function renounceAllowlister()',
  'function checkHCUForTrivialEncrypt(uint8,bytes32,address)',
  'function checkHCUForFheRand(uint8,bytes32,address)',
  'event AllowlistActivationChanged(bool activated)',
  'event AllowlistAdded(address indexed account, address indexed addedBy)',
  'event AllowlistRemoved(address indexed account)',
  'event AllowlisterAdded(address indexed account, uint256 deadline)',
  'event AllowlisterRemoved(address indexed account)',
  'error NotHostOwner(address)',
  'error CallerNotAllowlisted(address)',
  'error CallerNotACLOwnerOrAllowlister(address)',
  'error NotAllowlisted(address)',
  'error EmptyBatch()',
  'error InvalidAllowlisterDeadline(uint256)',
  'error NotAllowlister(address)',
  'error InvalidZeroAddress()',
  'error HCUBlockLimitExceeded()',
  'error HCUTransactionLimitExceeded()',
  'error HCUTransactionDepthLimitExceeded()',
  'error CallerMustBeFHEVMExecutorContract()',
];

const hcuLimitInterface = new ethers.Interface(HCU_LIMIT_ABI);

// Fixed gas limit to mine a transaction that is expected to revert.
const REVERTING_TX_GAS_LIMIT = 1_000_000;
// Lifetime of the short allowlister grant used to observe expiration.
const ALLOWLISTER_TTL_SECONDS = 20;
const ONE_DAY_SECONDS = 24 * 3600;
// Below the cost of a non-scalar euint64 addition.
const TIGHT_DEPTH_PER_TX = 100_000;
// Fits one trivial encryption (32 HCU), not two.
const ONE_TRIVIAL_ENCRYPTION_HCU = 40;
// FheType.Uint64 in the host contracts.
const FHE_TYPE_UINT64 = 5;

type Account = Signer & { address: string };

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

const latestTimestamp = async (): Promise<number> => {
  const block = await ethers.provider.getBlock('latest');
  if (!block) {
    throw new Error('latest block is missing');
  }
  return block.timestamp;
};

const waitForTimestamp = async (timestamp: number, timeoutMs = 120_000): Promise<void> => {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    if ((await latestTimestamp()) >= timestamp) {
      return;
    }
    await sleep(1_000);
  }
  throw new Error(`Timed out waiting for block timestamp ${timestamp}`);
};

const mined = async (tx: Promise<ContractTransactionResponse>): Promise<TransactionReceipt> => {
  const receipt = await (await tx).wait();
  if (!receipt) {
    throw new Error('receipt is missing');
  }
  return receipt;
};

// Collects every string nested in an error, where the node reports the revert data.
const errorStrings = (value: unknown, seen = new Set<unknown>(), depth = 0): string[] => {
  if (typeof value === 'string') {
    return [value];
  }
  if (typeof value !== 'object' || value === null || seen.has(value) || depth > 6) {
    return [];
  }
  seen.add(value);
  return Object.getOwnPropertyNames(value).flatMap((key) =>
    errorStrings((value as Record<string, unknown>)[key], seen, depth + 1),
  );
};

const expectHcuLimitRevert = async (
  action: Promise<unknown>,
  errorName: string,
  args: unknown[] = [],
  label = 'call',
) => {
  const expectedData = hcuLimitInterface.encodeErrorResult(errorName, args).slice(2).toLowerCase();
  let error: unknown;
  try {
    await action;
  } catch (caught) {
    error = caught;
  }
  if (error === undefined) {
    expect.fail(`${label} should revert with ${errorName}`);
  }
  expect(errorStrings(error).join(' ').toLowerCase()).to.include(
    expectedData,
    `${label} should revert with ${errorName}`,
  );
};

describe('HCU allowlist', function () {
  let hcuLimit: ReturnType<typeof hcuLimitAs>;
  let hcuLimitAddress: string;
  let owner: Wallet;
  let alice: Account;
  let allowlister: Account;
  let outsider: Account;
  let initialActivation: boolean;
  let initialHCUPerBlock: bigint;
  let initialHCUPerTx: bigint;
  let initialDepthPerTx: bigint;
  let blockWhitelisted: string[];

  // `Contract.connect` drops the dynamic method typing, so restore it.
  function hcuLimitAs(runner: Signer) {
    return new ethers.Contract(hcuLimitAddress, HCU_LIMIT_ABI, runner);
  }

  const eventsOf = (receipt: TransactionReceipt, name: string): unknown[][] =>
    receipt.logs
      .filter((log) => log.address.toLowerCase() === hcuLimitAddress.toLowerCase())
      .map((log) => hcuLimitInterface.parseLog(log))
      .filter((parsed) => parsed?.name === name)
      .map((parsed) => [...parsed!.args]);

  const blockMeterAt = async (blockNumber: number): Promise<bigint> => {
    const [, usedHCU] = await hcuLimit.getBlockMeter({ blockTag: blockNumber });
    return usedHCU;
  };

  const deployDapp = async (): Promise<HCUAllowlistDapp> => {
    const factory = await ethers.getContractFactory('HCUAllowlistDapp');
    const dapp = await factory.connect(alice).deploy();
    await dapp.waitForDeployment();
    return dapp;
  };

  const storeInput = async (ctx: Mocha.Context, dapp: HCUAllowlistDapp, value: bigint) => {
    const input = await ctx.instances.alice.encryptUint64({
      value,
      contractAddress: await dapp.getAddress(),
      userAddress: alice.address,
    });
    await mined(dapp.storeInput(input.handles[0], input.inputProof));
  };

  const publicDecrypt = async (ctx: Mocha.Context, handle: string) => {
    const res = await ctx.instances.alice.publicDecrypt([handle]);
    return res.clearValues[handle as `0x${string}`];
  };

  // Mines a computation expected to be rejected and checks it consumed no block HCU.
  const expectMinedRejection = async (dapp: HCUAllowlistDapp) => {
    const tx = await dapp.trivialEncrypt(1, { gasLimit: REVERTING_TX_GAS_LIMIT });
    const receipt = await waitForTransactionReceipt(tx.hash);
    expect(receipt.status).to.eq(0, 'computation should revert');
    expect(await blockMeterAt(receipt.blockNumber)).to.eq(0n, 'rejected computation should not consume block HCU');
  };

  before(async function () {
    if (isLiveNetwork()) {
      this.skip();
    }
    const address = process.env.HCU_LIMIT_CONTRACT_ADDRESS;
    const deployerKey = process.env.DEPLOYER_PRIVATE_KEY;
    if (!address || !deployerKey) {
      throw new Error('HCU_LIMIT_CONTRACT_ADDRESS and DEPLOYER_PRIVATE_KEY env vars are required for allowlist tests');
    }
    hcuLimitAddress = address;
    owner = new ethers.Wallet(deployerKey, ethers.provider);
    hcuLimit = hcuLimitAs(owner);

    // Skip on HCULimit deployments that predate the allowlist.
    try {
      initialActivation = await hcuLimit.isAllowlistActivated();
    } catch {
      this.skip();
    }
    initialHCUPerBlock = await hcuLimit.getGlobalHCUCapPerBlock();
    initialHCUPerTx = await hcuLimit.getMaxHCUPerTx();
    initialDepthPerTx = await hcuLimit.getMaxHCUDepthPerTx();

    await initSigners(3);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    alice = this.signers.alice;
    allowlister = this.signers.bob;
    outsider = this.signers.carol;
  });

  beforeEach(async function () {
    blockWhitelisted = [];
    if (await hcuLimit.isAllowlistActivated()) {
      await mined(hcuLimit.setAllowlistActivated(false));
    }
  });

  // Restores the shared HCULimit state, so a failing test cannot block other suites.
  afterEach(async function () {
    if (!hcuLimit || initialActivation === undefined) {
      return;
    }
    if ((await hcuLimit.isAllowlistActivated()) !== initialActivation) {
      await mined(hcuLimit.setAllowlistActivated(initialActivation));
    }
    // Raised in this order: block limit >= transaction limit >= depth limit must always hold.
    if ((await hcuLimit.getGlobalHCUCapPerBlock()) !== initialHCUPerBlock) {
      await mined(hcuLimit.setHCUPerBlock(initialHCUPerBlock));
    }
    if ((await hcuLimit.getMaxHCUPerTx()) !== initialHCUPerTx) {
      await mined(hcuLimit.setMaxHCUPerTx(initialHCUPerTx));
    }
    if ((await hcuLimit.getMaxHCUDepthPerTx()) !== initialDepthPerTx) {
      await mined(hcuLimit.setMaxHCUDepthPerTx(initialDepthPerTx));
    }
    for (const account of [allowlister, outsider]) {
      if ((await hcuLimit.allowlisterDeadline(account.address)) !== 0n) {
        await mined(hcuLimit.removeAllowlister(account.address));
      }
    }
    for (const account of blockWhitelisted) {
      if (await hcuLimit.isBlockHCUWhitelisted(account)) {
        await mined(hcuLimit.removeFromBlockHCUWhitelist(account));
      }
    }
  });

  describe('computation gate', function () {
    // Default state: no restriction until the ACL owner activates the allowlist.
    it('should leave the allowlist inactive on a fresh deployment', async function () {
      expect(initialActivation).to.eq(false);

      const dapp = await deployDapp();
      expect(await hcuLimit.isAllowlisted(await dapp.getAddress())).to.eq(false);

      await mined(dapp.addTrivialValues(40, 2));
      expect(await publicDecrypt(this, await dapp.result())).to.eq(42n);
    });

    // Each operation family is gated, and a rejected transaction consumes no block HCU.
    it('should reject every HCU-charged operation from a dapp absent from the active allowlist', async function () {
      const dapp = await deployDapp();
      const dappAddress = await dapp.getAddress();
      // Handles created before activation, so each call below is rejected on the operation it names.
      await storeInput(this, dapp, 7n);
      await mined(dapp.compareStored());
      const input = await this.instances.alice.encryptUint64({
        value: 7n,
        contractAddress: dappAddress,
        userAddress: alice.address,
      });

      await mined(hcuLimit.setAllowlistActivated(true));

      const rejected: [string, () => Promise<unknown>][] = [
        ['trivial encryption', () => dapp.trivialEncrypt.staticCall(1)],
        ['binary operation', () => dapp.addStoredToStored.staticCall()],
        ['binary operation on an input', () => dapp.addInputToStored.staticCall(input.handles[0], input.inputProof)],
        ['comparison', () => dapp.compareStored.staticCall()],
        ['scalar binary operation', () => dapp.addScalarToStored.staticCall(1)],
        ['unary operation', () => dapp.notStored.staticCall()],
        ['cast', () => dapp.castStored.staticCall()],
        ['ternary operation', () => dapp.selectStored.staticCall()],
        ['random generation', () => dapp.random.staticCall()],
        ['bounded random generation', () => dapp.randomBounded.staticCall(1024)],
        ['sum', () => dapp.sumStored.staticCall()],
        ['isIn', () => dapp.isInStored.staticCall()],
        ['mulDiv', () => dapp.mulDivStored.staticCall(3, 2)],
      ];
      for (const [label, call] of rejected) {
        await expectHcuLimitRevert(call(), 'CallerNotAllowlisted', [dappAddress], label);
      }

      await expectMinedRejection(dapp);
    });

    // Same operation families succeed once the dapp is allowlisted, and the results decrypt.
    it('should let an allowlisted dapp use every HCU-charged operation', async function () {
      const dapp = await deployDapp();
      const dappAddress = await dapp.getAddress();
      await storeInput(this, dapp, 7n);

      const receipt = await mined(hcuLimit.addToAllowlist([dappAddress]));
      assert.deepEqual(eventsOf(receipt, 'AllowlistAdded'), [[dappAddress, owner.address]]);
      const activation = await mined(hcuLimit.setAllowlistActivated(true));
      assert.deepEqual(eventsOf(activation, 'AllowlistActivationChanged'), [[true]]);
      expect(await hcuLimit.isAllowlistActivated()).to.eq(true);
      expect(await hcuLimit.isAllowlisted(dappAddress)).to.eq(true);

      await mined(dapp.trivialEncrypt(11));
      expect(await publicDecrypt(this, await dapp.result())).to.eq(11n);

      await mined(dapp.addScalarToStored(3));
      expect(await publicDecrypt(this, await dapp.result())).to.eq(10n);

      await mined(dapp.sumStored());
      expect(await publicDecrypt(this, await dapp.result())).to.eq(14n);

      await mined(dapp.mulDivStored(6, 2));
      expect(await publicDecrypt(this, await dapp.result())).to.eq(21n);

      await mined(dapp.randomBounded(1024));
      expect(await publicDecrypt(this, await dapp.result())).to.be.lessThan(1024n);

      // The remaining operations only need to be accepted on the host chain.
      await mined(dapp.notStored());
      await mined(dapp.castStored());
      await mined(dapp.compareStored());
      await mined(dapp.selectStored());
      await mined(dapp.random());
      await mined(dapp.isInStored());

      const input = await this.instances.alice.encryptUint64({
        value: 42n,
        contractAddress: dappAddress,
        userAddress: alice.address,
      });
      await mined(dapp.addStoredToStored());
      expect(await publicDecrypt(this, await dapp.result())).to.eq(14n);

      await mined(dapp.addInputToStored(input.handles[0], input.inputProof));
      const handle = await dapp.result();
      const decrypted = await this.instances.alice.userDecryptSingleHandle({
        handle,
        contractAddress: dappAddress,
        signer: alice,
      });
      expect(decrypted).to.eq(49n);
      expect(await publicDecrypt(this, handle)).to.eq(49n);
    });

    // The gate only covers HCU-charged computation.
    it('should keep input verification, ACL and decryption available to a dapp absent from the allowlist', async function () {
      const dapp = await deployDapp();
      const dappAddress = await dapp.getAddress();
      await mined(dapp.addTrivialValues(20, 3));
      const computedBeforeActivation = await dapp.result();

      await mined(hcuLimit.setAllowlistActivated(true));

      // Input verification and ACL permissions.
      await storeInput(this, dapp, 1234n);
      const stored = await dapp.stored();
      const decrypted = await this.instances.alice.userDecryptSingleHandle({
        handle: stored,
        contractAddress: dappAddress,
        signer: alice,
      });
      expect(decrypted).to.eq(1234n);

      // Public decryption permission and decryption.
      await mined(dapp.publishStored());
      expect(await publicDecrypt(this, stored)).to.eq(1234n);

      // Results computed before activation stay decryptable.
      expect(await publicDecrypt(this, computedBeforeActivation)).to.eq(23n);

      await expectHcuLimitRevert(dapp.addScalarToStored.staticCall(1), 'CallerNotAllowlisted', [dappAddress]);
    });

    // Rollout order: populate, activate, remove, deactivate, reactivate.
    it('should apply removal, deactivation and reactivation without clearing membership', async function () {
      const member = await deployDapp();
      const removed = await deployDapp();
      const stranger = await deployDapp();
      const memberAddress = await member.getAddress();
      const removedAddress = await removed.getAddress();
      const strangerAddress = await stranger.getAddress();

      // Population before activation has no effect on computation.
      await mined(hcuLimit.addToAllowlist([memberAddress, removedAddress]));
      await mined(stranger.trivialEncrypt(1));

      await mined(hcuLimit.setAllowlistActivated(true));
      await mined(member.trivialEncrypt(1));
      await mined(removed.trivialEncrypt(1));
      await expectHcuLimitRevert(stranger.trivialEncrypt.staticCall(1), 'CallerNotAllowlisted', [strangerAddress]);

      // Removal takes effect immediately.
      const removal = await mined(hcuLimit.removeFromAllowlist([removedAddress]));
      assert.deepEqual(eventsOf(removal, 'AllowlistRemoved'), [[removedAddress]]);
      expect(await hcuLimit.isAllowlisted(removedAddress)).to.eq(false);
      await expectHcuLimitRevert(removed.trivialEncrypt.staticCall(1), 'CallerNotAllowlisted', [removedAddress]);
      await expectMinedRejection(removed);

      // Deactivation restores access for everyone and keeps membership.
      const deactivation = await mined(hcuLimit.setAllowlistActivated(false));
      assert.deepEqual(eventsOf(deactivation, 'AllowlistActivationChanged'), [[false]]);
      await mined(removed.addTrivialValues(1, 2));
      expect(await publicDecrypt(this, await removed.result())).to.eq(3n);
      await mined(stranger.trivialEncrypt(1));
      expect(await hcuLimit.isAllowlisted(memberAddress)).to.eq(true);
      expect(await hcuLimit.isAllowlisted(removedAddress)).to.eq(false);

      // Reactivation enforces the preserved membership.
      await mined(hcuLimit.setAllowlistActivated(true));
      await mined(member.addTrivialValues(5, 6));
      expect(await publicDecrypt(this, await member.result())).to.eq(11n);
      await expectHcuLimitRevert(removed.trivialEncrypt.staticCall(1), 'CallerNotAllowlisted', [removedAddress]);
      await expectHcuLimitRevert(stranger.trivialEncrypt.staticCall(1), 'CallerNotAllowlisted', [strangerAddress]);
    });

    // Being allowlisted is not an exemption from the depth, block and transaction HCU limits.
    it('should keep an allowlisted dapp subject to the HCU limits', async function () {
      const dapp = await deployDapp();
      const dappAddress = await dapp.getAddress();
      await mined(hcuLimit.addToAllowlist([dappAddress]));
      await mined(hcuLimit.setAllowlistActivated(true));

      // Depth limit: the two trivial encryptions fit, the addition on top of them does not.
      await mined(hcuLimit.setMaxHCUDepthPerTx(TIGHT_DEPTH_PER_TX));
      await expectHcuLimitRevert(dapp.addTrivialValues.staticCall(1, 2), 'HCUTransactionDepthLimitExceeded');
      await mined(dapp.trivialEncrypt(1));

      // All three limits now fit one trivial encryption. The block limit is checked first.
      await mined(hcuLimit.setMaxHCUDepthPerTx(ONE_TRIVIAL_ENCRYPTION_HCU));
      await mined(hcuLimit.setMaxHCUPerTx(ONE_TRIVIAL_ENCRYPTION_HCU));
      await mined(hcuLimit.setHCUPerBlock(ONE_TRIVIAL_ENCRYPTION_HCU));
      await expectHcuLimitRevert(dapp.trivialEncryptTwice.staticCall(1, 2), 'HCUBlockLimitExceeded');
      await mined(dapp.trivialEncrypt(1));

      // Transaction limit: seen once the block HCU whitelist exempts the dapp from the block limit.
      blockWhitelisted.push(dappAddress);
      await mined(hcuLimit.addToBlockHCUWhitelist(dappAddress));
      await expectHcuLimitRevert(dapp.trivialEncryptTwice.staticCall(1, 2), 'HCUTransactionLimitExceeded');
      await mined(dapp.trivialEncrypt(1));
    });

    // Only the FHEVMExecutor reports computations, so no account can act on behalf of a dapp.
    it('should only accept HCU checks from the FHEVMExecutor', async function () {
      const dapp = await deployDapp();
      const stranger = await deployDapp();
      await mined(hcuLimit.addToAllowlist([await dapp.getAddress()]));
      await mined(hcuLimit.setAllowlistActivated(true));

      // The executor check comes first, whether or not the reported dapp is allowlisted.
      for (const signer of [outsider, owner]) {
        for (const reported of [dapp, stranger]) {
          const args = [FHE_TYPE_UINT64, ethers.ZeroHash, await reported.getAddress()];
          const asSigner = hcuLimitAs(signer);
          await expectHcuLimitRevert(
            asSigner.checkHCUForTrivialEncrypt.staticCall(...args),
            'CallerMustBeFHEVMExecutorContract',
          );
          await expectHcuLimitRevert(
            asSigner.checkHCUForFheRand.staticCall(...args),
            'CallerMustBeFHEVMExecutorContract',
          );
        }
      }
    });
  });

  describe('block HCU whitelist interaction', function () {
    // Every combination of allowlist activation, allowlist membership and block HCU whitelist membership.
    it('should require allowlist membership from a block HCU whitelisted dapp and keep both lists independent', async function () {
      const whitelisted = await deployDapp();
      const metered = await deployDapp();
      const whitelistedAddress = await whitelisted.getAddress();
      const meteredAddress = await metered.getAddress();

      blockWhitelisted.push(whitelistedAddress);
      await mined(hcuLimit.addToBlockHCUWhitelist(whitelistedAddress));
      expect(await hcuLimit.isAllowlisted(whitelistedAddress)).to.eq(false);

      const expectAccepted = async (dapp: HCUAllowlistDapp, bypassesBlockMeter: boolean) => {
        const receipt = await mined(dapp.addTrivialValues(1, 2));
        const usedHCU = await blockMeterAt(receipt.blockNumber);
        if (bypassesBlockMeter) {
          expect(usedHCU).to.eq(0n, 'block HCU whitelisted dapp should bypass the block meter');
        } else {
          expect(usedHCU).to.be.greaterThan(0n, 'block meter should count the computation');
        }
      };

      // Allowlist inactive: membership is irrelevant.
      await expectAccepted(whitelisted, true);
      await expectAccepted(metered, false);

      // Allowlist active, neither dapp allowlisted.
      await mined(hcuLimit.setAllowlistActivated(true));
      await expectHcuLimitRevert(whitelisted.trivialEncrypt.staticCall(1), 'CallerNotAllowlisted', [
        whitelistedAddress,
      ]);
      await expectHcuLimitRevert(metered.trivialEncrypt.staticCall(1), 'CallerNotAllowlisted', [meteredAddress]);
      await expectMinedRejection(whitelisted);
      await expectMinedRejection(metered);

      // Allowlist active, both dapps allowlisted.
      await mined(hcuLimit.addToAllowlist([whitelistedAddress, meteredAddress]));
      expect(await hcuLimit.isBlockHCUWhitelisted(meteredAddress)).to.eq(false);
      await expectAccepted(whitelisted, true);
      await expectAccepted(metered, false);

      // Leaving the block HCU whitelist keeps the allowlist membership.
      await mined(hcuLimit.removeFromBlockHCUWhitelist(whitelistedAddress));
      expect(await hcuLimit.isAllowlisted(whitelistedAddress)).to.eq(true);
      await expectAccepted(whitelisted, false);
    });
  });

  describe('allowlisters', function () {
    // Delegated role: additions only, attributed to the allowlister in the event.
    it('should let an active allowlister add dapps and nothing else', async function () {
      const dapp = await deployDapp();
      const dappAddress = await dapp.getAddress();
      const asAllowlister = hcuLimitAs(allowlister);
      const deadline = (await latestTimestamp()) + ONE_DAY_SECONDS;

      await expectHcuLimitRevert(
        asAllowlister.addToAllowlist.staticCall([dappAddress]),
        'CallerNotACLOwnerOrAllowlister',
        [allowlister.address],
      );

      const grant = await mined(hcuLimit.addAllowlister(allowlister.address, deadline));
      assert.deepEqual(eventsOf(grant, 'AllowlisterAdded'), [[allowlister.address, BigInt(deadline)]]);
      expect(await hcuLimit.isAllowlister(allowlister.address)).to.eq(true);
      expect(await hcuLimit.allowlisterDeadline(allowlister.address)).to.eq(BigInt(deadline));

      await mined(hcuLimit.setAllowlistActivated(true));
      await expectHcuLimitRevert(dapp.trivialEncrypt.staticCall(1), 'CallerNotAllowlisted', [dappAddress]);

      const addition = await mined(asAllowlister.addToAllowlist([dappAddress]));
      assert.deepEqual(eventsOf(addition, 'AllowlistAdded'), [[dappAddress, allowlister.address]]);
      await mined(dapp.addTrivialValues(30, 12));
      expect(await publicDecrypt(this, await dapp.result())).to.eq(42n);

      // Everything else stays with the ACL owner.
      const notOwner = [allowlister.address];
      await expectHcuLimitRevert(asAllowlister.removeFromAllowlist.staticCall([dappAddress]), 'NotHostOwner', notOwner);
      await expectHcuLimitRevert(asAllowlister.setAllowlistActivated.staticCall(false), 'NotHostOwner', notOwner);
      await expectHcuLimitRevert(
        asAllowlister.addAllowlister.staticCall(outsider.address, deadline),
        'NotHostOwner',
        notOwner,
      );
      await expectHcuLimitRevert(
        asAllowlister.removeAllowlister.staticCall(allowlister.address),
        'NotHostOwner',
        notOwner,
      );
      await expectHcuLimitRevert(
        asAllowlister.addToBlockHCUWhitelist.staticCall(dappAddress),
        'NotHostOwner',
        notOwner,
      );
    });

    // Expiration needs no transaction, is not paused by deactivation and keeps the dapps it added.
    it('should expire an allowlister at its deadline, even while the allowlist is inactive', async function () {
      const added = await deployDapp();
      const late = await deployDapp();
      const addedAddress = await added.getAddress();
      const lateAddress = await late.getAddress();
      const asAllowlister = hcuLimitAs(allowlister);

      const deadline = (await latestTimestamp()) + ALLOWLISTER_TTL_SECONDS;
      await mined(hcuLimit.addAllowlister(allowlister.address, deadline));
      const addition = await mined(asAllowlister.addToAllowlist([addedAddress]));
      const additionBlock = await ethers.provider.getBlock(addition.blockNumber);
      expect(additionBlock!.timestamp).to.be.lessThan(deadline);

      // The allowlist stays inactive while the grant expires.
      await waitForTimestamp(deadline);
      expect(await hcuLimit.isAllowlister(allowlister.address)).to.eq(false);
      expect(await hcuLimit.allowlisterDeadline(allowlister.address)).to.eq(BigInt(deadline));
      await expectHcuLimitRevert(
        asAllowlister.addToAllowlist.staticCall([lateAddress]),
        'CallerNotACLOwnerOrAllowlister',
        [allowlister.address],
      );

      // Expiration does not remove the dapps added before it.
      await mined(hcuLimit.setAllowlistActivated(true));
      expect(await hcuLimit.isAllowlisted(addedAddress)).to.eq(true);
      await mined(added.trivialEncrypt(1));
      await expectHcuLimitRevert(
        asAllowlister.addToAllowlist.staticCall([lateAddress]),
        'CallerNotACLOwnerOrAllowlister',
        [allowlister.address],
      );
      await expectHcuLimitRevert(late.trivialEncrypt.staticCall(1), 'CallerNotAllowlisted', [lateAddress]);

      // A deadline that is not in the future is rejected.
      const now = await latestTimestamp();
      await expectHcuLimitRevert(
        hcuLimit.addAllowlister.staticCall(allowlister.address, now),
        'InvalidAllowlisterDeadline',
        [now],
      );
      await expectHcuLimitRevert(
        hcuLimit.addAllowlister.staticCall(allowlister.address, 0),
        'InvalidAllowlisterDeadline',
        [0],
      );

      // An expired grant can still be renounced, which clears its stored deadline.
      const renunciation = await mined(asAllowlister.renounceAllowlister());
      assert.deepEqual(eventsOf(renunciation, 'AllowlisterRemoved'), [[allowlister.address]]);
      expect(await hcuLimit.allowlisterDeadline(allowlister.address)).to.eq(0n);

      // The ACL owner grants the role again.
      const renewedDeadline = (await latestTimestamp()) + ONE_DAY_SECONDS;
      await mined(hcuLimit.addAllowlister(allowlister.address, renewedDeadline));
      expect(await hcuLimit.isAllowlister(allowlister.address)).to.eq(true);
      await mined(asAllowlister.addToAllowlist([lateAddress]));
      await mined(late.addTrivialValues(2, 3));
      expect(await publicDecrypt(this, await late.result())).to.eq(5n);
    });

    // Both ways of ending a role early, including the idempotent revocation.
    it('should end an allowlister role on revocation or renunciation and keep the dapps it added', async function () {
      const first = await deployDapp();
      const second = await deployDapp();
      const firstAddress = await first.getAddress();
      const secondAddress = await second.getAddress();
      const asAllowlister = hcuLimitAs(allowlister);
      const asOutsider = hcuLimitAs(outsider);
      const deadline = (await latestTimestamp()) + ONE_DAY_SECONDS;
      const notAllowlister = [allowlister.address];

      await mined(hcuLimit.setAllowlistActivated(true));

      // Revocation by the ACL owner.
      await mined(hcuLimit.addAllowlister(allowlister.address, deadline));
      await mined(asAllowlister.addToAllowlist([firstAddress]));
      const revocation = await mined(hcuLimit.removeAllowlister(allowlister.address));
      assert.deepEqual(eventsOf(revocation, 'AllowlisterRemoved'), [[allowlister.address]]);
      expect(await hcuLimit.isAllowlister(allowlister.address)).to.eq(false);
      expect(await hcuLimit.allowlisterDeadline(allowlister.address)).to.eq(0n);
      await expectHcuLimitRevert(
        asAllowlister.addToAllowlist.staticCall([secondAddress]),
        'CallerNotACLOwnerOrAllowlister',
        notAllowlister,
      );
      await mined(first.trivialEncrypt(1));

      // Revoking an absent grant succeeds without an event.
      const repeated = await mined(hcuLimit.removeAllowlister(allowlister.address));
      assert.deepEqual(eventsOf(repeated, 'AllowlisterRemoved'), []);

      // Renunciation by the allowlister.
      await expectHcuLimitRevert(asAllowlister.renounceAllowlister.staticCall(), 'NotAllowlister', notAllowlister);
      await expectHcuLimitRevert(asOutsider.renounceAllowlister.staticCall(), 'NotAllowlister', [outsider.address]);
      await mined(hcuLimit.addAllowlister(allowlister.address, deadline));
      await mined(asAllowlister.addToAllowlist([secondAddress]));
      const renunciation = await mined(asAllowlister.renounceAllowlister());
      assert.deepEqual(eventsOf(renunciation, 'AllowlisterRemoved'), [[allowlister.address]]);
      expect(await hcuLimit.allowlisterDeadline(allowlister.address)).to.eq(0n);
      await expectHcuLimitRevert(
        asAllowlister.addToAllowlist.staticCall([firstAddress]),
        'CallerNotACLOwnerOrAllowlister',
        notAllowlister,
      );

      expect(await hcuLimit.isAllowlisted(firstAddress)).to.eq(true);
      expect(await hcuLimit.isAllowlisted(secondAddress)).to.eq(true);
      await mined(second.addTrivialValues(4, 4));
      expect(await publicDecrypt(this, await second.result())).to.eq(8n);
    });

    // An active allowlister can undo a removal, so the role must be revoked before the dapp is removed.
    it('should let an active allowlister re-add a removed dapp until its role is revoked', async function () {
      const dapp = await deployDapp();
      const dappAddress = await dapp.getAddress();
      const asAllowlister = hcuLimitAs(allowlister);

      await mined(hcuLimit.addAllowlister(allowlister.address, (await latestTimestamp()) + ONE_DAY_SECONDS));
      await mined(asAllowlister.addToAllowlist([dappAddress]));
      await mined(hcuLimit.setAllowlistActivated(true));

      // Removal alone does not hold.
      await mined(hcuLimit.removeFromAllowlist([dappAddress]));
      await expectHcuLimitRevert(dapp.trivialEncrypt.staticCall(1), 'CallerNotAllowlisted', [dappAddress]);
      const secondAddition = await mined(asAllowlister.addToAllowlist([dappAddress]));
      assert.deepEqual(eventsOf(secondAddition, 'AllowlistAdded'), [[dappAddress, allowlister.address]]);
      await mined(dapp.trivialEncrypt(1));

      // Revocation first, then removal.
      await mined(hcuLimit.removeAllowlister(allowlister.address));
      await mined(hcuLimit.removeFromAllowlist([dappAddress]));
      await expectHcuLimitRevert(
        asAllowlister.addToAllowlist.staticCall([dappAddress]),
        'CallerNotACLOwnerOrAllowlister',
        [allowlister.address],
      );
      await expectHcuLimitRevert(dapp.trivialEncrypt.staticCall(1), 'CallerNotAllowlisted', [dappAddress]);
      await expectMinedRejection(dapp);
    });

    // The ACL owner replaces the deadline of an active grant; the allowlister cannot extend its own.
    it('should let the ACL owner extend and shorten an active allowlister grant', async function () {
      const asAllowlister = hcuLimitAs(allowlister);
      const [early, late] = [1, 2].map(() => ethers.Wallet.createRandom().address);
      const deadline = (await latestTimestamp()) + ONE_DAY_SECONDS;
      await mined(hcuLimit.addAllowlister(allowlister.address, deadline));

      const extended = deadline + ONE_DAY_SECONDS;
      await expectHcuLimitRevert(
        asAllowlister.addAllowlister.staticCall(allowlister.address, extended),
        'NotHostOwner',
        [allowlister.address],
      );
      const extension = await mined(hcuLimit.addAllowlister(allowlister.address, extended));
      assert.deepEqual(eventsOf(extension, 'AllowlisterAdded'), [[allowlister.address, BigInt(extended)]]);
      expect(await hcuLimit.allowlisterDeadline(allowlister.address)).to.eq(BigInt(extended));

      // The shortened deadline is the one enforced.
      const shortened = (await latestTimestamp()) + ALLOWLISTER_TTL_SECONDS;
      const shortening = await mined(hcuLimit.addAllowlister(allowlister.address, shortened));
      assert.deepEqual(eventsOf(shortening, 'AllowlisterAdded'), [[allowlister.address, BigInt(shortened)]]);
      expect(await hcuLimit.allowlisterDeadline(allowlister.address)).to.eq(BigInt(shortened));
      expect(await hcuLimit.isAllowlister(allowlister.address)).to.eq(true);
      await mined(asAllowlister.addToAllowlist([early]));

      await waitForTimestamp(shortened);
      expect(await hcuLimit.isAllowlister(allowlister.address)).to.eq(false);
      await expectHcuLimitRevert(asAllowlister.addToAllowlist.staticCall([late]), 'CallerNotACLOwnerOrAllowlister', [
        allowlister.address,
      ]);

      await mined(hcuLimit.removeFromAllowlist([early]));
    });

    // An account with no role gets the authorization error of each function, even with an empty batch.
    it('should reserve allowlist administration to the ACL owner', async function () {
      const dapp = await deployDapp();
      const dappAddress = await dapp.getAddress();
      const asOutsider = hcuLimitAs(outsider);
      const deadline = (await latestTimestamp()) + ONE_DAY_SECONDS;
      const notOwner = [outsider.address];

      await mined(hcuLimit.addToAllowlist([dappAddress]));

      await expectHcuLimitRevert(asOutsider.setAllowlistActivated.staticCall(true), 'NotHostOwner', notOwner);
      await expectHcuLimitRevert(
        asOutsider.addToAllowlist.staticCall([dappAddress]),
        'CallerNotACLOwnerOrAllowlister',
        notOwner,
      );
      await expectHcuLimitRevert(asOutsider.removeFromAllowlist.staticCall([dappAddress]), 'NotHostOwner', notOwner);
      await expectHcuLimitRevert(
        asOutsider.addAllowlister.staticCall(outsider.address, deadline),
        'NotHostOwner',
        notOwner,
      );
      await expectHcuLimitRevert(
        asOutsider.removeAllowlister.staticCall(allowlister.address),
        'NotHostOwner',
        notOwner,
      );

      // Authorization is checked before the batch content.
      await expectHcuLimitRevert(asOutsider.addToAllowlist.staticCall([]), 'CallerNotACLOwnerOrAllowlister', notOwner);
      await expectHcuLimitRevert(asOutsider.removeFromAllowlist.staticCall([]), 'NotHostOwner', notOwner);
    });
  });

  describe('batches', function () {
    // Lenient additions (skip, one event per new member) against strict, atomic removals.
    it('should skip existing members on addition and revert a whole batch on invalid entries', async function () {
      const [existing, added, other] = [1, 2, 3].map(() => ethers.Wallet.createRandom().address);
      const asAllowlister = hcuLimitAs(allowlister);
      await mined(hcuLimit.addAllowlister(allowlister.address, (await latestTimestamp()) + ONE_DAY_SECONDS));

      await mined(hcuLimit.addToAllowlist([existing]));

      // Existing members and duplicates are skipped, with one event per new member.
      const mixed = await mined(asAllowlister.addToAllowlist([existing, added, added]));
      assert.deepEqual(eventsOf(mixed, 'AllowlistAdded'), [[added, allowlister.address]]);
      const replay = await mined(hcuLimit.addToAllowlist([existing, added]));
      assert.deepEqual(eventsOf(replay, 'AllowlistAdded'), []);

      // A zero address or an empty batch reverts the whole addition.
      await expectHcuLimitRevert(hcuLimit.addToAllowlist.staticCall([other, ethers.ZeroAddress]), 'InvalidZeroAddress');
      await expectHcuLimitRevert(hcuLimit.addToAllowlist.staticCall([]), 'EmptyBatch');
      await expectHcuLimitRevert(asAllowlister.addToAllowlist.staticCall([]), 'EmptyBatch');
      await expectHcuLimitRevert(
        hcuLimit.addAllowlister.staticCall(ethers.ZeroAddress, 2n ** 64n),
        'InvalidZeroAddress',
      );

      // Removal is strict: a missing or repeated member reverts the whole batch.
      await expectHcuLimitRevert(hcuLimit.removeFromAllowlist.staticCall([]), 'EmptyBatch');
      await expectHcuLimitRevert(hcuLimit.removeFromAllowlist.staticCall([existing, other]), 'NotAllowlisted', [other]);
      await expectHcuLimitRevert(hcuLimit.removeFromAllowlist.staticCall([existing, existing]), 'NotAllowlisted', [
        existing,
      ]);
      const tx = await hcuLimit.removeFromAllowlist([existing, other], { gasLimit: REVERTING_TX_GAS_LIMIT });
      expect((await waitForTransactionReceipt(tx.hash)).status).to.eq(0);
      expect(await hcuLimit.isAllowlisted(existing)).to.eq(true);

      const removal = await mined(hcuLimit.removeFromAllowlist([existing, added]));
      assert.deepEqual(eventsOf(removal, 'AllowlistRemoved'), [[existing], [added]]);
      expect(await hcuLimit.isAllowlisted(existing)).to.eq(false);
      expect(await hcuLimit.isAllowlisted(added)).to.eq(false);
    });

    // Empty batches are rejected the same way while the allowlist is active.
    it('should reject empty batches while the allowlist is active', async function () {
      const asAllowlister = hcuLimitAs(allowlister);
      const asOutsider = hcuLimitAs(outsider);
      await mined(hcuLimit.addAllowlister(allowlister.address, (await latestTimestamp()) + ONE_DAY_SECONDS));
      await mined(hcuLimit.setAllowlistActivated(true));

      await expectHcuLimitRevert(hcuLimit.addToAllowlist.staticCall([]), 'EmptyBatch');
      await expectHcuLimitRevert(asAllowlister.addToAllowlist.staticCall([]), 'EmptyBatch');
      await expectHcuLimitRevert(hcuLimit.removeFromAllowlist.staticCall([]), 'EmptyBatch');

      // Unauthorized callers still get the authorization error.
      await expectHcuLimitRevert(asOutsider.addToAllowlist.staticCall([]), 'CallerNotACLOwnerOrAllowlister', [
        outsider.address,
      ]);
      await expectHcuLimitRevert(asAllowlister.removeFromAllowlist.staticCall([]), 'NotHostOwner', [
        allowlister.address,
      ]);

      // A mined empty batch reverts and emits nothing.
      const tx = await asAllowlister.addToAllowlist([], { gasLimit: REVERTING_TX_GAS_LIMIT });
      const receipt = await waitForTransactionReceipt(tx.hash);
      expect(receipt.status).to.eq(0);
      expect(receipt.logs).to.have.lengthOf(0);
    });
  });

  describe('factories and constructors', function () {
    let factory: HCUAllowlistFactory;
    let factoryAddress: string;

    const deployedDapp = (receipt: TransactionReceipt): string => {
      const log = receipt.logs
        .filter((entry) => entry.address.toLowerCase() === factoryAddress.toLowerCase())
        .map((entry) => factory.interface.parseLog(entry))
        .find((parsed) => parsed?.name === 'DappDeployed');
      if (!log) {
        throw new Error('DappDeployed event is missing');
      }
      return log.args[0];
    };

    beforeEach(async function () {
      const contractFactory = await ethers.getContractFactory('HCUAllowlistFactory');
      factory = await contractFactory.connect(alice).deploy();
      await factory.waitForDeployment();
      factoryAddress = await factory.getAddress();
      await mined(hcuLimit.addToAllowlist([factoryAddress]));
      await mined(hcuLimit.addAllowlister(allowlister.address, (await latestTimestamp()) + ONE_DAY_SECONDS));
      await mined(hcuLimit.setAllowlistActivated(true));
    });

    // Each deployed dapp must be allowlisted itself.
    it('should not extend the allowlisting of a factory to the dapps it deploys', async function () {
      await mined(factory.compute(9));
      expect(await publicDecrypt(this, await factory.result())).to.eq(9n);

      const dappAddress = deployedDapp(await mined(factory.deployDapp()));
      const dapp = (await ethers.getContractAt('HCUAllowlistDapp', dappAddress, alice)) as unknown as HCUAllowlistDapp;
      expect(await hcuLimit.isAllowlisted(dappAddress)).to.eq(false);
      await expectHcuLimitRevert(dapp.trivialEncrypt.staticCall(1), 'CallerNotAllowlisted', [dappAddress]);

      await mined(hcuLimitAs(allowlister).addToAllowlist([dappAddress]));
      await mined(dapp.addTrivialValues(8, 9));
      expect(await publicDecrypt(this, await dapp.result())).to.eq(17n);
    });

    // Constructor computation needs the CREATE2 address allowlisted before deployment.
    it('should deploy a dapp computing in its constructor only once its address is allowlisted', async function () {
      const salt = ethers.hexlify(ethers.randomBytes(32));
      const predicted = await factory.predictConstructorDapp(salt, 77);

      await expectHcuLimitRevert(factory.deployConstructorDapp.staticCall(salt, 77), 'CallerNotAllowlisted', [
        predicted,
      ]);
      const tx = await factory.deployConstructorDapp(salt, 77, { gasLimit: 5_000_000 });
      expect((await waitForTransactionReceipt(tx.hash)).status).to.eq(0);
      expect(await ethers.provider.getCode(predicted)).to.eq('0x');

      // The predicted address is allowlisted before any code exists at it.
      const addition = await mined(hcuLimitAs(allowlister).addToAllowlist([predicted]));
      assert.deepEqual(eventsOf(addition, 'AllowlistAdded'), [[predicted, allowlister.address]]);

      expect(deployedDapp(await mined(factory.deployConstructorDapp(salt, 77)))).to.eq(predicted);
      const dapp = await ethers.getContractAt('HCUAllowlistConstructorDapp', predicted);
      expect(await publicDecrypt(this, await dapp.value())).to.eq(77n);
    });
  });
});
