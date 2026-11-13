import { getEthersTestConfig } from '../setup-ethers.js';
import { defineClientEncryptWithPadDecryptSlowTests } from '../ethers-common/clientEncryptWithPad.encryptDecrypt.slow.tests.js';
import {
  RELAYER_HTTP_URL,
  createRelayerHttpDecryptClient,
  createRelayerHttpEncryptWithPadClient,
} from './relayerHttpClient.js';

////////////////////////////////////////////////////////////////////////////////
//
// Encrypt with a pad fetched through relayer-http, submit the input package on chain, decrypt it back (cleartext
// decrypt, same runtime): the pad relayer-http returned is the one the chain unmasks with. Start the stack and relayer-http
// first (relayer-http/dev/README.md, "Pad decryption"), then:
//
// RELAYER_HTTP_URL=http://127.0.0.1:8080 CHAIN=localcleartext_v1_0 npx vitest run --config test/fheTest/vitest.config.ts ethers-relayer-http/clientEncryptWithPad.encryptDecrypt.slow.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptWithPadDecryptSlowTests({
  runIf: RELAYER_HTTP_URL !== undefined && getEthersTestConfig().chainName === 'localcleartext_v1_0',
  createFhevmEncryptWithPadClient: (params) => createRelayerHttpEncryptWithPadClient(params),
  createFhevmDecryptClient: (params) => createRelayerHttpDecryptClient(params),
});
