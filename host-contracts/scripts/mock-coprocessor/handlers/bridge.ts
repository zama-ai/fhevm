/**
 * ConfidentialBridge event handler — propagates clear-text associations across
 * chains. The actual cryptographic bridging is done by the LayerZero V2 OApp
 * machinery; this handler just keeps the mock DB consistent so destination-
 * chain handles resolve to the same plaintext as their source-chain twins.
 *
 * Events handled:
 *
 *   HandleBridged(receiverDapp, srcHandle, dstHandle, guid)
 *     The destination bridge has derived `dstHandle` from `srcHandle`. We look
 *     up `srcHandle`'s clear text in the (shared, cross-chain) DB and write it
 *     under `dstHandle`. If the source-side event hasn't been processed yet
 *     (different chain, different polling speed), we add the mapping to an
 *     in-memory retry queue and re-attempt on each poll cycle.
 *
 *   FallbackGrantedPlaintext(dstHandle, plaintext)
 *     The bridge owner has manually asserted that `dstHandle` decodes to
 *     `plaintext`. Trust the assertion (the on-chain check ensures the chain
 *     id, version, fheType and plaintext range all line up).
 *
 *   BridgeHandle(senderDapp, srcHandle, dstChainId, guid)
 *     Informational — announces the source side of a bridge call. The matching
 *     HandleBridged on the destination chain carries the dst-handle we
 *     actually need to associate. Nothing to do here.
 */
import type { Result } from 'ethers';
import { ethers } from 'ethers';

import { RUNTIME } from '../config';
import type { MockDb } from '../db';

export interface BridgeEvent {
  eventName: string;
  args: Result;
}

interface PendingBridge {
  srcHandle: string;
  dstHandle: string;
  attemptsLeft: number;
}

/** In-memory retry queue, shared across all chain workers. */
const pending: PendingBridge[] = [];

async function attempt(db: MockDb, entry: PendingBridge): Promise {
  const clearText = await db.getClearText(entry.srcHandle);
  if (clearText === null) return false;
  await db.insertCiphertext(entry.dstHandle, clearText);
  return true;
}

export async function applyBridgeEvent(event: BridgeEvent, db: MockDb): Promise {
  switch (event.eventName) {
    case 'HandleBridged': {
      const srcHandle = ethers.toBeHex(event.args.srcHandle, 32);
      const dstHandle = ethers.toBeHex(event.args.dstHandle, 32);
      const entry: PendingBridge = { srcHandle, dstHandle, attemptsLeft: RUNTIME.bridgeRetryLimit };
      const done = await attempt(db, entry);
      if (done) return 'inserted';
      pending.push(entry);
      return 'pending';
    }

    case 'FallbackGrantedPlaintext': {
      const dstHandle = ethers.toBeHex(event.args.dstHandle, 32);
      await db.insertCiphertext(dstHandle, BigInt(event.args.plaintext));
      return 'inserted';
    }

    case 'BridgeHandle':
      return 'noop';

    default:
      return 'noop';
  }
}

/**
 * Re-attempt every pending HandleBridged mapping. Called by each chain worker
 * after each poll cycle so cross-chain ordering quirks (chain B's
 * HandleBridged seen before chain A's TrivialEncrypt) resolve naturally.
 * Entries that exhaust `RUNTIME.bridgeRetryLimit` attempts are dropped with
 * a loud warning — usually because the daemon was started AFTER the
 * source-chain tx was mined (live-events-only mode skips it).
 */
export async function retryPendingBridges(db: MockDb, logger = console): Promise {
  if (pending.length === 0) return;
  const remaining: PendingBridge[] = [];
  for (const entry of pending) {
    const done = await attempt(db, entry);
    if (done) {
      logger.info(
        `[mock-coprocessor:bridge] resolved pending mapping ${entry.srcHandle.slice(0, 10)}… → ${entry.dstHandle.slice(
          0,
          10
        )}…`
      );
      continue;
    }
    entry.attemptsLeft -= 1;
    if (entry.attemptsLeft <= 0) {
      logger.warn(
        `[mock-coprocessor:bridge] giving up on pending mapping ${entry.srcHandle} → ${entry.dstHandle} — ` +
          `source-side TrivialEncrypt/FheOp event was not seen in time. The daemon only processes live events — ` +
          `the producing tx must be mined AFTER the daemon's "initialised at chain head" line on its origin chain.`
      );
      continue;
    }
    remaining.push(entry);
  }
  pending.length = 0;
  pending.push(...remaining);
}

export function pendingBridgeCount(): number {
  return pending.length;
}
