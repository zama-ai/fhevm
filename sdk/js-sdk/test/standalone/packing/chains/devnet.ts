import { defineFhevmChain } from '@fhevm/sdk/chains';

export const devnet = /*#__PURE__*/ defineFhevmChain({
  id: 11_155_111,
  fhevm: {
    contracts: {
      acl: {
        address: '0xBCA6F8De823a399Dc431930FD5EE550Bf1C0013e',
      },
      inputVerifier: {
        address: '0x6B32f47E39B0F8bE8bEAD5B8990F62E3e28ac08d',
      },
      kmsVerifier: {
        address: '0x3F3819BeBE4bD0EFEf8078Df6f9B574ADa80CCA4',
      },
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
