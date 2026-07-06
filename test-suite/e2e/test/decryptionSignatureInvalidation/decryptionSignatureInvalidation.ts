import { expect } from 'chai';
import { Contract, ContractTransactionReceipt, EventLog } from 'ethers';
import { ethers } from 'hardhat';

import { ERC1271ApproveHashWallet, UserDecrypt } from '../../types';
import { aclAddress, createInstances, relayerApiKey, relayerUrl, verifyingContractAddressDecryption } from '../instance';
import { Signers, getSigners, initSigners } from '../signers';
import { FhevmInstances } from '../types';
import { waitForBlock } from '../utils';
import type { UnifiedConfig, UnifiedDecryptRequest } from '../sdk/unified/unifiedUserDecrypt';
import {
  computeUnifiedDigest,
  directHandle,
  expectStuckAtKms,
  pollJob,
  requestUnifiedUserDecrypt,
  submitUnifiedRequest,
} from '../sdk/unified/unifiedUserDecrypt';

// Minimal ABI for the decryption-signature-invalidation surface on the host-chain ACL.
const ACL_ABI = [
  'function invalidateDecryptionSignaturesBefore(uint256 timestamp)',
  'function decryptionSignatureInvalidatedBefore(address account) view returns (uint256)',
  'event DecryptionSignaturesInvalidated(address indexed account, uint256 beforeTimestamp)',
];

const DURATION_SECONDS = 7 * 24 * 60 * 60;
const POSITIVE_TIMEOUT_MS = 4 * 60 * 1000;
const TIMEOUT_MARGIN_MS = 60 * 1000;
// Observation window for the stuck-at-KMS negatives. Unlike the unified suite
// this is a fixed floor rather than a runtime 3x-latency calibration: this
// suite's only positive control runs AFTER the negatives, so there's no earlier
// latency sample to calibrate from. 90s is comfortably above the positive
// latencies observed on this stack (KMS round trips ~2-30s), so a request that
// WOULD succeed has ample time to do so — a still-`pending` job at the window
// end is a genuine rejection, not a slow success.
const NEGATIVE_WINDOW_MS = 90 * 1000;
// The KMS Connector reads `decryptionSignatureInvalidatedBefore` from the host
// ACL at a lagging block tag (same reason delegation needs propagation). After
// an invalidation tx, mine/await this many blocks before submitting a request
// whose accept/reject depends on the new threshold being visible.
const PROPAGATION_BLOCKS = 15;

const wallNowSeconds = () => Math.floor(Date.now() / 1000);

/** Await `PROPAGATION_BLOCKS` on top of the current height so a just-written ACL value is visible to the KMS. */
const awaitPropagation = async (): Promise<void> => {
  const current = await ethers.provider.getBlockNumber();
  await waitForBlock(current + PROPAGATION_BLOCKS);
};
const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

const chainNowSeconds = async (): Promise<bigint> => {
  const block = await ethers.provider.getBlock('latest');
  return BigInt(block!.timestamp);
};

/**
 * A `startTimestamp` guaranteed to be strictly BEFORE `threshold` while still
 * (a) in the past for the relayer's wall clock (`validate_timestamp` rejects
 * future timestamps) and (b) within the validity window. `threshold` is a host
 * chain block timestamp, which may run ahead of wall time — take the min.
 */
const startBefore = (threshold: bigint, marginSeconds: number): number =>
  Math.min(Number(threshold), wallNowSeconds()) - marginSeconds;

/** Only an on-chain revert counts — a client-side throw must not false-pass. */
const expectReverts = async (send: () => Promise<{ wait: () => Promise<unknown> }>): Promise<void> => {
  let reverted = false;
  let error: unknown;
  try {
    const tx = await send();
    await tx.wait();
  } catch (e) {
    error = e;
    const code = (e as { code?: string }).code;
    reverted = code === 'CALL_EXCEPTION' || /revert/i.test(String(e));
  }
  expect(reverted, `expected an on-chain revert, got: ${String(error)}`).to.equal(true);
};

/**
 * On-chain decryption-signature invalidation
 * (`ACL.invalidateDecryptionSignaturesBefore`). The monotonic / zero-shorthand /
 * future-cap rules are asserted directly on the ACL; the end-to-end effect (the
 * KMS Connector rejecting a request whose `startTimestamp` predates the
 * invalidation) is asserted through the unified `/v3/user-decrypt` path,
 * including the multisig-rotation scenario that motivates the mechanism.
 *
 * Dedicated accounts (`eve` for EOA invalidation, a fresh wallet contract for
 * the rotation case) are used throughout so the (irreversible, monotonic)
 * invalidation state set here cannot interfere with the other suites. `dave`
 * never invalidates — he is the cross-account isolation control.
 */
