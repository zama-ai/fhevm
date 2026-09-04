import { type FhevmChain, defineFhevmChain, mainnet, sepolia } from '@fhevm/sdk/chains';

import constants from './constants';

/**
 * Chain definitions for `@fhevm/sdk`.
 *
 * `@fhevm/sdk` is the source of truth for every public network: `sepolia` and `mainnet` are imported
 * from it rather than restated here, so the plugin cannot drift from the SDK's view of the protocol.
 * (It had: the addresses this file replaces named a gateway `decryption` and `inputVerification` pair
 * that the SDK no longer uses, and had no slot at all for v13's `ProtocolConfig`.)
 *
 * The local cleartext chain is the one exception, and only because the SDK ships no definition for it.
 * Its addresses come from `@fhevm/host-contracts-cleartext`'s `LocalHostAddresses.sol` via
 * `FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE`, and the deploy asserts them on every run. If the SDK ever
 * publishes a `localcleartext` chain, delete this and import that instead.
 */

const CLEARTEXT = constants.FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE;

/**
 * The canonical local cleartext stack (chain id 31337).
 *
 * Mirrors `@fhevm/sdk`'s own `test/chains/localcleartext.ts`. `relayerUrl` is required by
 * `FhevmChain` but unused in cleartext mode — the SDK reads cleartexts straight from `CleartextDB`
 * instead of calling a relayer — so it is pointed at the node itself rather than left blank.
 */
export const localCleartext: FhevmChain = /*#__PURE__*/ defineFhevmChain({
  id: constants.DEVELOPMENT_NETWORK_CHAINID,
  fhevm: {
    contracts: {
      acl: { address: CLEARTEXT.fhevmAddresses.aclAddress as `0x${string}` },
      inputVerifier: { address: CLEARTEXT.fhevmAddresses.inputVerifierAddress as `0x${string}` },
      kmsVerifier: { address: CLEARTEXT.fhevmAddresses.kmsVerifierAddress as `0x${string}` },
      protocolConfig: { address: CLEARTEXT.fhevmAddresses.protocolConfigAddress as `0x${string}` },
    },
    relayerUrl: 'http://localhost:8545',
    gateway: {
      id: CLEARTEXT.gateway.chainId,
      contracts: {
        decryption: { address: CLEARTEXT.gateway.decryptionAddress as `0x${string}` },
        inputVerification: { address: CLEARTEXT.gateway.inputVerificationAddress as `0x${string}` },
      },
    },
  },
});

export { mainnet, sepolia };
export type { FhevmChain };
