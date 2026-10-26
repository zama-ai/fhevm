import { Given, When } from '@cucumber/cucumber';

Given('a step that passes', function () {});

// Partial execution: forbidden by the engine, reported as a failure.
When('a step that skips at runtime', function () {
  return 'skipped';
});
