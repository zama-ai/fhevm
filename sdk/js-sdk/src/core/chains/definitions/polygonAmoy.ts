import type { FhevmChain } from '../../types/fhevmChain.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import { defineFhevmChain } from '../utils.js';

export const polygonAmoy: FhevmChain = /*#__PURE__*/ defineFhevmChain({
  id: 80_002,
  fhevm: {
    contracts: {
      acl: {
        address: '0xFb957a4144c3EF17aeAc08307880Ccf41Fe558ec' as ChecksummedAddress,
      },
      inputVerifier: {
        address: '0x57D17124E69B500a235Ea58C154e097F0aE4a3E5' as ChecksummedAddress,
      },
      kmsVerifier: {
        address: '0xBC4845A34ac1bfDa56644CA093084911Da86E57c' as ChecksummedAddress,
      },
    },
    relayerUrl: 'https://relayer.dev.zama.cloud',
    gateway: {
      id: 10_900,
      contracts: {
        decryption: {
          address: '0xA4dc265D54D25D41565c60d36097E8955B03decD' as ChecksummedAddress,
        },
        inputVerification: {
          address: '0xf091D9B4C2da7ecd11858cDD1F4515a8a767D755' as ChecksummedAddress,
        },
      },
    },
  },
});
