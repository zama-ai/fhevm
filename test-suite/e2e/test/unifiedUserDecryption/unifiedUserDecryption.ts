import { expect } from 'chai';
import { ethers } from 'hardhat';

import { ERC1271OwnerWallet, EncryptedERC20, SmartWalletWithDelegation, UserDecrypt } from '../../types';
import {
  createInstances,
  protocolConfigAddress,
  relayerApiKey,
  relayerUrl,
  verifyingContractAddressDecryption,
} from '../instance';
import { Signers, getSigners, initSigners } from '../signers';
import { FhevmInstances } from '../types';
import { waitForBlock } from '../utils';
import type { UnifiedConfig, UnifiedDecryptRequest } from '../sdk/unified/unifiedUserDecrypt';
import {
  backdatedStartTimestamp,
  delegatedHandle,
  directHandle,
  requestUnifiedUserDecrypt,
  submitUnifiedRequest,
} from '../sdk/unified/unifiedUserDecrypt';

const DURATION_SECONDS = 7 * 24 * 60 * 60;
const POSITIVE_TIMEOUT_MS = 3 * 60 * 1000;
// Mocha timeout margin on top of the poll window (pre-poll on-chain work + POST).
const TIMEOUT_MARGIN_MS = 60 * 1000;
// Floor for the bounded window used by async-rejected requests. A correctly-
// rejected request is never forwarded to the KMS, so it never reaches
// `succeeded`; the relayer only flips it to `failed` after its own ~300s
// user-decrypt timeout. Asserting "not succeeded" within a bounded window keeps
// negatives deterministic and fast — the window is calibrated at runtime to a
// multiple of the observed positive-control latency (see below) so a slow stack
// cannot false-pass a negative that would have succeeded a moment later.
const NEGATIVE_WINDOW_FLOOR_MS = 60 * 1000;
const NEGATIVE_WINDOW_CAP_MS = 4 * 60 * 1000;
const SLOW_TEST_TIMEOUT_MS = 10 * 60 * 1000;
// Blocks to wait after an on-chain ACL delegation before the KMS Connector's
// host-chain reads observe it (same wait the delegated-user-decryption suite uses).
const PROPAGATION_BLOCKS = 15;
const ONE_DAY_SECONDS = 24 * 60 * 60;

const nowSeconds = () => Math.floor(Date.now() / 1000);

/**
 * RFC-016 — Unified EIP-712 decryption request with `allowedContracts`
 * (permissive vs. app-bounded), per-handle `ownerAddress` (direct + delegated in
 * one signature), the `userAddress ∉ allowedContracts` rule, and the validity
 * window.
 *
 * Positive authorizations are driven to a `succeeded` job (the KMS produced
 * shares — which only happens once every RFC-016 check passed). Validity-window
 * violations are rejected synchronously by the relayer (`400`); the remaining
 * authorization failures are checked in the KMS Connector asynchronously —
 * since the gateway emits the event unconditionally, a rejected request is
 * never forwarded and simply never succeeds (asserted within a bounded window,
 * calibrated against the positive-control latency).
 */
