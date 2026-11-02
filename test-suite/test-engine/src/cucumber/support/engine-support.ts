/**
 * Support code loaded into every scenario before the scenario's own support code.
 * It runs inside the scenario worker process, so it only affects that scenario.
 */
import { After, Before, setDefaultTimeout, setWorldConstructor } from '@cucumber/cucumber';

import { EngineWorld, disposeWorld } from '../../api/world.js';
import { getCurrentJob } from '../../runner/job-context.js';

const job = getCurrentJob();
if (job) setDefaultTimeout(job.stepTimeoutMs);

// Default World; a scenario calling setWorldConstructor later replaces it (with a subclass).
setWorldConstructor(EngineWorld);

Before({ name: 'engine: World must extend EngineWorld' }, function (this: unknown) {
  if (!(this instanceof EngineWorld)) {
    throw new Error("the World must extend EngineWorld (import { EngineWorld } from '#engine')");
  }
});

After({ name: 'engine: release scenario resources' }, function (this: unknown) {
  if (this instanceof EngineWorld) disposeWorld(this);
});
