import { randomBytes } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { expect } from 'chai';
import { ethers } from 'hardhat';

const fixturePath = '/tmp/manifest-healing-fixture.json';
const value = () => BigInt(`0x${randomBytes(6).toString('hex')}`);
const read = async () => JSON.parse(await readFile(fixturePath, 'utf8'));
const save = async (fixture: object) => writeFile(fixturePath, JSON.stringify(fixture));

(process.env.RUN_MANIFEST_LIFECYCLE === '1' ? describe : describe.skip)('manifest healing fixture', function () {
  this.timeout(420_000);

  it('seed matrix', async function () {
    const contract = await (await ethers.getContractFactory('ManifestHealingFixture')).deploy();
    await contract.waitForDeployment();
    const base = value();
    const receipt = await (await contract.seed(base)).wait();
    expect(receipt?.status).to.equal(1);
    for (const tx of [() => contract.derive(), () => contract.joinBranches(), () => contract.extendBranch()]) {
      expect((await (await tx()).wait())?.status).to.equal(1);
    }
    await save({
      chainId: Number((await ethers.provider.getNetwork()).chainId),
      contract: await contract.getAddress(), base: base.toString(),
      roots: await Promise.all(Array.from({ length: 7 }, (_, i) => contract.roots(i))),
      children: await Promise.all(Array.from({ length: 7 }, (_, i) => contract.children(i))),
      joined: await contract.joined(), tail: await contract.tail(),
      rootBlock: receipt!.blockNumber, rootBlockHash: receipt!.blockHash,
    });
  });

  it('submit consumers', async function () {
    const f = await read();
    const contract = await ethers.getContractAt('ManifestHealingFixture', f.contract);
    expect((await (await contract.consume(value())).wait())?.status).to.equal(1);
    await save({ ...f,
      consumers: await Promise.all(Array.from({ length: 7 }, (_, i) => contract.consumers(i))),
      queuedJoin: await contract.queuedJoin(), recovered: await contract.recovered(), independent: await contract.independent(),
    });
  });

  it('decrypt recovered', async function () {
    const f = await read();
    const { initSigners, getSigners } = await import('../signers');
    const { createInstances } = await import('../instance');
    await initSigners(2);
    const instances = await createInstances(await getSigners());
    const handles = [...f.consumers, f.recovered];
    const result = await instances.alice.publicDecrypt(handles);
    const base = BigInt(f.base);
    for (let i = 0; i < f.consumers.length; i++) expect(result.clearValues[f.consumers[i]]).to.equal(base + BigInt(i) + (i < 2 ? 110n : 100n));
    expect(result.clearValues[f.recovered]).to.equal(4n * base + 243n);
  });

  it('reuse healed chain', async function () {
    const f = await read();
    const contract = await ethers.getContractAt('ManifestHealingFixture', f.contract);
    expect((await (await contract.reuse()).wait())?.status).to.equal(1);
    await save({ ...f, reused: await contract.reused() });
  });

  it('decrypt reused', async function () {
    const f = await read();
    const { initSigners, getSigners } = await import('../signers');
    const { createInstances } = await import('../instance');
    await initSigners(2);
    const instances = await createInstances(await getSigners());
    const result = await instances.alice.publicDecrypt([f.reused]);
    expect(result.clearValues[f.reused]).to.equal(5n * BigInt(f.base) + 244n);
  });
});
