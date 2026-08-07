import { expect } from 'chai';
import type { Signer } from 'ethers';
import { ethers } from 'hardhat';

import {
  ERC1271ApproveHashWallet,
  ERC1271OwnerWallet,
  ERC1271RejectWallet,
  EncryptedValueHolder,
  UserDecrypt,
} from '../../types';
import { createInstances, relayerApiKey, relayerUrl, verifyingContractAddressDecryption } from '../instance';
import { isLiveNetwork } from '../network';
import type { UnifiedConfig, UnifiedDecryptRequest } from '../sdk/unified/unifiedUserDecrypt';
import {
  backdatedStartTimestamp,
  chainIdFromHandle,
  computeUnifiedDigest,
  concatSignatureParts,
  directHandle,
  isSignatureRejection,
  pollJob,
  requestUnifiedUserDecrypt,
  sortSignatureParts,
  submitUnifiedRequest,
} from '../sdk/unified/unifiedUserDecrypt';
import { Signers, getSigners, initSigners } from '../signers';
import { FhevmInstances } from '../types';
import {
  SafeAccount,
  approveSafeHash,
  buildSafeMultisigSignature,
  buildSafeNestedMultisigSignature,
  collectSafeOwnerParts,
  deploySafeAccount,
  resolveSafeInfra,
  safeApprovedHashPart,
  safeEthSignPart,
} from './safe';

// Trivially-encrypted value each wallet (or its holder contract) stores; the
// exact plaintext is irrelevant — a `succeeded` job proves the ERC-1271
// signature was accepted and the KMS produced re-encrypted shares.
const KNOWN_VALUE = 123456789n;
const DURATION_SECONDS = 7 * 24 * 60 * 60;
// Generous window for a full user-decrypt round trip through the KMS.
const POSITIVE_TIMEOUT_MS = 3 * 60 * 1000;

/**
 * On a local devnet a deployment lands in the next instant block; on a live
 * network every fixture costs real block time (~12s on Sepolia) and real gas.
 * Fixtures are provisioned LAZILY (see `lazy` below), so this budget is the
 * per-test allowance for whatever that test's first use has to deploy, not a
 * whole-suite up-front cost.
 */
const LIVE_NETWORK = isLiveNetwork();
const FIXTURE_BUDGET_MS = LIVE_NETWORK ? 5 * 60 * 1000 : 30 * 1000;
// Mocha timeout margin on top of the poll window (pre-poll on-chain reads + POST).
const TIMEOUT_MARGIN_MS = 60 * 1000 + FIXTURE_BUDGET_MS;

/** Deploy-on-first-use, then reuse: a `--grep`ped run only pays for the fixtures it touches. */
const lazy = <T>(create: () => Promise<T>): (() => Promise<T>) => {
  let pending: Promise<T> | undefined;
  return () => (pending ??= create());
};

/**
 * ERC-1271 support for smart-account signature verification.
 *
 * These exercise the relayer's synchronous signature pre-check (which runs the
 * shared `verify_signature`: `ecrecover` -> ERC-1271 `isValidSignature`
 * fallback) via the unified `/v3/user-decrypt` endpoint. A definitively-bad
 * signature is rejected synchronously (`400`); a valid one is accepted (`202`)
 * and — for the positive cases — driven to a `succeeded` job. The smart-account
 * positives cannot additionally assert the plaintext through the public SDK
 * (it signs as the connected signer and cannot act as a wallet userAddress);
 * the EOA fast-path positive does assert it.
 */
