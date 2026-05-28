import { createFhevmEncryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptEncryptTests } from '../viem-common/clientEncrypt.encrypt.tests.js';

defineClientEncryptEncryptTests(!isCleartext(getViemTestConfig().chainName), {
  createClient: (params) => createFhevmEncryptClient(params),
  moduleVersions: { tfhe: '1.5.3' }, // hardcoded for testnet protocol v0.12
});
