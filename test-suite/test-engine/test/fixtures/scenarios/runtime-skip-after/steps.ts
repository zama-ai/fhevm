import { After, Given } from '@cucumber/cucumber';

Given('a step that skips at runtime', function () {
  return 'skipped';
});

After({ name: 'failing after hook' }, function () {
  throw new Error('after hook failed');
});
