// relayer-http-request — one user-decrypt request to relayer-http, built from the SDK's own parts.
//
// A test can then send a request the SDK's client never would (a field changed after signing, a
// permit for another program, one already expired or for a destroyed context) and read the answer to
// that one request, without the client's retry loop. The package does not export those parts, so
// every SDK symbol whose types meet them (the client, its permits, the chain) comes from here, out
// of the source tree: one module instance and one set of types. `@fhevm/sdk` resolves to the built
// copy, a separate instance; fhe-vertical.ts builds its chain and client from that one.
import { createFinalizedRpc } from '@fhevm/solana-zama-host';

import { asBytes32Hex } from '../../../../sdk/js-sdk/src/core/base/index';
import type { FhevmSolanaChain } from '../../../../sdk/js-sdk/src/core/types/fhevmSolanaChain';
import { solanaDecryptionRelayer } from '../../../../sdk/js-sdk/src/solana/decryption/relayerHttp';
import {
  createFhevmDecryptClient,
  defineFhevmSolanaChain,
  setFhevmRuntimeConfig,
  type SolanaPermitSession,
} from '../../../../sdk/js-sdk/src/solana/index';
import { encodeSolanaKmsRouting } from '../../../../sdk/js-sdk/src/solana/permit/index';
import type { SolanaPermitWireFields } from '../../../../sdk/js-sdk/src/solana/permit/types';
import { relayerAuth } from '../layout';
import { type DecryptionCall, recordDecryptionCalls } from './decryption-trace';
import type { FheVerticalConfig } from './fhe-vertical';

export { fetchSolanaPermitInvalidation } from '../../../../sdk/js-sdk/src/solana/actions/revokePermits';
export { buildRevokePermitsInstruction, solanaPermitWalletFromSecretKey } from '../../../../sdk/js-sdk/src/solana/index';
export { decodeSolanaPermitFields, signSolanaPermit } from '../../../../sdk/js-sdk/src/solana/permit/index';
export {
  buildSolanaUserDecryptRequest,
  type SolanaUserDecryptHandleEntry,
} from '../../../../sdk/js-sdk/src/solana/userDecrypt/request';

/** The Solana chain the SDK decrypts on, with the relayer credentials set. */
export const decryptionChain = (config: FheVerticalConfig): FhevmSolanaChain => {
  setFhevmRuntimeConfig({ auth: relayerAuth() });
  return defineFhevmSolanaChain({
    id: config.chainId,
    fhevm: {
      relayerUrl: config.relayerUrl,
      decryptionRelayerUrl: config.decryptionRelayerUrl,
      programs: { host: { address: asBytes32Hex(config.verifyingProgramId) } },
    },
  });
};

/** The SDK's decrypt client on that chain. */
export const decryptionClient = (config: FheVerticalConfig) =>
  createFhevmDecryptClient({
    chain: decryptionChain(config),
    rpc: createFinalizedRpc(config.rpcUrl),
    fheParameter: config.fheParameter,
  });

/** Posts one user-decrypt body to relayer-http once, through the SDK's transport, and returns its answer. */
export const submitUserDecryptOnce = async (chain: FhevmSolanaChain, body: unknown): Promise<DecryptionCall> => {
  const relayer = solanaDecryptionRelayer(chain, 'USER_DECRYPT', { auth: relayerAuth() });
  const { calls } = await recordDecryptionCalls(() => relayer.submit(body));
  if (calls.length !== 1) throw new Error(`one submission made ${calls.length} decryption calls`);
  return calls[0]!;
};

/** The wire form of a signed permit's fields, to change one and sign again. */
export const permitWireFields = (session: SolanaPermitSession): SolanaPermitWireFields => {
  const fields = session.signedPermit.fields;
  return {
    userAddress: fields.userAddress,
    transportKey: fields.transportKey,
    allowedScopes: fields.allowedScopes,
    startTimestamp: fields.startTimestamp,
    durationSeconds: fields.durationSeconds,
    verifyingProgramId: fields.verifyingProgramId,
    chainId: fields.chainId,
    extraData: encodeSolanaKmsRouting(fields.kmsRouting),
  };
};
