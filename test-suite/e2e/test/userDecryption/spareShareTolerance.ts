import { expect } from 'chai';
import { ethers } from 'hardhat';

import { UserDecrypt } from '../../types';
import { createInstances, protocolConfigAddress } from '../instance';
import {
  ShareCorruptor,
  corruptPayloadBody,
  corruptPayloadFraming,
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
 * - `collectThreshold` (`2t+1`) is what decides success or failure: the wasm charges
 *   every share missing from a full `3t+1` committee against a fault budget of `t`
 *   (`num_bots = 3t+1 - accepted`), so it needs `2t+1` *valid* shares, and tolerance
 *   is `returned - (2t+1)`. At exactly `2t+1` returned, tolerance is zero.
 * - `reconstructMin` (`t+1`) is only the floor below which validation gives up
 *   early with a different error; it is not the success criterion.
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
 * corruptor that sized itself from the response would also resize when a KMS
 * party stops answering, and report success against a cluster that has no spare
 * left at all. Policy lives in the test; the interceptor only mutates and reports.
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
      `[spare-shares] signers=${topology.signerCount} t=${t} quorum=${collectThreshold} ` +
        `floor=${reconstructMin} returned=${returnedShares} ` +
        `tolerated=${returnedShares - collectThreshold} (ceiling ${t})`,
    );

    // Without a spare there is nothing to corrupt and every case below is vacuous.
    // Skipped rather than failed: the quorum alone reconstructs fine, so this is an
    // unsuitable cluster, not a defect. Usually a stopped party — which
    // spare-share-tolerance-kms-down covers on purpose.
    if (returnedShares <= collectThreshold) {
      // eslint-disable-next-line no-console
      console.log(
        `[spare-shares] skipping: got ${returnedShares} shares, need more than the ${collectThreshold} quorum ` +
          `so one can be corrupted and dropped. Is a KMS party down?`,
      );
      this.skip();
      return;
    }
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
   * Sweeps 1..returnedShares for a *droppable* fault — one the wasm rejects at
   * validation rather than choking on while parsing. Tolerance is `returned - (2t+1)`,
   * which reaches the scheme's ceiling `t` only when every one of the `3t+1` shares
   * comes back. Past the budget the failure mode splits: while at least
   * `reconstructMin` good shares remain it is a fault-budget refusal, below that
   * there is simply too little left to interpolate.
   *
   * Shared by the two droppable faults so their equivalence is structural: a bad
   * signature and a corrupted payload body must behave identically.
   */
  const sweepDroppableCorruption = async (kind: string, corrupt: ShareCorruptor) => {
    const tolerated = returnedShares - collectThreshold;
    const outcomes: string[] = [];
    for (let corrupted = 1; corrupted <= returnedShares; corrupted += 1) {
      const label = `spare-shares/${kind}-${corrupted}`;
      if (corrupted <= tolerated) {
        const { value, shareCount } = await expectCorruptedShareDecryptToSucceed(label, corrupt, decryptXUint64, {
          count: corrupted,
          from: 'tail',
        });
        expectSameShareCount(shareCount);
        expect(value, `${corrupted} corrupted ${kind}(s) should still reconstruct`).to.equal(X_UINT64_CLEARTEXT);
        outcomes.push(`${corrupted}: reconstructed`);
        continue;
      }
      // Matched on the invariant only; the file:line prefix moves with KMS versions.
      const expected =
        returnedShares - corrupted >= reconstructMin
          ? /num_bots \(\d+\) > threshold \(\d+\)/
          : /Not enough correct responses/;
      const { shareCount, message } = await expectCorruptedShareDecryptToFail(label, corrupt, decryptXUint64, {
        count: corrupted,
        from: 'tail',
      });
      expectSameShareCount(shareCount);
      expect(message, `${corrupted} corrupted ${kind}(s) failed for an unexpected reason`).to.match(expected);
      outcomes.push(`${corrupted}: refused (${returnedShares - corrupted} good left)`);
    }
    // eslint-disable-next-line no-console
    console.log(`[spare-shares] ${kind} — tolerated=${tolerated} — ${outcomes.join(' | ')}`);
  };

  /**
   * Signature corruption keeps a share parseable and fails only its verification,
   * so reconstruction can discard it.
   */
  it('corrupted signature bytes fail validation and are dropped, up to the spare count', async function () {
    await sweepDroppableCorruption('sig', corruptSignature);
  });

  /**
   * A payload byte flipped past every length prefix still deserializes, so unlike
   * `corruptPayloadFraming` it reaches EIP-712 validation — where it fails,
   * because the signature covers the whole serialized payload. It must therefore
   * be indistinguishable from a corrupted signature. Proves that payload
   * corruption is not inherently fatal: only *framing* corruption is.
   */
  it('corrupted payload body bytes fail signature validation, exactly like a corrupted signature', async function () {
    await sweepDroppableCorruption('body', corruptPayloadBody);
  });

  /**
   * The counterfactual the relayer can no longer produce: `user_decrypt_additional_shares`
   * only disables the optimistic wait, while the un-capping (dropping the share query's
   * `LIMIT`) is unconditional, so no config makes the relayer return `2t+1` again.
   * Truncating the response client-side is indistinguishable to the wasm, and isolates
   * the share count as the only variable: same stack, same config, same corruptor.
   *
   * The truncate-only case comes first on purpose — without it, a failure below could
   * be blamed on truncation rather than on the missing spare.
   */
  it('needs the spare: at 2t+1 shares the same single corruption is fatal', async function () {
    const { value, shareCount } = await expectCorruptedShareDecryptToSucceed(
      'spare-shares/truncated-clean',
      corruptSignature,
      decryptXUint64,
      { count: 0, from: 'tail', keep: collectThreshold },
    );
    expect(shareCount, 'truncation did not reach the SDK').to.equal(collectThreshold);
    expect(value, `${collectThreshold} uncorrupted shares should reconstruct`).to.equal(X_UINT64_CLEARTEXT);

    const { shareCount: corruptedCount, message } = await expectCorruptedShareDecryptToFail(
      'spare-shares/truncated-sig-1',
      corruptSignature,
      decryptXUint64,
      { count: 1, from: 'tail', keep: collectThreshold },
    );
    expect(corruptedCount, 'truncation did not reach the SDK').to.equal(collectThreshold);
    // The budget is spent on the shares that never arrived: num_bots = 3t+1 - (2t+1 - 1) = t+1.
    expect(message, 'a corrupted share at exactly the quorum should exhaust the fault budget').to.match(
      /num_bots \(\d+\) > threshold \(\d+\)/,
    );
    // eslint-disable-next-line no-console
    console.log(
      `[spare-shares] counterfactual — ${collectThreshold} shares: clean reconstructs, ` +
        `1 corrupted refused; at ${returnedShares} the same corruption survives`,
    );
  });

  /**
   * TODO(fhevm-internal#1738): this case pins CURRENT behaviour, not desired
   * behaviour — flip the expectation when the fix below lands.
   *
   * A framing fault (see `corruptPayloadFraming`) is fatal at any count, and
   * spares cannot help. That is not a threshold property: `js_to_resp` decodes the
   * whole response vector with `?` per response, so one undecodable payload
   * discards every share — including the good ones — before validation runs.
   *
   * It should behave like any other single bad share: skip that response, keep the
   * rest, reconstruct. Nothing is lost by skipping, since a response that fails to
   * deserialize could never have passed validation anyway. The inconsistency is the
   * argument: a bad *signature* is already skipped with a warning, so the same
   * principle simply is not applied to a bad *encoding*.
   *
   * The fix belongs in the KMS wasm (`js_to_resp`), not here or in the SDK — every
   * exclusion decision lives behind that boundary. When it lands, framing corruption
   * becomes droppable and this case should move to `sweepDroppableCorruption`
   * alongside signatures and payload bodies.
   */
  it('corrupted payload framing bytes break parsing and abort every share (known defect)', async function () {
    const outcomes: string[] = [];
    for (let corrupted = 1; corrupted <= returnedShares; corrupted += 1) {
      const { shareCount, message } = await expectCorruptedShareDecryptToFail(
        `spare-shares/framing-${corrupted}`,
        corruptPayloadFraming,
        decryptXUint64,
        { count: corrupted, from: 'tail' },
      );
      expectSameShareCount(shareCount);
      expect(message, `${corrupted} malformed payload(s) failed for an unexpected reason`).to.match(
        /response parsing failed/,
      );
      outcomes.push(`${corrupted}: parse aborted`);
    }
    // eslint-disable-next-line no-console
    console.log(`[spare-shares] framing (known defect, fhevm-internal#1738) — ${outcomes.join(' | ')}`);
  });
});
