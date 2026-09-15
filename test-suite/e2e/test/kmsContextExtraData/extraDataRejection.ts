// Container half of the KMS-context QA case `extradata-rejection`.
//
// The sibling spec in this directory (`kmsContextExtraData.ts`) proves the SDK puts the RIGHT
// context/epoch pair into the permit's extraData. This one proves the protocol refuses a WRONG one:
// the same envelope, corrupted on the wire, must not be accepted. Same field, same versioned format,
// opposite direction — which is why both live under `test/kmsContextExtraData/` and are driven by
// the same profile rather than sitting apart as a generic relayer suite.
//
// The relayer validates the `extraData` wire format before it does anything else with a
// user-decryption request (`validate_extra_data_field_decryption`, wired as a `#[validate(custom)]`
// on the v3 envelope's `extra_data` field). Only three shapes are accepted:
//
//   0x00                                        legacy
//   0x01 || contextId(32)                       contextId 0x07-tagged
//   0x02 || contextId(32) || epochId(32)        contextId 0x07-tagged, epochId 0x08-tagged
//
// Anything else is a synchronous `400`. This suite covers the two things that distinguish a
// corrupted request from a merely wrong one, and that the existing coverage under
// `test/unifiedUserDecryption` does not reach:
//
//   1. **Tampering after signing.** Every other extraData test signs the EIP-712 payload *over* the
//      malformed value, so the signature is valid and only the extraData is wrong. The scenario this
//      suite implements is the opposite: take a request the real SDK built and signed, change one
//      byte, and send it. That request now has TWO defects — and which one the relayer reports is a
//      property of the order it validates in, not of the extraData rules.
//
//   2. **Asserting which field was rejected.** A malformed extraData and a bad signature are both
//      `400` with `label: "validation_failed"`; they differ only in `error.details[].field`. Every
//      assertion here goes down to the field and the issue text, so a request bounced for the wrong
//      reason fails instead of passing.
//
// The Relayer is where this check lives, but the SUBJECT is the KMS context envelope: `extraData` is
// how a request names the context and epoch it is for, so a malformed one is a KMS-context failure
// that the relayer happens to catch first.
//
// Not covered here, deliberately: the SEMANTIC checks on the ids themselves — an unknown contextId,
// an inactive epochId. Those are enforced by the KMS Connector and the Gateway, not by the relayer,
// and they already have coverage in `test/unifiedUserDecryption/unifiedUserDecryption.ts`. This
// suite is only about the wire format, at the relayer.
import { expect } from 'chai';
import { ethers } from 'hardhat';

import { UserDecrypt } from '../../types';
import {
  createInstance,
  createInstances,
  protocolConfigAddress,
  relayerApiKey,
  relayerUrl,
  verifyingContractAddressDecryption,
} from '../instance';
import { FhevmSdk } from '../sdk/fhevm-sdk/sdk';
import type { UnifiedConfig, UnifiedDecryptRequest } from '../sdk/unified/unifiedUserDecrypt';
import { backdatedStartTimestamp, directHandle, submitUnifiedRequest } from '../sdk/unified/unifiedUserDecrypt';
import { Signers, getSigners, initSigners } from '../signers';
import {
  appendBytes,
  captureUnifiedRequest,
  expectAccepted,
  expectExtraDataRejection,
  postUnifiedEnvelope,
  truncateToBytes,
  untagIdAt,
  withVersionByte,
} from './extraDataRejectionHttp';

const SETUP_TIMEOUT_MS = 3 * 60 * 1000;
const CASE_TIMEOUT_MS = 3 * 60 * 1000;
const DURATION_SECONDS = 10 * 24 * 3600;

/** extraData v2: 1 version byte + 32-byte contextId + 32-byte epochId. */
const EXTRA_DATA_V2_LENGTH = 132;

/** Byte offsets of the two ids inside a v2 payload, for the tag-corruption cases. */
const CONTEXT_ID_OFFSET = 1;
const EPOCH_ID_OFFSET = 33;

const hex32 = (value: bigint) => value.toString(16).padStart(64, '0');

/** Reads an expected id injected by the CLI profile, or undefined when running standalone. */
const expectedFromEnv = (name: string): bigint | undefined => {
  const raw = process.env[name];
  if (raw === undefined || raw.trim() === '') return undefined;
  return BigInt(raw.trim());
};

