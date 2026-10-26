import type { EngineWorld } from '#engine';
import { Before, Given } from '@cucumber/cucumber';

Before({ name: 'recording hook' }, function (this: EngineWorld) {
  this.engine.evidence.record('fixture.hook', { phase: 'before' }, { label: 'From a hook' });
});

Given('a step that records evidence', function (this: EngineWorld) {
  this.engine.log('profile in use:', this.engine.profile.name);
  this.engine.evidence.record('fixture.step', { scenario: this.engine.scenario.id, answer: 42 });
});
