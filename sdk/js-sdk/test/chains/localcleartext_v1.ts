import { defineFhevmChain } from '@fhevm/sdk/chains';

// The cleartext v1.0.0 stack (contracts/src/v1.0.0): its canonical addresses and gateway values, DEPLOY.md §4-5.
export const localcleartext_v1 = /*#__PURE__*/ defineFhevmChain({
  id: 31_337,
  fhevm: {
    contracts: {
      acl: {
        address: '0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D',
      },
      inputVerifier: {
        address: '0x36772142b74871f255CbD7A3e89B401d3e45825f',
      },
      kmsVerifier: {
        address: '0x901F8942346f7AB3a01F6D7613119Bca447Bb030',
      },
      protocolConfig: {
        address: '0x44aA028fd264C76BF4A8f8B4d8A5272f6AE25CAc',
      },
    },
    relayerUrl: 'http://localhost:8545',
    gateway: {
      // uint48(uint256(keccak256("fhevm.cheat.chainId cleartext gateway"))), v1.0.0's GATEWAY_CHAIN_ID.
      id: 100_733_346_448_153,
      contracts: {
        decryption: {
          address: '0xEaaA2FC6BC259dF015Aa7Dc8e59e0B67df622721',
        },
        inputVerification: {
          address: '0x6189F6c0c3E40B4a3c72ec86262295D78d845297',
        },
      },
    },
  },
});
