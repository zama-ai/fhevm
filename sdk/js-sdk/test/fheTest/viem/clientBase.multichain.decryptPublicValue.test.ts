import { createFhevmBaseClient } from '@fhevm/sdk/viem';
import { areAllViemTestConfigsCleartext, isMultichain } from '../setup-viem.js';
import { defineClientBaseMultichainDecryptPublicValueTests } from '../viem-common/clientBase.multichain.decryptPublicValue.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=devnet,polygon_devnet npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.multichain.decryptPublicValue.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientBaseMultichainDecryptPublicValueTests({
  runIf: !areAllViemTestConfigsCleartext() && isMultichain(),
  createFhevmBaseClient: (params) => createFhevmBaseClient(params),
});
