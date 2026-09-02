// The RPC evidence source: host state for the account, the proof service for a replaced handle.
//
// Each request carries its encrypted value *identity* — the 32-byte id requests carry on the wire —
// and this source resolves it to the account the id names: the PDA under the host program, the
// same `find_program_address([seed, id], program)` the host and the Connector run. The two values
// travel together in the evidence because they answer different questions: the id is what the
// request says, the account pubkey is what the historical-access leaves bind.
//
// One account read decides everything about one resolution. If the requested handle is the
// account's current one, the access is current: no proof, no peaks, a zero leaf count. If it is
// not, an update has replaced it, and the proof service is asked for the historical-access proof —
// but the peaks and the leaf count the evidence carries still come from that same read, because the
// builder verifies the proof against them and a proof only verifies against the snapshot it was
// built for. A service answer built against any other leaf count is refused as an incoherent
// snapshot rather than assembled into evidence that contradicts itself; the account moved between
// the two calls, and the caller's retry re-reads both.
//
// Reads observe the chain at `confirmed` — the same commitment the authorization pipeline reads
// at; `finalized` would lag the very update whose evidence is being resolved.

import type { SolanaAccessEvidence, SolanaAccessEvidenceSource, SolanaHandleRequest } from './evidence.js';
import type { SolanaAccessProofServiceConfig } from './proofService.js';
import type { SolanaRpc } from '../encryptedValueAccount.js';
import { getAddressDecoder, getAddressEncoder } from '@solana/kit';
import { fetchSolanaEncryptedValueState, solanaEncryptedValueAccountAddress } from '../encryptedValueAccount.js';
import { fetchSolanaHistoricalAccessProof } from './proofService.js';
import { unsafeBytesEquals } from '../../core/base/bytes.js';

/**
 * Builds the evidence source a user-decrypt session resolves through.
 *
 * @param config.rpc - The Solana RPC the account is read through.
 * @param config.proofService - Where the standalone proof service lives.
 * @param config.hostProgramId - The 32-byte zama-host program id encrypted value accounts live
 * under; the same deployment identity the permit is signed for.
 */
export function createSolanaRpcAccessEvidenceSource(config: {
  readonly rpc: SolanaRpc;
  readonly proofService: SolanaAccessProofServiceConfig;
  readonly hostProgramId: Uint8Array;
}): SolanaAccessEvidenceSource {
  return {
    async resolve(request: SolanaHandleRequest): Promise<SolanaAccessEvidence> {
      const encryptedValueId = request.encryptedValueId;
      const accountAddress = await solanaEncryptedValueAccountAddress(config.hostProgramId, encryptedValueId);
      const encryptedValueAccount = new Uint8Array(getAddressEncoder().encode(accountAddress));
      const state = await fetchSolanaEncryptedValueState(
        config.rpc,
        accountAddress,
        { commitment: 'confirmed' },
        getAddressDecoder().decode(config.hostProgramId),
      );

      if (unsafeBytesEquals(state.currentHandle, request.handle)) {
        return {
          handle: request.handle,
          subject: request.subject,
          encryptedValueId,
          encryptedValueAccount,
          proofLeafCount: 0n,
          accessProof: new Uint8Array(0),
          peaks: [],
        };
      }

      const proof = await fetchSolanaHistoricalAccessProof(config.proofService, {
        encryptedValueAccount,
        handle: request.handle,
        subject: request.subject,
      });
      // The proof and the peaks must describe one MMR. The service answering for another leaf
      // count means the account moved between the read and the proof; neither party is wrong, and
      // the resolution fails so the caller retries both together.
      if (proof.leafCount !== state.leafCount) {
        throw new Error(
          `the proof service built its proof against leaf count ${proof.leafCount}, and the account read shows ` +
            `${state.leafCount}: the two calls straddle an append and describe different snapshots`,
        );
      }

      return {
        handle: request.handle,
        subject: request.subject,
        encryptedValueId,
        encryptedValueAccount,
        proofLeafCount: state.leafCount,
        accessProof: proof.accessProof,
        peaks: state.peaks,
      };
    },
  };
}
