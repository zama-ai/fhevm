import { expect } from 'chai';
import { ethers } from 'hardhat';

import { UserDecrypt } from '../../types';
import { createInstances, protocolConfigAddress, relayerUrl } from '../instance';
import {
  corruptSignature,
  expectCorruptedShareDecryptToFail,
  measureReturnedShares,
} from '../sdk/corruption/interceptShares';
import { Signers, getSigners, initSigners } from '../signers';
import { FhevmInstances } from '../types';

const X_UINT64_CLEARTEXT = 18446744073709551600n;

/** Smallest committee that is a real MPC cluster: 3t+1 with t=1. */
const THRESHOLD_MIN_SIGNERS = 4;

/** Printed in the skip message so it is obvious how to make this suite run. */
const DEGRADE_HINT = 'docker stop kms-core-4';

const WAIT_PARAM = 'user_decrypt_additional_shares_timeout_secs';

/**
 * Wait window used while probing it, in seconds. Deliberately far above a normal
 * decrypt (~8s here) so the hold dominates the measurement: asserting against the
 * 5s default would pass even with the wait removed. Well under the relayer's 30m
 * `user_decrypt_timeout`.
 */
const WAIT_PROBE_SECS = 30;

/**
 * What a crashed KMS party costs, which is more than it looks.
 *
 * A `3t+1` committee nominally tolerates `t` faults, but the wasm charges every
 * share missing from a full committee against that same budget
 * (`num_bots = 3t+1 - accepted`). A crashed party and a corrupted share therefore
 * cost the same. With one party down only `2t+1` shares exist, the budget is spent
 * before validation starts, and tolerance for corruption is zero.
 *
 * So the stack survives EITHER one crashed party OR one corrupted share, never
 * both — `spareShareTolerance` measures the healthy best case, this measures what
 * survives a single crash. Needs a deliberately degraded cluster, hence its own
 * profile; it skips, loudly, when every party is answering.
 */
