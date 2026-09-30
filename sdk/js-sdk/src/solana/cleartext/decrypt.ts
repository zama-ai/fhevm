// The KMS's part of a decryption, played by the cleartext client: plaintexts come from the accounts
// the cleartext host wrote. Everything around them runs as in production: the permit and request
// admission of a user decryption, and the on-chain checks of a public-decrypt certificate, which
// the client signs with the registered cleartext KMS key.
//
// Before answering a user decryption, the client runs the real stack's checks in its order: the
// relayer's at submission and its delegation pre-check, the gateway's validity window, then the KMS
// Connector's authorization over the same host state (./authorization.ts). A public decryption is
// judged by the Connector's rules for one. What the client does not reproduce is the KMS itself: no
// signcryption, no response signatures, and so no check of the FHE parameter or the KMS epoch. A
// refusal reaches the caller as it does on the real stack, except that a request the Connector will
// never answer fails at once instead of timing out.
import { getAddressDecoder, parseBase64RpcAccount, type Address } from '@solana/kit';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaRpc } from '../encryptedStore.js';
import type { SolanaDecryptTrust, SolanaUserDecryptExecution } from '../clients/decorators/permitDecrypt.js';
import type {
  SolanaUserDecryptPlaintext,
  SolanaUserDecryptRejection,
  SolanaUserDecryptTransport,
} from '../userDecrypt/index.js';
import type { SolanaPublicDecryptCertifier } from '../actions/publicDecryptCertificate.js';
import { asBytes32, bytesToHex, bytesToHexNo0x, hexToBytes } from '../../core/base/bytes.js';
import { bytes32ToHandle, toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { createKmsPublicDecryptEip712, publicDecryptDigest } from '../../core/kms/createKmsPublicDecryptEip712.js';
import { runSolanaUserDecrypt } from '../userDecrypt/index.js';
import { createSolanaUserDecryptDeadline } from '../userDecrypt/deadline.js';
import { verifySolanaUserDecryptPlaintexts } from '../userDecrypt/response.js';
import { solanaPublicDecryptExtraData } from '../actions/publicDecryptCertificate.js';
import { findHostConfigPda } from '../internal/generated/zamaHost/pdas/hostConfig.js';
import { findKmsContextPda } from '../internal/generated/zamaHost/pdas/kmsContext.js';
import { fetchHostConfig, type HostConfig } from '../internal/generated/zamaHost/accounts/hostConfig.js';
import { fetchKmsContext, type KmsContext } from '../internal/generated/zamaHost/accounts/kmsContext.js';
import { solanaHostProgram } from '../clients/createFhevmBaseClient.js';
import { signAsCleartextParty } from './parties.js';
import { verifySolanaPermitSignature } from '../permit/envelope.js';
import {
  CONNECTOR_FAILURE_RECOVERABLE,
  judgeSolanaPublicDecryption,
  judgeSolanaUserDecryption,
  solanaRelayerDelegationRefusal,
  type ConnectorVerdict,
  type SolanaHostAccountsReader,
} from './authorization.js';
import type { SolanaLeafProofReader } from './leafProofs.js';
import { fetchCleartextStoreValue } from './storeValues.js';

////////////////////////////////////////////////////////////////////////////////

const CONFIRMED = { commitment: 'confirmed' } as const;
/** How often a public decryption is judged before a failure the Connector would retry stands. */
const PUBLIC_DECRYPT_ATTEMPTS = 20;
const PUBLIC_DECRYPT_RETRY_MS = 250;

/**
 * Answers a user decryption with the plaintexts the host recorded for its handles, once the request
 * passes the Connector's authorization, through the production retry loop. The leaf proofs come
 * from `readLeafProofs`.
 */
export function cleartextUserDecryptExecution(
  rpc: SolanaRpc,
  chain: FhevmSolanaChain,
  trust: SolanaDecryptTrust,
  readLeafProofs: SolanaLeafProofReader,
): SolanaUserDecryptExecution {
  const programAddress = solanaHostProgram(chain);
  const readAccounts = hostAccountsReader(rpc);
  return async ({ session, entries, attempts, options }) => {
    const { fields, signature } = session.signedPermit;
    // The caller's timeout and abort signal bound the whole run, as on the relayer path.
    const deadline = createSolanaUserDecryptDeadline({ url: `cleartext host ${programAddress}`, options });
    const transport: SolanaUserDecryptTransport<readonly SolanaUserDecryptPlaintext[]> = {
      async submit() {
        deadline.throwIfAbortedOrExpired();
        // Each attempt is judged against the host as it is then, as the Connector rechecks the KMS
        // context on every attempt.
        const host = await fetchHostDecryptionState(rpc, programAddress, fields.kmsRouting.kmsContextId);
        assertTrustMatchesHost(host, trust, fields.kmsRouting.kmsContextId);

        // The relayer, when the request is posted: the signature over the chain the handles name,
        // which admission made the permit's, then that chain.
        try {
          verifySolanaPermitSignature(fields, signature);
        } catch (error) {
          const message = `the permit signature does not verify: ${String(error)}`;
          return { ok: false, rejection: { kind: 'refused', label: 'validation_failed', message } };
        }
        if (fields.chainId !== host.config.chainId) {
          const message = `the handles are on host chain ${fields.chainId}; this host is chain ${host.config.chainId}`;
          return { ok: false, rejection: { kind: 'refused', label: 'host_chain_id_not_supported', message } };
        }
        // The relayer's pre-check, before it spends a gateway transaction.
        const delegationRefusal = await solanaRelayerDelegationRefusal({
          programAddress,
          fields,
          entries,
          readAccounts,
        });
        if (delegationRefusal !== undefined) {
          return {
            ok: false,
            rejection: { kind: 'failed', label: 'not_allowed_on_host_acl', message: delegationRefusal },
          };
        }
        // The gateway reverts a request outside its window, and the relayer reports the revert as its own error.
        const now = BigInt(Math.floor(Date.now() / 1000));
        const end = fields.startTimestamp + fields.durationSeconds;
        if (fields.startTimestamp > now || end < now) {
          const message = `the gateway refuses the window [${fields.startTimestamp}, ${end}] at ${now}`;
          return { ok: false, rejection: { kind: 'failed', label: 'internal_server_error', message } };
        }

        // The Connector.
        if (host.context.destroyed) {
          neverAnswered(
            'KmsContextDestroyed',
            `KMS context ${bytesToHex(fields.kmsRouting.kmsContextId)} is destroyed`,
          );
        }
        const verdict = await judgeSolanaUserDecryption({
          programAddress,
          now,
          fields,
          signature,
          entries,
          readAccounts,
          readLeafProofs,
        });
        const rejection = cleartextUserDecryptRejection(verdict);
        if (rejection !== undefined) return { ok: false, rejection };
        const plaintexts = await Promise.all(
          entries.map(async (entry) => ({
            bytes: await fetchCleartextStoreValue(rpc, programAddress, entry.encryptedStore, entry.handle),
            fheTypeId: bytes32ToHandle(asBytes32(entry.handle)).fheTypeId,
          })),
        );
        return { ok: true, response: plaintexts };
      },
    };
    const { response } = await runSolanaUserDecrypt({
      signedPermit: session.signedPermit,
      entries,
      transport,
      clock: deadline,
      ...(attempts === undefined ? {} : { attempts }),
    });
    deadline.throwIfAbortedOrExpired();
    verifySolanaUserDecryptPlaintexts(
      response,
      entries.map((entry) => entry.handle),
    );
    return response;
  };
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
): Promise<HostDecryptionState> {
  const [{ data: config }, { data: context }] = await Promise.all([
    fetchHostConfig(rpc, (await findHostConfigPda({ programAddress }))[0], CONFIRMED),
    fetchKmsContext(rpc, (await findKmsContextPda({ contextId }, { programAddress }))[0], CONFIRMED),
  ]);
  return { config, context };
}

/**
 * Throws when the trust configuration names another KMS or gateway than the host registers: on the
 * real stack, response verification would reject every answer.
 */
function assertTrustMatchesHost(
  { config, context }: HostDecryptionState,
  trust: SolanaDecryptTrust,
  contextId: Uint8Array,
): void {
  const domain = trust.gatewayEip712Domain;
  const decryptionContract = bytesToHex(new Uint8Array(config.decryptionContract));
  if (domain.chainId !== config.gatewayChainId || domain.verifyingContract.toLowerCase() !== decryptionContract) {
    throw new Error(
      `trust.gatewayEip712Domain is chain ${domain.chainId}, contract ${domain.verifyingContract}; the host registers ` +
        `chain ${config.gatewayChainId}, contract ${decryptionContract}`,
    );
  }
  const trusted = trust.kmsSigners.map((signer) => signer.address.toLowerCase()).sort();
  const registered = context.signers.map((signer) => bytesToHex(new Uint8Array(signer))).sort();
  if (trusted.join() !== registered.join()) {
    throw new Error(
      `trust.kmsSigners are ${trusted.join(', ')}; KMS context ${bytesToHex(contextId)} registers ${registered.join(', ')}`,
    );
  }
}

/** One `getMultipleAccounts` read of the host accounts, at a slot no older than `minContextSlot`. */
const hostAccountsReader =
  (rpc: SolanaRpc): SolanaHostAccountsReader =>
  async (keys, minContextSlot) => {
    const { context, value } = await rpc
      .getMultipleAccounts(keys, {
        ...CONFIRMED,
        encoding: 'base64',
        ...(minContextSlot === undefined ? {} : { minContextSlot }),
      })
      .send();
    if (value.length !== keys.length) {
      throw new Error(`getMultipleAccounts returned ${value.length} accounts for ${keys.length} keys`);
    }
    return {
      slot: context.slot,
      accounts: keys.map((key, index) => parseBase64RpcAccount(key, value[index] ?? null)),
    };
  };

/** A request the Connector will never answer: the relayer would time it out after its own timeout. */
function neverAnswered(failure: string, message: string): never {
  throw new Error(
    `the KMS Connector refuses this request for good (${failure}: ${message}); on the real stack it is never ` +
      `answered, and the relayer ends it as response_timed_out`,
  );
}

/**
 * What the caller sees of the Connector's verdict on the real stack. A failure a later attempt may
 * clear, the Connector retries itself: here the attempt goes unanswered and the retry loop submits
 * it again. Any other failure fails at once, where the real stack would leave the request to time out.
 */
export function cleartextUserDecryptRejection(verdict: ConnectorVerdict): SolanaUserDecryptRejection | undefined {
  if (verdict.authorized) return undefined;
  if (CONNECTOR_FAILURE_RECOVERABLE[verdict.failure]) return { kind: 'unanswered' };
  return neverAnswered(verdict.failure, verdict.message);
}

/**
 * Certifies the plaintext the host recorded for a handle made public, signed by the registered
 * cleartext KMS key under the certificate's context, for the on-chain verifier to check.
 */
export function cleartextPublicDecryptCertifier(
  rpc: SolanaRpc,
  chain: FhevmSolanaChain,
  readLeafProofs: SolanaLeafProofReader,
): SolanaPublicDecryptCertifier {
  const programAddress = solanaHostProgram(chain);
  const readAccounts = hostAccountsReader(rpc);
  return async (parameters) => {
    const handle = toFhevmHandle(parameters.handle);
    const handleBytes = hexToBytes(handle.bytes32Hex);
    const encryptedStore = getAddressDecoder().decode(parameters.encryptedStore);
    // The Connector retries a failure a later attempt may clear, such as a leaf record behind the store.
    for (let attempt = 1; ; attempt += 1) {
      parameters.options?.signal?.throwIfAborted();
      const verdict = await judgeSolanaPublicDecryption({
        programAddress,
        handles: [{ handle: handleBytes, encryptedStore }],
        readAccounts,
        readLeafProofs,
      });
      if (verdict.authorized) break;
      if (!CONNECTOR_FAILURE_RECOVERABLE[verdict.failure] || attempt === PUBLIC_DECRYPT_ATTEMPTS) {
        throw new Error(`the KMS Connector refuses to decrypt (${verdict.failure}): ${verdict.message}`);
      }
      await new Promise((resolve) => setTimeout(resolve, PUBLIC_DECRYPT_RETRY_MS));
    }
    const cleartext = await fetchCleartextStoreValue(rpc, programAddress, parameters.encryptedStore, handleBytes);
    const { config, context } = await fetchHostDecryptionState(rpc, programAddress, parameters.contextId);
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
