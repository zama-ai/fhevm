import { createFhevmCleartextDecryptClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptTransportKeyPairTests } from '../viem-common/clientDecrypt.transportKeyPair.tests.js';

defineClientDecryptTransportKeyPairTests({
  runIf: isCleartext(getViemTestConfig().chainName),
  createFhevmDecryptClient: (params) => createFhevmCleartextDecryptClient(params),
});
