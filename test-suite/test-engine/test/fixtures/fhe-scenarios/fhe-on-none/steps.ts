import type { EngineWorld } from '#engine';
import { When } from '@cucumber/cucumber';

When('the scenario asks for the FHE module', function (this: EngineWorld) {
  this.engine.log('FHE module:', typeof this.engine.fhe);
});
