import { createFhevmBaseClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseChainTests } from '../viem-common/clientBase.chain.tests.js';

defineClientBaseChainTests(!isCleartext(getViemTestConfig().chainName), (params) => createFhevmBaseClient(params));
