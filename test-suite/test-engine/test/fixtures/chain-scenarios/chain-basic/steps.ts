import { type EngineWorld, loadArtifact } from '#engine';
import { Given, Then, When } from '@cucumber/cucumber';
import type { Contract } from 'ethers';
import assert from 'node:assert/strict';
import path from 'node:path';

interface CounterWorld extends EngineWorld {
  counter?: Contract;
}

const artifactOf = (world: EngineWorld) =>
  loadArtifact(path.join(world.engine.scenario.directory, '..', '..', 'contracts', 'Counter.json'));

Given('alice deploys a Counter starting at {int}', async function (this: CounterWorld, initial: number) {
  this.counter = await this.engine.chain.deploy(artifactOf(this), [initial], {
    from: 'alice',
    label: 'Deploy Counter',
  });
});

When('alice increments the counter', async function (this: CounterWorld) {
  await this.engine.chain.send(this.counter!.getFunction('increment')(), { label: 'alice increments' });
});

When('bob increments the counter by {int}', async function (this: CounterWorld, amount: number) {
  const bob = this.engine.chain.wallet('bob');
  const counter = this.counter!.connect(bob) as Contract;
  await this.engine.chain.send(counter.getFunction('incrementBy')(amount), { label: 'bob increments' });
});

Then('the counter is {int}', async function (this: CounterWorld, expected: number) {
  assert.equal(await this.counter!.getFunction('count')(), BigInt(expected));
});
