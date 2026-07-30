import { expect } from 'chai';
import { ethers } from 'hardhat';

import { UserDecrypt } from '../../types';
import { createInstances, protocolConfigAddress } from '../instance';
import {
  bitFlipPayload,
  corruptSignature,
  expectCorruptedShareDecryptToFail,
  expectCorruptedShareDecryptToSucceed,
  measureReturnedShares,
} from '../sdk/corruption/interceptShares';
import { Signers, getSigners, initSigners } from '../signers';
import { FhevmInstances } from '../types';

const X_UINT64_CLEARTEXT = 18446744073709551600n;

/** Smallest committee that is a real MPC cluster: 3t+1 with t=1. */
const THRESHOLD_MIN_SIGNERS = 4;

/**
 * Reads the signer count, not `getMpcThreshold()`: that returns 1 both on a
 * centralized stack and on a 4-party t=1 cluster, so it cannot tell them apart.
 *
 * - `t` is the MPC threshold, derived the way the SDK's wasm derives it.
 * - `reconstructMin` (`t+1`) is what decides success or failure.
 * - `collectThreshold` (`2t+1`) is what the relayer waits for before completing;
 *   being larger than `reconstructMin` is why the stack already tolerated `t` bad
 *   shares before spare shares existed.
 */
const readKmsTopology = async () => {
  const protocolConfig = new ethers.Contract(
    protocolConfigAddress,
    ['function getKmsSigners() view returns (address[])'],
    ethers.provider,
  );
  const signers: string[] = await protocolConfig.getKmsSigners();
  const signerCount = signers.length;
  const t = Math.floor((signerCount - 1) / 3);
  return { signerCount, t, reconstructMin: t + 1, collectThreshold: 2 * t + 1 };
};

/**
 * The relayer returns more shares than reconstruction needs, so a corrupted share
 * no longer fails a user decryption. Vacuous on a centralized stack — one signer
 * means no spare — so the suite skips itself there rather than passing falsely.
 *
 * Counts are derived here rather than inside the interceptor on purpose: a
 * corruptor that adapted to the response would also adapt to the relayer
 * regressing back to `2t+1` shares, and pass either way.
 */
