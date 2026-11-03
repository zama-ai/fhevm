import type { EngineWorld } from '#engine';
import { When } from '@cucumber/cucumber';

When('the scenario logs the profile name', function (this: EngineWorld) {
  this.engine.log('profile:', this.engine.profile.name);
});
