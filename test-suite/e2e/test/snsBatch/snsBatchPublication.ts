import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstance } from '../instance';
import { getSigners, initSigners } from '../signers';

// The CLI stages work on an isolated Anvil stack and checks the same operator's
// PBS/digest rows before asking the Gateway and KMS for the public results.
const HOST_TRANSACTIONS = 40;
const EXPECTED_HANDLES = HOST_TRANSACTIONS * 16;

describe('SNS batch publication', function () {
  this.timeout(420_000);
  before(async function () {
    if (!process.env.SNS_BATCH_PHASE) this.skip();
    expect(await ethers.provider.send('web3_clientVersion', [])).to.match(/anvil/i);
    expect((await ethers.provider.getNetwork()).chainId).to.equal(12345n);
  });

  it('issues the public handle cohort', async function () {
    expect(process.env.SNS_BATCH_PHASE).to.equal('issue');
    await initSigners(1);
    const { alice } = await getSigners();
    const contract = await (await ethers.getContractFactory('SnsBatchPublication', alice)).deploy();
    await contract.waitForDeployment();
    for (let i = 0; i < HOST_TRANSACTIONS; i++) {
      await (await contract.issue()).wait();
    }
    const handles = (await contract.handles()) as `0x${string}`[];
    expect(new Set(handles).size).to.equal(EXPECTED_HANDLES);
    console.log('SNS_BATCH_COHORT=' + JSON.stringify({ address: await contract.getAddress(), handles }));
  });

  it('verifies the published cohort', async function () {
    expect(process.env.SNS_BATCH_PHASE).to.equal('verify');
    const contract = await ethers.getContractAt('SnsBatchPublication', process.env.SNS_BATCH_CONTRACT!);
    const handles = (await contract.handles()) as `0x${string}`[];
    expect(new Set(handles).size).to.equal(EXPECTED_HANDLES);

    const gateway = new ethers.JsonRpcProvider(process.env.GATEWAY_RPC_URL);
    try {
      const commits = new ethers.Contract(
        process.env.CIPHERTEXT_COMMITS_ADDRESS!,
        ['function isCiphertextMaterialAdded(bytes32) view returns (bool)'],
        gateway,
      );
      const deadline = Date.now() + 300_000;
      let pending = handles;
      while (pending.length && Date.now() < deadline) {
        const missing: typeof handles = [];
        for (let i = 0; i < pending.length; i += 32) {
          const group = pending.slice(i, i + 32);
          const added = await Promise.all(group.map((handle) => commits.isCiphertextMaterialAdded(handle)));
          missing.push(...group.filter((_, index) => !added[index]));
        }
        pending = missing;
        if (pending.length) await new Promise((resolve) => setTimeout(resolve, 1500));
      }
      expect(pending, 'handles without Gateway material').to.be.empty;
    } finally {
      gateway.destroy();
    }

    const sdk = await createInstance();
    const indices = [0, Math.floor(handles.length / 2), handles.length - 1];
    const { clearValues } = await sdk.publicDecrypt(indices.map((i) => handles[i]));
    for (const i of indices) expect(clearValues[handles[i]]).to.equal(BigInt(i % 256));
  });
});