describe('User decryption spare-share tolerance', function () {
  this.timeout(5 * 60 * 1000);

  let signers: Signers;
  let instances: FhevmInstances;
  let contract: UserDecrypt;
  let contractAddress: string;

  let t: number;
  let reconstructMin: number;
  let collectThreshold: number;
  /** Shares the relayer returned for an uncorrupted decrypt. */
  let returnedShares: number;

  const decryptXUint64 = async () => {
    const handle = await contract.xUint64();
    return instances.alice.userDecryptSingleHandle({
      handle,
      contractAddress,
      signer: signers.alice,
    });
  };

  before(async function () {
    const topology = await readKmsTopology();
    if (topology.signerCount < THRESHOLD_MIN_SIGNERS) {
      // eslint-disable-next-line no-console
      console.log(
        `[spare-shares] skipping: ${topology.signerCount} KMS signer(s), need >= ${THRESHOLD_MIN_SIGNERS} for a spare`,
      );
      this.skip();
      return;
    }
    ({ t, reconstructMin, collectThreshold } = topology);

    await initSigners(2);
    signers = await getSigners();
    instances = await createInstances(signers);
    const contractFactory = await ethers.getContractFactory('UserDecrypt');
    contract = await contractFactory.connect(signers.alice).deploy();
    await contract.waitForDeployment();
    contractAddress = await contract.getAddress();

    const measured = await measureReturnedShares(decryptXUint64);
    expect(measured.value).to.equal(X_UINT64_CLEARTEXT);
    returnedShares = measured.shareCount;

    // eslint-disable-next-line no-console
    console.log(
      `[spare-shares] signers=${topology.signerCount} t=${t} needs=${reconstructMin} ` +
        `collects=${collectThreshold} returned=${returnedShares} faultBudget=${t}`,
    );

    // A precondition, not a case: without the relayer's spare-share wait the
    // response carries exactly `collectThreshold`. Failing here names the cause
    // instead of silently shifting every count below.
    expect(
      returnedShares,
      `Relayer returned ${returnedShares} shares; expected more than the ${collectThreshold} it collects before ` +
        `completing. Is user_decrypt_additional_shares > 0, and did the spare arrive inside the wait window?`,
    ).to.be.greaterThan(collectThreshold);
  });

  /** Re-checked per case so a late spare is diagnosed, not silently misread. */
  const expectSameShareCount = (observed: number) => {
    expect(
      observed,
      `Relayer returned ${observed} shares this run but ${returnedShares} when measured; the derived ` +
        `corruption counts no longer describe this response`,
    ).to.equal(returnedShares);
  };

  /**
   * Signature corruption keeps a share parseable and fails only its verification,
   * so the reconstruction can discard it. The budget for discards is `t` — set by
   * the scheme, not by how many shares arrived — because more than `t` faulty
   * parties breaks the assumption the remaining shares are trustworthy at all.
   * Past the budget the failure mode splits: while at least `reconstructMin` good
   * shares remain it is a fault-budget refusal, below that there is simply too
   * little left to interpolate.
   */
  it('survives up to t corrupted signatures and no more', async function () {
    const outcomes: string[] = [];
    for (let corrupted = 1; corrupted <= returnedShares; corrupted += 1) {
      const label = `spare-shares/sig-${corrupted}`;
      if (corrupted <= t) {
        const { value, shareCount } = await expectCorruptedShareDecryptToSucceed(
          label,
          corruptSignature,
          decryptXUint64,
          { count: corrupted, from: 'tail' },
        );
        expectSameShareCount(shareCount);
        expect(value, `${corrupted} corrupted signature(s) should still reconstruct`).to.equal(X_UINT64_CLEARTEXT);
        outcomes.push(`${corrupted}: reconstructed`);
        continue;
      }
      // Matched on the invariant only; the file:line prefix moves with KMS versions.
      const expected =
        returnedShares - corrupted >= reconstructMin
          ? /num_bots \(\d+\) > threshold \(\d+\)/
          : /Not enough correct responses/;
      const { shareCount, message } = await expectCorruptedShareDecryptToFail(label, corruptSignature, decryptXUint64, {
        count: corrupted,
        from: 'tail',
      });
      expectSameShareCount(shareCount);
      expect(message, `${corrupted} corrupted signature(s) failed for an unexpected reason`).to.match(expected);
      outcomes.push(`${corrupted}: refused (${returnedShares - corrupted} good left)`);
    }
    // eslint-disable-next-line no-console
    console.log(`[spare-shares] signatures — ${outcomes.join(' | ')}`);
  });

  /**
   * Payload corruption is unsurvivable at any count: the payload is signcrypted,
   * so a flipped byte decrypts to garbage and deserialization dies on a nonsense
   * length. The whole response is deserialized before any share is weighed, so a
   * single bad payload aborts the request no matter how many spares arrived.
   * Flip the expectation once the SDK drops undecodable shares (#1738).
   */
  it('survives no corrupted payloads at all, however many spares arrive', async function () {
    const outcomes: string[] = [];
    for (let corrupted = 1; corrupted <= returnedShares; corrupted += 1) {
      const { shareCount, message } = await expectCorruptedShareDecryptToFail(
        `spare-shares/payload-${corrupted}`,
        bitFlipPayload,
        decryptXUint64,
        { count: corrupted, from: 'tail' },
      );
      expectSameShareCount(shareCount);
      expect(message, `${corrupted} corrupted payload(s) failed for an unexpected reason`).to.match(
        /response parsing failed/,
      );
      outcomes.push(`${corrupted}: parse aborted`);
    }
    // eslint-disable-next-line no-console
    console.log(`[spare-shares] payloads — ${outcomes.join(' | ')}`);
  });
});
