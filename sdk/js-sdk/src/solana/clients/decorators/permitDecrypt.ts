// The permit-path decrypt actions: sign a permit once, run requests under it.
//
// This decorator is assembly and nothing else. Every rule it relies on lives in the modules it
// fastens together — the permit builder and channel, the RPC evidence source, the relayer
// transport, the retry session, the response verification — and what it adds is the wiring: the
// chain says where the deployment is, the trust configuration says whom to believe, and the caller
// brings the wallet and the handles.
//
// The trust configuration is caller-supplied deliberately. The KMS signer set and the routing pair
// live in on-chain and management state whose read paths are open questions to their owners (the
// party-id mapping of `KmsContext.signers`, the epoch source); until those close, a caller that
// knows its deployment hands the values in explicitly, and nothing here pretends to have read them
// from a canonical source.

import type { Bytes32Hex } from '../../../core/types/primitives.js';
import type { ClearValue } from '../../../core/types/encryptedTypes-p.js';
import type { FhevmRuntime } from '../../../core/types/coreFhevmRuntime.js';
import type { FhevmSolanaChain } from '../../../core/types/fhevmSolanaChain.js';
import type { RelayerUserDecryptOptions } from '../../../core/types/relayer.js';
import type { SolanaPermitWallet } from '../../permit/index.js';
import type {
  SolanaGatewayEip712Domain,
  SolanaHandleRequest,
  SolanaKmsSigner,
  SolanaPermitSession,
  SolanaUserDecryptPlaintext,
} from '../../userDecrypt/index.js';
import { createSolanaRpc } from '@solana/kit';
import {
  PERMIT_KMS_ROUTING_VERSION,
  decodeSolanaPermitFields,
  encodeSolanaKmsRouting,
  normalizeSolanaPermitStart,
  signSolanaPermit,
  solanaPermitWarnings,
} from '../../permit/index.js';
import {
  createSolanaRpcAccessEvidenceSource,
  createSolanaUserDecryptRelayerTransport,
  executeSolanaUserDecrypt,
  generateSolanaTransportKeyPair,
} from '../../userDecrypt/index.js';
import { bytes32ToHandle } from '../../../core/handle/FhevmHandle.js';
import { bytesToClearValueType } from '../../../core/handle/FheType.js';
import { createClearValue } from '../../../core/handle/ClearValue.js';
import { hexToBytes32, isBytes32 } from '../../../core/base/bytes.js';

/** The origin of every clear value this path produces; nothing outside this module can mint one. */
const SOLANA_PERMIT_USER_DECRYPT_TOKEN = Symbol('fhevm.solana.permit-user-decrypt');

/**
 * Whom the permit path believes: the KMS signer set, the routing pair permits are minted for, the
 * FHE parameter, and the gateway domain node signatures verify under.
 *
 * Every field is caller-supplied while the canonical read paths remain open questions to their
 * owners; see the module comment.
 */
export interface SolanaDecryptTrust {
  /** The registered KMS signer set — the trust anchor of response verification. */
  readonly kmsSigners: readonly SolanaKmsSigner[];
  /** The KMS context id new permits are minted for. */
  readonly kmsContextId: Bytes32Hex;
  /** The KMS epoch id new permits are minted for. */
  readonly kmsEpochId: Bytes32Hex;
  /** The FHE parameter choice this deployment runs, e.g. `default` or `test`. */
  readonly fheParameter: string;
  /** The gateway EIP-712 domain; absent, every real response is refused fail-closed. */
  readonly gatewayEip712Domain?: SolanaGatewayEip712Domain | undefined;
}

export interface SolanaSignPermitParameters {
  /** The wallet account that is the permit's user; asked to sign exactly once. */
  readonly wallet: SolanaPermitWallet;
  /** How long the permit lives, in seconds from its start. */
  readonly durationSeconds: bigint;
  /** The ACL-domain scope; defaults to the chain's. An empty list is the permissive permit. */
  readonly allowedAclDomainKeys?: readonly Bytes32Hex[] | undefined;
  /** The signer's revocation watermark; `0n` (the default) when none was ever recorded. */
  readonly invalidationWatermark?: bigint | undefined;
}

/** One handle to decrypt, and the caller's knowledge of where its value lives. */
export interface SolanaUserDecryptEntry {
  /** The 32-byte ciphertext handle. */
  readonly handle: Uint8Array;
  /** The 32-byte id of the `EncryptedValue` account holding this value. */
  readonly encryptedValueId: Uint8Array;
  /** The pubkey whose value this asks for; defaults to the permit's own user. */
  readonly subject?: Uint8Array | undefined;
}

export interface SolanaUserDecryptParameters {
  readonly session: SolanaPermitSession;
  readonly entries: readonly SolanaUserDecryptEntry[];
  /** Submission budget; the session default applies when absent. */
  readonly attempts?: number | undefined;
  /** Core relayer options (auth, timeout, abort signal, progress callback). */
  readonly options?: RelayerUserDecryptOptions | undefined;
}

export type SolanaPermitDecryptActions = {
  /** Creates a permit and takes it to the wallet once; the session it returns is reusable. */
  readonly signPermit: (parameters: SolanaSignPermitParameters) => Promise<SolanaPermitSession>;
  /** Runs one user decryption under a signed permit, to typed clear values. */
  readonly userDecrypt: (parameters: SolanaUserDecryptParameters) => Promise<readonly ClearValue[]>;
};

