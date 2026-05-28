import { createFhevmDecryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptDecryptTests } from '../viem-common/clientDecrypt.decrypt.tests.js';

defineClientDecryptDecryptTests(!isCleartext(getViemTestConfig().chainName), (params) =>
  createFhevmDecryptClient(params),
);
