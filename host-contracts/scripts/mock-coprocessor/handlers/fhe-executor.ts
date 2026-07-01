/**
 * FHEVMExecutor event handler — derives the plaintext result for each FHE
 * operation and stores it in the mock DB.
 *
 * Ported from `test/coprocessorUtils.ts::insertHandleFromEvent`. The behaviour
 * is identical; the only changes are:
 *   - No dependency on hardhat or its `ethers.provider`. Operand clear texts
 *     are read from the passed-in `MockDb` (per-handle, globally unique).
 *   - Idempotent: `MockDb.insertCiphertext` uses `INSERT OR IGNORE` so
 *     re-processing a block range during reorg recovery is safe.
 *   - Throws on operand-not-found rather than infinite-retrying like the
 *     in-process mock — the chain worker processes events in block order so
 *     this only fires on a real desync (e.g. starting from a block that's
 *     past an operand's creation).
 *
 * Keep this file in lockstep with `coprocessorUtils.ts` when new FHE
 * operators are added.
 */
import crypto from 'crypto';
import type { Result } from 'ethers';
import { ethers } from 'ethers';
import { log2 } from 'extra-bigint';

import type { MockDb } from '../db';

/** Per-FheType bit width — keyed by the byte at handle[30]. */
const NUM_BITS = {
  0: 1n, // ebool
  2: 8n, // euint8
  3: 16n, // euint16
  4: 32n, // euint32
  5: 64n, // euint64
  6: 128n, // euint128
  7: 160n, // eaddress
  8: 256n, // euint256
} as const;

type FheTypeKey = keyof typeof NUM_BITS;

function getRandomBigInt(numBits: number): bigint {
  if (numBits <= 0) {
    throw new Error('Number of bits must be greater than 0');
  }
  const numBytes = Math.ceil(numBits / 8);
  const randomBytes = new Uint8Array(numBytes);
  crypto.getRandomValues(randomBytes);
  let randomBigInt = 0n;
  for (let i = 0; i < numBytes; i++) {
    randomBigInt = (randomBigInt << 8n) | BigInt(randomBytes[i]);
  }
  const mask = (1n << BigInt(numBits)) - 1n;
  return randomBigInt & mask;
}

function bitwiseNotUintBits(value: bigint, numBits: number): bigint {
  const BIT_MASK = (1n << BigInt(numBits)) - 1n;
  return ~value & BIT_MASK;
}

async function lookup(db: MockDb, handleRaw: string | bigint): Promise<bigint> {
  const handle = ethers.toBeHex(handleRaw, 32);
  const value = await db.getClearText(handle);
  if (value === null) {
    throw new Error(
      `Operand handle not found in mock DB: ${handle}. ` +
        `The daemon is live-events-only and seeds its cursor at the chain head on startup, so the producing tx ` +
        `must be mined AFTER the daemon's "initialised at chain head" line. Stop the daemon, run \`pnpm mock:reset\`, ` +
        `restart it, then re-submit the producing tx.`,
    );
  }
  return value;
}

function resultTypeFromHandle(handle: string): FheTypeKey {
  // Byte 30 of the handle carries the FheType selector — see
  // contracts/FHEVMExecutor.sol::_appendMetadataToPrehandle.
  return parseInt(handle.slice(-4, -2), 16) as FheTypeKey;
}

function modN(value: bigint, fheType: FheTypeKey): bigint {
  return value % 2n ** NUM_BITS[fheType];
}

export interface ExecutorEvent {
  eventName: string;
  args: Result;
}

/**
 * Apply one FHEVMExecutor event to the mock DB.
 *
 * Returns:
 *   `'inserted'` — a new ciphertext entry was written
 *   `'noop'`     — the event doesn't produce a ciphertext (VerifyInput etc.)
 *
 * Throws on:
 *   - Missing operand (see `lookup`)
 *   - Unknown event names (silent so we don't crash on future ABI additions)
 *     are returned as `'noop'` with a warning logged by the caller.
 */
