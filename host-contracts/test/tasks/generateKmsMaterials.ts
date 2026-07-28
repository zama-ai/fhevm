import { expect } from 'chai';
import { ethers, run } from 'hardhat';

import { CRS_COUNTER_BASE, KEY_COUNTER_BASE, PREP_KEYGEN_COUNTER_BASE } from '../../tasks/utils/kmsGenerationConstants';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import type { KMSGeneration } from '../../types';
import { deployFreshKMSGenerationProxy } from './taskHelpers';

describe('task:abortKeygen / task:abortCrsgen', function () {
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);
  let kmsGeneration: KMSGeneration;
  let previousAddressEnv: string | undefined;

  beforeEach(async function () {
    kmsGeneration = await deployFreshKMSGenerationProxy(deployer);
    // The tasks resolve the proxy through KMS_GENERATION_CONTRACT_ADDRESS, not a CLI flag.
    previousAddressEnv = process.env.KMS_GENERATION_CONTRACT_ADDRESS;
    process.env.KMS_GENERATION_CONTRACT_ADDRESS = await kmsGeneration.getAddress();
  });

  afterEach(function () {
    if (previousAddressEnv === undefined) {
      delete process.env.KMS_GENERATION_CONTRACT_ADDRESS;
    } else {
      process.env.KMS_GENERATION_CONTRACT_ADDRESS = previousAddressEnv;
    }
  });

  it('moves a keygen from pending to aborted (not completed)', async function () {
    await kmsGeneration.keygen(0);
    const prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1n;
    const keyId = KEY_COUNTER_BASE + 1n;

    // Pending: requested but no consensus yet.
    expect(await kmsGeneration.isRequestDone(keyId)).to.equal(false);

    await run('task:abortKeygen', { prepKeygenId: prepKeygenId.toString() });

    // Aborted: both prep and paired key are marked done...
    expect(await kmsGeneration.isRequestDone(prepKeygenId)).to.equal(true);
    expect(await kmsGeneration.isRequestDone(keyId)).to.equal(true);
    // ...but with a zero consensus digest, so the read side reports "aborted", not "valid".
    await expect(kmsGeneration.getKeyParamsType(keyId))
      .to.be.revertedWithCustomError(kmsGeneration, 'KeyAborted')
      .withArgs(keyId);
    await expect(kmsGeneration.getKeyMaterials(keyId))
      .to.be.revertedWithCustomError(kmsGeneration, 'KeyAborted')
      .withArgs(keyId);
    // A zero consensus digest means no consensus tx senders are recorded.
    expect(await kmsGeneration.getConsensusTxSenders(prepKeygenId)).to.deep.equal([]);
    expect(await kmsGeneration.getConsensusTxSenders(keyId)).to.deep.equal([]);
  });

  it('moves a CRS generation from pending to aborted (not completed)', async function () {
    await kmsGeneration.crsgenRequest(4096, 0);
    const crsId = CRS_COUNTER_BASE + 1n;

    // Pending: requested but no consensus yet.
    expect(await kmsGeneration.isRequestDone(crsId)).to.equal(false);

    await run('task:abortCrsgen', { crsId: crsId.toString() });

    // Aborted: marked done...
    expect(await kmsGeneration.isRequestDone(crsId)).to.equal(true);
    // ...but with a zero consensus digest, so the read side reports "aborted", not "valid".
    await expect(kmsGeneration.getCrsParamsType(crsId))
      .to.be.revertedWithCustomError(kmsGeneration, 'CrsAborted')
      .withArgs(crsId);
    await expect(kmsGeneration.getCrsMaterials(crsId))
      .to.be.revertedWithCustomError(kmsGeneration, 'CrsAborted')
      .withArgs(crsId);
    expect(await kmsGeneration.getConsensusTxSenders(crsId)).to.deep.equal([]);
  });
});
