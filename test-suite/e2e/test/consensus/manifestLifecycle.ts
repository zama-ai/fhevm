import { randomBytes } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { expect } from 'chai';
import { ethers } from 'hardhat';

const enabled = process.env.RUN_MANIFEST_LIFECYCLE === '1';
const fixturePath = '/tmp/manifest-lifecycle-fixture.json';
const value = () => BigInt(`0x${randomBytes(6).toString('hex')}`);

(enabled ? describe : describe.skip)('manifest lifecycle fixture', function () {
  this.timeout(180_000);

  it('seed graph', async function () {
    const factory = await ethers.getContractFactory('ManifestLifecycleFixture');
    const contract = await factory.deploy();
    await contract.waitForDeployment();
    const rootReceipt = await (await contract.seed(value())).wait();
    const childReceipt = await (await contract.derive()).wait();
    expect(rootReceipt?.status).to.equal(1);
    expect(childReceipt?.status).to.equal(1);
    const fixture = {
      chainId: Number((await ethers.provider.getNetwork()).chainId),
      contract: await contract.getAddress(),
      root: await contract.root(),
      child: await contract.child(),
      rootBlock: rootReceipt!.blockNumber,
      rootBlockHash: rootReceipt!.blockHash,
    };
    await writeFile(fixturePath, JSON.stringify(fixture));
  });

  it('submit consumers', async function () {
    const fixture = JSON.parse(await readFile(fixturePath, 'utf8'));
    const contract = await ethers.getContractAt('ManifestLifecycleFixture', fixture.contract);
    const receipt = await (await contract.consume(value())).wait();
    expect(receipt?.status).to.equal(1);
    await writeFile(fixturePath, JSON.stringify({
      ...fixture,
      blocked: await contract.blocked(),
      independent: await contract.independent(),
    }));
  });
});
