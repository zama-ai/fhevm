// Container half of the KMS context/epoch QA scenarios.
//
// One suite serves both, because the client-side claim is the same in each: the SDK must embed the
// pair that is ACTIVE ON CHAIN in the extraData of the decryption permit it signs. What differs is
// only what the host half did first —
//
//   - case `epoch-rotation`         : the epoch advanced under the same context;
//   - case `context-switch`         : both the context and the epoch advanced;
//   - case `epoch-rotation-pending` : a rotation was requested but is being held Pending, so nothing
//                                     advanced and the SDK must not anticipate the epoch to come.
//
// The host half establishes and verifies the precondition, waiting until the transition is genuinely
// active on chain, then drives this suite.
//
// Scope: the REQUEST extraData only. The response extraData is neither verified nor exposed by the
// SDK — see test/qa-extradata-check.md for the evidence and the decision to defer that clause.
//
// Two independent staleness checks:
//
//   1. SDK vs chain. The suite reads getCurrentKmsContextAndEpoch itself, through ethers, so its
//      source of truth never passes through the SDK's cache. The SDK's own read is memoised for 15
//      minutes keyed by runtime uid (sdk/js-sdk/src/core/host-contracts/getCurrentKmsContextAndEpoch-p.ts),
//      so a stale client shows up here as a mismatch rather than being hidden.
//   2. Chain vs orchestrator. When driven by the CLI profile, KMS_QA_EXPECTED_CONTEXT_ID and
//      KMS_QA_EXPECTED_EPOCH_ID carry the pair the host observed after activation. If they are set,
//      the suite asserts its own read agrees — catching a rotation that landed between the two
//      reads. Unset, the suite still runs standalone and the check is skipped.
//
// The client is built fresh inside the suite (createInstance() mints a new runtime uid, so the
// 15-minute cache starts cold) because the epoch may have rotated moments ago.
import { expect } from 'chai';
import { ethers } from 'hardhat';

import { UserDecrypt } from '../../types';
import { createInstance, createInstances, protocolConfigAddress } from '../instance';
import { FhevmSdk } from '../sdk/fhevm-sdk/sdk';
import { Signers, getSigners, initSigners } from '../signers';

/** A permit signature plus a decryption round-trip; generous under amd64 emulation. */
const DECRYPT_TIMEOUT_MS = 3 * 60 * 1000;
const SETUP_TIMEOUT_MS = 3 * 60 * 1000;

/** Permit validity; matches the window the other SDK suites use. */
const DURATION_SECONDS = 10 * 24 * 3600;

/** extraData v2: 1 version byte + 32-byte context id + 32-byte epoch id = 65 bytes = 132 chars. */
const EXTRA_DATA_V2_LENGTH = 132;
const EXTRA_DATA_V2_VERSION = '02';

/** Renders a uint256 as the 64 hex chars one extraData word occupies. */
const hex32 = (value: bigint) => value.toString(16).padStart(64, '0');

/**
 * Decodes a v2 extraData payload.
 *
 * `createKmsExtraDataFromBytesHex` is internal to the SDK (`kmsExtraData-p.ts`, not re-exported by
 * any subpath), so the layout is decoded here instead. It is fixed and documented at that module:
 *
 *   0x 02 <contextId: 64 hex chars> <epochId: 64 hex chars>
 *      ^^ chars 2..4   chars 4..68            chars 68..132
 */
const decodeExtraDataV2 = (extraData: string) => ({
  version: extraData.slice(2, 4),
  contextId: BigInt(`0x${extraData.slice(4, 68)}`),
  epochId: BigInt(`0x${extraData.slice(68, 132)}`),
});

/** Reads an expected id injected by the CLI profile, or undefined when running standalone. */
const expectedFromEnv = (name: string): bigint | undefined => {
  const raw = process.env[name];
  if (raw === undefined || raw.trim() === '') return undefined;
  return BigInt(raw.trim());
};

