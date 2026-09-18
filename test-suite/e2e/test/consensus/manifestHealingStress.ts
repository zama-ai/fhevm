import { randomBytes } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { expect } from 'chai';
import { ethers } from 'hardhat';

const fixturePath = '/tmp/manifest-healing-stress-fixture.json';
const mask = (1n << 64n) - 1n;
const value = () => BigInt(`0x${randomBytes(6).toString('hex')}`);
const read = async () => JSON.parse(await readFile(fixturePath, 'utf8'));
const save = async (f: object) => writeFile(fixturePath, JSON.stringify(f));

(process.env.RUN_MANIFEST_LIFECYCLE === '1' ? describe : describe.skip)('manifest healing stress fixture', function () {
  this.timeout(420_000);

  it('seed', async function () {
    const contract = await (await ethers.getContractFactory('ManifestHealingStressFixture')).deploy();
    await contract.waitForDeployment();
    const base = value();
    expect((await (await contract.seed(base)).wait())?.status).to.equal(1);
    const heads = await Promise.all([0, 1, 2, 3].map(i => contract.heads(i)));
    const expected = [0n, 1n, 2n, 3n].map(i => (base + i).toString());
    await save({ chainId: Number((await ethers.provider.getNetwork()).chainId), contract: await contract.getAddress(),
      roots: heads, heads, expected, rootValues: expected, handles: heads, rounds: 0 });
  });

  it('advance', async function () {
    const f = await read();
    const contract = await ethers.getContractAt('ManifestHealingStressFixture', f.contract);
    const independentValue = value();
    expect((await (await contract.advance(independentValue)).wait())?.status).to.equal(1);
    const previous = f.expected.map((v: string) => BigInt(v));
    f.expected = previous.map((v: bigint, i: number) => ((v + previous[(i + 1) % 4] + BigInt(f.rootValues[i])) & mask).toString());
    f.heads = await Promise.all([0, 1, 2, 3].map(i => contract.heads(i)));
    const mixed = await Promise.all([0, 1, 2, 3].map(i => contract.mixed(i)));
    f.independent = await contract.independent();
    f.independentValue = independentValue.toString();
    f.handles = [...new Set([...f.handles, ...mixed, ...f.heads, f.independent])];
    f.rounds++;
    await save(f);
  });

  it('pin roots', async function () {
    const f = await read();
    const contract = await ethers.getContractAt('ManifestHealingStressFixture', f.contract);
    expect((await (await contract.pinRoots()).wait())?.status).to.equal(1);
    await save({ ...f, roots: f.heads, rootValues: f.expected });
  });

  it('decrypt', async function () {
    const f = await read();
    const { initSigners, getSigners } = await import('../signers');
    const { createInstances } = await import('../instance');
    await initSigners(2);
    const instances = await createInstances(await getSigners());
    const handles = [...f.heads, f.independent];
    const result = await instances.alice.publicDecrypt(handles);
    for (let i = 0; i < 4; i++) expect(result.clearValues[f.heads[i]]).to.equal(BigInt(f.expected[i]));
    expect(result.clearValues[f.independent]).to.equal(BigInt(f.independentValue));
  });
});
