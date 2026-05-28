import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineChainTests } from '../ethers-common/chain.tests.js';

defineChainTests(isCleartext(getEthersTestConfig().chainName));
