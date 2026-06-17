import { defineFhevmChain } from '@fhevm/sdk/chains';

export const polygon_devnet = /*#__PURE__*/ defineFhevmChain({
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
      protocolConfig: undefined, // To be filled
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
});
