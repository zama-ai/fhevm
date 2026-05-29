import { createFhevmDecryptClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptTransportKeyPairTests } from '../ethers-common/clientDecrypt.transportKeyPair.tests.js';

defineClientDecryptTransportKeyPairTests(!isCleartext(getEthersTestConfig().chainName), (params) =>
  createFhevmDecryptClient(params),
);
