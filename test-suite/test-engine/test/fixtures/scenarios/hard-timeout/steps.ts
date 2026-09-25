import { Given } from '@cucumber/cucumber';

// Blocks the event loop, so Cucumber's own step timeout can never fire: only the engine's
// hard kill can stop this scenario.
Given('a step that blocks the event loop for {int} ms', function (ms: number) {
  const end = Date.now() + ms;
  while (Date.now() < end) {
    // busy wait
  }
});
