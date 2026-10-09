import type { Address } from '@solana/kit';
import { getAddressDecoder, getAddressEncoder } from '@solana/kit';
// The permit-path decrypt actions: sign a permit once, run requests under it.
//
// This decorator is assembly and nothing else. Every rule it relies on lives in the modules it
// fastens together — the permit builder and channel, the relayer transport, the retry session, the
// response verification — and what it adds is the wiring: the chain says where the deployment is,
// the host program says whom to believe, and the caller brings the wallet and the handles.
//
// The KMS trust is read from the host, as the EVM SDK reads it from `ProtocolConfig` and
// `KMSVerifier`: a permit is minted for the host's active context and epoch, and a response is
// verified against the signers and gateway domain of the context the permit names.

import type { TypedValue } from '../../../core/types/primitives.js';
import type { FhevmRuntime } from '../../../core/types/coreFhevmRuntime.js';
import type { FhevmSolanaChain } from '../../../core/types/fhevmSolanaChain.js';
import type { RelayerUserDecryptOptions } from '../../../core/types/relayer.js';
import type { SolanaPermitWallet } from '../../permit/index.js';
import type {
  SolanaPermitSession,
  SolanaTransportKeyPair,
  SolanaUserDecryptHandleEntry,
  SolanaUserDecryptPlaintext,
} from '../../userDecrypt/index.js';
import {
  PERMIT_KMS_ROUTING_VERSION,
  decodeSolanaPermitFields,
  encodeSolanaKmsRouting,
  normalizeSolanaPermitStart,
  signSolanaPermit,
  solanaPermitWarnings,
} from '../../permit/index.js';
import {
  createSolanaUserDecryptRelayerTransport,
  executeSolanaUserDecrypt,
  generateSolanaTransportKeyPair,
} from '../../userDecrypt/index.js';
import { bytes32ToHandle } from '../../../core/handle/FhevmHandle.js';
import { bytesToClearValueType } from '../../../core/handle/FheType.js';
import { createClearValue, clearValueToTypedValue } from '../../../core/handle/ClearValue.js';
import { hexToBytes32, isBytes32 } from '../../../core/base/bytes.js';
import { readActiveKmsRouting, userDecryptVerification, type SolanaHostKmsReads } from '../../actions/hostKms.js';

/** The origin of every clear value this path produces; nothing outside this module can mint one. */
const SOLANA_PERMIT_USER_DECRYPT_TOKEN = Symbol('fhevm.solana.permit-user-decrypt');

/** One `(program, scope)` pair a permit may be restricted to. */
export interface SolanaPermitScope {
  /** The application program. */
  readonly program: Address;
  /** The scope: an account that program owns, e.g. the mint for the token program. */
  readonly scope: Address;
}

export interface SolanaSignPermitParameters {
  /** The wallet account that is the permit's user; asked to sign exactly once. */
  readonly wallet: SolanaPermitWallet;
  /** How long the permit lives, in seconds from its start. */
  readonly durationSeconds: bigint;
  /**
   * The `(program, scope)` pairs the permit covers, in any order; the permit signs them sorted by
   * bytes. Absent or empty is the permissive permit.
   */
  readonly allowedScopes?: readonly SolanaPermitScope[] | undefined;
}

/** One handle to decrypt, and the caller's knowledge of where its value lives. */
export interface SolanaUserDecryptEntry {
  /** The 32-byte ciphertext handle. */
  readonly handle: Uint8Array;
  /** The 32-byte address of the `EncryptedStore` account holding this value. */
  readonly encryptedStore: Uint8Array;
  /**
   * The key whose allow on the handle this asks under: the delegator on a delegated entry.
   * Defaults to the permit's own user.
   */
  readonly ownerAddress?: Uint8Array | undefined;
}

export interface SolanaUserDecryptParameters {
  readonly session: SolanaPermitSession;
  readonly entries: readonly SolanaUserDecryptEntry[];
  /** Submission budget; the session default applies when absent. */
  readonly attempts?: number | undefined;
  /** Relayer options; timeout bounds the whole operation, including retries and backoff. */
  readonly options?: RelayerUserDecryptOptions | undefined;
}

/**
 * How user decryptions under a permit are answered: through the relayer and the KMS response
 * verification, unless a cleartext client reads the plaintexts.
 */
export interface SolanaUserDecryptExecution {
  /** The transport keypair a new permit commits to; called once per permit. */
  readonly transportKeyPair: () => Promise<SolanaTransportKeyPair>;
  /** Answers one request with plaintexts verified against the handles. */
  readonly execute: (run: {
    readonly session: SolanaPermitSession;
    readonly entries: readonly SolanaUserDecryptHandleEntry[];
    readonly attempts: number | undefined;
    readonly options: RelayerUserDecryptOptions | undefined;
  }) => Promise<readonly SolanaUserDecryptPlaintext[]>;
}

export type SolanaPermitDecryptActions = {
  /** Creates a permit and takes it to the wallet once; the session it returns is reusable. */
  readonly signPermit: (parameters: SolanaSignPermitParameters) => Promise<SolanaPermitSession>;
  /** Runs one user decryption under a signed permit, to typed clear values. */
  readonly decryptValues: (parameters: SolanaUserDecryptParameters) => Promise<readonly TypedValue[]>;
};

/**
 * Answers user decryptions through the relayer, verifying each response against the host's record
 * of the permit's KMS context.
 *
 * @param chain - Where the deployment is; the relayer it names answers.
 * @param host - The client's host KMS reads.
 * @param runtime - The client runtime; its configured auth reaches every relayer submission,
 * with per-call options taking precedence — the same merge the public-decrypt action runs.
 * @param fheParameter - The FHE parameter choice this deployment runs, e.g. `default` or `test`.
 */