describe('User decryption with a KMS party down', function () {
  this.timeout(5 * 60 * 1000);

  let signers: Signers;
  let instances: FhevmInstances;
  let contract: UserDecrypt;
  let contractAddress: string;

  let quorum: number;
  /** Configured wait window read from the relayer; `undefined` if admin is off. */
  let configuredWaitSecs: number | undefined;

  const adminConfigUrl = `${new URL(relayerUrl).origin}/admin/config`;

  /** Read rather than assumed, so nothing here hardcodes the configured value. */
  const readWaitWindowSecs = async (): Promise<number | undefined> => {
    try {
      const response = await fetch(adminConfigUrl);
      if (!response.ok) return undefined;
      const body = (await response.json()) as { values?: Record<string, unknown> };
      const value = body.values?.UserDecryptAdditionalSharesTimeoutSecs;
      return typeof value === 'number' ? value : undefined;
    } catch {
      return undefined;
    }
  };

  const setWaitWindowSecs = async (secs: number) => {
    const response = await fetch(adminConfigUrl, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ param: WAIT_PARAM, value: secs }),
    });
    if (!response.ok) throw new Error(`Failed to set ${WAIT_PARAM}=${secs}: HTTP ${response.status}`);
  };

  const decryptXUint64 = async () => {
    const handle = await contract.xUint64();
    return instances.alice.userDecryptSingleHandle({
      handle,
      contractAddress,
      signer: signers.alice,
    });
  };

  before(async function () {
    const protocolConfig = new ethers.Contract(
      protocolConfigAddress,
      ['function getKmsSigners() view returns (address[])'],
      ethers.provider,
    );
    const registered: string[] = await protocolConfig.getKmsSigners();
    if (registered.length < THRESHOLD_MIN_SIGNERS) {
      // eslint-disable-next-line no-console
      console.log(`[degraded-kms] skipping: ${registered.length} KMS signer(s), need >= ${THRESHOLD_MIN_SIGNERS}`);
      this.skip();
      return;
    }
    // The roster still lists every party when one is stopped; only the shares that
    // actually arrive drop, so the quorum comes from the roster.
    quorum = 2 * Math.floor((registered.length - 1) / 3) + 1;

    await initSigners(2);
    signers = await getSigners();
    instances = await createInstances(signers);
    const factory = await ethers.getContractFactory('UserDecrypt');
    contract = await factory.connect(signers.alice).deploy();
    await contract.waitForDeployment();
    contractAddress = await contract.getAddress();

    const measured = await measureReturnedShares(decryptXUint64);
    // Needs shares down to exactly the quorum, which takes `committee - quorum`
    // parties stopped: one at 4 parties, four at 13.
    const spares = measured.shareCount - quorum;
    if (spares > 0) {
      // Loud, not silent: this suite only means anything against a degraded cluster.
      // eslint-disable-next-line no-console
      console.log(
        `[degraded-kms] skipping: ${measured.shareCount} shares arrived against a quorum of ${quorum}, so ` +
          `${spares} spare(s) remain. Stop ${spares} more KMS core(s) (e.g. \`${DEGRADE_HINT}\`) and rerun, ` +
          `restarting them afterwards.`,
      );
      this.skip();
      return;
    }

    expect(
      measured.shareCount,
      `Relayer returned ${measured.shareCount} shares, fewer than the ${quorum} needed to reconstruct; ` +
        `more than one KMS party is unavailable`,
    ).to.equal(quorum);
    expect(measured.value, 'a degraded cluster should still decrypt when no share is corrupted').to.equal(
      X_UINT64_CLEARTEXT,
    );

    configuredWaitSecs = await readWaitWindowSecs();
    // eslint-disable-next-line no-console
    console.log(`[degraded-kms] quorum=${quorum} returned=${measured.shareCount} tolerated=0`);
  });

  it('still decrypts on exactly the quorum while every share is intact', async function () {
    const { value, shareCount } = await measureReturnedShares(decryptXUint64);
    expect(shareCount, 'share count changed mid-suite; did the stopped party come back?').to.equal(quorum);
    expect(value).to.equal(X_UINT64_CLEARTEXT);
  });

  /**
   * The missing share never arrives, so the relayer can only release once the wait
   * window elapses — which makes the window observable as a floor on response time.
   * Lower bound only: the upper bound is unbounded, so pinning one buys flakiness.
   *
   * Without this case the suite would still pass with the wait window deleted, as
   * the relayer would simply return the quorum immediately.
   */
  it('holds the request for the wait window before giving up on the missing share', async function () {
    if (configuredWaitSecs === undefined) {
      // eslint-disable-next-line no-console
      console.log(`[degraded-kms] skipping wait-window probe: ${adminConfigUrl} unavailable`);
      this.skip();
      return;
    }

    await setWaitWindowSecs(WAIT_PROBE_SECS);
    const startedAt = Date.now();
    let elapsedMs: number;
    try {
      await measureReturnedShares(decryptXUint64);
    } finally {
      elapsedMs = Date.now() - startedAt;
      await setWaitWindowSecs(configuredWaitSecs);
    }

    // eslint-disable-next-line no-console
    console.log(`[degraded-kms] window ${WAIT_PROBE_SECS}s — released after ${(elapsedMs / 1000).toFixed(1)}s`);
    expect(
      elapsedMs,
      `Released after ${(elapsedMs / 1000).toFixed(1)}s with the window set to ${WAIT_PROBE_SECS}s. A share is ` +
        `missing, so it should have held for the full window — the wait window did not engage.`,
    ).to.be.at.least(WAIT_PROBE_SECS * 1000 - 2_000); // whole-second comparison upstream
  });

  it('has no tolerance left: one corrupted signature is now fatal', async function () {
    const { shareCount, message } = await expectCorruptedShareDecryptToFail(
      'degraded-kms/sig-1',
      corruptSignature,
      decryptXUint64,
      { count: 1, from: 'tail' },
    );
    expect(shareCount, 'share count changed mid-suite; did the stopped party come back?').to.equal(quorum);
    // The same error a healthy cluster gives for TWO corrupted shares: the crashed
    // party and the corrupted share come out of one budget.
    expect(message, 'expected the fault budget to be exhausted').to.match(/num_bots \(\d+\) > threshold \(\d+\)/);
  });
});
