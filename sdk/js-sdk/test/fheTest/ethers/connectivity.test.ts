import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineConnectivityTests } from '../ethers-common/connectivity.tests.js';

defineConnectivityTests(!isCleartext(getEthersTestConfig().chainName));