describe('Unified user decryption (RFC-016)', function () {
  let signers: Signers;
  let instances: FhevmInstances;
  let cfg: UnifiedConfig;
  let publicKey: string;

  let aliceContract: UserDecrypt;
  let aliceContractAddress: string;
  let bobContract: UserDecrypt;
  let bobContractAddress: string;

  // Calibrated by the first positive control: max(floor, 3x observed positive
  // latency), capped. Used as the observation window for all async negatives.
  let negativeWindowMs = NEGATIVE_WINDOW_FLOOR_MS;

  before(async function () {
    this.timeout(180_000);
    await initSigners(3);
    signers = await getSigners();
    instances = await createInstances(signers);
    cfg = {
      relayerUrl,
      decryptionContractAddress: verifyingContractAddressDecryption,
      apiKey: relayerApiKey || undefined,
    };

    const factory = await ethers.getContractFactory('UserDecrypt');
    aliceContract = await factory.connect(signers.alice).deploy();
    await aliceContract.waitForDeployment();
    aliceContractAddress = await aliceContract.getAddress();

    bobContract = await factory.connect(signers.bob).deploy();
    await bobContract.waitForDeployment();
    bobContractAddress = await bobContract.getAddress();

    publicKey = (await instances.alice.generateKeypair()).publicKey;
  });

  it('test unified user decrypt permissive mode (empty allowedContracts) succeeds', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const handle = await aliceContract.xUint64();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, aliceContractAddress, signers.alice.address)],
      userAddress: signers.alice.address,
      allowedContracts: [],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    const startedAt = Date.now();
    const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.alice }, {
      waitForTerminal: true,
      timeoutMs: POSITIVE_TIMEOUT_MS,
    });
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
    // Calibrate the async-negative observation window to this stack's latency.
    const elapsedMs = Date.now() - startedAt;
    negativeWindowMs = Math.min(Math.max(NEGATIVE_WINDOW_FLOOR_MS, 3 * elapsedMs), NEGATIVE_WINDOW_CAP_MS);
  });

  it('test unified user decrypt app-bounded mode (allowedContracts=[app]) succeeds', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const handle = await aliceContract.xUint32();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, aliceContractAddress, signers.alice.address)],
      userAddress: signers.alice.address,
      allowedContracts: [aliceContractAddress],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.alice }, {
      waitForTerminal: true,
      timeoutMs: POSITIVE_TIMEOUT_MS,
    });
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
  });

  it('test unified user decrypt app-bounded mode accepts when ANY listed contract is allowed (any-of semantics)', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const handle = await aliceContract.xUint8();
    // First entry is NOT allowed on the handle; the second is. RFC-016 requires
    // "at least one listed contract" — an implementation wrongly requiring ALL
    // entries to be allowed would fail this test.
    const unrelated = ethers.Wallet.createRandom().address;
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, aliceContractAddress, signers.alice.address)],
      userAddress: signers.alice.address,
      allowedContracts: [unrelated, aliceContractAddress],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.alice }, {
      waitForTerminal: true,
      timeoutMs: POSITIVE_TIMEOUT_MS,
    });
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
  });

  it('test unified user decrypt multi-handle batch (same owner, one signature) succeeds', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const handle32 = await bobContract.xUint32();
    const handle64 = await bobContract.xUint64();
    const req: UnifiedDecryptRequest = {
      handles: [
        directHandle(handle32, bobContractAddress, signers.bob.address),
        directHandle(handle64, bobContractAddress, signers.bob.address),
      ],
      userAddress: signers.bob.address,
      allowedContracts: [],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.bob }, {
      waitForTerminal: true,
      timeoutMs: POSITIVE_TIMEOUT_MS,
    });
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
  });

  it('test unified user decrypt rejects an expired validity window', async function () {
    // startTimestamp + durationSeconds is in the past -> the relayer rejects
    // synchronously ("requestValidity window has already expired"); the KMS
    // Connector applies the same always-executed window check (step 3).
    const handle = await aliceContract.xUint64();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, aliceContractAddress, signers.alice.address)],
      userAddress: signers.alice.address,
      allowedContracts: [],
      publicKey,
      startTimestamp: backdatedStartTimestamp(7200),
      durationSeconds: 3600,
    };
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'eoa', signer: signers.alice });
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(400);
    expect(JSON.stringify(post.raw).toLowerCase()).to.include('expired');
  });

  it('test unified user decrypt rejects a future startTimestamp (invalidation-bypass vector)', async function () {
    // RFC-016 warns that a future-dated startTimestamp would survive an
    // invalidation set to block.timestamp; the relayer rejects it up front
    // ("Timestamp must not be in the future").
    const handle = await aliceContract.xUint64();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, aliceContractAddress, signers.alice.address)],
      userAddress: signers.alice.address,
      allowedContracts: [],
      publicKey,
      startTimestamp: nowSeconds() + 3600,
      durationSeconds: DURATION_SECONDS,
    };
    const { post } = await submitUnifiedRequest(cfg, req, { kind: 'eoa', signer: signers.alice });
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(400);
    expect(JSON.stringify(post.raw).toLowerCase()).to.include('future');
  });

  it('test unified user decrypt rejects an allowedContracts list that does not cover the handle', async function () {
    this.timeout(negativeWindowMs + TIMEOUT_MARGIN_MS);
    const handle = await aliceContract.xUint64();
    const unrelated = ethers.Wallet.createRandom().address; // not allowed on the handle
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, aliceContractAddress, signers.alice.address)],
      userAddress: signers.alice.address,
      allowedContracts: [unrelated],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.alice }, {
      waitForTerminal: true,
      timeoutMs: negativeWindowMs,
    });
    // Signature is valid, so the relayer accepts; the KMS then rejects on the
    // contract-allowance check and the job never succeeds.
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.not.equal('succeeded');
  });

  it('test unified user decrypt rejects when userAddress appears in allowedContracts', async function () {
    this.timeout(negativeWindowMs + TIMEOUT_MARGIN_MS);
    const handle = await aliceContract.xUint64();
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, aliceContractAddress, signers.alice.address)],
      userAddress: signers.alice.address,
      allowedContracts: [signers.alice.address], // forbidden: userAddress ∈ allowedContracts
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.alice }, {
      waitForTerminal: true,
      timeoutMs: negativeWindowMs,
    });
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.not.equal('succeeded');
  });

  it('test unified user decrypt rejects a spoofed ownerAddress (handle not owned by userAddress)', async function () {
    this.timeout(negativeWindowMs + TIMEOUT_MARGIN_MS);
    const handle = await aliceContract.xUint64(); // owned by alice, not bob
    const req: UnifiedDecryptRequest = {
      handles: [directHandle(handle, aliceContractAddress, signers.bob.address)],
      userAddress: signers.bob.address,
      allowedContracts: [],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.bob }, {
      waitForTerminal: true,
      timeoutMs: negativeWindowMs,
    });
    // bob's signature is valid, so the relayer accepts; the ACL ownership check
    // (isAllowed(handle, bob) == false) rejects it in the KMS Connector.
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.not.equal('succeeded');
  });

  it('test unified user decrypt rejects a delegated handle entry when no delegation exists', async function () {
    this.timeout(negativeWindowMs + TIMEOUT_MARGIN_MS);
    // ownerAddress = alice != userAddress = bob triggers the delegated branch:
    // isHandleDelegatedForUserDecryption(alice, bob, contract, handle). No
    // delegation alice -> bob exists, so the KMS rejects (RFC-016 "ownerAddress
    // = X != userAddress" spoof scenario).
    const handle = await aliceContract.xUint64();
    const req: UnifiedDecryptRequest = {
      handles: [delegatedHandle(handle, aliceContractAddress, signers.alice.address)],
      userAddress: signers.bob.address,
      allowedContracts: [],
      publicKey,
      startTimestamp: backdatedStartTimestamp(),
      durationSeconds: DURATION_SECONDS,
    };
    const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.bob }, {
      waitForTerminal: true,
      timeoutMs: negativeWindowMs,
    });
    expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
    expect(poll?.status, JSON.stringify(poll?.raw)).to.not.equal('succeeded');
  });

  describe('extraData versions', function () {
    // Backend behavior on the unified path (relayer
    // `validate_extra_data_field_decryption`, kms-connector `validate_context`):
    // the relayer accepts extraData v0 ("0x00"), v1 (0x01 + 32B contextId), and
    // v2 (0x02 + 32B contextId + 32B epochId). The KMS Connector validates the
    // contextId of v1/v2 (DB, falling back to on-chain ProtocolConfig) and —
    // for v2 — that the epochId is the ACTIVE epoch of that context; it SKIPS
    // all checks for v0 (backward compat with pre-context SDKs, normalized to
    // empty for KMS core). New SDKs are expected to send v2 with the values
    // from `ProtocolConfig.getCurrentKmsContextAndEpoch()`; v0/v1 may be
    // dropped later — the explicit v0 test below is the canary that fails
    // loudly when that happens, forcing a conscious update instead of a
    // silent break.
    let extraDataPublicKey: string;
    let currentContextId: bigint;
    let currentEpochId: bigint;

    const hex32 = (v: bigint) => v.toString(16).padStart(64, '0');

    before(async function () {
      // Fresh re-encryption key: the relayer dedups requests on
      // (handles, userAddress, allowedContracts, publicKey, extraData) — a
      // fresh key guarantees these are real jobs, not cache hits from the
      // earlier tests that also use extraData 0x00 on the same handles.
      extraDataPublicKey = (await instances.alice.generateKeypair()).publicKey;

      // The values a compliant new SDK must embed: the CURRENT context id and
      // its ACTIVE epoch id, read from the host chain. Note: epochId 0 is NOT
      // accepted on latest-main backends ("Epoch #0 ... is not active
      // on-chain") — the active epoch must be resolved, not hardcoded.
      const protocolConfig = new ethers.Contract(
        protocolConfigAddress,
        ['function getCurrentKmsContextAndEpoch() view returns (uint256 contextId, uint256 epochId)'],
        ethers.provider,
      );
      [currentContextId, currentEpochId] = await protocolConfig.getCurrentKmsContextAndEpoch();
    });

    it('test unified user decrypt accepts legacy extraData v0 (0x00, no contextId/epochId)', async function () {
      this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
      const handle = await aliceContract.xUint64();
      const req: UnifiedDecryptRequest = {
        handles: [directHandle(handle, aliceContractAddress, signers.alice.address)],
        userAddress: signers.alice.address,
        allowedContracts: [],
        publicKey: extraDataPublicKey,
        startTimestamp: backdatedStartTimestamp(),
        durationSeconds: DURATION_SECONDS,
        extraData: '0x00',
      };
      const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.alice }, {
        waitForTerminal: true,
        timeoutMs: POSITIVE_TIMEOUT_MS,
      });
      expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
      expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
    });

    it('test unified user decrypt accepts extraData v1 with the current contextId', async function () {
      this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
      // v1 carries a contextId but no epochId — the connector validates the
      // context and skips the epoch check ("extraData without epochId" is
      // allowed as long as the contextId is the current one).
      expect(currentContextId, 'stack reports no current KMS context id').to.not.equal(0n);
      const handle = await aliceContract.xUint32();
      const req: UnifiedDecryptRequest = {
        handles: [directHandle(handle, aliceContractAddress, signers.alice.address)],
        userAddress: signers.alice.address,
        allowedContracts: [],
        publicKey: extraDataPublicKey,
        startTimestamp: backdatedStartTimestamp(),
        durationSeconds: DURATION_SECONDS,
        extraData: `0x01${hex32(currentContextId)}`,
      };
      const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.alice }, {
        waitForTerminal: true,
        timeoutMs: POSITIVE_TIMEOUT_MS,
      });
      expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
      expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
    });

    it('test unified user decrypt accepts extraData v2 with the current contextId and active epochId (new-SDK path)', async function () {
      this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
      // The full forward path: v2 with BOTH values from
      // getCurrentKmsContextAndEpoch(). The connector rejects any epochId
      // that is not the active epoch of the context (including 0).
      expect(currentContextId, 'stack reports no current KMS context id').to.not.equal(0n);
      expect(currentEpochId, 'stack reports no active KMS epoch id').to.not.equal(0n);
      const handle = await aliceContract.xUint16();
      const req: UnifiedDecryptRequest = {
        handles: [directHandle(handle, aliceContractAddress, signers.alice.address)],
        userAddress: signers.alice.address,
        allowedContracts: [],
        publicKey: extraDataPublicKey,
        startTimestamp: backdatedStartTimestamp(),
        durationSeconds: DURATION_SECONDS,
        extraData: `0x02${hex32(currentContextId)}${hex32(currentEpochId)}`,
      };
      const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.alice }, {
        waitForTerminal: true,
        timeoutMs: POSITIVE_TIMEOUT_MS,
      });
      expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
      expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
    });

    it('test unified user decrypt rejects extraData v2 with an inactive epochId (0)', async function () {
      this.timeout(negativeWindowMs + TIMEOUT_MARGIN_MS);
      // Empirical pin: epoch validation is LIVE on the unified path. A v2
      // extraData with the correct contextId but epochId 0 (the js-sdk's
      // pre-epoch default) is rejected by the connector:
      // "Epoch #0 of context #... is not active on-chain".
      const handle = await aliceContract.xUint64();
      const req: UnifiedDecryptRequest = {
        handles: [directHandle(handle, aliceContractAddress, signers.alice.address)],
        userAddress: signers.alice.address,
        allowedContracts: [],
        publicKey: extraDataPublicKey,
        startTimestamp: backdatedStartTimestamp(),
        durationSeconds: DURATION_SECONDS,
        extraData: `0x02${hex32(currentContextId)}${hex32(0n)}`,
      };
      const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.alice }, {
        waitForTerminal: true,
        timeoutMs: negativeWindowMs,
      });
      expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
      expect(poll?.status, JSON.stringify(poll?.raw)).to.not.equal('succeeded');
    });

    it('test unified user decrypt rejects extraData with an unknown contextId', async function () {
      this.timeout(negativeWindowMs + TIMEOUT_MARGIN_MS);
      // Versioned extraData is NOT decorative: the connector validates the
      // embedded contextId against its context store. A fabricated id passes
      // the relayer's format check (and the signature covers it), but the KMS
      // Connector rejects it and the job never succeeds.
      const handle = await aliceContract.xUint64();
      const req: UnifiedDecryptRequest = {
        handles: [directHandle(handle, aliceContractAddress, signers.alice.address)],
        userAddress: signers.alice.address,
        allowedContracts: [],
        publicKey: extraDataPublicKey,
        startTimestamp: backdatedStartTimestamp(),
        durationSeconds: DURATION_SECONDS,
        extraData: `0x01${hex32(0xdeadbeefn)}`,
      };
      const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.alice }, {
        waitForTerminal: true,
        timeoutMs: negativeWindowMs,
      });
      expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
      expect(poll?.status, JSON.stringify(poll?.raw)).to.not.equal('succeeded');
    });

    it('test unified user decrypt rejects a malformed extraData version', async function () {
      // Unknown version byte -> rejected synchronously by the relayer's wire
      // validation (only 0x00, v1, and v2 shapes are accepted).
      const handle = await aliceContract.xUint64();
      const req: UnifiedDecryptRequest = {
        handles: [directHandle(handle, aliceContractAddress, signers.alice.address)],
        userAddress: signers.alice.address,
        allowedContracts: [],
        publicKey: extraDataPublicKey,
        startTimestamp: backdatedStartTimestamp(),
        durationSeconds: DURATION_SECONDS,
        extraData: `0x03${hex32(1n)}`,
      };
      const { post } = await submitUnifiedRequest(cfg, req, { kind: 'eoa', signer: signers.alice });
      expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(400);
    });
  });

  describe('delegated legs', function () {
    let token: EncryptedERC20;
    let tokenAddress: string;
    let smartWallet: SmartWalletWithDelegation;
    let smartWalletAddress: string;
    let erc1271Wallet: ERC1271OwnerWallet;
    let erc1271WalletAddress: string;
    let delegatedCtHandle: string;

    before(async function () {
      this.timeout(SLOW_TEST_TIMEOUT_MS);

      // carol owns a SmartWallet holding an encrypted token balance. She
      // delegates decryption of the token contract's handles to (a) bob's EOA
      // and (b) an ERC-1271 wallet owned by bob — covering both the mixed-batch
      // and the three-address cases with a single propagation wait.
      const tokenFactory = await ethers.getContractFactory('EncryptedERC20');
      token = await tokenFactory.connect(signers.alice).deploy('Zama Confidential Token', 'ZAMA');
      await token.waitForDeployment();
      tokenAddress = await token.getAddress();

      const walletFactory = await ethers.getContractFactory('SmartWalletWithDelegation');
      smartWallet = await walletFactory.connect(signers.carol).deploy(signers.carol.address);
      await smartWallet.waitForDeployment();
      smartWalletAddress = await smartWallet.getAddress();

      const erc1271Factory = await ethers.getContractFactory('ERC1271OwnerWallet');
      erc1271Wallet = await erc1271Factory.connect(signers.alice).deploy(signers.bob.address);
      await erc1271Wallet.waitForDeployment();
      erc1271WalletAddress = await erc1271Wallet.getAddress();

      await (await token.connect(signers.alice).mint(1_000_000n)).wait();
      const enc = await instances.alice.encryptUint64({
        value: 500_000n,
        contractAddress: tokenAddress,
        userAddress: signers.alice.address,
      });
      await (
        await token
          .connect(signers.alice)
          ['transfer(address,bytes32,bytes)'](smartWalletAddress, enc.handles[0], enc.inputProof)
      ).wait();

      const expiration = nowSeconds() + ONE_DAY_SECONDS;
      await (
        await smartWallet.connect(signers.carol).delegateUserDecryption(signers.bob.address, tokenAddress, expiration)
      ).wait();
      await (
        await smartWallet.connect(signers.carol).delegateUserDecryption(erc1271WalletAddress, tokenAddress, expiration)
      ).wait();
      const currentBlock = await ethers.provider.getBlockNumber();
      await waitForBlock(currentBlock + PROPAGATION_BLOCKS);

      delegatedCtHandle = await token.balanceOf(smartWalletAddress);
    });

    it('test unified user decrypt mixed batch (direct + delegated handles) in one signature succeeds', async function () {
      this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
      const directCtHandle = await bobContract.xUint64();
      const req: UnifiedDecryptRequest = {
        handles: [
          directHandle(directCtHandle, bobContractAddress, signers.bob.address),
          delegatedHandle(delegatedCtHandle, tokenAddress, smartWalletAddress),
        ],
        userAddress: signers.bob.address,
        allowedContracts: [],
        publicKey,
        startTimestamp: backdatedStartTimestamp(),
        durationSeconds: DURATION_SECONDS,
      };
      const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.bob }, {
        waitForTerminal: true,
        timeoutMs: POSITIVE_TIMEOUT_MS,
      });
      expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
      expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
    });

    it('test unified user decrypt three-address case (ERC-1271 wallet decrypting a delegated handle) succeeds', async function () {
      this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
      // The composition both RFCs single out: the ECDSA signer is bob (owner
      // key inside the wallet), userAddress is the ERC-1271 wallet (verified
      // via isValidSignature), and ownerAddress is the third-party delegator
      // (the SmartWallet) — isValidSignature + isHandleDelegatedForUserDecryption
      // in a single request.
      const req: UnifiedDecryptRequest = {
        handles: [delegatedHandle(delegatedCtHandle, tokenAddress, smartWalletAddress)],
        userAddress: erc1271WalletAddress,
        allowedContracts: [],
        publicKey,
        startTimestamp: backdatedStartTimestamp(),
        durationSeconds: DURATION_SECONDS,
      };
      const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'erc1271', ownerSigner: signers.bob }, {
        waitForTerminal: true,
        timeoutMs: POSITIVE_TIMEOUT_MS,
      });
      expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
      expect(poll?.status, JSON.stringify(poll?.raw)).to.equal('succeeded');
    });

    it('test unified user decrypt rejects a delegated handle with a fabricated contractAddress', async function () {
      this.timeout(negativeWindowMs + TIMEOUT_MARGIN_MS);
      // The delegation smartWallet -> bob exists via tokenAddress only.
      // Substituting a different contractAddress in the (unsigned) HandleEntry
      // must fail isHandleDelegatedForUserDecryption — delegation is
      // contract-scoped (RFC-016 "fabricated contractAddress" spoof scenario).
      const req: UnifiedDecryptRequest = {
        handles: [delegatedHandle(delegatedCtHandle, bobContractAddress, smartWalletAddress)],
        userAddress: signers.bob.address,
        allowedContracts: [],
        publicKey,
        startTimestamp: backdatedStartTimestamp(),
        durationSeconds: DURATION_SECONDS,
      };
      const { post, poll } = await requestUnifiedUserDecrypt(cfg, req, { kind: 'eoa', signer: signers.bob }, {
        waitForTerminal: true,
        timeoutMs: negativeWindowMs,
      });
      expect(post.httpStatus, JSON.stringify(post.raw)).to.equal(202);
      expect(poll?.status, JSON.stringify(poll?.raw)).to.not.equal('succeeded');
    });
  });
});