describe('ERC-1271 user decryption', function () {
  // Every test can be the one that provisions its fixture, so the suite-wide
  // default has to cover a deployment; positives raise it further below.
  this.timeout(TIMEOUT_MARGIN_MS);

  let signers: Signers;
  let instances: FhevmInstances;
  let cfg: UnifiedConfig;
  let publicKey: string;

  let userDecrypt: UserDecrypt;
  let userDecryptAddress: string;
  let ownerWallet: ERC1271OwnerWallet;
  let ownerWalletAddress: string;
  let approveWallet: ERC1271ApproveHashWallet;
  let approveWalletAddress: string;
  let rejectWallet: ERC1271RejectWallet;
  let rejectWalletAddress: string;
  // Safe fixtures are lazy thunks, not values: nothing below is deployed until
  // a test awaits it, and each deploys at most once per run.
  let safe2of3: () => Promise<SafeFixture>;
  let safe3of3: () => Promise<SafeFixture>;
  let safe1of1: () => Promise<SafeFixture>;
  let safeNoHandler: () => Promise<SafeFixture>;
  // Nested v=0 contract-signature fixtures: outer 2-of-3 Safes whose first
  // owner is itself a Safe (1-of-1 dave, and 2-of-3 bob/dave/eve).
  let innerSafe1of1: () => Promise<SafeAccount>;
  let innerSafe2of3: () => Promise<SafeAccount>;
  let nestedOuter1of1: () => Promise<SafeFixture>;
  let nestedOuter2of3: () => Promise<SafeFixture>;

  /**
   * A real Safe holds no encrypted state (and has no FHE coprocessor config),
   * so each Safe is paired with an `EncryptedValueHolder` carrying the handle
   * with `FHE.allow(value, safe)` — the realistic shape where the handle lives
   * on a dapp contract and the Safe is only the `userAddress`.
   */
  interface SafeFixture {
    readonly safe: SafeAccount;
    readonly holder: EncryptedValueHolder;
    readonly holderAddress: string;
  }

  before(async function () {
    // Only the four mock wallets deploy here now; the Safe matrix is lazy.
    this.timeout(LIVE_NETWORK ? 10 * 60 * 1000 : 180_000);
    // 5, not 3: the multisig tests use dave (owner) and eve (non-owner), and
    // sibling suites that touch dave/eve all pass 5. The count only limits
    // funding under HARDHAT_PARALLEL (signers.ts funds all 5 otherwise), and
    // `initSigners` funds only on its FIRST call per process — matching 5
    // keeps combined parallel runs safe whichever suite's before() runs first.
    await initSigners(5);
    signers = await getSigners();
    instances = await createInstances(signers);
    cfg = {
      relayerUrl,
      decryptionContractAddress: verifyingContractAddressDecryption,
      apiKey: relayerApiKey || undefined,
    };

    // A normal dapp contract with an alice-owned handle (for the EOA fast path).
    const userDecryptFactory = await ethers.getContractFactory('UserDecrypt');
    userDecrypt = await userDecryptFactory.connect(signers.alice).deploy();
    await userDecrypt.waitForDeployment();
    userDecryptAddress = await userDecrypt.getAddress();

    // ERC-1271 owner wallet: validates bob's ECDSA signature.
    const ownerWalletFactory = await ethers.getContractFactory('ERC1271OwnerWallet');
    ownerWallet = await ownerWalletFactory.connect(signers.alice).deploy(signers.bob.address);
    await ownerWallet.waitForDeployment();
    ownerWalletAddress = await ownerWallet.getAddress();
    await (await ownerWallet.connect(signers.alice).initValue(KNOWN_VALUE)).wait();

    // ERC-1271 approveHash wallet: validates an empty signature iff the digest is approved.
    const approveWalletFactory = await ethers.getContractFactory('ERC1271ApproveHashWallet');
    approveWallet = await approveWalletFactory.connect(signers.alice).deploy(signers.bob.address);
    await approveWallet.waitForDeployment();
    approveWalletAddress = await approveWallet.getAddress();
    await (await approveWallet.connect(signers.alice).initValue(KNOWN_VALUE)).wait();

    // ERC-1271 wallet that rejects every signature.
    const rejectWalletFactory = await ethers.getContractFactory('ERC1271RejectWallet');
    rejectWallet = await rejectWalletFactory.connect(signers.alice).deploy();
    await rejectWallet.waitForDeployment();
    rejectWalletAddress = await rejectWallet.getAddress();
    await (await rejectWallet.connect(signers.alice).initValue(KNOWN_VALUE)).wait();

    // Real Safe v1.4.1 multisig wallets, from the canonical prebuilt artifacts
    // (singleton + fallback handler + proxy factory, then one proxy per
    // configuration). Owners bob/carol/dave only ever sign SafeMessage typed
    // data offline — they pay no gas (except the approveHash positive, where
    // bob sends the approval tx).
    //
    // Everything here is wired as a lazy thunk: the full matrix is ~23
    // transactions, which is free against instant local blocks but minutes of
    // real block time on a live network. Deferring means a run only pays for
    // the fixtures its selected tests actually reach.
    const multisigOwners = [signers.bob.address, signers.carol.address, signers.dave.address];
    const infra = lazy(() => resolveSafeInfra(signers.alice));
    const deploySafeFixture = async (
      owners: readonly string[],
      threshold: number,
      opts?: { readonly fallbackHandler?: string },
    ): Promise<SafeFixture> => {
      const safe = await deploySafeAccount(await infra(), signers.alice, owners, threshold, opts);
      const holderFactory = await ethers.getContractFactory('EncryptedValueHolder');
      const holder = await holderFactory.connect(signers.alice).deploy();
      await holder.waitForDeployment();
      const holderAddress = await holder.getAddress();
      await (await holder.connect(signers.alice).initValueFor(KNOWN_VALUE, safe.address)).wait();
      return { safe, holder, holderAddress };
    };
    safe2of3 = lazy(() => deploySafeFixture(multisigOwners, 2));
    safe3of3 = lazy(() => deploySafeFixture(multisigOwners, 3));
    safe1of1 = lazy(() => deploySafeFixture([signers.dave.address], 1));
    // A Safe set up WITHOUT the fallback handler: it has code, but no
    // `isValidSignature` to dispatch to (see ./safe.ts).
    safeNoHandler = lazy(() => deploySafeFixture(multisigOwners, 2, { fallbackHandler: ethers.ZeroAddress }));

    // Inner Safes acting as CONTRACT owners of outer Safes (v=0 parts).
    innerSafe1of1 = lazy(async () => deploySafeAccount(await infra(), signers.alice, [signers.dave.address], 1));
    innerSafe2of3 = lazy(async () =>
      deploySafeAccount(
        await infra(),
        signers.alice,
        [signers.bob.address, signers.dave.address, signers.eve.address],
        2,
      ),
    );
    nestedOuter1of1 = lazy(async () =>
      deploySafeFixture([(await innerSafe1of1()).address, signers.bob.address, signers.carol.address], 2),
    );
    nestedOuter2of3 = lazy(async () =>
      deploySafeFixture([(await innerSafe2of3()).address, signers.bob.address, signers.carol.address], 2),
    );

    publicKey = (await instances.alice.generateKeypair()).publicKey;
  });

  it('test erc1271 user decrypt EOA fast path (ecrecover match) through the unified route', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const handle = await userDecrypt.xUint64();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, userDecryptAddress, signers.alice.address)],
      userAddress: signers.alice.address,
      allowedContracts: [],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    const { post, poll } = await requestUnifiedUserDecrypt(
      cfg,
      req,
      { kind: 'eoa', signer: signers.alice },
      {
        waitForTerminal: true,
        timeoutMs: POSITIVE_TIMEOUT_MS,
      },
    );
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
    // Decrypt the same handle through the public SDK and assert the known plaintext.
    const clear = await instances.alice.userDecryptSingleHandle({
      handle,
      contractAddress: userDecryptAddress,
      signer: signers.alice,
    });
    expect(clear).to.equal(18446744073709551600n);
  });

  it('test erc1271 user decrypt smart account (owner ECDSA signature) succeeds', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const handle = await ownerWallet.value();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, ownerWalletAddress, ownerWalletAddress)],
      userAddress: ownerWalletAddress,
      allowedContracts: [],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    // bob is the wallet owner; he signs, but userAddress is the wallet contract.
    const { post, poll } = await requestUnifiedUserDecrypt(
      cfg,
      req,
      { kind: 'erc1271', ownerSigner: signers.bob },
      {
        waitForTerminal: true,
        timeoutMs: POSITIVE_TIMEOUT_MS,
      },
    );
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
  });

  it('test erc1271 user decrypt smart account (approveHash empty signature) succeeds', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const handle = await approveWallet.value();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, approveWalletAddress, approveWalletAddress)],
      userAddress: approveWalletAddress,
      allowedContracts: [],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    // Pre-approve the exact EIP-712 digest on-chain, then submit with an empty signature.
    const digest = computeUnifiedDigest(cfg, req);
    await (await approveWallet.connect(signers.bob).approveHash(digest)).wait();

    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'empty' });
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    const poll = await pollJob(cfg, post.jobId!, { timeoutMs: POSITIVE_TIMEOUT_MS });
    expect(poll.status, JSON.stringify(poll.raw)).to.equal('succeeded');
  });

  it('test erc1271 user decrypt rejects a non-owner ECDSA signature', async function () {
    const handle = await ownerWallet.value();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, ownerWalletAddress, ownerWalletAddress)],
      userAddress: ownerWalletAddress,
      allowedContracts: [],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    // carol is NOT the wallet owner -> isValidSignature returns a non-magic value.
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'erc1271', ownerSigner: signers.carol });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt rejects an approveHash wallet when the digest was not approved', async function () {
    const handle = await approveWallet.value();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, approveWalletAddress, approveWalletAddress)],
      userAddress: approveWalletAddress,
      allowedContracts: [],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    // No approveHash call -> empty signature is invalid.
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'empty' });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  // The ERC-1271 verification has three rejection branches for a contract
  // userAddress: wrong return value, revert, and returndata shorter than 32
  // bytes. Each gets its own mode on the reject wallet.

  it('test erc1271 user decrypt rejects a wallet that returns the wrong magic value', async function () {
    await (await rejectWallet.setMode(0)).wait(); // RejectMode.WrongMagic
    const handle = await rejectWallet.value();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, rejectWalletAddress, rejectWalletAddress)],
      userAddress: rejectWalletAddress,
      allowedContracts: [],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'erc1271', ownerSigner: signers.bob });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt rejects a wallet whose isValidSignature reverts', async function () {
    await (await rejectWallet.setMode(1)).wait(); // RejectMode.Revert
    const handle = await rejectWallet.value();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, rejectWalletAddress, rejectWalletAddress)],
      userAddress: rejectWalletAddress,
      allowedContracts: [],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    // A revert inside isValidSignature is a definitive rejection, not a transport error.
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'erc1271', ownerSigner: signers.bob });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt rejects a wallet returning short returndata (non-compliant fallback)', async function () {
    await (await rejectWallet.setMode(2)).wait(); // RejectMode.ShortReturndata
    const handle = await rejectWallet.value();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, rejectWalletAddress, rejectWalletAddress)],
      userAddress: rejectWalletAddress,
      allowedContracts: [],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    // `bytes4` is ABI-encoded as a full 32-byte word; returndata < 32 bytes
    // (e.g. a proxy fallback) must be rejected before magic-value comparison.
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'erc1271', ownerSigner: signers.bob });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt rejects a contract userAddress with no code', async function () {
    const handle = await userDecrypt.xUint64();
    const noCodeAddress = ethers.Wallet.createRandom().address;
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, userDecryptAddress, noCodeAddress)],
      userAddress: noCodeAddress,
      allowedContracts: [],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    // alice signs, but userAddress is a random no-code address -> ecrecover mismatch,
    // no contract to fall back to -> rejected.
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'erc1271', ownerSigner: signers.alice });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt rejects a Safe deployed without its fallback handler', async function () {
    // The same "empty returndata" verifier branch as the no-code case above,
    // reached by an account that DOES have code: a Safe set up with
    // `fallbackHandler = address(0)` has no `isValidSignature` to dispatch to,
    // and Safe's FallbackManager answers `return(0, 0)` — SUCCESS with zero
    // bytes, not a revert. The signatures themselves are perfectly valid for
    // the Safe's owners, so this isolates the missing-handler misconfiguration.
    const req = await freshMultisigRequest(safeNoHandler);
    const signature = await multisigSignature(safeNoHandler, req, [signers.bob, signers.carol]);
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt rejects an empty signature for an EOA userAddress', async function () {
    const handle = await userDecrypt.xUint64();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, userDecryptAddress, signers.alice.address)],
      userAddress: signers.alice.address,
      allowedContracts: [],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    // Empty signature is only valid for a contract (ERC-1271); an EOA must be rejected.
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'empty' });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  // Multisig (Safe static encoding): the signature is a concatenation of
  // 65-byte {r,s,v} owner parts sorted strictly ascending by signer address.
  // The blob is longer than a single ECDSA signature, so `ecrecover` on it is
  // impossible — every layer must forward it opaquely to the Safe's
  // `isValidSignature`. Unlike the retired mock, a real Safe REVERTS on a bad
  // blob (GS020/GS025/GS026) instead of returning a non-magic value; either
  // way the relayer's pre-check rejects synchronously (400), and the KMS
  // Connector runs the same shared verifier again before the KMS produces
  // shares. Owners sign the SafeMessage EIP-712 wrap of the unified digest,
  // never the digest itself (see ./safe.ts).

  /**
   * A request for the Safe fixture's held handle with a FRESH re-encryption
   * key: the relayer dedups accepted jobs on a content hash that EXCLUDES the
   * signature, so a second positive differing only in its multisig blob would
   * collapse onto the first job and pass vacuously. (Definitively-bad
   * signatures are 400-rejected by the pre-check before dedup is consulted —
   * the fresh key just keeps every request, negative included, independent.)
   * The handle lives on the fixture's holder contract; the Safe is only the
   * `userAddress` (a real Safe cannot hold encrypted state itself).
   */
  async function freshMultisigRequest(provision: () => Promise<SafeFixture>): Promise<UnifiedDecryptRequest> {
    const fixture = await provision();
    const handle = await fixture.holder.value();
    const freshKey = (await instances.alice.generateKeypair()).publicKey;
    return {
      handles: [directHandle(handle, fixture.holderAddress, fixture.safe.address)],
      userAddress: fixture.safe.address,
      allowedContracts: [],
      publicKey: freshKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
  }

  /** Safe-multisig blob over the unified EIP-712 digest of `req`. */
  async function multisigSignature(
    provision: () => Promise<SafeFixture>,
    req: UnifiedDecryptRequest,
    owners: readonly Signer[],
    opts?: { readonly order?: 'ascending' | 'descending'; readonly trailingHex?: string },
  ): Promise<string> {
    const fixture = await provision();
    return buildSafeMultisigSignature(fixture.safe, computeUnifiedDigest(cfg, req), owners, opts);
  }

  it('test erc1271 user decrypt multisig 2-of-3 concatenated owner signatures succeed', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const req = await freshMultisigRequest(safe2of3);
    const signature = await multisigSignature(safe2of3, req, [signers.bob, signers.carol]);
    // Two 65-byte parts: the whole point is a >65-byte opaque blob end to end.
    expect(signature.length).to.equal(2 + 130 * 2);
    const { post, poll } = await requestUnifiedUserDecrypt(
      cfg,
      req,
      { kind: 'raw', signature },
      {
        waitForTerminal: true,
        timeoutMs: POSITIVE_TIMEOUT_MS,
      },
    );
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
  });

  it('test erc1271 user decrypt Safe 1-of-1 single owner signature succeeds', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const req = await freshMultisigRequest(safe1of1);
    const signature = await multisigSignature(safe1of1, req, [signers.dave]);
    // Exactly 65 bytes — the same length as a plain EOA signature, so every
    // layer runs `ecrecover` FIRST and recovers a pseudo-random address (dave
    // signed the SafeMessage wrap, not the digest). The mismatch against the
    // Safe userAddress must fall through to ERC-1271 rather than reject: this
    // is the 65-byte contract-wallet path the >65-byte blobs never exercise.
    expect(signature.length).to.equal(2 + 65 * 2);
    const { post, poll } = await requestUnifiedUserDecrypt(
      cfg,
      req,
      { kind: 'raw', signature },
      { waitForTerminal: true, timeoutMs: POSITIVE_TIMEOUT_MS },
    );
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
  });

  it('test erc1271 user decrypt multisig 3-of-3 concatenated owner signatures succeed', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const req = await freshMultisigRequest(safe3of3);
    const signature = await multisigSignature(safe3of3, req, [signers.bob, signers.carol, signers.dave]);
    // Three parts (195 bytes) through relayer -> gateway calldata -> event -> connector.
    expect(signature.length).to.equal(2 + 195 * 2);
    const { post, poll } = await requestUnifiedUserDecrypt(
      cfg,
      req,
      { kind: 'raw', signature },
      {
        waitForTerminal: true,
        timeoutMs: POSITIVE_TIMEOUT_MS,
      },
    );
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
  });

  it('test erc1271 user decrypt multisig rejects a blob below threshold (1 of 3 parts)', async function () {
    const req = await freshMultisigRequest(safe2of3);
    // A single owner part is exactly 65 bytes: `ecrecover` parses it but
    // recovers a pseudo-random address (bob signed the SafeMessage wrap, not
    // the digest), never the Safe, so verification falls through to
    // ERC-1271 — where one part is below the threshold of two (GS020).
    const signature = await multisigSignature(safe2of3, req, [signers.bob]);
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt multisig rejects a blob containing a non-owner signature', async function () {
    const req = await freshMultisigRequest(safe2of3);
    // eve is not an owner; her part is well-formed but recovers a non-owner (GS026).
    const signature = await multisigSignature(safe2of3, req, [signers.bob, signers.eve]);
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt multisig rejects a duplicated owner signature (threshold inflation)', async function () {
    const req = await freshMultisigRequest(safe2of3);
    // Two copies of bob's part: the strictly-ascending signer rule (GS026) is
    // what stops one owner from inflating the approval count to the threshold.
    const signature = await multisigSignature(safe2of3, req, [signers.bob, signers.bob]);
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt multisig rejects parts in descending signer order', async function () {
    const req = await freshMultisigRequest(safe2of3);
    // Valid owner parts in descending order: Safe's canonical encoding
    // requires ascending signer addresses (GS026).
    const signature = await multisigSignature(safe2of3, req, [signers.bob, signers.carol], { order: 'descending' });
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt multisig rejects a garbage blob below the threshold minimum length', async function () {
    const req = await freshMultisigRequest(safe2of3);
    // 100 bytes of junk: neither a valid ECDSA signature nor enough bytes for
    // two 65-byte parts — every layer must hand it to the Safe without
    // choking, and the Safe reverts on its length rule (GS020). A clean
    // revert is an equally definitive rejection at every verifying layer.
    const signature = `0x${'11'.repeat(100)}`;
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  // Safe overloads the `v` byte of each 65-byte part as a type selector:
  // 27/28 plain ECDSA, >30 eth_sign, 1 pre-approved hash, and 0 for a
  // CONTRACT signature with a dynamic tail (a nested Safe owner, verified
  // through a second full ERC-1271 round trip) — all four exercised below
  // against the real implementation.

  it('test erc1271 user decrypt multisig accepts a blob with trailing bytes (length not a multiple of 65)', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const req = await freshMultisigRequest(safe2of3);
    // Two valid parts + 35 junk bytes = 165 bytes, NOT a multiple of 65:
    // every layer must forward the unusual length untouched, and Safe ignores
    // anything past the static threshold*65 section — the region where it
    // reads dynamic data for contract-signature parts.
    const signature = await multisigSignature(safe2of3, req, [signers.bob, signers.carol], {
      trailingHex: '11'.repeat(35),
    });
    expect(signature.length).to.equal(2 + 165 * 2);
    const { post, poll } = await requestUnifiedUserDecrypt(
      cfg,
      req,
      { kind: 'raw', signature },
      { waitForTerminal: true, timeoutMs: POSITIVE_TIMEOUT_MS },
    );
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
  });

  it('test erc1271 user decrypt multisig accepts a mixed blob (ECDSA part + pre-approved-hash part)', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const req = await freshMultisigRequest(safe2of3);
    const digest = computeUnifiedDigest(cfg, req);
    // bob pre-approves on-chain (Safe's approveHash flow — the approval
    // targets the SafeMessage hash of the digest, not the digest itself);
    // carol signs normally. The blob mixes a v=1 part (r = bob's address)
    // with a plain ECDSA part — a realistic Safe part-type combination.
    const fixture = await safe2of3();
    await approveSafeHash(fixture.safe, signers.bob, digest);
    const [carolPart] = await collectSafeOwnerParts(fixture.safe, digest, [signers.carol]);
    const signature = concatSignatureParts(sortSignatureParts([safeApprovedHashPart(signers.bob.address), carolPart]));
    const { post, poll } = await requestUnifiedUserDecrypt(
      cfg,
      req,
      { kind: 'raw', signature },
      { waitForTerminal: true, timeoutMs: POSITIVE_TIMEOUT_MS },
    );
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
  });

  it('test erc1271 user decrypt multisig rejects a pre-approved-hash part that was never approved', async function () {
    const req = await freshMultisigRequest(safe2of3);
    const digest = computeUnifiedDigest(cfg, req);
    // The mixed blob above, minus the on-chain approval: bob's v=1 part claims
    // an approval that `approvedHashes[bob][safeMessageHash]` does not carry,
    // so Safe reverts GS025. (The mock-wallet negative keys approvals off the
    // RAW digest; only a real Safe pins the SafeMessage-hash rule.) Note the
    // v=1 part carries no signature at all — nothing here is forgeable, which
    // is exactly why the on-chain approval must be the thing that gates it.
    const [carolPart] = await collectSafeOwnerParts((await safe2of3()).safe, digest, [signers.carol]);
    const signature = concatSignatureParts(sortSignatureParts([safeApprovedHashPart(signers.bob.address), carolPart]));
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt multisig accepts eth_sign parts (v shifted by 4)', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const req = await freshMultisigRequest(safe2of3);
    const digest = computeUnifiedDigest(cfg, req);
    // Safe's eth_sign encoding: owners personal_sign the SafeMessage hash of
    // the digest and the part stores v+4 (31/32) as the type selector.
    const parts = sortSignatureParts([
      await safeEthSignPart((await safe2of3()).safe, digest, signers.bob),
      await safeEthSignPart((await safe2of3()).safe, digest, signers.carol),
    ]);
    const signature = concatSignatureParts(parts);
    const { post, poll } = await requestUnifiedUserDecrypt(
      cfg,
      req,
      { kind: 'raw', signature },
      { waitForTerminal: true, timeoutMs: POSITIVE_TIMEOUT_MS },
    );
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
  });

  it('test erc1271 user decrypt multisig accepts a v=0 contract-signature part (nested Safe owner)', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const req = await freshMultisigRequest(nestedOuter1of1);
    const digest = computeUnifiedDigest(cfg, req);
    // The outer 2-of-3's approvals: a v=0 part from the inner 1-of-1 Safe
    // (dave signs the inner SafeMessage wrap of the outer preimage) + bob's
    // plain ECDSA part. 227 bytes: 130 static + 32-byte length + 65 inner.
    // Measured ~83k gas incl. intrinsic vs the 100k erc1271_gas_limit.
    const signature = await buildSafeNestedMultisigSignature(
      (await nestedOuter1of1()).safe,
      digest,
      { safe: await innerSafe1of1(), owners: [signers.dave] },
      [signers.bob],
    );
    expect(signature.length).to.equal(2 + 227 * 2);
    const { post, poll } = await requestUnifiedUserDecrypt(
      cfg,
      req,
      { kind: 'raw', signature },
      { waitForTerminal: true, timeoutMs: POSITIVE_TIMEOUT_MS },
    );
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
  });

  it('test erc1271 user decrypt multisig accepts a v=0 part from a nested 2-of-3 Safe owner', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const req = await freshMultisigRequest(nestedOuter2of3);
    const digest = computeUnifiedDigest(cfg, req);
    // Same shape with a multisig INNER Safe (bob+dave of a 2-of-3): 292 bytes
    // = 130 static + 32-byte length + 130 inner. Measured ~91k gas incl.
    // intrinsic — only ~9% under the default 100k erc1271_gas_limit, so this
    // positive doubles as a gas-headroom canary. Exceeding the cap does NOT
    // read as an invalid signature: only an RPC error carrying revert data
    // becomes `Rejected` (shared/user-decryption-signature/src/lib.rs:136-149)
    // and out-of-gas carries none (anvil answers `EVM error OutOfGas`, no
    // `data`), so it lands in `Transport` — the relayer retries and then fails
    // the request with a 500 without ever queuing it
    // (relayer/src/host/signature_prechecker.rs:130-157), while the connector
    // treats the same class as recoverable and retries until it gives up.
    const signature = await buildSafeNestedMultisigSignature(
      (await nestedOuter2of3()).safe,
      digest,
      { safe: await innerSafe2of3(), owners: [signers.bob, signers.dave] },
      [signers.bob],
    );
    expect(signature.length).to.equal(2 + 292 * 2);
    const { post, poll } = await requestUnifiedUserDecrypt(
      cfg,
      req,
      { kind: 'raw', signature },
      { waitForTerminal: true, timeoutMs: POSITIVE_TIMEOUT_MS },
    );
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
  });

  it('test erc1271 user decrypt multisig rejects a v=0 part whose inner signature is from a non-owner', async function () {
    const req = await freshMultisigRequest(nestedOuter1of1);
    const digest = computeUnifiedDigest(cfg, req);
    // eve signs the inner wrap but is not an owner of the inner 1-of-1 Safe:
    // the nested isValidSignature reverts (inner GS026), which bubbles up
    // through the outer checkSignatures.
    const signature = await buildSafeNestedMultisigSignature(
      (await nestedOuter1of1()).safe,
      digest,
      { safe: await innerSafe1of1(), owners: [signers.eve] },
      [signers.bob],
    );
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt multisig rejects a v=0 part with a malformed dynamic offset', async function () {
    const req = await freshMultisigRequest(nestedOuter1of1);
    const digest = computeUnifiedDigest(cfg, req);
    // Offset 65 points INSIDE the 130-byte static section — Safe's dynamic
    // bounds checks (GS021) revert before any nested call happens.
    const signature = await buildSafeNestedMultisigSignature(
      (await nestedOuter1of1()).safe,
      digest,
      { safe: await innerSafe1of1(), owners: [signers.dave] },
      [signers.bob],
      { dynamicOffsetOverride: 65 },
    );
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt multisig rejects a 130-byte blob below a threshold of three', async function () {
    const req = await freshMultisigRequest(safe3of3);
    // Two valid owner parts (130 bytes — genuinely longer than one ECDSA
    // signature) still below the 3-of-3 threshold: pins the part-count rule
    // for >65-byte blobs (the 1-of-3 negative only covers a single part).
    const signature = await multisigSignature(safe3of3, req, [signers.bob, signers.carol]);
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt multisig rejects a 3-part blob with one out-of-place part', async function () {
    const req = await freshMultisigRequest(safe3of3);
    const sorted = sortSignatureParts(
      await collectSafeOwnerParts((await safe3of3()).safe, computeUnifiedDigest(cfg, req), [
        signers.bob,
        signers.carol,
        signers.dave,
      ]),
    );
    // Swap the last two parts: the first pair stays ascending, the second
    // violates the rule — pins that ordering is checked PAIRWISE (the
    // descending 2-part negative cannot distinguish pairwise from
    // first-vs-last checking).
    const signature = concatSignatureParts([sorted[0], sorted[2], sorted[1]]);
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt legacy v2 route rejects signatures longer than 65 bytes', async function () {
    // 'Both v2 and v3 should work' cannot hold for ERC-1271: /v2 validates
    // the signature as EXACTLY 130 raw-hex chars and the legacy gateway path
    // verifies with on-chain ecrecover only — multisig ERC-1271 is v3-only by
    // design. Pin the v2 wire-level rejection as executable documentation.
    const req = await freshMultisigRequest(safe2of3);
    const blob = (await multisigSignature(safe2of3, req, [signers.bob, signers.carol])).slice(2); // 260 hex chars
    const body = {
      handleContractPairs: [{ handle: req.handles[0].ctHandle, contractAddress: req.handles[0].contractAddress }],
      requestValidity: { startTimestamp: String(req.startTimestamp), durationDays: '7' },
      contractsChainId: String(chainIdFromHandle(req.handles[0].ctHandle)),
      contractAddresses: [(await safe2of3()).holderAddress],
      userAddress: (await safe2of3()).safe.address,
      signature: blob,
      publicKey: req.publicKey.replace(/^0x/, ''),
      extraData: '0x00',
    };
    const baseUrl = relayerUrl.replace(/\/(v[0-9]+)\/?$/, '').replace(/\/$/, '');
    const resp = await fetch(`${baseUrl}/v2/user-decrypt`, {
      method: 'POST',
      headers: { 'content-type': 'application/json', ...(relayerApiKey ? { 'x-api-key': relayerApiKey } : {}) },
      body: JSON.stringify(body),
    });
    const raw = JSON.stringify(await resp.json().catch(() => ({})));
    expect(resp.status, raw).to.equal(400);
    // The rejection must be the signature-length rule, not an unrelated field.
    expect(raw, raw).to.match(/signature/i);
    expect(raw, raw).to.match(/130/);
  });
});
