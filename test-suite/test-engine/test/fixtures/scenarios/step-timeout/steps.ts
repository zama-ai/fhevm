import { Given } from '@cucumber/cucumber';
import { setTimeout as sleep } from 'node:timers/promises';

Given('an asynchronous step that takes {int} ms', async function (ms: number) {
  await sleep(ms);
});
