import { defineFhevmChain } from '@fhevm/sdk/chains';

export const localhostFhevm = /*#__PURE__*/ defineFhevmChain({
  id: 12_345,
  fhevm: {
    contracts: {
      acl: {
        address: '0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c',
      },
      inputVerifier: {
        address: '0x857Ca72A957920Fa0FB138602995839866Bd4005',
      },
      kmsVerifier: {
        address: '0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11',
      },
    },
    relayerUrl: 'http://localhost:3000',
    gateway: {
      id: 54_321,
      contracts: {
        decryption: {
          address: '0xF0bFB159C7381F7CB332586004d8247252C5b816',
        },
        inputVerification: {
          address: '0x3b12Fc766Eb598b285998877e8E90F3e43a1F8d2',
        },
      },
    },
  },
});
