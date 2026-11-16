import { createFhevmDecryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext, isLocalstackChain, isRealDeployedChain } from '../setupCommon.js';
import { defineClientDecryptUnifiedPermitTests } from '../viem-common/clientDecrypt.unifiedPermit.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.unifiedPermit.test.ts
//
////////////////////////////////////////////////////////////////////////////////

const chainName = getViemTestConfig().chainName;

defineClientDecryptUnifiedPermitTests({
  // Unified (V2) permits require protocol v14+ (KMSVerifier >= 0.4.0 + ProtocolConfig).
  runIf: !isCleartext(chainName) && (isLocalstackChain(chainName) || isRealDeployedChain(chainName)),
  checkRelayerSupportsUnifiedPermit: isRealDeployedChain(chainName),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
