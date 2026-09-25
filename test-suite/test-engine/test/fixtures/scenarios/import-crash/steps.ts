import { Given } from '@cucumber/cucumber';

Given('a step that passes', function () {});

// Simulates support code doing work at import time and failing (e.g. a missing env var).
throw new Error('support code failed to load');
