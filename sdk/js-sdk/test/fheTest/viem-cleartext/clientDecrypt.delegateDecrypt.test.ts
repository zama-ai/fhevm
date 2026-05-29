import { createFhevmCleartextDecryptClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptDelegateDecryptTests } from '../viem-common/clientDecrypt.delegateDecrypt.tests.js';

defineClientDecryptDelegateDecryptTests(isCleartext(getViemTestConfig().chainName), (params) =>
  createFhevmCleartextDecryptClient(params),
);
