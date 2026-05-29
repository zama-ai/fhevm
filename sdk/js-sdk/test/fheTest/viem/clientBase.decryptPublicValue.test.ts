import { createFhevmBaseClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseDecryptPublicValueTests } from '../viem-common/clientBase.decryptPublicValue.tests.js';

defineClientBaseDecryptPublicValueTests(!isCleartext(getViemTestConfig().chainName), (params) =>
  createFhevmBaseClient(params),
);
