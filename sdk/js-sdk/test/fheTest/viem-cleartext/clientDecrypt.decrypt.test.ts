import { createFhevmCleartextDecryptClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptDecryptTests } from '../viem-common/clientDecrypt.decrypt.tests.js';

defineClientDecryptDecryptTests(isCleartext(getViemTestConfig().chainName), (params) =>
  createFhevmCleartextDecryptClient(params),
);
