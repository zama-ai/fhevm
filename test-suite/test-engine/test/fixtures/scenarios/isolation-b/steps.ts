import { Then, World, setWorldConstructor } from '@cucumber/cucumber';
import assert from 'node:assert/strict';
import { setTimeout as sleep } from 'node:timers/promises';

class WorldB extends World {}
setWorldConstructor(WorldB);

// Same text as in scenario A: only unambiguous because each scenario has its own process.
Then('the active World is {string}', function (this: World, expected: string) {
  assert.equal(this.constructor.name, expected);
});

// Would exceed scenario A's 200 ms default timeout if it leaked here.
Then('an asynchronous step that takes {int} ms', async function (ms: number) {
  await sleep(ms);
});
