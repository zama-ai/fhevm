import type { FhevmChain } from '@fhevm/sdk/chains';

/**
 * UNSUPPORTED: Polygon devnet still runs the pre-v0.14 host stack (KMSVerifier
 * v0.3.0) and has no ProtocolConfig, which this SDK requires. The definition is
 * kept for when devnet is upgraded: then add its `protocolConfig` address, wrap
 * it in `defineFhevmChain` and drop the unsupported guard in
 * `test/fheTest/setupCommon.ts`.
 */
export const POLYGON_DEVNET_UNSUPPORTED_REASON =
  'polygon_devnet is unsupported: Polygon devnet still runs the pre-v0.14 host stack (KMSVerifier v0.3.0) and has no ProtocolConfig, which this SDK requires.';

type FhevmChainWithoutProtocolConfig = {
  readonly id: FhevmChain['id'];
  readonly fhevm: Omit<FhevmChain['fhevm'], 'contracts'> & {
    readonly contracts: Omit<FhevmChain['fhevm']['contracts'], 'protocolConfig'>;
  };
};

// Not a valid FhevmChain (no ProtocolConfig): do not pass to defineFhevmChain
// or createFhevmClient until devnet is upgraded.
export const polygon_devnet_unsupported: FhevmChainWithoutProtocolConfig = {
  id: 80_002,
  fhevm: {
    contracts: {
      acl: {
        address: '0xFb957a4144c3EF17aeAc08307880Ccf41Fe558ec',
      },
      inputVerifier: {
        address: '0xb25f1408C5318Ec34Fca59d2496Ff3465306a249',
      },
      kmsVerifier: {
        address: '0xBC4845A34ac1bfDa56644CA093084911Da86E57c',
      },
      // protocolConfig: none deployed on Polygon devnet yet.
    },
    relayerUrl: 'https://relayer.dev.zama.cloud',
    gateway: {
      id: 10_900,
      contracts: {
        decryption: {
          address: '0xA4dc265D54D25D41565c60d36097E8955B03decD',
        },
        inputVerification: {
          address: '0xf091D9B4C2da7ecd11858cDD1F4515a8a767D755',
        },
      },
    },
  },
};
