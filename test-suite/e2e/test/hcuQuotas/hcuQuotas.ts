// E2E coverage of HCULimit quotas as a computation gate.
//
// Every contract computes in a quota: the one the ACL owner or a quota admin assigned it to, else the
// default quota. While the default quota is stopped (maximum 0), only assigned contracts may request
// HCU-charged FHE computation on the host chain. The ACL owner sets quotas, assigns contracts and names
// quota admins. A quota admin only moves contracts between the default quota and its own quota.
//
// Test cases:
//
//   - computation gate
//       . the default quota is open on a fresh deployment, so computation is unrestricted
//       . every operation family is refused for a dapp in the stopped default quota (trivial
//         encryption, binary, comparison, scalar, unary, cast, ternary, random, bounded random,
//         sum, isIn, mulDiv), and a refused transaction emits nothing
//       . the same operations succeed for a dapp assigned to a quota and their results decrypt
//       . input verification, ACL permissions and decryption stay outside the gate
//       . reassignment, reopening and stopping again, with assignments preserved across them
//       . a dapp in a quota stays subject to the depth, transaction and quota limits
//       . only the FHEVMExecutor may report a computation to HCULimit
//   - capped, unmetered and paused quotas
//   - quota admins
//       . a quota admin moves contracts into and out of its quota, and can do nothing else
//       . a pause freezes the quota's members for its admin
//       . removing an admin ends its rights and keeps the contracts it granted
//       . an account with no role is refused by every administration function
//   - factories and constructors
//       . a dapp deployed by a factory in a quota must be assigned itself
//       . a dapp computing in its constructor needs its CREATE2 address assigned before deployment
//
// Not covered here: ACL ownership transfer (it would change the owner of the shared stack) and the
// upgrade from the allowlist (covered by the host-contracts tests).
//
// Shared state: HCULimit is shared by every suite on the stack, and a stopped default quota blocks
// all other dapps. afterEach restores the default quota, the HCU limits and the quotas this suite
// uses, even when a test fails.
import { assert, expect } from 'chai';
import type { ContractTransactionResponse, Signer, TransactionReceipt, Wallet } from 'ethers';
import { ethers } from 'hardhat';

import type { HCUQuotaDapp, HCUQuotaFactory } from '../../types';
import { createInstances } from '../instance';
import { isLiveNetwork } from '../network';
import { getSigners, initSigners } from '../signers';
import { waitForTransactionReceipt } from '../utils';

// HCULimit is deployed by the host-sc stack but not compiled in the E2E test suite.
const HCU_LIMIT_ABI = [
  'function getVersion() view returns (string)',
  'function getQuota(uint16) view returns (uint48, uint48, uint48)',
  'function getContractQuota(address) view returns (uint16)',
  'function isQuotaPaused(uint16) view returns (bool)',
  'function getMaxHCUPerTx() view returns (uint48)',
  'function getMaxHCUDepthPerTx() view returns (uint48)',
  'function getHCUConfiguratorAddress() view returns (address)',
  'function checkHCUForTrivialEncrypt(uint8,bytes32,address)',
  'function checkHCUForFheRand(uint8,bytes32,address)',
  'error HCUQuotaStopped(address,uint16)',
  'error HCUQuotaPaused(address,uint16)',
  'error HCUQuotaExceeded(address,uint16,uint256)',
  'error HCUTransactionLimitExceeded()',
  'error HCUTransactionDepthLimitExceeded()',
  'error CallerMustBeFHEVMExecutorContract()',
];

// HCULimit accepts its settings from HCUConfigurator only.
const HCU_CONFIGURATOR_ABI = [
  'function getQuotaAdmin(uint16) view returns (address)',
  'function setQuota(uint16,uint48,uint48)',
  'function assignQuota(uint16,address)',
  'function setQuotaAdmin(uint16,address)',
  'function grantQuota(uint16,address)',
  'function revokeQuota(uint16,address)',
  'function pauseQuota(uint16)',
  'function unpauseQuota(uint16)',
  'function setMaxHCUPerTx(uint48)',
  'function setMaxHCUDepthPerTx(uint48)',
  'event QuotaAdminSet(uint16 indexed quotaId, address indexed admin)',
  'event ContractQuotaAssigned(uint16 indexed quotaId, address indexed account, address indexed setBy)',
  'error NotHostOwner(address)',
  'error NotPauser(address)',
  'error NotQuotaAdmin(uint16,address)',
  'error QuotaMembershipPaused(uint16)',
  'error DefaultQuotaNotStopped()',
];

