import { assert } from 'chai';
import { existsSync, writeFileSync } from 'fs';
import { ethers } from 'hardhat';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

// Host harness pauses the sender before this test and resumes it after the marker.
(process.env.READINESS_CAMPAIGN === '1' ? describe : describe.skip)('readiness campaign', function () {
  this.timeout(420000);
  it('surfaces readiness expiry then decrypts identical ciphertext after sender recovery', async function () {
    await initSigners(2);
    const signers = await getSigners();
    const instances = await createInstances(signers);
    const contract = await (await ethers.getContractFactory('HTTPPublicDecrypt')).connect(signers.alice).deploy();
    await contract.waitForDeployment();
    const handle = await contract.xBool();
    const started = Date.now();
    let failure: any;
    try {
      await instances.alice.publicDecrypt([handle]);
    } catch (error) {
      failure = error;
    }
    assert.exists(failure, 'Registration held through readiness window must fail');
    assert.include(String(failure), 'readiness_check_timed_out');
    console.log(JSON.stringify({ stage: 'terminal', elapsedMs: Date.now() - started }));
    writeFileSync('/tmp/readiness-campaign-resume', 'resume');
    const deadline = Date.now() + 90000;
    while (!existsSync('/tmp/readiness-campaign-resumed')) {
      assert.isBelow(Date.now(), deadline, 'Host did not resume sender');
      await new Promise((resolve) => setTimeout(resolve, 250));
    }
    const recovered = await instances.alice.publicDecrypt([handle]);
    assert.deepEqual(recovered.clearValues, { [handle]: true });
    console.log(JSON.stringify({ stage: 'recovered', elapsedMs: Date.now() - started }));
  });
});
