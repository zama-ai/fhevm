// The KMS's part of a decryption, played by the cleartext client: plaintexts come from the accounts
// the cleartext host wrote. Everything around them runs as in production: the permit and request
// admission of a user decryption, and the on-chain checks of a public-decrypt certificate, which
// the client signs with the registered cleartext KMS key.
//
// What this does not reproduce is the KMS Connector's authorization: a cleartext user decryption
// does not check the requester's allow on the handle, nor a public decryption that the handle was
// made public. Scenarios that depend on a refusal run against the real stack.
import { fetchEncodedAccount, type Address } from '@solana/kit';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaRpc } from '../encryptedStore.js';
import type { SolanaUserDecryptExecution } from '../clients/decorators/permitDecrypt.js';
import type { SolanaUserDecryptPlaintext, SolanaUserDecryptTransport } from '../userDecrypt/index.js';
import type { SolanaPublicDecryptCertifier } from '../actions/publicDecryptCertificate.js';
import { bytesToHex, bytesToHexNo0x, hexToBytes, hexToBytes32 } from '../../core/base/bytes.js';
import { bytes32ToHandle, toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { createKmsPublicDecryptEip712, publicDecryptDigest } from '../../core/kms/createKmsPublicDecryptEip712.js';
import { runSolanaUserDecrypt } from '../userDecrypt/index.js';
import { verifySolanaUserDecryptPlaintexts } from '../userDecrypt/response.js';
import { solanaPublicDecryptExtraData } from '../actions/publicDecryptCertificate.js';
import { findHostConfigPda } from '../internal/generated/zamaHost/pdas/hostConfig.js';
import { findKmsContextPda } from '../internal/generated/zamaHost/pdas/kmsContext.js';
import { getHostConfigDecoder } from '../internal/generated/zamaHost/accounts/hostConfig.js';
import { getKmsContextDecoder } from '../internal/generated/zamaHost/accounts/kmsContext.js';
import { solanaHostProgram } from '../clients/createFhevmBaseClient.js';
import { signAsCleartextParty } from './parties.js';
import { fetchCleartextStoreValue } from './storeValues.js';

////////////////////////////////////////////////////////////////////////////////

/** Answers an admitted user decryption with the plaintexts the host recorded for its handles. */
export function cleartextUserDecryptExecution(rpc: SolanaRpc, chain: FhevmSolanaChain): SolanaUserDecryptExecution {
  const programAddress = solanaHostProgram(chain);
  const transport: SolanaUserDecryptTransport<readonly SolanaUserDecryptPlaintext[]> = {
    async submit(request) {
      const plaintexts = await Promise.all(
        request.attestedPayload.handles.map(async (entry) => {
          const handle = hexToBytes32(entry.handle);
          const bytes = await fetchCleartextStoreValue(rpc, programAddress, hexToBytes(entry.encryptedStore), handle);
          return { bytes, fheTypeId: bytes32ToHandle(handle).fheTypeId };
        }),
      );
      return { ok: true, response: plaintexts };
    },
  };
  return async ({ session, entries, attempts }) => {
    const { response } = await runSolanaUserDecrypt({
      signedPermit: session.signedPermit,
      entries,
      transport,
      clock: { delay: () => Promise.resolve() },
      ...(attempts === undefined ? {} : { attempts }),
    });
    verifySolanaUserDecryptPlaintexts(
      response,
      entries.map((entry) => entry.handle),
    );
    return response;
  };
}

/**
 * Certifies the plaintext the host recorded for a handle, signed by the registered cleartext KMS
 * key under the certificate's context, for the on-chain verifier to check.
 */
export function cleartextPublicDecryptCertifier(rpc: SolanaRpc, chain: FhevmSolanaChain): SolanaPublicDecryptCertifier {
  const programAddress = solanaHostProgram(chain);
  return async (parameters) => {
    const handle = toFhevmHandle(parameters.handle);
    const cleartext = await fetchCleartextStoreValue(
      rpc,
      programAddress,
      parameters.encryptedStore,
      hexToBytes(handle.bytes32Hex),
    );
    const [config, context] = await Promise.all([
      fetchHostAccount(rpc, programAddress, (await findHostConfigPda({ programAddress }))[0], getHostConfigDecoder()),
      fetchHostAccount(
        rpc,
        programAddress,
        (await findKmsContextPda({ contextId: parameters.contextId }, { programAddress }))[0],
        getKmsContextDecoder(),
      ),
    ]);
    const extraData = solanaPublicDecryptExtraData(parameters.contextId);
    const digest = publicDecryptDigest(
      createKmsPublicDecryptEip712({
        verifyingContractAddressDecryption: bytesToHex(new Uint8Array(config.decryptionContract)),
        chainId: config.gatewayChainId,
        handles: [handle],
        decryptedResult: bytesToHex(cleartext),
        extraData,
      }),
    );
    const signatures = signAsCleartextParty(
      'kms',
      context.signers,
      context.thresholds.publicDecryption,
      bytesToHex(digest),
    );
    // The relayer's wire form: unprefixed hex.
    return {
      handle: handle.bytes32Hex,
      abiEncodedCleartext: bytesToHexNo0x(cleartext),
      signatures: signatures.map((signature) => signature.slice(2)),
      extraData,
    };
  };
}

async function fetchHostAccount<T>(
  rpc: SolanaRpc,
  programAddress: Address,
  address: Address,
  decoder: { decode(bytes: Uint8Array): T },
): Promise<T> {
  const account = await fetchEncodedAccount(rpc, address, { commitment: 'confirmed' });
  if (!account.exists || account.programAddress !== programAddress) {
    throw new Error(`No host account of ${programAddress} at ${address}`);
  }
  return decoder.decode(new Uint8Array(account.data));
}
