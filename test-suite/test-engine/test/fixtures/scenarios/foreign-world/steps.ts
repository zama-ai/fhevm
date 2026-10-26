import { Given, World, setWorldConstructor } from '@cucumber/cucumber';

// Does not extend EngineWorld: the engine must fail the scenario instead of letting
// `this.engine` silently be undefined.
class ForeignWorld extends World {}
setWorldConstructor(ForeignWorld);

Given('a step that passes', function () {});
