import type { FhevmChain } from '../../types/fhevmChain.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import { defineFhevmChain } from '../utils.js';

export const sepolia: FhevmChain = /*#__PURE__*/ defineFhevmChain({
  id: 11_155_111,
  fhevm: {
    contracts: {
      acl: {
        address: '0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D' as ChecksummedAddress,
      },
      inputVerifier: {
        address: '0xBBC1fFCdc7C316aAAd72E807D9b0272BE8F84DA0' as ChecksummedAddress,
      },
      kmsVerifier: {
        address: '0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A' as ChecksummedAddress,
      },
    },
    relayerUrl: 'https://relayer.testnet.zama.org',
    gateway: {
      id: 10_901,
      contracts: {
        decryption: {
          address: '0x5D8BD78e2ea6bbE41f26dFe9fdaEAa349e077478' as ChecksummedAddress,
        },
        inputVerification: {
          address: '0x483b9dE06E4E4C7D35CCf5837A1668487406D955' as ChecksummedAddress,
        },
      },
    },
  },
});
