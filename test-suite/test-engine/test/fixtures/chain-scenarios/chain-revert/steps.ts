import { type EngineWorld, loadArtifact } from '#engine';
import { When } from '@cucumber/cucumber';
import path from 'node:path';

When('alice sends a transaction that reverts on chain', async function (this: EngineWorld) {
  const { chain } = this.engine;
  const artifact = loadArtifact(path.join(this.engine.scenario.directory, '..', '..', 'contracts', 'Counter.json'));
  const counter = await chain.deploy(artifact, [0], { from: 'alice' });
  // An explicit gas limit skips the estimation, so the transaction is mined and reverts on chain.
  await chain.send(counter.getFunction('fail')({ gasLimit: 100_000 }), { label: 'always fails' });
});