export async function applyExecutorEvent(event: ExecutorEvent, db: MockDb): Promise<'inserted' | 'noop'> {
  let handle: string;
  let clearText: bigint;
  let clearLHS: bigint;
  let clearRHS: bigint;
  let resultType: FheTypeKey;
  let shift: bigint;

  switch (event.eventName) {
    case 'TrivialEncrypt':
      clearText = BigInt(event.args[1]);
      handle = ethers.toBeHex(event.args[3], 32);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';

    case 'TrivialEncryptBytes':
      clearText = BigInt(event.args[1]);
      handle = ethers.toBeHex(event.args[3], 32);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';

    case 'FheAdd': {
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = resultTypeFromHandle(handle);
      clearLHS = await lookup(db, event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = modN(clearLHS + BigInt(event.args[2]), resultType);
      } else {
        clearRHS = await lookup(db, event.args[2]);
        clearText = modN(clearLHS + clearRHS, resultType);
      }
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheSub': {
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = resultTypeFromHandle(handle);
      clearLHS = await lookup(db, event.args[1]);
      let raw =
        event.args[3] === '0x01' ? clearLHS - BigInt(event.args[2]) : clearLHS - (await lookup(db, event.args[2]));
      if (raw < 0n) raw += 2n ** NUM_BITS[resultType];
      clearText = modN(raw, resultType);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheMul': {
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = resultTypeFromHandle(handle);
      clearLHS = await lookup(db, event.args[1]);
      clearText =
        event.args[3] === '0x01'
          ? modN(clearLHS * BigInt(event.args[2]), resultType)
          : modN(clearLHS * (await lookup(db, event.args[2])), resultType);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheDiv': {
      handle = ethers.toBeHex(event.args[4], 32);
      clearLHS = await lookup(db, event.args[1]);
      if (event.args[3] !== '0x01') throw new Error('Non-scalar div not implemented yet');
      clearText = clearLHS / BigInt(event.args[2]);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheRem': {
      handle = ethers.toBeHex(event.args[4], 32);
      clearLHS = await lookup(db, event.args[1]);
      if (event.args[3] !== '0x01') throw new Error('Non-scalar rem not implemented yet');
      clearText = clearLHS % BigInt(event.args[2]);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheBitAnd': {
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = resultTypeFromHandle(handle);
      clearLHS = await lookup(db, event.args[1]);
      clearText =
        event.args[3] === '0x01'
          ? modN(clearLHS & BigInt(event.args[2]), resultType)
          : modN(clearLHS & (await lookup(db, event.args[2])), resultType);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheBitOr': {
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = resultTypeFromHandle(handle);
      clearLHS = await lookup(db, event.args[1]);
      clearText =
        event.args[3] === '0x01'
          ? modN(clearLHS | BigInt(event.args[2]), resultType)
          : modN(clearLHS | (await lookup(db, event.args[2])), resultType);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheBitXor': {
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = resultTypeFromHandle(handle);
      clearLHS = await lookup(db, event.args[1]);
      clearText =
        event.args[3] === '0x01'
          ? modN(clearLHS ^ BigInt(event.args[2]), resultType)
          : modN(clearLHS ^ (await lookup(db, event.args[2])), resultType);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheShl': {
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = resultTypeFromHandle(handle);
      clearLHS = await lookup(db, event.args[1]);
      const rhs = event.args[3] === '0x01' ? BigInt(event.args[2]) : await lookup(db, event.args[2]);
      clearText = modN(clearLHS << (rhs % NUM_BITS[resultType]), resultType);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheShr': {
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = resultTypeFromHandle(handle);
      clearLHS = await lookup(db, event.args[1]);
      const rhs = event.args[3] === '0x01' ? BigInt(event.args[2]) : await lookup(db, event.args[2]);
      clearText = modN(clearLHS >> (rhs % NUM_BITS[resultType]), resultType);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheRotl': {
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = resultTypeFromHandle(handle);
      clearLHS = await lookup(db, event.args[1]);
      const rhs = event.args[3] === '0x01' ? BigInt(event.args[2]) : await lookup(db, event.args[2]);
      shift = rhs % NUM_BITS[resultType];
      clearText = modN((clearLHS << shift) | (clearLHS >> (NUM_BITS[resultType] - shift)), resultType);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheRotr': {
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = resultTypeFromHandle(handle);
      clearLHS = await lookup(db, event.args[1]);
      const rhs = event.args[3] === '0x01' ? BigInt(event.args[2]) : await lookup(db, event.args[2]);
      shift = rhs % NUM_BITS[resultType];
      clearText = modN((clearLHS >> shift) | (clearLHS << (NUM_BITS[resultType] - shift)), resultType);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheEq':
    case 'FheEqBytes': {
      handle = ethers.toBeHex(event.args[4], 32);
      clearLHS = await lookup(db, event.args[1]);
      const rhs = event.args[3] === '0x01' ? BigInt(event.args[2]) : await lookup(db, event.args[2]);
      clearText = clearLHS === rhs ? 1n : 0n;
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheNe':
    case 'FheNeBytes': {
      handle = ethers.toBeHex(event.args[4], 32);
      clearLHS = await lookup(db, event.args[1]);
      const rhs = event.args[3] === '0x01' ? BigInt(event.args[2]) : await lookup(db, event.args[2]);
      clearText = clearLHS !== rhs ? 1n : 0n;
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheGe': {
      handle = ethers.toBeHex(event.args[4], 32);
      clearLHS = await lookup(db, event.args[1]);
      const rhs = event.args[3] === '0x01' ? BigInt(event.args[2]) : await lookup(db, event.args[2]);
      clearText = clearLHS >= rhs ? 1n : 0n;
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheGt': {
      handle = ethers.toBeHex(event.args[4], 32);
      clearLHS = await lookup(db, event.args[1]);
      const rhs = event.args[3] === '0x01' ? BigInt(event.args[2]) : await lookup(db, event.args[2]);
      clearText = clearLHS > rhs ? 1n : 0n;
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheLe': {
      handle = ethers.toBeHex(event.args[4], 32);
      clearLHS = await lookup(db, event.args[1]);
      const rhs = event.args[3] === '0x01' ? BigInt(event.args[2]) : await lookup(db, event.args[2]);
      clearText = clearLHS <= rhs ? 1n : 0n;
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheLt': {
      handle = ethers.toBeHex(event.args[4], 32);
      clearLHS = await lookup(db, event.args[1]);
      const rhs = event.args[3] === '0x01' ? BigInt(event.args[2]) : await lookup(db, event.args[2]);
      clearText = clearLHS < rhs ? 1n : 0n;
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheMax': {
      handle = ethers.toBeHex(event.args[4], 32);
      clearLHS = await lookup(db, event.args[1]);
      const rhs = event.args[3] === '0x01' ? BigInt(event.args[2]) : await lookup(db, event.args[2]);
      clearText = clearLHS > rhs ? clearLHS : rhs;
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheMin': {
      handle = ethers.toBeHex(event.args[4], 32);
      clearLHS = await lookup(db, event.args[1]);
      const rhs = event.args[3] === '0x01' ? BigInt(event.args[2]) : await lookup(db, event.args[2]);
      clearText = clearLHS < rhs ? clearLHS : rhs;
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'Cast': {
      resultType = parseInt(event.args[2]) as FheTypeKey;
      handle = ethers.toBeHex(event.args[3], 32);
      clearText = modN(await lookup(db, event.args[1]), resultType);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheNot': {
      handle = ethers.toBeHex(event.args[2], 32);
      resultType = resultTypeFromHandle(handle);
      const operand = await lookup(db, event.args[1]);
      clearText = bitwiseNotUintBits(operand, Number(NUM_BITS[resultType]));
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheNeg': {
      handle = ethers.toBeHex(event.args[2], 32);
      resultType = resultTypeFromHandle(handle);
      const operand = await lookup(db, event.args[1]);
      let negated = bitwiseNotUintBits(operand, Number(NUM_BITS[resultType]));
      negated = (negated + 1n) % 2n ** NUM_BITS[resultType];
      await db.insertCiphertext(handle, negated);
      return 'inserted';
    }

    case 'VerifyInput': {
      // Just a sanity check that the user-input handle was registered out-of-band
      // (typically via the relayer / input verifier flow — which the mock doesn't
      // emulate). If it's not in the DB we can't compute downstream ops; warn
      // loudly so the operator notices.
      handle = ethers.toBeHex(event.args[1], 32);
      const value = await db.getClearText(handle);
      if (value === null) {
        throw new Error(
          `VerifyInput handle ${handle} not present in mock DB — the relayer/input-verifier path is not emulated. ` +
            `Either pre-populate the handle manually (insertCiphertext) or only use TrivialEncrypt-based flows.`,
        );
      }
      return 'noop';
    }

    case 'FheIfThenElse': {
      handle = ethers.toBeHex(event.args[4], 32);
      const control = await lookup(db, event.args[1]);
      const ifTrue = await lookup(db, event.args[2]);
      const ifFalse = await lookup(db, event.args[3]);
      clearText = control === 1n ? ifTrue : ifFalse;
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheRand': {
      resultType = parseInt(event.args[1]) as FheTypeKey;
      handle = ethers.toBeHex(event.args[3], 32);
      clearText = getRandomBigInt(Number(NUM_BITS[resultType]));
      // FheRand is non-deterministic; if we re-process the same event after a
      // reorg we MUST overwrite (otherwise the operator believes the old
      // value). REPLACE is safe here because the producing event is the
      // single source of truth — no other op can yield the same handle.
      await db.insertCiphertext(handle, clearText, true);
      return 'inserted';
    }

    case 'FheRandBounded': {
      handle = ethers.toBeHex(event.args[4], 32);
      clearText = getRandomBigInt(Number(log2(BigInt(event.args[1]))));
      await db.insertCiphertext(handle, clearText, true);
      return 'inserted';
    }

    case 'FheSum': {
      handle = ethers.toBeHex(event.args[2], 32);
      resultType = resultTypeFromHandle(handle);
      let sum = 0n;
      for (const valueHandle of event.args[1] as string[]) {
        sum += await lookup(db, valueHandle);
      }
      clearText = modN(sum, resultType);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheIsIn': {
      handle = ethers.toBeHex(event.args[3], 32);
      const value = await lookup(db, event.args[1]);
      const setHandles = event.args[2] as string[];
      const setClearTexts = await Promise.all(setHandles.map((h) => lookup(db, h)));
      clearText = setClearTexts.some((s) => s === value) ? 1n : 0n;
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    case 'FheMulDiv': {
      // args: [caller, lhs, rhs, divisor, scalarByte, result]
      handle = ethers.toBeHex(event.args[5], 32);
      resultType = resultTypeFromHandle(handle);
      clearLHS = await lookup(db, event.args[1]);
      const divisor = BigInt(event.args[3]);
      const product =
        event.args[4] === '0x01' ? clearLHS * BigInt(event.args[2]) : clearLHS * (await lookup(db, event.args[2]));
      clearText = modN(product / divisor, resultType);
      await db.insertCiphertext(handle, clearText);
      return 'inserted';
    }

    default:
      return 'noop';
  }
}