describe('Decryption signature invalidation', function () {
  let signers: Signers;
  let instances: FhevmInstances;
  let cfg: UnifiedConfig;
  let publicKey: string;
  let acl: Contract;
  let eveContract: UserDecrypt;
  let eveContractAddress: string;
  let daveContract: UserDecrypt;
  let daveContractAddress: string;

  // True whenever eve's on-chain threshold has been written since the KMS last
  // observed it. Set by every eve invalidation in this suite; cleared by
  // `awaitEvePropagation()`. Lets the KMS-dependent test wait for exactly one
  // propagation regardless of WHICH earlier test wrote the threshold (in suite
  // order it's `invalidate(0)` / the explicit-timestamp test; in a grep-isolated
  // run it's `ensureEveInvalidated` itself). On-chain reads/asserts don't need
  // it — the host ACL reflects the write immediately; only the KMS's lagging
  // read does.
  let eveWriteUnpropagated = false;

  /** Send an eve invalidation tx and flag it as not-yet-visible to the KMS. */
  async function invalidateEve(timestamp: number | bigint): Promise<ContractTransactionReceipt> {
    const receipt = await (await acl.invalidateDecryptionSignaturesBefore(timestamp)).wait();
    eveWriteUnpropagated = true;
    return receipt;
  }

  /** Read eve's current threshold, invalidating once if she never has. */
  async function ensureEveInvalidated(): Promise<bigint> {
    let threshold: bigint = await acl.decryptionSignatureInvalidatedBefore(signers.eve.address);
    if (threshold === 0n) {
      await invalidateEve(0);
      threshold = await acl.decryptionSignatureInvalidatedBefore(signers.eve.address);
    }
    return threshold;
  }

  /** Wait for the KMS's lagging host-ACL read to observe eve's latest threshold. */
  async function awaitEvePropagation(): Promise<void> {
    if (eveWriteUnpropagated) {
      await awaitPropagation();
      eveWriteUnpropagated = false;
    }
  }

  before(async function () {
    this.timeout(180_000);
    await initSigners(5);
    signers = await getSigners();
    instances = await createInstances(signers);
    cfg = {
      relayerUrl,
      decryptionContractAddress: verifyingContractAddressDecryption,
      apiKey: relayerApiKey || undefined,
    };
    // Connected to eve: every direct invalidation tx in this suite is sent as
    // eve, and reads work through eve's provider.
    acl = new ethers.Contract(aclAddress, ACL_ABI, signers.eve);

    const factory = await ethers.getContractFactory('UserDecrypt');
    eveContract = await factory.connect(signers.eve).deploy();
    await eveContract.waitForDeployment();
    eveContractAddress = await eveContract.getAddress();

    daveContract = await factory.connect(signers.dave).deploy();
    await daveContract.waitForDeployment();
    daveContractAddress = await daveContract.getAddress();

    publicKey = (await instances.eve.generateKeypair()).publicKey;
  });

  it('test decryption signature invalidation invalidate(0) resolves to block.timestamp and emits the event', async function () {
    const before = await acl.decryptionSignatureInvalidatedBefore(signers.eve.address);
    const receipt = await invalidateEve(0);
    const block = await ethers.provider.getBlock(receipt.blockNumber);

    const threshold: bigint = await acl.decryptionSignatureInvalidatedBefore(signers.eve.address);
    expect(threshold).to.equal(BigInt(block!.timestamp));
    expect(threshold).to.be.greaterThan(before);

    const events = await acl.queryFilter(
      acl.filters.DecryptionSignaturesInvalidated(signers.eve.address),
      receipt.blockNumber,
      receipt.blockNumber,
    );
    expect(events.length).to.be.greaterThanOrEqual(1);
    expect((events[events.length - 1] as EventLog).args.beforeTimestamp).to.equal(threshold);
  });

  it('test decryption signature invalidation rejects a non-increasing timestamp', async function () {
    const threshold = await ensureEveInvalidated();
    await expectReverts(() => acl.invalidateDecryptionSignaturesBefore(threshold - 100n));
  });

  it('test decryption signature invalidation rejects a future timestamp', async function () {
    // Relative to CHAIN time — the ACL compares against block.timestamp.
    const future = (await chainNowSeconds()) + 100_000n;
    await expectReverts(() => acl.invalidateDecryptionSignaturesBefore(future));
  });

  it('test decryption signature invalidation accepts an explicit non-zero timestamp and stores it exactly', async function () {
    this.timeout(60_000);
    const prior = await ensureEveInvalidated();
    // Pick a target satisfying prior < target <= block.timestamp-at-execution:
    // the latest block timestamp, once it has advanced past `prior`.
    let target = await chainNowSeconds();
    const deadline = Date.now() + 30_000;
    while (target <= prior && Date.now() < deadline) {
      await sleep(1_000);
      target = await chainNowSeconds();
    }
    expect(target > prior, 'chain time did not advance past the current threshold').to.equal(true);

    await invalidateEve(target);
    expect(await acl.decryptionSignatureInvalidatedBefore(signers.eve.address)).to.equal(target);
  });

  it('test decryption signature invalidation rejects a request signed before the invalidation timestamp', async function () {
    this.timeout(NEGATIVE_WINDOW_MS + TIMEOUT_MARGIN_MS);
    const threshold = await ensureEveInvalidated();
    // Ensure the KMS observes eve's threshold before this KMS-dependent request,
    // whichever earlier test wrote it (or this test, in a grep-isolated run).
    await awaitEvePropagation();
    const handle = await eveContract.xUint64();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, eveContractAddress, signers.eve.address)],
      userAddress: signers.eve.address,
      allowedContracts: [],
      publicKey,
      // Strictly before the threshold AND in the past for the relayer's clock.
      startTimestamp: startBefore(threshold, 3600),
      durationSeconds: DURATION_SECONDS,
    };
    const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.eve }, {
      waitForTerminal: true,
      timeoutMs: NEGATIVE_WINDOW_MS,
    });
    // Signature is valid, so the relayer accepts; the invalidation check is
    // enforced only by the KMS Connector (startTimestamp < invalidatedBefore),
    // which rejects without responding — the job stays queued.
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expectStuckAtKms(poll);
  });

  it('test decryption signature invalidation accepts a request signed at/after the invalidation timestamp', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    // Use a fresh account (carol) so the threshold is set explicitly here, not
    // polluted by eve's monotonic history. Pick T safely in the PAST for BOTH
    // the chain (the ACL caps invalidation at block.timestamp) and the KMS wall
    // clock (validity requires now >= startTimestamp) — so the ONLY thing under
    // test is the `startTimestamp >= threshold` boundary.
    const carolFactory = await ethers.getContractFactory('UserDecrypt');
    const carolContract = await carolFactory.connect(signers.carol).deploy();
    await carolContract.waitForDeployment();
    const carolContractAddress = await carolContract.getAddress();

    const T = Math.min(Number(await chainNowSeconds()), wallNowSeconds()) - 120;
    const aclCarol = new ethers.Contract(aclAddress, ACL_ABI, signers.carol);
    await (await aclCarol.invalidateDecryptionSignaturesBefore(T)).wait();
    expect(await aclCarol.decryptionSignatureInvalidatedBefore(signers.carol.address)).to.equal(BigInt(T));
    // Let the KMS's lagging host-ACL read observe the new threshold, so start==T
    // genuinely exercises the boundary (rather than being trivially accepted
    // against a still-zero threshold).
    await awaitPropagation();

    const handle = await carolContract.xUint64();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, carolContractAddress, signers.carol.address)],
      userAddress: signers.carol.address,
      allowedContracts: [],
      publicKey,
      startTimestamp: T, // exactly AT the threshold — pins the >= boundary
      durationSeconds: DURATION_SECONDS,
    };
    const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.carol }, {
      waitForTerminal: true,
      timeoutMs: POSITIVE_TIMEOUT_MS,
    });
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
    // Decrypt the same handle through the public SDK and assert the known plaintext.
    const clear = await instances.carol.userDecryptSingleHandle({
      handle,
      contractAddress: carolContractAddress,
      signer: signers.carol,
    });
    expect(clear).to.equal(18446744073709551600n);
  });

  it('test decryption signature invalidation for one account leaves other accounts unaffected', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    // dave never invalidated: a request whose startTimestamp predates EVE's
    // threshold must still succeed for dave — the mapping is per msg.sender.
    const threshold = await ensureEveInvalidated();
    const handle = await daveContract.xUint64();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, daveContractAddress, signers.dave.address)],
      userAddress: signers.dave.address,
      allowedContracts: [],
      publicKey,
      startTimestamp: startBefore(threshold, 3600),
      durationSeconds: DURATION_SECONDS,
    };
    const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.dave }, {
      waitForTerminal: true,
      timeoutMs: POSITIVE_TIMEOUT_MS,
    });
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
    // Decrypt the same handle through the public SDK and assert the known plaintext.
    const clear = await instances.dave.userDecryptSingleHandle({
      handle,
      contractAddress: daveContractAddress,
      signer: signers.dave,
    });
    expect(clear).to.equal(18446744073709551600n);
  });

  it('test decryption signature invalidation kills pre-approved smart-account requests after signer rotation', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + NEGATIVE_WINDOW_MS + 3 * TIMEOUT_MARGIN_MS);
    // The motivating scenario for invalidation: a Safe-style wallet pre-approves a
    // decryption request; after a signer rotation the pre-approval is STILL
    // ERC-1271-valid (approveHash survives rotation), so the wallet must call
    // invalidateDecryptionSignaturesBefore(0) — as the wallet, since the
    // mapping is keyed by msg.sender — to kill every pre-rotation approval.
    const walletFactory = await ethers.getContractFactory('ERC1271ApproveHashWallet');
    const wallet: ERC1271ApproveHashWallet = await walletFactory.connect(signers.alice).deploy(signers.bob.address);
    await wallet.waitForDeployment();
    const walletAddress = await wallet.getAddress();
    await (await wallet.connect(signers.alice).initValue(42n)).wait();
    const handle = await wallet.value();

    // Both requests are dated before the (upcoming) rotation. They MUST differ
    // in a field covered by the relayer's dedup key — which excludes
    // `startTimestamp` and `signature` (see relayer `content_hash`) — otherwise
    // reqB would be deduplicated onto reqA's cached success and never re-checked
    // against the invalidation. A distinct `publicKey` gives them distinct dedup
    // keys (and distinct EIP-712 digests to pre-approve).
    const t0 = startBefore(await chainNowSeconds(), 60);
    const publicKeyB = (await instances.bob.generateKeypair()).publicKey;
    const reqA: UnifiedDecryptRequest = {
      handles: [directHandle(handle, walletAddress, walletAddress)],
      userAddress: walletAddress,
      allowedContracts: [],
      publicKey,
      startTimestamp: t0,
      durationSeconds: DURATION_SECONDS,
    };
    const reqB: UnifiedDecryptRequest = { ...reqA, publicKey: publicKeyB };
    await (await wallet.connect(signers.bob).approveHash(computeUnifiedDigest(cfg, reqA))).wait();
    await (await wallet.connect(signers.bob).approveHash(computeUnifiedDigest(cfg, reqB))).wait();

    // Control: a pre-approved empty-signature request works before rotation.
    const { post: postA } = await submitUnifiedRequest(cfg, reqA, { kind: 'empty' });
    expect(postA.httpStatus, JSON.stringify(postA.raw)).to.equal(202);
    expect(postA.jobId, JSON.stringify(postA.raw)).to.be.a('string');
    const pollA = await pollJob(cfg, postA.jobId!, { timeoutMs: POSITIVE_TIMEOUT_MS });
    expect(pollA.status, JSON.stringify(pollA.raw)).to.equal('succeeded');

    // "Rotation": the wallet invalidates all its pre-rotation signatures.
    await (await wallet.connect(signers.bob).invalidateDecryptionSignatures(aclAddress)).wait();
    // Let the KMS's lagging host-ACL read observe the wallet's new threshold
    // before we submit reqB (otherwise the read still returns 0 and reqB slips
    // through — the exact race this scenario is meant to catch).
    await awaitPropagation();

    // The second pre-approval is STILL accepted by the signature pre-check
    // (approveHash is untouched), but the KMS invalidation check rejects it:
    // startTimestamp t0 < the wallet's new threshold.
    const { post: postB } = await submitUnifiedRequest(cfg, reqB, { kind: 'empty' });
    expect(postB.httpStatus, JSON.stringify(postB.raw)).to.equal(202);
    // Guard against a malformed 202-without-jobId making the stuck-at-KMS
    // assertion below pass vacuously (pollJob on an absent id never succeeds).
    expect(postB.jobId, JSON.stringify(postB.raw)).to.be.a('string');
    const pollB = await pollJob(cfg, postB.jobId!, { timeoutMs: NEGATIVE_WINDOW_MS });
    expectStuckAtKms(pollB);
  });
});
