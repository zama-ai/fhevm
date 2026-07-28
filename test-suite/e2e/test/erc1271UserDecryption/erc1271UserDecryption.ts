import { expect } from 'chai';
import type { Signer } from 'ethers';
import { getBytes, zeroPadValue } from 'ethers';
import { ethers } from 'hardhat';

import {
  ERC1271ApproveHashWallet,
  ERC1271MultisigWallet,
  ERC1271OwnerWallet,
  ERC1271RejectWallet,
  UserDecrypt,
} from '../../types';
import { createInstances, relayerApiKey, relayerUrl, verifyingContractAddressDecryption } from '../instance';
import type { SignaturePart, UnifiedConfig, UnifiedDecryptRequest } from '../sdk/unified/unifiedUserDecrypt';
import {
  backdatedStartTimestamp,
  buildMultisigSignature,
  chainIdFromHandle,
  collectOwnerParts,
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

// Trivially-encrypted value each mock wallet stores; the exact plaintext is
// irrelevant — a `succeeded` job proves the ERC-1271 signature was accepted and
// the KMS produced re-encrypted shares.
const KNOWN_VALUE = 123456789n;
const DURATION_SECONDS = 7 * 24 * 60 * 60;
// Generous window for a full user-decrypt round trip through the KMS.
const POSITIVE_TIMEOUT_MS = 3 * 60 * 1000;
// Mocha timeout margin on top of the poll window (pre-poll on-chain reads + POST).
const TIMEOUT_MARGIN_MS = 60 * 1000;

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
  let multisig2of3: ERC1271MultisigWallet;
  let multisig2of3Address: string;
  let multisig3of3: ERC1271MultisigWallet;
  let multisig3of3Address: string;

  before(async function () {
    this.timeout(180_000);
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

    // ERC-1271 multisig wallets (Safe-style concatenated signatures): owners
    // bob/carol/dave only ever sign typed data offline — they pay no gas.
    const multisigFactory = await ethers.getContractFactory('ERC1271MultisigWallet');
    const multisigOwners = [signers.bob.address, signers.carol.address, signers.dave.address];
    multisig2of3 = await multisigFactory.connect(signers.alice).deploy(multisigOwners, 2);
    await multisig2of3.waitForDeployment();
    multisig2of3Address = await multisig2of3.getAddress();
    await (await multisig2of3.connect(signers.alice).initValue(KNOWN_VALUE)).wait();

    multisig3of3 = await multisigFactory.connect(signers.alice).deploy(multisigOwners, 3);
    await multisig3of3.waitForDeployment();
    multisig3of3Address = await multisig3of3.getAddress();
    await (await multisig3of3.connect(signers.alice).initValue(KNOWN_VALUE)).wait();

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
    expect(post.jobId, JSON.stringify(post.raw)).to.be.a('string');
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

  // Multisig (Safe-style static encoding): the signature is a concatenation of
  // 65-byte {r,s,v} owner parts sorted strictly ascending by signer address.
  // The blob is longer than a single ECDSA signature, so `ecrecover` on it is
  // impossible — every layer must forward it opaquely to the wallet's
  // `isValidSignature`. Bad blobs are rejected synchronously (400) by the
  // relayer's pre-check; the KMS Connector runs the same shared verifier again
  // before the KMS produces shares.

  /**
   * A request for a multisig wallet's stored handle with a FRESH re-encryption
   * key: the relayer dedups accepted jobs on a content hash that EXCLUDES the
   * signature, so a second positive differing only in its multisig blob would
   * collapse onto the first job and pass vacuously. (Definitively-bad
   * signatures are 400-rejected by the pre-check before dedup is consulted —
   * the fresh key just keeps every request, negative included, independent.)
   */
  async function freshMultisigRequest(
    wallet: ERC1271MultisigWallet,
    walletAddress: string,
  ): Promise<UnifiedDecryptRequest> {
    const handle = await wallet.value();
    const freshKey = (await instances.alice.generateKeypair()).publicKey;
    return {
      handles: [directHandle(handle, walletAddress, walletAddress)],
      userAddress: walletAddress,
      allowedContracts: [],
      publicKey: freshKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
  }

  it('test erc1271 user decrypt multisig 2-of-3 concatenated owner signatures succeed', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const req = await freshMultisigRequest(multisig2of3, multisig2of3Address);
    const signature = await buildMultisigSignature(cfg, req, [signers.bob, signers.carol]);
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

  it('test erc1271 user decrypt multisig 3-of-3 concatenated owner signatures succeed', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const req = await freshMultisigRequest(multisig3of3, multisig3of3Address);
    const signature = await buildMultisigSignature(cfg, req, [signers.bob, signers.carol, signers.dave]);
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
    const req = await freshMultisigRequest(multisig2of3, multisig2of3Address);
    // A single owner part is exactly 65 bytes: `ecrecover` parses it but
    // recovers bob, not the wallet, so verification falls through to
    // ERC-1271 — where one part is below the threshold of two.
    const signature = await buildMultisigSignature(cfg, req, [signers.bob]);
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt multisig rejects a blob containing a non-owner signature', async function () {
    const req = await freshMultisigRequest(multisig2of3, multisig2of3Address);
    // eve is not an owner; her part is well-formed but recovers a non-owner.
    const signature = await buildMultisigSignature(cfg, req, [signers.bob, signers.eve]);
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt multisig rejects a duplicated owner signature (threshold inflation)', async function () {
    const req = await freshMultisigRequest(multisig2of3, multisig2of3Address);
    // Two copies of bob's part: the strictly-ascending signer rule is what
    // stops one owner from inflating the approval count to the threshold.
    const signature = await buildMultisigSignature(cfg, req, [signers.bob, signers.bob]);
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt multisig rejects parts in descending signer order', async function () {
    const req = await freshMultisigRequest(multisig2of3, multisig2of3Address);
    // Valid owner parts in descending order: Safe's canonical encoding
    // requires ascending signer addresses.
    const signature = await buildMultisigSignature(cfg, req, [signers.bob, signers.carol], { order: 'descending' });
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt multisig rejects a garbage blob below the threshold minimum length', async function () {
    const req = await freshMultisigRequest(multisig2of3, multisig2of3Address);
    // 100 bytes of junk: neither a valid ECDSA signature nor enough bytes for
    // two 65-byte parts (Safe's length rule) — every layer must hand it to
    // the wallet without choking, and the wallet must answer with a non-magic
    // value (no revert).
    const signature = `0x${'11'.repeat(100)}`;
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  // Safe overloads the `v` byte of each 65-byte part as a type selector; the
  // mock mirrors the two additional static types (the dynamic v=0
  // contract-signature type is deferred to a real-Safe interop follow-up).

  /** Safe pre-approved-hash part (v=1): `r` carries the approving owner's address, `s` is unused. */
  function approvedHashPart(ownerAddress: string): SignaturePart {
    return {
      address: ownerAddress.toLowerCase(),
      signature: `${zeroPadValue(ownerAddress, 32)}${'00'.repeat(32)}01`,
    };
  }

  /** Safe eth_sign part (v > 30): the owner eth_signs the digest and `v` is stored shifted by +4. */
  async function ethSignPart(digest: string, owner: Signer & { address: string }): Promise<SignaturePart> {
    const sig = await owner.signMessage(getBytes(digest));
    const v = parseInt(sig.slice(-2), 16) + 4;
    return { address: owner.address.toLowerCase(), signature: `${sig.slice(0, -2)}${v.toString(16)}` };
  }

  it('test erc1271 user decrypt multisig accepts a blob with trailing bytes (length not a multiple of 65)', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const req = await freshMultisigRequest(multisig2of3, multisig2of3Address);
    const parts = sortSignatureParts(await collectOwnerParts(cfg, req, [signers.bob, signers.carol]));
    // Two valid parts + 35 junk bytes = 165 bytes, NOT a multiple of 65:
    // every layer must forward the unusual length untouched, and the wallet
    // must ignore anything past the static threshold*65 section — exactly
    // where real Safe appends dynamic data for contract-signature parts.
    const signature = concatSignatureParts(parts, '11'.repeat(35));
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
    const req = await freshMultisigRequest(multisig2of3, multisig2of3Address);
    const digest = computeUnifiedDigest(cfg, req);
    // bob pre-approves the digest on-chain (Safe's approveHash flow); carol
    // signs normally. The blob mixes a v=1 part (r = bob's address) with a
    // plain ECDSA part — a realistic Safe part-type combination.
    await (await multisig2of3.connect(signers.bob).approveHash(digest)).wait();
    const [carolPart] = await collectOwnerParts(cfg, req, [signers.carol]);
    const signature = concatSignatureParts(sortSignatureParts([approvedHashPart(signers.bob.address), carolPart]));
    const { post, poll } = await requestUnifiedUserDecrypt(
      cfg,
      req,
      { kind: 'raw', signature },
      { waitForTerminal: true, timeoutMs: POSITIVE_TIMEOUT_MS },
    );
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
  });

  it('test erc1271 user decrypt multisig accepts eth_sign parts (v shifted by 4)', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const req = await freshMultisigRequest(multisig2of3, multisig2of3Address);
    const digest = computeUnifiedDigest(cfg, req);
    // Safe's eth_sign encoding: owners sign the eth_sign wrap of the digest
    // and the part stores v+4 (31/32) as the type selector.
    const parts = sortSignatureParts([
      await ethSignPart(digest, signers.bob),
      await ethSignPart(digest, signers.carol),
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

  it('test erc1271 user decrypt multisig rejects a 130-byte blob below a threshold of three', async function () {
    const req = await freshMultisigRequest(multisig3of3, multisig3of3Address);
    // Two valid owner parts (130 bytes — genuinely longer than one ECDSA
    // signature) still below the 3-of-3 threshold: pins the part-count rule
    // for >65-byte blobs (the 1-of-3 negative only covers a single part).
    const signature = await buildMultisigSignature(cfg, req, [signers.bob, signers.carol]);
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'raw', signature });
    expect(isSignatureRejection(post), JSON.stringify(post.raw)).to.equal(true);
  });

  it('test erc1271 user decrypt multisig rejects a 3-part blob with one out-of-place part', async function () {
    const req = await freshMultisigRequest(multisig3of3, multisig3of3Address);
    const sorted = sortSignatureParts(await collectOwnerParts(cfg, req, [signers.bob, signers.carol, signers.dave]));
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
    const req = await freshMultisigRequest(multisig2of3, multisig2of3Address);
    const blob = (await buildMultisigSignature(cfg, req, [signers.bob, signers.carol])).slice(2); // 260 hex chars
    const body = {
      handleContractPairs: [{ handle: req.handles[0].ctHandle, contractAddress: req.handles[0].contractAddress }],
      requestValidity: { startTimestamp: String(req.startTimestamp), durationDays: '7' },
      contractsChainId: String(chainIdFromHandle(req.handles[0].ctHandle)),
      contractAddresses: [multisig2of3Address],
      userAddress: multisig2of3Address,
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