/**
 * Builds the permit-path actions for one deployment and one trust configuration.
 *
 * @param chain - Where the deployment is; `rpcUrl`, `proofServiceUrl` and `verifyingProgramId`
 * are required here, unlike on the public-decrypt-only surface.
 * @param trust - Whom to believe; see {@link SolanaDecryptTrust}.
 * @param runtime - The client runtime; its configured auth reaches every relayer submission,
 * with per-call options taking precedence — the same merge the public-decrypt action runs.
 * @throws If the chain does not name the endpoints and identity the permit path stands on.
 */
export function solanaPermitDecryptActions(
  chain: FhevmSolanaChain,
  trust: SolanaDecryptTrust,
  runtime: FhevmRuntime,
): SolanaPermitDecryptActions {
  // Fail at construction, not mid-session: a chain missing a deployment field would otherwise
  // surface as a failure of whichever request first needed it.
  const rpcUrl = requiredChainField(chain.fhevm.rpcUrl, 'rpcUrl');
  const proofServiceUrl = requiredChainField(chain.fhevm.proofServiceUrl, 'proofServiceUrl');
  const verifyingProgramId = hexToBytes32(requiredChainField(chain.fhevm.verifyingProgramId, 'verifyingProgramId'));

  return {
    async signPermit(parameters: SolanaSignPermitParameters): Promise<SolanaPermitSession> {
      const keyPair = await generateSolanaTransportKeyPair();
      const now = BigInt(Math.floor(Date.now() / 1000));
      const startTimestamp = normalizeSolanaPermitStart({
        now,
        invalidationWatermark: parameters.invalidationWatermark ?? 0n,
      });
      const fields = decodeSolanaPermitFields({
        userPubkey: Uint8Array.from(parameters.wallet.account.publicKey),
        transportKey: keyPair.publicKeyBytes,
        allowedAclDomainKeys: (parameters.allowedAclDomainKeys ?? chain.fhevm.acl.domainKeys).map((key) =>
          hexToBytes32(key),
        ),
        startTimestamp,
        durationSeconds: parameters.durationSeconds,
        verifyingProgramId,
        chainId: chain.id,
        extraData: encodeSolanaKmsRouting({
          version: PERMIT_KMS_ROUTING_VERSION,
          kmsContextId: hexToBytes32(trust.kmsContextId),
          kmsEpochId: hexToBytes32(trust.kmsEpochId),
        }),
      });
      const warnings = solanaPermitWarnings(fields);
      const signedPermit = await signSolanaPermit(parameters.wallet, fields);
      return { signedPermit, keyPair, warnings };
    },

    async userDecrypt(parameters: SolanaUserDecryptParameters): Promise<readonly ClearValue[]> {
      const userPubkey = parameters.session.signedPermit.fields.userPubkey;
      const requests: readonly SolanaHandleRequest[] = parameters.entries.map((entry) => ({
        handle: entry.handle,
        subject: entry.subject ?? userPubkey,
        encryptedValueId: entry.encryptedValueId,
      }));

      const plaintexts = await executeSolanaUserDecrypt({
        session: parameters.session,
        requests,
        evidence: createSolanaRpcAccessEvidenceSource({
          rpc: createSolanaRpc(rpcUrl),
          proofService: { proofServiceUrl },
          // The permit's verifying program IS the host program encrypted value accounts live under.
          hostProgramId: verifyingProgramId,
        }),
        transport: createSolanaUserDecryptRelayerTransport({
          relayerUrl: chain.fhevm.relayerUrl,
          options: { auth: runtime.config.auth, ...parameters.options },
        }),
        clock: { delay: (seconds) => new Promise((resolve) => setTimeout(resolve, seconds * 1000)) },
        attempts: parameters.attempts,
        verification: {
          signers: trust.kmsSigners,
          fheParameter: trust.fheParameter,
          gatewayEip712Domain: trust.gatewayEip712Domain,
        },
      });

      return toClearValues(
        plaintexts,
        requests.map((request) => request.handle),
      );
    },
  };
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
): readonly ClearValue[] {
  return plaintexts.map((plaintext, index) => {
    const handle = handles[index];
    // The response layer verified one plaintext per handle; the narrowings re-state that for the
    // type system.
    if (handle === undefined || !isBytes32(handle)) {
      throw new Error(`no 32-byte handle stands at position ${index} of a verified response`);
    }
    const fhevmHandle = bytes32ToHandle(handle);
    return createClearValue({
      value: bytesToClearValueType(fhevmHandle.fheType, plaintext.bytes),
      handle: fhevmHandle,
      originToken: SOLANA_PERMIT_USER_DECRYPT_TOKEN,
    });
  });
}

/**
 * A chain field the permit path stands on, or a refusal naming it.
 *
 * @param value - The configured value, possibly absent.
 * @param name - The field's name in the chain definition.
 */
function requiredChainField<T>(value: T | undefined, name: string): T {
  if (value === undefined) {
    throw new Error(
      `the permit-path decrypt actions need \`chain.fhevm.${name}\`, and this chain definition does not set it`,
    );
  }
  return value;
}
