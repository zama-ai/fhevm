import type { EngineWorld } from '#engine';
import { Then, When } from '@cucumber/cucumber';
import assert from 'node:assert/strict';

Then('the settings are resolved from the profile', function (this: EngineWorld) {
  const { profile } = this.engine;
  assert.equal(profile.setting('fixture.literal'), 'hello-literal');
  assert.equal(profile.setting('fixture.from-file'), 'value-from-file');
  assert.equal(profile.setting('fixture.with-default'), 'fallback-value');
  assert.equal(String(profile.setting('fixture.secret')).length > 0, true);
});

Then('the process environment holds no profile value', function (this: EngineWorld) {
  const secret = String(this.engine.profile.setting('fixture.secret'));
  assert.equal(process.env.TE_FIXTURE_SECRET, undefined, 'the secret env var must not reach the worker');
  assert.equal(process.env.TE_FIXTURE_FROM_FILE, undefined, 'env-file values must not reach process.env');
  const leaking = Object.entries(process.env).filter(([, value]) => value?.includes(secret));
  assert.deepEqual(leaking, [], 'no environment variable may contain the secret');
});

When('the secret is printed, logged and recorded as evidence', function (this: EngineWorld) {
  const secret = String(this.engine.profile.setting('fixture.secret'));
  console.log(`console output with the secret: ${secret}`);
  console.error(`stderr output with the secret in a URL: https://rpc.invalid/v3/${encodeURIComponent(secret)}`);
  this.engine.log('engine log with the secret:', secret);
  this.engine.evidence.record('fixture.secret', { leaked: secret, nested: [{ again: `x-${secret}-y` }] });
});

Then('a step fails with an error containing the secret', function (this: EngineWorld) {
  const secret = String(this.engine.profile.setting('fixture.secret'));
  throw new Error(`request to https://rpc.invalid/v3/${secret} failed`);
});