const hcuInterface = new ethers.Interface([...HCU_LIMIT_ABI, ...HCU_CONFIGURATOR_ABI]);

const DEFAULT_QUOTA = 0;
// Quotas 1 and 2 replaced the allowlist; 3 belongs to the EncryptedERC20 HCU suite.
const DAPP_QUOTA = 10;
const UNMETERED_QUOTA = 11;
const ADMIN_QUOTA = 12;
const SUITE_QUOTAS = [DAPP_QUOTA, UNMETERED_QUOTA, ADMIN_QUOTA];
const NO_CAP = 2n ** 48n - 1n;
// A capped quota large enough for every computation of a test.
const ROOMY_QUOTA = 100_000_000_000n;
// Fixed gas limit to mine a transaction that is expected to revert.
const REVERTING_TX_GAS_LIMIT = 1_000_000;
// Below the cost of a non-scalar euint64 addition.
const TIGHT_DEPTH_PER_TX = 100_000;
// Fits one trivial encryption (32 HCU), not two.
const ONE_TRIVIAL_ENCRYPTION_HCU = 40;
// FheType.Uint64 in the host contracts.
const FHE_TYPE_UINT64 = 5;

type Account = Signer & { address: string };

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

const expectHcuRevert = async (action: Promise<unknown>, errorName: string, args: unknown[] = [], label = 'call') => {
  const expectedData = hcuInterface.encodeErrorResult(errorName, args).slice(2).toLowerCase();
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

describe('HCU quotas', function () {
  let hcuLimit: ReturnType<typeof hcuLimitAs>;
  let hcuLimitAddress: string;
  let hcuConfigurator: ReturnType<typeof hcuConfiguratorAs>;
  let hcuConfiguratorAddress: string;
  let owner: Wallet;
  let alice: Account;
  let admin: Account;
  let outsider: Account;
  let initialDefaultMax: bigint;
  let initialDefaultDecay: bigint;
  let initialHCUPerTx: bigint;
  let initialDepthPerTx: bigint;

  // `Contract.connect` drops the dynamic method typing, so restore it.
  function hcuLimitAs(runner: Signer) {
    return new ethers.Contract(hcuLimitAddress, HCU_LIMIT_ABI, runner);
  }

  function hcuConfiguratorAs(runner: Signer) {
    return new ethers.Contract(hcuConfiguratorAddress, HCU_CONFIGURATOR_ABI, runner);
  }

  const eventsOf = (receipt: TransactionReceipt, name: string): unknown[][] =>
    receipt.logs
      .filter((log) => log.address.toLowerCase() === hcuConfiguratorAddress.toLowerCase())
      .map((log) => hcuInterface.parseLog(log))
      .filter((parsed) => parsed?.name === name)
      .map((parsed) => [...parsed!.args]);

  const quotaUsedAt = async (quotaId: number, blockNumber: number): Promise<bigint> => {
    const [, , usedHCU] = await hcuLimit.getQuota(quotaId, { blockTag: blockNumber });
    return usedHCU;
  };

  const stopDefaultQuota = () => mined(hcuConfigurator.setQuota(DEFAULT_QUOTA, 0, 0));
  const reopenDefaultQuota = () =>
    mined(hcuConfigurator.setQuota(DEFAULT_QUOTA, initialDefaultMax, initialDefaultDecay));

  const deployDapp = async (): Promise<HCUQuotaDapp> => {
    const factory = await ethers.getContractFactory('HCUQuotaDapp');
    const dapp = await factory.connect(alice).deploy();
    await dapp.waitForDeployment();
    return dapp;
  };

  const storeInput = async (ctx: Mocha.Context, dapp: HCUQuotaDapp, value: bigint) => {
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

  // Mines a computation expected to be refused and checks it changed nothing.
  const expectMinedRejection = async (dapp: HCUQuotaDapp) => {
    const tx = await dapp.trivialEncrypt(1, { gasLimit: REVERTING_TX_GAS_LIMIT });
    const receipt = await waitForTransactionReceipt(tx.hash);
    expect(receipt.status).to.eq(0, 'computation should revert');
    expect(receipt.logs).to.have.lengthOf(0, 'refused computation should emit nothing');
  };

  before(async function () {
    if (isLiveNetwork()) {
      this.skip();
    }
    const address = process.env.HCU_LIMIT_CONTRACT_ADDRESS;
    const deployerKey = process.env.DEPLOYER_PRIVATE_KEY;
    if (!address || !deployerKey) {
      throw new Error('HCU_LIMIT_CONTRACT_ADDRESS and DEPLOYER_PRIVATE_KEY env vars are required for quota tests');
    }
    hcuLimitAddress = address;
    owner = new ethers.Wallet(deployerKey, ethers.provider);
    hcuLimit = hcuLimitAs(owner);

    // Quotas replaced the global block cap in HCULimit v0.6.0; skip on earlier deployments.
    const [, major, minor] = ((await hcuLimit.getVersion()) as string).match(/v(\d+)\.(\d+)\./) ?? [];
    if (Number(major) === 0 && Number(minor) < 6) {
      this.skip();
    }
    hcuConfiguratorAddress = await hcuLimit.getHCUConfiguratorAddress();
    hcuConfigurator = hcuConfiguratorAs(owner);
    [initialDefaultMax, initialDefaultDecay] = await hcuLimit.getQuota(DEFAULT_QUOTA);
    initialHCUPerTx = await hcuLimit.getMaxHCUPerTx();
    initialDepthPerTx = await hcuLimit.getMaxHCUDepthPerTx();

    await initSigners(3);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    alice = this.signers.alice;
    admin = this.signers.bob;
    outsider = this.signers.carol;
  });

  // Restores the shared HCULimit state, so a failing test cannot block other suites.
  afterEach(async function () {
    if (!hcuLimit || initialDefaultMax === undefined) {
      return;
    }
    const [defaultMax, defaultDecay] = await hcuLimit.getQuota(DEFAULT_QUOTA);
    if (defaultMax !== initialDefaultMax || defaultDecay !== initialDefaultDecay) {
      await reopenDefaultQuota();
    }
    if ((await hcuLimit.getMaxHCUPerTx()) !== initialHCUPerTx) {
      await mined(hcuConfigurator.setMaxHCUPerTx(initialHCUPerTx));
    }
    if ((await hcuLimit.getMaxHCUDepthPerTx()) !== initialDepthPerTx) {
      await mined(hcuConfigurator.setMaxHCUDepthPerTx(initialDepthPerTx));
    }
    for (const quotaId of SUITE_QUOTAS) {
      if (await hcuLimit.isQuotaPaused(quotaId)) {
        await mined(hcuConfigurator.unpauseQuota(quotaId));
      }
      if ((await hcuConfigurator.getQuotaAdmin(quotaId)) !== ethers.ZeroAddress) {
        await mined(hcuConfigurator.setQuotaAdmin(quotaId, ethers.ZeroAddress));
      }
      const [maxHCU] = await hcuLimit.getQuota(quotaId);
      if (maxHCU !== 0n) {
        await mined(hcuConfigurator.setQuota(quotaId, 0, 0));
      }
    }
  });

  describe('computation gate', function () {
    // Default state: no restriction until the ACL owner stops the default quota.
    it('should leave the default quota open on a fresh deployment', async function () {
      expect(initialDefaultMax).to.be.greaterThan(0n);

      const dapp = await deployDapp();
      expect(await hcuLimit.getContractQuota(await dapp.getAddress())).to.eq(BigInt(DEFAULT_QUOTA));

      await mined(dapp.addTrivialValues(40, 2));
      expect(await publicDecrypt(this, await dapp.result())).to.eq(42n);
    });

    // Each operation family is gated, and a refused transaction emits nothing.
    it('should refuse every HCU-charged operation from a dapp in the stopped default quota', async function () {
      const dapp = await deployDapp();
      const dappAddress = await dapp.getAddress();
      // Handles created before the stop, so each call below is refused on the operation it names.
      await storeInput(this, dapp, 7n);
      await mined(dapp.compareStored());
      const input = await this.instances.alice.encryptUint64({
        value: 7n,
        contractAddress: dappAddress,
        userAddress: alice.address,
      });

      await stopDefaultQuota();

      const refused: [string, () => Promise<unknown>][] = [
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
      for (const [label, call] of refused) {
        await expectHcuRevert(call(), 'HCUQuotaStopped', [dappAddress, DEFAULT_QUOTA], label);
      }

      await expectMinedRejection(dapp);
    });

    // Same operation families succeed once the dapp is assigned to a quota, and the results decrypt.
    it('should let a dapp assigned to a quota use every HCU-charged operation', async function () {
      const dapp = await deployDapp();
      const dappAddress = await dapp.getAddress();
      await storeInput(this, dapp, 7n);

      await mined(hcuConfigurator.setQuota(DAPP_QUOTA, ROOMY_QUOTA, ROOMY_QUOTA));
      const receipt = await mined(hcuConfigurator.assignQuota(DAPP_QUOTA, dappAddress));
      assert.deepEqual(eventsOf(receipt, 'ContractQuotaAssigned'), [[BigInt(DAPP_QUOTA), dappAddress, owner.address]]);
      await stopDefaultQuota();
      expect(await hcuLimit.getContractQuota(dappAddress)).to.eq(BigInt(DAPP_QUOTA));

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
    it('should keep input verification, ACL and decryption available to a dapp in the stopped default quota', async function () {
      const dapp = await deployDapp();
      const dappAddress = await dapp.getAddress();
      await mined(dapp.addTrivialValues(20, 3));
      const computedBeforeStop = await dapp.result();

      await stopDefaultQuota();

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

      // Results computed before the stop stay decryptable.
      expect(await publicDecrypt(this, computedBeforeStop)).to.eq(23n);

      await expectHcuRevert(dapp.addScalarToStored.staticCall(1), 'HCUQuotaStopped', [dappAddress, DEFAULT_QUOTA]);
    });

    // Rollout order: assign, stop the default quota, send one dapp back, reopen, stop again.
    it('should apply reassignment, reopening and stopping again without losing assignments', async function () {
      const member = await deployDapp();
      const removed = await deployDapp();
      const stranger = await deployDapp();
      const memberAddress = await member.getAddress();
      const removedAddress = await removed.getAddress();
      const strangerAddress = await stranger.getAddress();

      // Assignment while the default quota is open changes nothing for the others.
      await mined(hcuConfigurator.setQuota(DAPP_QUOTA, ROOMY_QUOTA, ROOMY_QUOTA));
      await mined(hcuConfigurator.assignQuota(DAPP_QUOTA, memberAddress));
      await mined(hcuConfigurator.assignQuota(DAPP_QUOTA, removedAddress));
      await mined(stranger.trivialEncrypt(1));

      await stopDefaultQuota();
      await mined(member.trivialEncrypt(1));
      await mined(removed.trivialEncrypt(1));
      await expectHcuRevert(stranger.trivialEncrypt.staticCall(1), 'HCUQuotaStopped', [strangerAddress, DEFAULT_QUOTA]);

      // Sending a dapp back to the default quota takes effect immediately.
      await mined(hcuConfigurator.assignQuota(DEFAULT_QUOTA, removedAddress));
      expect(await hcuLimit.getContractQuota(removedAddress)).to.eq(BigInt(DEFAULT_QUOTA));
      await expectHcuRevert(removed.trivialEncrypt.staticCall(1), 'HCUQuotaStopped', [removedAddress, DEFAULT_QUOTA]);
      await expectMinedRejection(removed);

      // Reopening the default quota restores access for everyone and keeps the assignments.
      await reopenDefaultQuota();
      await mined(removed.addTrivialValues(1, 2));
      expect(await publicDecrypt(this, await removed.result())).to.eq(3n);
      await mined(stranger.trivialEncrypt(1));
      expect(await hcuLimit.getContractQuota(memberAddress)).to.eq(BigInt(DAPP_QUOTA));

      // Stopping it again enforces the preserved assignments.
      await stopDefaultQuota();
      await mined(member.addTrivialValues(5, 6));
      expect(await publicDecrypt(this, await member.result())).to.eq(11n);
      await expectHcuRevert(removed.trivialEncrypt.staticCall(1), 'HCUQuotaStopped', [removedAddress, DEFAULT_QUOTA]);
      await expectHcuRevert(stranger.trivialEncrypt.staticCall(1), 'HCUQuotaStopped', [strangerAddress, DEFAULT_QUOTA]);
    });

    // A quota is not an exemption from the depth and transaction HCU limits.
    it('should keep a dapp in a quota subject to the HCU limits', async function () {
      const dapp = await deployDapp();
      const dappAddress = await dapp.getAddress();
      await mined(hcuConfigurator.setQuota(DAPP_QUOTA, NO_CAP, NO_CAP));
      await mined(hcuConfigurator.assignQuota(DAPP_QUOTA, dappAddress));
      await stopDefaultQuota();

      // Depth limit: the two trivial encryptions fit, the addition on top of them does not.
      await mined(hcuConfigurator.setMaxHCUDepthPerTx(TIGHT_DEPTH_PER_TX));
      await expectHcuRevert(dapp.addTrivialValues.staticCall(1, 2), 'HCUTransactionDepthLimitExceeded');
      await mined(dapp.trivialEncrypt(1));

      // Transaction limit: an unmetered quota leaves only the transaction limit to refuse a second encryption.
      await mined(hcuConfigurator.setMaxHCUDepthPerTx(ONE_TRIVIAL_ENCRYPTION_HCU));
      await mined(hcuConfigurator.setMaxHCUPerTx(ONE_TRIVIAL_ENCRYPTION_HCU));
      await expectHcuRevert(dapp.trivialEncryptTwice.staticCall(1, 2), 'HCUTransactionLimitExceeded');
      await mined(dapp.trivialEncrypt(1));

      // Quota limit, checked before the transaction limit: the second encryption needs one block of decay.
      await mined(hcuConfigurator.setQuota(DAPP_QUOTA, ONE_TRIVIAL_ENCRYPTION_HCU, ONE_TRIVIAL_ENCRYPTION_HCU));
      await expectHcuRevert(dapp.trivialEncryptTwice.staticCall(1, 2), 'HCUQuotaExceeded', [
        dappAddress,
        DAPP_QUOTA,
        1,
      ]);
      await mined(dapp.trivialEncrypt(1));
    });

    // Only the FHEVMExecutor reports computations, so no account can act on behalf of a dapp.
    it('should only accept HCU checks from the FHEVMExecutor', async function () {
      const dapp = await deployDapp();
      const stranger = await deployDapp();
      await mined(hcuConfigurator.setQuota(DAPP_QUOTA, ROOMY_QUOTA, ROOMY_QUOTA));
      await mined(hcuConfigurator.assignQuota(DAPP_QUOTA, await dapp.getAddress()));
      await stopDefaultQuota();

      // The executor check comes first, whatever the quota of the reported dapp.
      for (const signer of [outsider, owner]) {
        for (const reported of [dapp, stranger]) {
          const args = [FHE_TYPE_UINT64, ethers.ZeroHash, await reported.getAddress()];
          const asSigner = hcuLimitAs(signer);
          await expectHcuRevert(
            asSigner.checkHCUForTrivialEncrypt.staticCall(...args),
            'CallerMustBeFHEVMExecutorContract',
          );
          await expectHcuRevert(asSigner.checkHCUForFheRand.staticCall(...args), 'CallerMustBeFHEVMExecutorContract');
        }
      }
    });
  });

  describe('capped, unmetered and paused quotas', function () {
    // A capped quota counts the HCU of its dapps, an unmetered one does not, and a pause stops either.
    it('should meter a dapp in a capped quota, not one in an unmetered quota, and refuse both while paused', async function () {
      const capped = await deployDapp();
      const unmetered = await deployDapp();
      const cappedAddress = await capped.getAddress();
      const unmeteredAddress = await unmetered.getAddress();
      await mined(hcuConfigurator.setQuota(DAPP_QUOTA, ROOMY_QUOTA, ROOMY_QUOTA));
      await mined(hcuConfigurator.setQuota(UNMETERED_QUOTA, NO_CAP, NO_CAP));
      await mined(hcuConfigurator.assignQuota(DAPP_QUOTA, cappedAddress));
      await mined(hcuConfigurator.assignQuota(UNMETERED_QUOTA, unmeteredAddress));
      await stopDefaultQuota();

      const cappedReceipt = await mined(capped.addTrivialValues(1, 2));
      expect(await quotaUsedAt(DAPP_QUOTA, cappedReceipt.blockNumber)).to.be.greaterThan(0n);
      const unmeteredReceipt = await mined(unmetered.addTrivialValues(1, 2));
      expect(await quotaUsedAt(UNMETERED_QUOTA, unmeteredReceipt.blockNumber)).to.eq(0n);

      for (const [quotaId, dapp, address] of [
        [DAPP_QUOTA, capped, cappedAddress],
        [UNMETERED_QUOTA, unmetered, unmeteredAddress],
      ] as const) {
        await mined(hcuConfigurator.pauseQuota(quotaId));
        await expectHcuRevert(dapp.trivialEncrypt.staticCall(1), 'HCUQuotaPaused', [address, quotaId]);
        await expectMinedRejection(dapp);
        await mined(hcuConfigurator.unpauseQuota(quotaId));
        await mined(dapp.trivialEncrypt(1));
      }
    });
  });

  describe('quota admins', function () {
    // Delegated role: moves between the default quota and its own quota only, attributed to the admin.
    it('should let a quota admin move contracts into and out of its quota and nothing else', async function () {
      const dapp = await deployDapp();
      const dappAddress = await dapp.getAddress();
      const asAdmin = hcuConfiguratorAs(admin);
      await mined(hcuConfigurator.setQuota(ADMIN_QUOTA, ROOMY_QUOTA, ROOMY_QUOTA));

      await expectHcuRevert(asAdmin.grantQuota.staticCall(ADMIN_QUOTA, dappAddress), 'NotQuotaAdmin', [
        ADMIN_QUOTA,
        admin.address,
      ]);
      const naming = await mined(hcuConfigurator.setQuotaAdmin(ADMIN_QUOTA, admin.address));
      assert.deepEqual(eventsOf(naming, 'QuotaAdminSet'), [[BigInt(ADMIN_QUOTA), admin.address]]);

      // An admin adds contracts only while the default quota is stopped.
      await expectHcuRevert(asAdmin.grantQuota.staticCall(ADMIN_QUOTA, dappAddress), 'DefaultQuotaNotStopped');
      await stopDefaultQuota();
      await expectHcuRevert(dapp.trivialEncrypt.staticCall(1), 'HCUQuotaStopped', [dappAddress, DEFAULT_QUOTA]);

      const grant = await mined(asAdmin.grantQuota(ADMIN_QUOTA, dappAddress));
      assert.deepEqual(eventsOf(grant, 'ContractQuotaAssigned'), [[BigInt(ADMIN_QUOTA), dappAddress, admin.address]]);
      await mined(dapp.addTrivialValues(30, 12));
      expect(await publicDecrypt(this, await dapp.result())).to.eq(42n);

      // Everything else stays with the ACL owner, and other quotas with their admins.
      const notOwner = [admin.address];
      await expectHcuRevert(asAdmin.setQuota.staticCall(ADMIN_QUOTA, 1, 1), 'NotHostOwner', notOwner);
      await expectHcuRevert(asAdmin.assignQuota.staticCall(ADMIN_QUOTA, dappAddress), 'NotHostOwner', notOwner);
      await expectHcuRevert(asAdmin.setQuotaAdmin.staticCall(DAPP_QUOTA, admin.address), 'NotHostOwner', notOwner);
      await expectHcuRevert(asAdmin.unpauseQuota.staticCall(ADMIN_QUOTA), 'NotHostOwner', notOwner);
      await expectHcuRevert(asAdmin.pauseQuota.staticCall(ADMIN_QUOTA), 'NotPauser', notOwner);
      await expectHcuRevert(asAdmin.grantQuota.staticCall(DAPP_QUOTA, dappAddress), 'NotQuotaAdmin', [
        DAPP_QUOTA,
        admin.address,
      ]);

      await mined(asAdmin.revokeQuota(ADMIN_QUOTA, dappAddress));
      await expectHcuRevert(dapp.trivialEncrypt.staticCall(1), 'HCUQuotaStopped', [dappAddress, DEFAULT_QUOTA]);
    });

    // While a quota is paused, its admin cannot move its contracts; the owner can.
    it("should freeze a paused quota's members for its admin", async function () {
      const dapp = await deployDapp();
      const dappAddress = await dapp.getAddress();
      const asAdmin = hcuConfiguratorAs(admin);
      await mined(hcuConfigurator.setQuota(ADMIN_QUOTA, ROOMY_QUOTA, ROOMY_QUOTA));
      await mined(hcuConfigurator.setQuotaAdmin(ADMIN_QUOTA, admin.address));
      await stopDefaultQuota();
      await mined(asAdmin.grantQuota(ADMIN_QUOTA, dappAddress));

      await mined(hcuConfigurator.pauseQuota(ADMIN_QUOTA));
      await expectHcuRevert(dapp.trivialEncrypt.staticCall(1), 'HCUQuotaPaused', [dappAddress, ADMIN_QUOTA]);
      await expectHcuRevert(asAdmin.revokeQuota.staticCall(ADMIN_QUOTA, dappAddress), 'QuotaMembershipPaused', [
        ADMIN_QUOTA,
      ]);

      await mined(hcuConfigurator.unpauseQuota(ADMIN_QUOTA));
      await mined(dapp.trivialEncrypt(1));
    });

    // Removing the admin ends its rights at once; the contracts it granted keep their quota.
    it("should end a quota admin's rights on removal and keep the contracts it granted", async function () {
      const first = await deployDapp();
      const second = await deployDapp();
      const firstAddress = await first.getAddress();
      const secondAddress = await second.getAddress();
      const asAdmin = hcuConfiguratorAs(admin);
      await mined(hcuConfigurator.setQuota(ADMIN_QUOTA, ROOMY_QUOTA, ROOMY_QUOTA));
      await mined(hcuConfigurator.setQuotaAdmin(ADMIN_QUOTA, admin.address));
      await stopDefaultQuota();
      await mined(asAdmin.grantQuota(ADMIN_QUOTA, firstAddress));

      const removal = await mined(hcuConfigurator.setQuotaAdmin(ADMIN_QUOTA, ethers.ZeroAddress));
      assert.deepEqual(eventsOf(removal, 'QuotaAdminSet'), [[BigInt(ADMIN_QUOTA), ethers.ZeroAddress]]);
      await expectHcuRevert(asAdmin.grantQuota.staticCall(ADMIN_QUOTA, secondAddress), 'NotQuotaAdmin', [
        ADMIN_QUOTA,
        admin.address,
      ]);
      await expectHcuRevert(asAdmin.revokeQuota.staticCall(ADMIN_QUOTA, firstAddress), 'NotQuotaAdmin', [
        ADMIN_QUOTA,
        admin.address,
      ]);

      await mined(first.addTrivialValues(4, 4));
      expect(await publicDecrypt(this, await first.result())).to.eq(8n);
      await expectHcuRevert(second.trivialEncrypt.staticCall(1), 'HCUQuotaStopped', [secondAddress, DEFAULT_QUOTA]);
    });

    // An account with no role gets the authorization error of each function.
    it('should reserve quota administration to the ACL owner', async function () {
      const dapp = await deployDapp();
      const dappAddress = await dapp.getAddress();
      const asOutsider = hcuConfiguratorAs(outsider);
      const notOwner = [outsider.address];
      await mined(hcuConfigurator.setQuota(ADMIN_QUOTA, ROOMY_QUOTA, ROOMY_QUOTA));
      await mined(hcuConfigurator.setQuotaAdmin(ADMIN_QUOTA, admin.address));

      await expectHcuRevert(asOutsider.setQuota.staticCall(DEFAULT_QUOTA, 0, 0), 'NotHostOwner', notOwner);
      await expectHcuRevert(asOutsider.assignQuota.staticCall(ADMIN_QUOTA, dappAddress), 'NotHostOwner', notOwner);
      await expectHcuRevert(
        asOutsider.setQuotaAdmin.staticCall(ADMIN_QUOTA, outsider.address),
        'NotHostOwner',
        notOwner,
      );
      await expectHcuRevert(asOutsider.unpauseQuota.staticCall(ADMIN_QUOTA), 'NotHostOwner', notOwner);
      await expectHcuRevert(asOutsider.pauseQuota.staticCall(ADMIN_QUOTA), 'NotPauser', notOwner);
      await expectHcuRevert(asOutsider.grantQuota.staticCall(ADMIN_QUOTA, dappAddress), 'NotQuotaAdmin', [
        ADMIN_QUOTA,
        outsider.address,
      ]);
      await expectHcuRevert(asOutsider.revokeQuota.staticCall(ADMIN_QUOTA, dappAddress), 'NotQuotaAdmin', [
        ADMIN_QUOTA,
        outsider.address,
      ]);
    });
  });

  describe('factories and constructors', function () {
    let factory: HCUQuotaFactory;
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
      const contractFactory = await ethers.getContractFactory('HCUQuotaFactory');
      factory = await contractFactory.connect(alice).deploy();
      await factory.waitForDeployment();
      factoryAddress = await factory.getAddress();
      await mined(hcuConfigurator.setQuota(ADMIN_QUOTA, ROOMY_QUOTA, ROOMY_QUOTA));
      await mined(hcuConfigurator.setQuotaAdmin(ADMIN_QUOTA, admin.address));
      await mined(hcuConfigurator.assignQuota(ADMIN_QUOTA, factoryAddress));
      await stopDefaultQuota();
    });

    // Each deployed dapp must be assigned itself.
    it("should not extend a factory's quota to the dapps it deploys", async function () {
      await mined(factory.compute(9));
      expect(await publicDecrypt(this, await factory.result())).to.eq(9n);

      const dappAddress = deployedDapp(await mined(factory.deployDapp()));
      const dapp = (await ethers.getContractAt('HCUQuotaDapp', dappAddress, alice)) as unknown as HCUQuotaDapp;
      expect(await hcuLimit.getContractQuota(dappAddress)).to.eq(BigInt(DEFAULT_QUOTA));
      await expectHcuRevert(dapp.trivialEncrypt.staticCall(1), 'HCUQuotaStopped', [dappAddress, DEFAULT_QUOTA]);

      await mined(hcuConfiguratorAs(admin).grantQuota(ADMIN_QUOTA, dappAddress));
      await mined(dapp.addTrivialValues(8, 9));
      expect(await publicDecrypt(this, await dapp.result())).to.eq(17n);
    });

    // Constructor computation needs the CREATE2 address assigned before deployment.
    it('should deploy a dapp computing in its constructor only once its address is in a quota', async function () {
      const salt = ethers.hexlify(ethers.randomBytes(32));
      const predicted = await factory.predictConstructorDapp(salt, 77);

      await expectHcuRevert(factory.deployConstructorDapp.staticCall(salt, 77), 'HCUQuotaStopped', [
        predicted,
        DEFAULT_QUOTA,
      ]);
      const tx = await factory.deployConstructorDapp(salt, 77, { gasLimit: 5_000_000 });
      expect((await waitForTransactionReceipt(tx.hash)).status).to.eq(0);
      expect(await ethers.provider.getCode(predicted)).to.eq('0x');

      // The predicted address is granted before any code exists at it.
      const grant = await mined(hcuConfiguratorAs(admin).grantQuota(ADMIN_QUOTA, predicted));
      assert.deepEqual(eventsOf(grant, 'ContractQuotaAssigned'), [[BigInt(ADMIN_QUOTA), predicted, admin.address]]);

      expect(deployedDapp(await mined(factory.deployConstructorDapp(salt, 77)))).to.eq(predicted);
      const dapp = await ethers.getContractAt('HCUQuotaConstructorDapp', predicted);
      expect(await publicDecrypt(this, await dapp.value())).to.eq(77n);
    });
  });
});
