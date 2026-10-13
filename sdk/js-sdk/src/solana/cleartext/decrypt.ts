// The KMS's part of a decryption, played by the cleartext client: plaintexts come from the accounts
// the cleartext host wrote. Everything around them runs as in production: the permit and request
// admission of a user decryption, and the on-chain checks of a public-decrypt certificate, which
// the client signs with the registered cleartext KMS key.
//
// Before answering a user decryption, the client runs the real stack's checks in its order: the
// decryption relayer's stateless admission (the handles' chain, the validity window against wall
// time), then the KMS Connector's authorization over the same host state (./authorization.ts). A
// public decryption is judged by the Connector's rules for one. What the client does not reproduce is
// the KMS itself: no signcryption, no response signatures, and so no check of the FHE parameter or
// the KMS epoch. A refusal reaches the production retry loop with the code the real stack answers it
// with, so the caller sees what it would see there.
import { getAddressDecoder, parseBase64RpcAccount, type Address } from '@solana/kit';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaRpc } from '../encryptedStore.js';
import type { SolanaUserDecryptExecution } from '../clients/decorators/permitDecrypt.js';
import type { SolanaTransportKeyPair, SolanaUserDecryptPlaintext } from '../userDecrypt/index.js';
import type { SolanaDecryptionOutcome, SolanaDecryptionRejection } from '../decryption/retry.js';
import type { SolanaPublicDecryptCertifier } from '../actions/publicDecryptCertificate.js';
import { asBytes32, bytesToHex, bytesToHexNo0x, concatBytes, hexToBytes } from '../../core/base/bytes.js';
import { bytes32ToHandle, toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { createKmsPublicDecryptEip712 } from '../../core/kms/createKmsPublicDecryptEip712.js';
import { publicDecryptDigest } from '../actions/decryptPublicValue.js';
import { buildSolanaUserDecryptRequest } from '../userDecrypt/index.js';
import { createSolanaDecryptionDeadline } from '../decryption/deadline.js';
import { retrySolanaDecryption } from '../decryption/retry.js';
import { verifySolanaUserDecryptPlaintexts } from '../userDecrypt/response.js';
import { solanaPublicDecryptExtraData } from '../actions/publicDecryptCertificate.js';
import {
  fetchHostConfig,
  fetchKmsContext,
  findHostConfigPda,
  findKmsContextPda,
  type HostConfig,
  type KmsContext,
} from '@fhevm/solana-zama-host';
import { solanaHostProgram } from '../clients/createFhevmBaseClient.js';
import { signAsCleartextParty } from './parties.js';
import { PERMIT_TRANSPORT_KEY_LEN } from '../permit/index.js';
import {
  CONNECTOR_FAILURE_CODE,
  judgeSolanaPublicDecryption,
  judgeSolanaUserDecryption,
  type ConnectorVerdict,
  type SolanaHostAccountsReader,
} from './authorization.js';
import type { SolanaLeafReader } from './leafRecord.js';
import { fetchCleartextStoreValue } from './storeValues.js';

////////////////////////////////////////////////////////////////////////////////

const FINALIZED = { commitment: 'finalized' } as const;

/**
 * Answers a user decryption with the plaintexts the host recorded for its handles, once the request
 * passes the Connector's authorization, through the production retry loop. The leaves come from
 * `readLeaves`.
 */
export function cleartextUserDecryptExecution(
  rpc: SolanaRpc,
  chain: FhevmSolanaChain,
  readLeaves: SolanaLeafReader,
): SolanaUserDecryptExecution {
  const programAddress = solanaHostProgram(chain);
  const execute: SolanaUserDecryptExecution['execute'] = async ({ session, entries, attempts, options }) => {
    const { fields, signature } = session.signedPermit;
    // The caller's timeout and abort signal bound the whole run, as on the relayer path.
    const deadline = createSolanaDecryptionDeadline({
      operation: 'USER_DECRYPT',
      url: `cleartext host ${programAddress}`,
      options,
    });
    // Admission as on the relayer path: a request that can never be submitted is refused here.
    buildSolanaUserDecryptRequest({ signedPermit: session.signedPermit, entries });
    const response = await retrySolanaDecryption({
      // The deadline ends an attempt stalled on a read, as it ends a stalled relayer request.
      submit: () =>
        deadline.bound(async (signal): Promise<SolanaDecryptionOutcome<readonly SolanaUserDecryptPlaintext[]>> => {
          const readAccounts = hostAccountsReader(rpc, signal);
          // Each attempt is judged against the host as it is then, as the Connector rechecks the KMS
          // context on every attempt.
          const host = await fetchHostDecryptionState(rpc, programAddress, fields.kmsRouting.kmsContextId, signal);

          // The decryption relayer's admission: the handles' chain, which admission made the permit's,
          // and the validity window against its wall clock.
          if (fields.chainId !== host.config.chainId) {
            const message = `the handles are on host chain ${fields.chainId}; this host is chain ${host.config.chainId}`;
            return { ok: false, rejection: { code: 'malformed', message } };
          }
          const now = BigInt(Math.floor(Date.now() / 1000));
          const end = fields.startTimestamp + fields.durationSeconds;
          if (fields.startTimestamp > now || end <= now) {
            const message = `payload.requestValidity: [${fields.startTimestamp}, ${end}) does not hold ${now}`;
            return { ok: false, rejection: { code: 'malformed', message } };
          }

          // The Connector.
          if (host.context.destroyed) {
            const message = `KMS context ${bytesToHex(fields.kmsRouting.kmsContextId)} is destroyed`;
            return { ok: false, rejection: { code: 'kms_context_destroyed', message } };
          }
          const verdict = await judgeSolanaUserDecryption({
            programAddress,
            now,
            fields,
            signature,
            entries,
            readAccounts,
            readLeaves,
          });
          const rejection = cleartextConnectorRejection(verdict);
          if (rejection !== undefined) return { ok: false, rejection };
          const plaintexts = await Promise.all(
            entries.map(async (entry) => ({
              bytes: await fetchCleartextStoreValue(rpc, programAddress, entry.encryptedStore, entry.handle, signal),
              fheTypeId: bytes32ToHandle(asBytes32(entry.handle)).fheTypeId,
            })),
          );
          return { ok: true, response: plaintexts };
        }),
      clock: deadline,
      attempts,
    });
    deadline.throwIfAbortedOrExpired();
    verifySolanaUserDecryptPlaintexts(
      response,
      entries.map((entry) => entry.handle),
    );
    return response;
  };
  return { transportKeyPair: cleartextTransportKeyPair, execute };
}

/**
 * A transport keypair with no KMS WASM behind it, as EVM's cleartext decrypt module has none: no
 * share is signcrypted to it, so only the bytes the permit commits to matter, at the length the
 * permit requires. Random, so each permit still commits to its own key.
 */
function cleartextTransportKeyPair(): Promise<SolanaTransportKeyPair> {
  const noWasmKey = { free: () => undefined, [Symbol.dispose]: () => undefined };
  return Promise.resolve({
    secretKey: noWasmKey,
    publicKey: noWasmKey,
    publicKeyBytes: crypto.getRandomValues(new Uint8Array(PERMIT_TRANSPORT_KEY_LEN)),
  });
}

/** The host records a user decryption is judged against: its HostConfig and the permit's KMS context. */
type HostDecryptionState = {
  readonly config: HostConfig;
  readonly context: KmsContext;
};

async function fetchHostDecryptionState(
  rpc: SolanaRpc,
  programAddress: Address,
  contextId: Uint8Array,
  abortSignal?: AbortSignal,
): Promise<HostDecryptionState> {
  const fetchConfig = { ...FINALIZED, ...(abortSignal === undefined ? {} : { abortSignal }) };
  const [{ data: config }, { data: context }] = await Promise.all([
    fetchHostConfig(rpc, (await findHostConfigPda({ programAddress }))[0], fetchConfig),
    fetchKmsContext(rpc, (await findKmsContextPda({ contextId }, { programAddress }))[0], fetchConfig),
  ]);
  return { config, context };
}

/** One `getMultipleAccounts` read of the host accounts, at a slot no older than `minContextSlot`. */
const hostAccountsReader =
  (rpc: SolanaRpc, abortSignal?: AbortSignal): SolanaHostAccountsReader =>
  async (keys, minContextSlot) => {
    const { context, value } = await rpc
      .getMultipleAccounts(keys, {
        ...FINALIZED,
        encoding: 'base64',
        ...(minContextSlot === undefined ? {} : { minContextSlot }),
      })
      .send(abortSignal === undefined ? {} : { abortSignal });
    if (value.length !== keys.length) {
      throw new Error(`getMultipleAccounts returned ${value.length} accounts for ${keys.length} keys`);
    }
    return {
      slot: context.slot,
      accounts: keys.map((key, index) => parseBase64RpcAccount(key, value[index] ?? null)),
    };
  };

/**
 * What the caller sees of the Connector's verdict on the real stack: the code the Connector answers
 * the failure with, which the decryption relayer forwards.
 */
export function cleartextConnectorRejection(verdict: ConnectorVerdict): SolanaDecryptionRejection | undefined {
  if (verdict.authorized) return undefined;
  return { code: CONNECTOR_FAILURE_CODE[verdict.failure], message: `${verdict.failure}: ${verdict.message}` };
}

/**
 * Certifies the plaintexts the host recorded for handles made public, signed by the registered
 * cleartext KMS key under the certificate's context, as the KMS signs one batch. The Connector's
 * verdict runs through the production retry loop, as on the relayer path.
 */
export function cleartextPublicDecryptCertifier(
  rpc: SolanaRpc,
  chain: FhevmSolanaChain,
  readLeaves: SolanaLeafReader,
): SolanaPublicDecryptCertifier {
  const programAddress = solanaHostProgram(chain);
  return async (parameters) => {
    const signal = parameters.options?.signal;
    const deadline = createSolanaDecryptionDeadline({
      operation: 'PUBLIC_DECRYPT',
      url: `cleartext host ${programAddress}`,
      options: parameters.options,
    });
    const entries = parameters.entries.map((entry) => {
      const handle = toFhevmHandle(entry.handle);
      return { handle, handleBytes: hexToBytes(handle.bytes32Hex), encryptedStore: entry.encryptedStore };
    });
    await retrySolanaDecryption({
      submit: () =>
        deadline.bound(async (attemptSignal): Promise<SolanaDecryptionOutcome<undefined>> => {
          const rejection = cleartextConnectorRejection(
            await judgeSolanaPublicDecryption({
              programAddress,
              handles: entries.map(({ handleBytes, encryptedStore }) => ({
                handle: handleBytes,
                encryptedStore: getAddressDecoder().decode(encryptedStore),
              })),
              readAccounts: hostAccountsReader(rpc, attemptSignal),
              readLeaves,
            }),
          );
          return rejection === undefined ? { ok: true, response: undefined } : { ok: false, rejection };
        }),
      clock: deadline,
    });
    const cleartexts = await Promise.all(
      entries.map(({ handleBytes, encryptedStore }) =>
        fetchCleartextStoreValue(rpc, programAddress, encryptedStore, handleBytes, signal),
      ),
    );
    const cleartext = concatBytes(...cleartexts);
    const { config, context } = await fetchHostDecryptionState(rpc, programAddress, parameters.contextId, signal);
    const extraData = solanaPublicDecryptExtraData(parameters.contextId, parameters.epochId);
    const digest = publicDecryptDigest(
      createKmsPublicDecryptEip712({
        verifyingContractAddressDecryption: bytesToHex(new Uint8Array(config.decryptionContract)),
        chainId: config.gatewayChainId,
        handles: entries.map(({ handle }) => handle),
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
      handles: entries.map(({ handle }) => handle.bytes32Hex),
      abiEncodedCleartext: bytesToHexNo0x(cleartext),
      signatures: signatures.map((signature) => signature.slice(2)),
      extraData,
    };
  };
}
