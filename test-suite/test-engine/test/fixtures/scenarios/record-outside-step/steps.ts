import { EngineWorld, type EngineWorldParameters } from '#engine';
import { Given, type IWorldOptions, setWorldConstructor } from '@cucumber/cucumber';

// Records evidence from the World constructor, when no step or hook is running.
class EagerWorld extends EngineWorld {
  constructor(options: IWorldOptions<EngineWorldParameters>) {
    super(options);
    this.engine.evidence.record('fixture.too-early', { when: 'constructor' });
  }
}
setWorldConstructor(EagerWorld);

Given('a step that passes', function () {});
