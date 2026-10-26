import { Before, Given } from '@cucumber/cucumber';

Before({ name: 'skipping hook' }, function () {
  return 'skipped';
});

Given('a step that passes', function () {});
