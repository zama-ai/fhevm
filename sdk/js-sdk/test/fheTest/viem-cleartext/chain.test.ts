import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineChainTests } from '../viem-common/chain.tests.js';

defineChainTests(isCleartext(getViemTestConfig().chainName));
