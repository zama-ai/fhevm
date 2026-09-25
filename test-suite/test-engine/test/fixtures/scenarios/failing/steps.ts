import { Given, Then, When } from '@cucumber/cucumber';

Given('a step that passes', function () {});

When('a step fails with {string}', function (message: string) {
  throw new Error(message);
});

Then('a step that is never reached', function () {
  throw new Error('must not be executed');
});
