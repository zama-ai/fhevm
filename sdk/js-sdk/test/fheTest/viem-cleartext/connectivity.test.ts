import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineConnectivityTests } from '../viem-common/connectivity.tests.js';

defineConnectivityTests(isCleartext(getViemTestConfig().chainName));