export function relayerUserDecryptExecution(
  chain: FhevmSolanaChain,
  host: SolanaHostKmsReads,
  runtime: FhevmRuntime,
  fheParameter: string,
): SolanaUserDecryptExecution {
  return {
    transportKeyPair: () => generateSolanaTransportKeyPair(runtime),
    execute: async ({ session, entries, attempts, options }) => {
      const transport = createSolanaUserDecryptRelayerTransport({
        relayerUrl: chain.fhevm.relayerUrl,
        logger: runtime.config.logger,
        options: { auth: runtime.config.auth, ...options },
      });
      const verification = await userDecryptVerification(
        host,
        session.signedPermit.fields.kmsRouting.kmsContextId,
        fheParameter,
      );
      const plaintexts = await executeSolanaUserDecrypt({
        runtime,
        session,
        entries,
        transport,
        clock: transport,
        attempts,
        verification,
      });
      transport.throwIfAbortedOrExpired();
      return plaintexts;
    },
  };
}

/**
 * Builds the permit-path actions for one deployment.
 *
 * @param chain - Where the deployment is; permits are signed for its host program id.
 * @param host - The client's host KMS reads; permits are minted for the active context and epoch.
 * @param execution - How requests under a permit are answered.
 */
export function solanaPermitDecryptActions(
  chain: FhevmSolanaChain,
  host: SolanaHostKmsReads,
  fetchPermitInvalidation: (user: Address) => Promise<bigint>,
  { transportKeyPair, execute }: SolanaUserDecryptExecution,
): SolanaPermitDecryptActions {
  return {
    async signPermit(parameters: SolanaSignPermitParameters): Promise<SolanaPermitSession> {
      const invalidationWatermark = await fetchPermitInvalidation(
        getAddressDecoder().decode(parameters.wallet.account.publicKey),
      );
      const { contextId, epochId } = await readActiveKmsRouting(chain, host);
      const keyPair = await transportKeyPair();
      const now = BigInt(Math.floor(Date.now() / 1000));
      const startTimestamp = normalizeSolanaPermitStart({
        now,
        invalidationWatermark,
      });
      const fields = decodeSolanaPermitFields({
        userAddress: Uint8Array.from(parameters.wallet.account.publicKey),
        transportKey: keyPair.publicKeyBytes,
        allowedScopes: sortedScopes(parameters.allowedScopes ?? []),
        startTimestamp,
        durationSeconds: parameters.durationSeconds,
        verifyingProgramId: hexToBytes32(chain.fhevm.programs.host.address),
        chainId: chain.id,
        extraData: encodeSolanaKmsRouting({
          version: PERMIT_KMS_ROUTING_VERSION,
          kmsContextId: contextId,
          kmsEpochId: epochId,
        }),
      });
      const warnings = solanaPermitWarnings(fields);
      const signedPermit = await signSolanaPermit(parameters.wallet, fields);
      return { signedPermit, keyPair, warnings };
    },

    async decryptValues(parameters: SolanaUserDecryptParameters): Promise<readonly TypedValue[]> {
      const userAddress = parameters.session.signedPermit.fields.userAddress;
      const entries: readonly SolanaUserDecryptHandleEntry[] = parameters.entries.map((entry) => ({
        handle: entry.handle,
        ownerAddress: entry.ownerAddress ?? userAddress,
        encryptedStore: entry.encryptedStore,
      }));

      const plaintexts = await execute({
        session: parameters.session,
        entries,
        attempts: parameters.attempts,
        options: parameters.options,
      });
      return toClearValues(
        plaintexts,
        entries.map((entry) => entry.handle),
      );
    },
  };
}

/**
 * The 64-byte `program ‖ scope` entries in the order the permit signs them: ascending by bytes.
 * Sorting here is what lets a caller list scopes in any order; a duplicate still reaches the
 * decoder and is refused there.
 *
 * @param scopes - The pairs, in any order.
 */
function sortedScopes(scopes: readonly SolanaPermitScope[]): readonly Uint8Array[] {
  const encoder = getAddressEncoder();
  return scopes
    .map((pair) => {
      const bytes = new Uint8Array(64);
      bytes.set(encoder.encode(pair.program), 0);
      bytes.set(encoder.encode(pair.scope), 32);
      return bytes;
    })
    .sort((a, b) => {
      for (let index = 0; index < a.length; index += 1) {
        const difference = (a[index] ?? 0) - (b[index] ?? 0);
        if (difference !== 0) {
          return difference;
        }
      }
      return 0;
    });
}

/**
 * Decodes verified plaintexts into typed clear values, under this path's origin token.
 *
 * The count and the per-position FHE type were already verified against the handles by the
 * response layer; this is the same decode the EVM path runs after its own identical check.
 *
 * @param plaintexts - The verified plaintexts, one per handle.
 * @param handles - The requested handles, in request order.
 */
function toClearValues(
  plaintexts: readonly SolanaUserDecryptPlaintext[],
  handles: readonly Uint8Array[],
): readonly TypedValue[] {
  return plaintexts.map((plaintext, index) => {
    const handle = handles[index];
    // The response layer verified one plaintext per handle; the narrowings re-state that for the
    // type system.
    if (handle === undefined || !isBytes32(handle)) {
      throw new Error(`no 32-byte handle stands at position ${index} of a verified response`);
    }
    const fhevmHandle = bytes32ToHandle(handle);
    return clearValueToTypedValue(
      createClearValue({
        value: bytesToClearValueType(fhevmHandle.fheType, plaintext.bytes),
        handle: fhevmHandle,
        originToken: SOLANA_PERMIT_USER_DECRYPT_TOKEN,
      }),
      SOLANA_PERMIT_USER_DECRYPT_TOKEN,
    );
  });
}
