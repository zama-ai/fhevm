import { Given } from '@cucumber/cucumber';

Given('a step that must never run', function () {
  throw new Error('disabled scenario was executed');
});