describe('KMS context extraData', function () {
  let signers: Signers;
  let contract: UserDecrypt;
  let contractAddress: string;

  /** The active pair, read straight from the chain — never through the SDK. */
  let chainContextId: bigint;
  let chainEpochId: bigint;

  before(async function () {
    this.timeout(SETUP_TIMEOUT_MS);
    await initSigners(2);
    signers = await getSigners();

    // A guard, not a convenience: without it the suite would pass vacuously on a build whose
    // instance is not the @fhevm/sdk client whose behaviour is under test.
    const instances = await createInstances(signers);
    if (!(instances.alice instanceof FhevmSdk)) {
      this.skip();
    }

    // The constructor grants FHE.allow to msg.sender, so the deployer is the authorized reader.
    const factory = await ethers.getContractFactory('UserDecrypt');
    contract = await factory.connect(signers.alice).deploy();
    await contract.waitForDeployment();
    contractAddress = await contract.getAddress();

    const protocolConfig = new ethers.Contract(
      protocolConfigAddress,
      ['function getCurrentKmsContextAndEpoch() view returns (uint256 contextId, uint256 epochId)'],
      ethers.provider,
    );
    [chainContextId, chainEpochId] = await protocolConfig.getCurrentKmsContextAndEpoch();
  });

  it('test kms context extraData agrees with the orchestrator', async function () {
    // Cross-check 2: only meaningful when the CLI profile drove the rotation and injected what it
    // observed. Standalone, there is nothing to compare against and the suite skips this.
    const expectedContextId = expectedFromEnv('KMS_QA_EXPECTED_CONTEXT_ID');
    const expectedEpochId = expectedFromEnv('KMS_QA_EXPECTED_EPOCH_ID');
    if (expectedContextId === undefined || expectedEpochId === undefined) {
      this.skip();
    }

    expect(
      chainContextId,
      'the active context moved between the orchestrator read and this suite — a concurrent lifecycle operation',
    ).to.equal(expectedContextId);
    expect(
      chainEpochId,
      'the active epoch moved between the orchestrator read and this suite — a concurrent rotation',
    ).to.equal(expectedEpochId);
  });

  it('test kms context extraData carries the active epoch in the signed permit', async function () {
    this.timeout(DECRYPT_TIMEOUT_MS);

    // Fresh client: createInstance() mints a new runtime uid, and the SDK's 15-minute cache for
    // getCurrentKmsContextAndEpoch is keyed on that uid. A reused client could still be serving a
    // pre-rotation pair, which would make this assertion measure the cache instead of the protocol.
    const instance = await createInstance();
    // Narrow rather than cast: `FhevmSdk.create` is typed as returning the `SdkInstance` interface,
    // and only the FhevmSdk implementation exposes `rawClient`. A blind cast would fail later, and
    // confusingly, on a build wired to a different instance.
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

    const extraData = (permit.eip712.message as { extraData: string }).extraData;

    expect(extraData, 'the permit carries no extraData').to.be.a('string');
    expect(extraData.length, `expected a v2 extraData of ${EXTRA_DATA_V2_LENGTH} chars, got "${extraData}"`).to.equal(
      EXTRA_DATA_V2_LENGTH,
    );

    const decoded = decodeExtraDataV2(extraData);

    expect(decoded.version, 'extraData is not version v2 — the SDK is not on the context/epoch envelope').to.equal(
      EXTRA_DATA_V2_VERSION,
    );
    expect(decoded.contextId, 'the permit embeds a different context than the one active on chain').to.equal(
      chainContextId,
    );
    expect(
      decoded.epochId,
      'the permit embeds a different epoch than the one active on chain — the SDK did not follow the rotation ' +
        '(a stale 15-minute context cache is the usual cause)',
    ).to.equal(chainEpochId);

    // Byte-exact, so a future change to the word order or padding cannot pass the field checks.
    expect(extraData).to.equal(`0x${EXTRA_DATA_V2_VERSION}${hex32(chainContextId)}${hex32(chainEpochId)}`);

    // The scenarios' negative clause, made explicit rather than left implied by the equalities
    // above: the superseded ids must not appear. Injected by the profile, absent when standalone.
    const previousContextId = expectedFromEnv('KMS_QA_PREVIOUS_CONTEXT_ID');
    const previousEpochId = expectedFromEnv('KMS_QA_PREVIOUS_EPOCH_ID');
    if (previousEpochId !== undefined) {
      expect(previousEpochId, 'the superseded epoch equals the active one — the transition did not advance').to.not.equal(
        chainEpochId,
      );
      expect(decoded.epochId, 'the permit still carries the superseded epoch').to.not.equal(previousEpochId);
    }
    if (previousContextId !== undefined) {
      expect(
        previousContextId,
        'the superseded context equals the active one — the switch did not advance the context',
      ).to.not.equal(chainContextId);
      expect(decoded.contextId, 'the permit still carries the superseded context').to.not.equal(previousContextId);
    }

    // The mirror of the clause above, for the `epoch-rotation-pending` case: an epoch that exists on
    // chain but is NOT active yet, because its rotation is still Pending. Where KMS_QA_PREVIOUS_*
    // forbids an id the protocol has left behind, this forbids one it has not reached — the SDK must
    // not read ahead of activation. Injected by the profile, absent when standalone.
    const forbiddenEpochId = expectedFromEnv('KMS_QA_FORBIDDEN_EPOCH_ID');
    if (forbiddenEpochId !== undefined) {
      expect(
        forbiddenEpochId,
        'the pending epoch is already the active one — the rotation activated before this suite ran, so the ' +
          'scenario never observed the pending window',
      ).to.not.equal(chainEpochId);
      expect(
        decoded.epochId,
        'the permit already carries the pending epoch — the SDK anticipated an activation that has not happened',
      ).to.not.equal(forbiddenEpochId);
    }

    // The permit must not merely look right — it must work. This closes the scenario's
    // "the decryption must complete successfully" clause under the pair that is active.
    const handle = await contract.xUint64();
    const result = await client.decryptValue({
      contractAddress: contractAddress as `0x${string}`,
      transportKeyPair,
      signedPermit: permit,
      encryptedValue: handle as `0x${string}`,
    });
    expect(BigInt(result.value as bigint | number)).to.equal(18446744073709551600n);
  });
});
