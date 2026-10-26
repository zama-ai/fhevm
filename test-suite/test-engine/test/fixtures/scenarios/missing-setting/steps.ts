import type { EngineWorld } from '#engine';
import { Given } from '@cucumber/cucumber';

Given('the setting {string} is read', function (this: EngineWorld, key: string) {
  this.engine.profile.setting(key);
});
