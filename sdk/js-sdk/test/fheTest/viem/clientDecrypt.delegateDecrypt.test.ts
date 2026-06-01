import { createFhevmDecryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptDelegateDecryptTests } from '../viem-common/clientDecrypt.delegateDecrypt.tests.js';

defineClientDecryptDelegateDecryptTests({
  runIf: !isCleartext(getViemTestConfig().chainName),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