describe('KMS context extraData rejection', function () {
  let signers: Signers;
  let cfg: UnifiedConfig;
  let contract: UserDecrypt;
  let contractAddress: string;
  let publicKey: string;

  /** The active pair, read from the chain — the values a compliant SDK embeds. */
  let currentContextId: bigint;
  let currentEpochId: bigint;

  before(async function () {
    this.timeout(SETUP_TIMEOUT_MS);
    if (!protocolConfigAddress) {
      throw new Error('PROTOCOL_CONFIG_CONTRACT_ADDRESS is required');
    }
    await initSigners(2);
    signers = await getSigners();

    // A guard, not a convenience: without it the suite would pass vacuously on a build whose
    // instance is not the @fhevm/sdk client whose outgoing request is being captured.
    const instances = await createInstances(signers);
    if (!(instances.alice instanceof FhevmSdk)) {
      this.skip();
    }

    cfg = {
      relayerUrl,
      decryptionContractAddress: verifyingContractAddressDecryption,
      apiKey: relayerApiKey || undefined,
    };

    // The constructor grants FHE.allow to msg.sender, so the deployer is the authorized reader.
    const factory = await ethers.getContractFactory('UserDecrypt');
    contract = await factory.connect(signers.alice).deploy();
    await contract.waitForDeployment();
    contractAddress = await contract.getAddress();

    // A fresh re-encryption key: the relayer dedups on
    // (handles, userAddress, allowedContracts, publicKey, extraData), and every request below must
    // be judged on its own extraData rather than answered from an earlier suite's cache entry.
    publicKey = (await instances.alice.generateKeypair()).publicKey;

    const protocolConfig = new ethers.Contract(
      protocolConfigAddress,
      ['function getCurrentKmsContextAndEpoch() view returns (uint256 contextId, uint256 epochId)'],
      ethers.provider,
    );
    [currentContextId, currentEpochId] = await protocolConfig.getCurrentKmsContextAndEpoch();
  });

  it('test kms context extraData rejection agrees with the orchestrator', async function () {
    // The same chain-vs-orchestrator cross-check the sibling spec makes: when the profile drove this
    // run it injected the pair it observed, and a mismatch means a lifecycle operation landed between
    // the two reads — which would make the "valid v2 envelope" premise below describe a pair that is
    // no longer active. Standalone, there is nothing to compare against and this is skipped.
    const expectedContextId = expectedFromEnv('KMS_QA_EXPECTED_CONTEXT_ID');
    const expectedEpochId = expectedFromEnv('KMS_QA_EXPECTED_EPOCH_ID');
    if (expectedContextId === undefined || expectedEpochId === undefined) {
      this.skip();
    }
    expect(
      currentContextId,
      'the active context moved between the orchestrator read and this suite — a concurrent lifecycle operation',
    ).to.equal(expectedContextId);
    expect(
      currentEpochId,
      'the active epoch moved between the orchestrator read and this suite — a concurrent rotation',
    ).to.equal(expectedEpochId);
  });

  /**
   * Captures the envelope the real SDK builds and signs for a user decryption, without letting it
   * reach the relayer. Returns it for the caller to replay, tampered or intact.
   */
  const captureSdkEnvelope = async function (this: Mocha.Context) {
    const instance = await createInstance();
    if (!(instance instanceof FhevmSdk)) {
      this.skip();
    }
    const client = instance.rawClient;
    const transportKeyPair = await client.generateTransportKeyPair();
    const permit = await client.signUnifiedDecryptionPermit({
      contractAddresses: [contractAddress as `0x${string}`],
      durationSeconds: DURATION_SECONDS,
      startTimestamp: Math.floor(Date.now() / 1000),
      transportKeyPair,
      signer: signers.alice,
      signerAddress: signers.alice.address as `0x${string}`,
    });
    const handle = await contract.xUint64();

    return captureUnifiedRequest(() =>
      client.decryptValue({
        contractAddress: contractAddress as `0x${string}`,
        transportKeyPair,
        signedPermit: permit,
        encryptedValue: handle as `0x${string}`,
      }),
    );
  };

  describe('a request built by the SDK, corrupted afterwards', function () {
    it('test kms context extraData rejection refuses a tampered version byte on an SDK-built request', async function () {
      this.timeout(CASE_TIMEOUT_MS);
      const envelope = await captureSdkEnvelope.call(this);
      const original = envelope.attestedPayload.extraData;

      // The Given of the scenario, verified rather than assumed: the SDK really did build a valid
      // v2 envelope carrying the pair that is active on chain. Without this the tamper below could
      // be corrupting something that was never valid to begin with.
      expect(original.length, `the SDK sent a non-v2 extraData: ${original}`).to.equal(EXTRA_DATA_V2_LENGTH);
      expect(original.toLowerCase()).to.equal(
        `0x02${hex32(currentContextId)}${hex32(currentEpochId)}`.toLowerCase(),
      );

      // Change ONLY the version byte. The contextId and epochId stay valid and active; the
      // signature, which covers extraData, is now stale.
      const tampered = { ...envelope, attestedPayload: { ...envelope.attestedPayload, extraData: withVersionByte(original, '03') } };
      expect(tampered.attestedPayload.extraData.slice(4)).to.equal(original.slice(4));

      const post = await postUnifiedEnvelope(relayerUrl, cfg.apiKey, tampered);
      expectExtraDataRejection(post, 'tampered version byte on an SDK-built request');
    });

    it('test kms context extraData rejection accepts the same SDK-built request when it is not corrupted', async function () {
      this.timeout(CASE_TIMEOUT_MS);
      // The control. Without it, the test above proves only that the relayer rejects *something*:
      // a harness that mangled the envelope while capturing it would produce the same 400. Replaying
      // the untouched bytes is what establishes that the corruption is the only difference.
      const envelope = await captureSdkEnvelope.call(this);
      const post = await postUnifiedEnvelope(relayerUrl, cfg.apiKey, envelope);
      expectAccepted(post, 'untouched SDK-built request');
    });
  });

  describe('malformed extraData shapes, signed over the corruption', function () {
    // These sign the EIP-712 payload over the malformed extraData, so the signature is valid and the
    // extraData is the request's only defect. That isolates the format rules from the ordering
    // question the tamper case above probes.
    const corruptions = (): ReadonlyArray<{ readonly what: string; readonly extraData: string; readonly why: string }> => {
      const valid = `0x02${hex32(currentContextId)}${hex32(currentEpochId)}`;
      return [
        {
          what: 'an unsupported version byte',
          extraData: withVersionByte(valid, 'ff'),
          why: 'only 0x00, 0x01 and 0x02 are known versions',
        },
        {
          what: 'a v2 payload truncated to the contextId',
          extraData: truncateToBytes(valid, 33),
          why: 'v2 is a fixed 65 bytes; a short one must not be read as a v1',
        },
        {
          what: 'a v2 payload with a trailing byte',
          extraData: appendBytes(valid, 'ff'),
          why: 'each version has a fixed size — new fields come from a new version, not from appending',
        },
        {
          what: 'a v1 version byte on a v2-sized payload',
          extraData: withVersionByte(valid, '01'),
          why: 'the length is checked against the declared version, not merely for being a known one',
        },
        {
          what: 'an untagged contextId',
          extraData: untagIdAt(valid, CONTEXT_ID_OFFSET),
          why: 'a contextId must carry the 0x07 domain tag',
        },
        {
          what: 'an untagged epochId',
          extraData: untagIdAt(valid, EPOCH_ID_OFFSET),
          why: 'an epochId must carry the 0x08 domain tag',
        },
      ];
    };

    it('test kms context extraData rejection refuses every malformed shape with a field-level error', async function () {
      this.timeout(CASE_TIMEOUT_MS);
      const handle = await contract.xUint64();

      for (const { what, extraData, why } of corruptions()) {
        const request: UnifiedDecryptRequest = {
          handles: [directHandle(handle, contractAddress, signers.alice.address)],
          userAddress: signers.alice.address,
          allowedContracts: [],
          publicKey,
          startTimestamp: backdatedStartTimestamp(),
          durationSeconds: DURATION_SECONDS,
          extraData,
        };
        const { post } = await submitUnifiedRequest(cfg, request, { kind: 'eoa', signer: signers.alice });
        // `PostResult.raw` is typed `unknown` by the shared helper — narrow here rather than widen
        // the shared type, which every other suite consumes as an opaque blob for error messages.
        expectExtraDataRejection(
          { httpStatus: post.httpStatus, raw: (post.raw ?? {}) as Record<string, unknown> },
          `${what} (${why})`,
        );
      }
    });
  });
});
