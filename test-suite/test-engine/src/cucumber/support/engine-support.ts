/**
 * Support code loaded into every scenario before the scenario's own support code.
 * It runs inside the scenario worker process, so it only affects that scenario.
 */
import { setDefaultTimeout } from '@cucumber/cucumber';

import { STEP_TIMEOUT_ENV } from '../../runner/types.js';

const stepTimeoutMs = Number(process.env[STEP_TIMEOUT_ENV]);
if (Number.isInteger(stepTimeoutMs) && stepTimeoutMs > 0) {
  setDefaultTimeout(stepTimeoutMs);
}
