// Container half of the KMS-context QA case `extradata-echo`.
//
// Its only job is to put three *successful* user decryptions on the wire, one per extraData version,
// so the host half can inspect what came back in each share. It asserts nothing about the echo
// itself: the response extraData is not observable from here — the SDK never exposes it, and even
// the raw unified client only sees the aggregated outcome, not the per-share rows. That evidence
// lives in the relayer's database, which the host reads.
//
// ## Why three versions, and why v2 alone would prove nothing
//
// The claim under test is that each share echoes the request's extraData **byte for byte**, rather
// than the KMS regenerating the field from its own view of the active context.
//
//   - v2 (`0x02 || C || E`) is the CONTROL. A regenerated value would be built from the active pair
//     and would therefore be identical to what was sent — echo and regeneration are
//     indistinguishable here. It proves the path works, nothing more.
//   - v1 (`0x01 || C`) DISCRIMINATES: 33 bytes, no epoch. Anything regenerated from the active pair
//     comes back as 65 bytes.
//   - v0 (`0x00`) discriminates hardest: one byte. Any regeneration at all is visible.
//
// All three are known-good values — each already has a passing test in
// `test/unifiedUserDecryption/unifiedUserDecryption.ts` (v0 at :552, v1 at :583, v2 at :618) proving
// it decrypts successfully. If one of them fails to decrypt here, that is a real regression in this
// suite's precondition, not a reason to weaken the case.
import { expect } from 'chai';
import { ethers } from 'hardhat';

import { UserDecrypt } from '../../types';
import {
  createInstances,
  protocolConfigAddress,
  relayerApiKey,
  relayerUrl,
  verifyingContractAddressDecryption,
} from '../instance';
import { FhevmSdk } from '../sdk/fhevm-sdk/sdk';
import type { UnifiedConfig, UnifiedDecryptRequest } from '../sdk/unified/unifiedUserDecrypt';
import {
  backdatedStartTimestamp,
  directHandle,
  requestUnifiedUserDecrypt,
} from '../sdk/unified/unifiedUserDecrypt';
import { Signers, getSigners, initSigners } from '../signers';

const SETUP_TIMEOUT_MS = 3 * 60 * 1000;
const CASE_TIMEOUT_MS = 8 * 60 * 1000;
const DURATION_SECONDS = 10 * 24 * 3600;

/** The plaintext the `UserDecrypt` contract initializes `xUint64` to. */
const EXPECTED_CLEAR = 18446744073709551600n;

const hex32 = (value: bigint) => value.toString(16).padStart(64, '0');

describe('KMS context extraData echo', function () {
  let signers: Signers;
  let cfg: UnifiedConfig;
  let contract: UserDecrypt;
  let contractAddress: string;

  let currentContextId: bigint;
  let currentEpochId: bigint;

  before(async function () {
    this.timeout(SETUP_TIMEOUT_MS);
    if (!protocolConfigAddress) {
      throw new Error('PROTOCOL_CONFIG_CONTRACT_ADDRESS is required');
    }
    await initSigners(2);
    signers = await getSigners();

    const instances = await createInstances(signers);
    if (!(instances.alice instanceof FhevmSdk)) {
      this.skip();
    }

    cfg = {
      relayerUrl,
      decryptionContractAddress: verifyingContractAddressDecryption,
      apiKey: relayerApiKey || undefined,
    };

    const factory = await ethers.getContractFactory('UserDecrypt');
    contract = await factory.connect(signers.alice).deploy();
    await contract.waitForDeployment();
    contractAddress = await contract.getAddress();

    const protocolConfig = new ethers.Contract(
      protocolConfigAddress,
      ['function getCurrentKmsContextAndEpoch() view returns (uint256 contextId, uint256 epochId)'],
      ethers.provider,
    );
    [currentContextId, currentEpochId] = await protocolConfig.getCurrentKmsContextAndEpoch();
  });

  it('test kms context extraData echo drives one successful decryption per extraData version', async function () {
    this.timeout(CASE_TIMEOUT_MS);

    const cases = [
      { version: 'v2', extraData: `0x02${hex32(currentContextId)}${hex32(currentEpochId)}` },
      { version: 'v1', extraData: `0x01${hex32(currentContextId)}` },
      { version: 'v0', extraData: '0x00' },
    ];

    for (const { version, extraData } of cases) {
      // A fresh re-encryption key per version: the relayer dedups on
      // (handles, userAddress, allowedContracts, publicKey, extraData). The extraData already
      // differs, but a fresh key removes any doubt that these are three real jobs rather than two
      // plus a cache hit — and the host half counts on three distinct request rows.
      const { publicKey } = await (await createInstances(signers)).alice.generateKeypair();
      const handle = await contract.xUint64();

      const request: UnifiedDecryptRequest = {
        handles: [directHandle(handle, contractAddress, signers.alice.address)],
        userAddress: signers.alice.address,
        allowedContracts: [],
        publicKey,
        startTimestamp: backdatedStartTimestamp(),
        durationSeconds: DURATION_SECONDS,
        extraData,
      };

      const { post, poll } = await requestUnifiedUserDecrypt(
        cfg,
        request,
        { kind: 'eoa', signer: signers.alice },
        { waitForTerminal: true },
      );

      // These are known-good values — each has its own passing test in
      // test/unifiedUserDecryption. A failure here is a broken precondition, and the host half's
      // echo assertion would otherwise have no shares to inspect.
      expect(post.httpStatus, `${version}: relayer rejected a known-good extraData. ${JSON.stringify(post.raw)}`).to.equal(
        202,
      );
      expect(poll?.status, `${version}: decryption did not succeed. ${JSON.stringify(poll?.raw)}`).to.equal('succeeded');
      console.log(`[kms-context-extradata-echo] ${version} succeeded with extraData=${extraData.slice(0, 14)}… (${(extraData.length - 2) / 2} bytes)`);
    }
  });
});
