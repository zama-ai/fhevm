import type { EngineWorld } from '#engine';
import { When } from '@cucumber/cucumber';

When('the scenario reads the chain ID', function (this: EngineWorld) {
  this.engine.log('chain ID:', this.engine.chain.chainId);
});
