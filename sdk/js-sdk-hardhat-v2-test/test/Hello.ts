import { expect } from 'chai';
import { ethers } from 'hardhat';

describe('Hello', function () {
  it('returns the message set at deployment', async function () {
    const Hello = await ethers.getContractFactory('Hello');
    const hello = await Hello.deploy('Hello, world!');
    expect(await hello.message()).to.equal('Hello, world!');
  });

  it('updates the message via setMessage', async function () {
    const Hello = await ethers.getContractFactory('Hello');
    const hello = await Hello.deploy('initial');
    await (await hello.setMessage('updated')).wait();
    expect(await hello.message()).to.equal('updated');
  });
});
