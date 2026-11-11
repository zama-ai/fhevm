import type { EngineWorld } from '#engine';
import { When } from '@cucumber/cucumber';
import assert from 'node:assert/strict';

When('the scenario loads the compiled Counter contract', function (this: EngineWorld) {
  const artifact = this.engine.contracts.artifact('Counter');
  assert.equal(artifact.contractName, 'Counter');
  this.engine.log(`loaded ${artifact.contractName} from ${this.engine.contracts.directory}`);
});
