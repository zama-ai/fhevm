// The KMS's part of a decryption, played by the cleartext client: plaintexts come from the accounts
// the cleartext host wrote. Everything around them runs as in production: the permit and request
// admission of a user decryption, and the on-chain checks of a public-decrypt certificate, which
// the client signs with the registered cleartext KMS key.
//
// Before answering, the client judges a request as the KMS Connector does
// (kms-connector/crates/kms-worker/src/core/solana/pipeline.rs), reading the same host state: a user
// decryption needs a signed permit for this host, inside its validity window and not revoked, a
// store in the permit's scopes, a live delegation when the entry's owner is not the signer, and an
// allow leaf for the owner on the handle; a public decryption needs the handle made public. What it
// does not reproduce is the KMS itself: no signcryption, no response signatures, and so no check of
// the FHE parameter or the KMS epoch. On the real stack the Connector's refusals come back
// unanswered and the relayer's pre-check refuses some of them earlier; here every refusal comes
// back as the relayer's `not_allowed_on_host_acl`.
import { fetchEncodedAccount, getAddressDecoder, getAddressEncoder, getI64Decoder, type Address } from '@solana/kit';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaRpc } from '../encryptedStore.js';
import type { SolanaDecryptTrust, SolanaUserDecryptExecution } from '../clients/decorators/permitDecrypt.js';
import type { SolanaPermitFields } from '../permit/types.js';
import type {
  SolanaUserDecryptHandleEntry,
  SolanaUserDecryptPlaintext,
  SolanaUserDecryptTransport,
} from '../userDecrypt/index.js';
import type { SolanaPublicDecryptCertifier } from '../actions/publicDecryptCertificate.js';
import { asBytes32, bytesToHex, bytesToHexNo0x, concatBytes, hexToBytes } from '../../core/base/bytes.js';
import { bytes32ToHandle, toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { createKmsPublicDecryptEip712, publicDecryptDigest } from '../../core/kms/createKmsPublicDecryptEip712.js';
import { runSolanaUserDecrypt } from '../userDecrypt/index.js';
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
import { fetchHostConfig } from '../internal/generated/zamaHost/accounts/hostConfig.js';
import { fetchKmsContext } from '../internal/generated/zamaHost/accounts/kmsContext.js';
import { solanaHostProgram } from '../clients/createFhevmBaseClient.js';
import { signAsCleartextParty } from './parties.js';
import { fetchSolanaStoreHistory } from './storeHistory.js';
import { fetchCleartextStoreValue } from './storeValues.js';

////////////////////////////////////////////////////////////////////////////////

const CONFIRMED = { commitment: 'confirmed' } as const;
const CLOCK_SYSVAR = 'SysvarC1ock11111111111111111111111111111111' as Address;
/** Offset of `unix_timestamp` in the Clock sysvar. */
const CLOCK_UNIX_TIMESTAMP_OFFSET = 32;

/**
 * Answers a user decryption with the plaintexts the host recorded for its handles, once the request
 * passes the Connector's authorization. A refusal goes through the production retry loop, which
 * gives up on it as on the relayer's.
 */
export function cleartextUserDecryptExecution(
  rpc: SolanaRpc,
  chain: FhevmSolanaChain,
  trust: SolanaDecryptTrust,
): SolanaUserDecryptExecution {
  const programAddress = solanaHostProgram(chain);
  return async ({ session, entries, attempts }) => {
    const { fields, signature } = session.signedPermit;
    await assertTrustMatchesHost(rpc, programAddress, trust, fields.kmsRouting.kmsContextId);
    const transport: SolanaUserDecryptTransport<readonly SolanaUserDecryptPlaintext[]> = {
      async submit() {
        const refusal = await userDecryptRefusal(rpc, programAddress, fields, signature, entries);
        if (refusal !== undefined) {
          return { ok: false, rejection: { kind: 'refused', label: 'not_allowed_on_host_acl', message: refusal } };
        }
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
 * Throws when the trust configuration names another KMS or gateway than the host registers: on the
 * real stack, response verification would reject every answer.
 */
async function assertTrustMatchesHost(
  rpc: SolanaRpc,
  programAddress: Address,
  trust: SolanaDecryptTrust,
  contextId: Uint8Array,
): Promise<void> {
  const [{ data: config }, { data: context }] = await Promise.all([
    fetchHostConfig(rpc, (await findHostConfigPda({ programAddress }))[0], CONFIRMED),
    fetchKmsContext(rpc, (await findKmsContextPda({ contextId }, { programAddress }))[0], CONFIRMED),
  ]);
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

/** Why the Connector would refuse the request, in its order of checks, or nothing if it would not. */
async function userDecryptRefusal(
  rpc: SolanaRpc,
  programAddress: Address,
  fields: SolanaPermitFields,
  signature: Uint8Array,
  entries: readonly SolanaUserDecryptHandleEntry[],
): Promise<string | undefined> {
  try {
    verifySolanaPermitSignature(fields, signature);
  } catch (error) {
    return `the permit signature does not verify: ${String(error)}`;
  }
  const now = BigInt(Math.floor(Date.now() / 1000));
  const end = fields.startTimestamp + fields.durationSeconds;
  if (now < fields.startTimestamp || now > end) {
    return `the permit is valid from ${fields.startTimestamp} to ${end}, not at ${now}`;
  }
  const decodeAddress = (bytes: Uint8Array): Address => getAddressDecoder().decode(bytes);
  if (decodeAddress(fields.verifyingProgramId) !== programAddress) {
    return `the permit is signed for host program ${decodeAddress(fields.verifyingProgramId)}, not ${programAddress}`;
  }
  const signer = decodeAddress(fields.userAddress);
  const watermark = await fetchSolanaPermitInvalidation(rpc, signer, { ...CONFIRMED, programAddress });
  if (fields.startTimestamp < watermark) {
    return `the permit starts at ${fields.startTimestamp}, before ${signer} revoked permits up to ${watermark}`;
  }
  const allowedScopes = new Set(fields.allowedScopes.map((scope) => bytesToHex(scope)));
  for (const [index, entry] of entries.entries()) {
    const storeAddress = decodeAddress(entry.encryptedStore);
    const store = await fetchSolanaEncryptedStore(rpc, storeAddress, CONFIRMED, programAddress);
    const application = concatBytes(
      new Uint8Array(getAddressEncoder().encode(store.program)),
      new Uint8Array(getAddressEncoder().encode(store.scope)),
    );
    if (allowedScopes.size > 0 && !allowedScopes.has(bytesToHex(application))) {
      return `entry ${index}: the permit's scopes do not include (${store.program}, ${store.scope})`;
    }
    const owner = decodeAddress(entry.ownerAddress);
    if (owner !== signer) {
      const rows = await fetchSolanaUserDecryptionDelegation(
        rpc,
        { delegator: owner, delegate: signer, program: store.program, scope: store.scope },
        { ...CONFIRMED, programAddress },
      );
      const hostNow = await fetchHostUnixTimestamp(rpc);
      if (
        ![rows.exact, rows.wildcard].some((row) => row !== null && isSolanaUserDecryptionDelegationLiveAt(row, hostNow))
      ) {
        return `entry ${index}: ${owner} has no live delegation to ${signer} for (${store.program}, ${store.scope})`;
      }
    }
    const [handle, key] = [bytesToHex(entry.handle), bytesToHex(entry.ownerAddress)];
    const history = await fetchSolanaStoreHistory(rpc, storeAddress, programAddress);
    if (
      !history.some(
        (event) => event.kind === 'allowed' && bytesToHex(event.handle) === handle && bytesToHex(event.key) === key,
      )
    ) {
      return `entry ${index}: ${owner} is not allowed on handle ${handle} in EncryptedStore ${storeAddress}`;
    }
  }
  return undefined;
}

/** The host's Clock, which the Connector judges delegations against. */
async function fetchHostUnixTimestamp(rpc: SolanaRpc): Promise<bigint> {
  const clock = await fetchEncodedAccount(rpc, CLOCK_SYSVAR, CONFIRMED);
  if (!clock.exists) throw new Error('the Clock sysvar is missing');
  return getI64Decoder().decode(clock.data, CLOCK_UNIX_TIMESTAMP_OFFSET);
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
