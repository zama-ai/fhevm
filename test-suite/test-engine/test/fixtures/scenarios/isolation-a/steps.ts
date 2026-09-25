import { Then, World, setDefaultTimeout, setWorldConstructor } from '@cucumber/cucumber';
import assert from 'node:assert/strict';

// Registered globally in this process. If scenario B shared the process, it would get this
// World, this 200 ms timeout, and an ambiguous definition for the step below.
class WorldA extends World {}
setWorldConstructor(WorldA);
setDefaultTimeout(200);

Then('the active World is {string}', function (this: World, expected: string) {
  assert.equal(this.constructor.name, expected);
});
