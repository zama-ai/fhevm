import { createFhevmDecryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptTransportKeyPairTests } from '../viem-common/clientDecrypt.transportKeyPair.tests.js';

defineClientDecryptTransportKeyPairTests(!isCleartext(getViemTestConfig().chainName), (params) =>
  createFhevmDecryptClient(params),
);
