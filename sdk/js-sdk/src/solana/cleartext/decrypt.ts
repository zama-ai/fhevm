// The KMS's part of a decryption, played by the cleartext client: plaintexts come from the accounts
// the cleartext host wrote. Everything around them runs as in production: the permit and request
// admission of a user decryption, and the on-chain checks of a public-decrypt certificate, which
// the client signs with the registered cleartext KMS key.
//
// Before answering, the client judges a request as the KMS Connector does
// (kms-connector/crates/kms-worker/src/core/solana/pipeline.rs), reading the same host state: a user
// decryption needs a KMS context that is not destroyed, a signed permit for this host chain and
// program, inside its validity window and not revoked, a store in the permit's scopes, a live
// delegation when the entry's owner is not the signer, and an allow leaf for the owner on the handle;
// a public decryption needs the handle made public. What it
// does not reproduce is the KMS itself: no signcryption, no response signatures, and so no check of
// the FHE parameter or the KMS epoch. On the real stack the Connector's refusals come back
// unanswered and the relayer's pre-check refuses some of them earlier; here every refusal comes
// back as the relayer's `not_allowed_on_host_acl`, and a store history that lags the store's leaf
// count comes back unanswered, which the retry loop submits again.
import { getAddressDecoder, getAddressEncoder, type Address } from '@solana/kit';
import { fetchSysvarClock } from '@solana/sysvars';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaEncryptedStore, SolanaRpc } from '../encryptedStore.js';
import type { SolanaStoreHistoryEvent } from '../proof.js';
import type { SolanaDecryptTrust, SolanaUserDecryptExecution } from '../clients/decorators/permitDecrypt.js';
import type { SolanaPermitFields } from '../permit/types.js';
import type {
  SolanaUserDecryptHandleEntry,
  SolanaUserDecryptPlaintext,
  SolanaUserDecryptRejection,
  SolanaUserDecryptTransport,
} from '../userDecrypt/index.js';
import type { SolanaPublicDecryptCertifier } from '../actions/publicDecryptCertificate.js';
import { asBytes32, bytesToHex, bytesToHexNo0x, concatBytes, hexToBytes } from '../../core/base/bytes.js';
import { bytes32ToHandle, toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { createKmsPublicDecryptEip712, publicDecryptDigest } from '../../core/kms/createKmsPublicDecryptEip712.js';
import { runSolanaUserDecrypt } from '../userDecrypt/index.js';
import { createSolanaUserDecryptDeadline } from '../userDecrypt/deadline.js';
import { verifySolanaUserDecryptPlaintexts } from '../userDecrypt/response.js';
import { verifySolanaPermitSignature } from '../permit/envelope.js';
import { fetchSolanaEncryptedStore } from '../encryptedStore.js';
import { fetchSolanaPermitInvalidation } from '../actions/revokePermits.js';
import {
  fetchSolanaUserDecryptionDelegation,
  isSolanaUserDecryptionDelegationLiveAt,
} from '../actions/userDecryptionDelegation.js';
import { solanaPublicDecryptExtraData } from '../actions/publicDecryptCertificate.js';
import { findHostConfigPda } from '../internal/generated/zamaHost/pdas/hostConfig.js';
import { findKmsContextPda } from '../internal/generated/zamaHost/pdas/kmsContext.js';
import { fetchHostConfig, type HostConfig } from '../internal/generated/zamaHost/accounts/hostConfig.js';
import { fetchKmsContext, type KmsContext } from '../internal/generated/zamaHost/accounts/kmsContext.js';
import { solanaHostProgram } from '../clients/createFhevmBaseClient.js';
import { signAsCleartextParty } from './parties.js';
import { fetchSolanaStoreHistory } from './storeHistory.js';
import { fetchCleartextStoreValue } from './storeValues.js';

////////////////////////////////////////////////////////////////////////////////

const CONFIRMED = { commitment: 'confirmed' } as const;

/**
 * Answers a user decryption with the plaintexts the host recorded for its handles, once the request
 * passes the Connector's authorization, through the production retry loop.
 */
export function cleartextUserDecryptExecution(
  rpc: SolanaRpc,
  chain: FhevmSolanaChain,
  trust: SolanaDecryptTrust,
): SolanaUserDecryptExecution {
  const programAddress = solanaHostProgram(chain);
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
        const rejection = await userDecryptRejection(rpc, programAddress, host, fields, signature, entries);
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

/** A Connector refusal, raised by a check and returned as the relayer's label. */
class HostAclRefusal extends Error {}

const refuse = (message: string): never => {
  throw new HostAclRefusal(message);
};

/**
 * What the Connector would do with the request, in its order of checks: nothing if it would answer,
 * the relayer's `not_allowed_on_host_acl` if it would refuse, and no answer (which the retry loop
 * submits again) while the store's history does not reach its leaf count yet, as when coprocessors
 * have not indexed a leaf. A host record it cannot read refuses the request, as in the Connector.
 */
export async function userDecryptRejection(
  rpc: SolanaRpc,
  programAddress: Address,
  { config, context }: HostDecryptionState,
  fields: SolanaPermitFields,
  signature: Uint8Array,
  entries: readonly SolanaUserDecryptHandleEntry[],
): Promise<SolanaUserDecryptRejection | undefined> {
  try {
    if (fields.chainId !== config.chainId) {
      refuse(`the permit is for host chain ${fields.chainId}; this host is chain ${config.chainId}`);
    }
    if (context.destroyed) {
      refuse(`KMS context ${bytesToHex(fields.kmsRouting.kmsContextId)} is destroyed`);
    }
    try {
      verifySolanaPermitSignature(fields, signature);
    } catch (error) {
      refuse(`the permit signature does not verify: ${String(error)}`);
    }
    const now = BigInt(Math.floor(Date.now() / 1000));
    const end = fields.startTimestamp + fields.durationSeconds;
    if (now < fields.startTimestamp || now > end) {
      refuse(`the permit is valid from ${fields.startTimestamp} to ${end}, not at ${now}`);
    }
    const decodeAddress = (bytes: Uint8Array): Address => getAddressDecoder().decode(bytes);
    if (decodeAddress(fields.verifyingProgramId) !== programAddress) {
      refuse(
        `the permit is signed for host program ${decodeAddress(fields.verifyingProgramId)}, not ${programAddress}`,
      );
    }
    const signer = decodeAddress(fields.userAddress);
    const watermark = await fetchSolanaPermitInvalidation(rpc, signer, { ...CONFIRMED, programAddress }).catch(
      (error: unknown) => refuse(`the permit invalidation of ${signer} is unreadable: ${String(error)}`),
    );
    if (fields.startTimestamp < watermark) {
      refuse(`the permit starts at ${fields.startTimestamp}, before ${signer} revoked permits up to ${watermark}`);
    }
    const allowedScopes = new Set(fields.allowedScopes.map((scope) => bytesToHex(scope)));
    let hostNow: bigint | undefined;
    // One read of each store and of its history per attempt, however many entries name it.
    const stores = new Map<Address, SolanaEncryptedStore>();
    const histories = new Map<Address, SolanaStoreHistoryEvent[]>();
    for (const [index, entry] of entries.entries()) {
      const storeAddress = decodeAddress(entry.encryptedStore);
      const store =
        stores.get(storeAddress) ??
        (await fetchSolanaEncryptedStore(rpc, storeAddress, CONFIRMED, programAddress).catch((error: unknown) =>
          refuse(`entry ${index}: ${String(error)}`),
        ));
      stores.set(storeAddress, store);
      const application = concatBytes(
        new Uint8Array(getAddressEncoder().encode(store.program)),
        new Uint8Array(getAddressEncoder().encode(store.scope)),
      );
      if (allowedScopes.size > 0 && !allowedScopes.has(bytesToHex(application))) {
        refuse(`entry ${index}: the permit's scopes do not include (${store.program}, ${store.scope})`);
      }
      const owner = decodeAddress(entry.ownerAddress);
      if (owner !== signer) {
        const rows = await fetchSolanaUserDecryptionDelegation(
          rpc,
          { delegator: owner, delegate: signer, program: store.program, scope: store.scope },
          { ...CONFIRMED, programAddress },
        ).catch((error: unknown) => refuse(`entry ${index}: a delegation record is unreadable: ${String(error)}`));
        // The Connector judges delegations against the host's Clock, not the local one.
        hostNow ??= (await fetchSysvarClock(rpc, CONFIRMED)).unixTimestamp;
        const live = hostNow;
        if (
          ![rows.exact, rows.wildcard].some((row) => row !== null && isSolanaUserDecryptionDelegationLiveAt(row, live))
        ) {
          refuse(`entry ${index}: ${owner} has no live delegation to ${signer} for (${store.program}, ${store.scope})`);
        }
      }
      // The history is read after the store, so it reaches the store's leaf count unless it lags.
      const history = histories.get(storeAddress) ?? (await fetchSolanaStoreHistory(rpc, storeAddress, programAddress));
      histories.set(storeAddress, history);
      if (BigInt(history.length) < store.leafCount) return { kind: 'unanswered' };
      const [handle, key] = [bytesToHex(entry.handle), bytesToHex(entry.ownerAddress)];
      if (
        !history.some(
          (event) => event.kind === 'allowed' && bytesToHex(event.handle) === handle && bytesToHex(event.key) === key,
        )
      ) {
        refuse(`entry ${index}: ${owner} is not allowed on handle ${handle} in EncryptedStore ${storeAddress}`);
      }
    }
    return undefined;
  } catch (error) {
    if (!(error instanceof HostAclRefusal)) throw error;
    return { kind: 'refused', label: 'not_allowed_on_host_acl', message: error.message };
  }
}

/**
 * Certifies the plaintext the host recorded for a handle made public, signed by the registered
 * cleartext KMS key under the certificate's context, for the on-chain verifier to check.
 */
export function cleartextPublicDecryptCertifier(rpc: SolanaRpc, chain: FhevmSolanaChain): SolanaPublicDecryptCertifier {
  const programAddress = solanaHostProgram(chain);
  return async (parameters) => {
    const handle = toFhevmHandle(parameters.handle);
    const handleBytes = hexToBytes(handle.bytes32Hex);
    const encryptedStore = getAddressDecoder().decode(parameters.encryptedStore);
    const history = await fetchSolanaStoreHistory(rpc, encryptedStore, programAddress);
    if (!history.some((event) => event.kind === 'markedPublic' && bytesToHex(event.handle) === handle.bytes32Hex)) {
      throw new Error(`handle ${handle.bytes32Hex} was not made public in EncryptedStore ${encryptedStore}`);
    }
    const cleartext = await fetchCleartextStoreValue(rpc, programAddress, parameters.encryptedStore, handleBytes);
    const [{ data: config }, { data: context }] = await Promise.all([
      fetchHostConfig(rpc, (await findHostConfigPda({ programAddress }))[0], CONFIRMED),
      fetchKmsContext(
        rpc,
        (await findKmsContextPda({ contextId: parameters.contextId }, { programAddress }))[0],
        CONFIRMED,
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
