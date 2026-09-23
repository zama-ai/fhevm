/**
 * The structured handshake between an in-container suite and its host
 * orchestrator.
 *
 * Fault injection lives host-side -- the runner needs process and container
 * control that a test runner should not hold -- while the workload lives in the
 * container, which is where a Hardhat signer and the SDK are. The two therefore
 * have to agree on WHICH work the fault is aimed at, and until now they did not:
 * the crash runner watched for `COUNT(*) FROM computations WHERE is_completed =
 * false` and fired at the first non-zero reading. That count includes fixture
 * setup, unrelated traffic and work the suite had not minted yet, so the kill
 * could land before the victim had touched anything the suite later checked --
 * and the run then proved that a worker which restarts can execute new work.
 *
 * So the suite publishes its target identifiers and the runner waits for THOSE.
 * A file inside the test container is the whole mechanism: the runner reads it
 * with `docker exec cat`, which needs no new port, no shared volume, and no
 * privilege in either direction.
 */
import { mkdirSync, readFileSync, renameSync, writeFileSync } from 'node:fs';
import path from 'node:path';

/** Where handshake files live inside the test container. */
export const HANDSHAKE_DIR = process.env.CONSENSUS_HANDSHAKE_DIR ?? '/tmp/consensus-handshake';

export interface CrashTarget {
  /** Which operator the runner is to interrupt. */
  victimOperator: number;
  /** Producing transaction hashes, `0x`-prefixed lowercase. */
  transactionHashes: string[];
  /** Output handles the suite will compare afterwards. */
  handles: string[];
  /** Contract the workload was minted from, for a reader reconstructing the run. */
  contractAddress: string;
  /** The boundary the runner should aim at. */
  boundary: 'before-commit' | 'after-commit' | 'expired-lease';
  publishedAt: string;
}

export interface HandshakeEnvelope<T> {
  name: string;
  ready: boolean;
  payload: T;
}

const filePath = (name: string) => path.join(HANDSHAKE_DIR, `${name}.json`);

/**
 * Publishes a payload for the orchestrator, atomically.
 *
 * Written to a temporary name and renamed, because the reader is polling: a
 * partially written file would otherwise parse as malformed JSON, and the
 * runner would have to decide whether that meant "not yet" or "broken".
 */
export function publishHandshake<T>(name: string, payload: T): string {
  mkdirSync(HANDSHAKE_DIR, { recursive: true });
  const envelope: HandshakeEnvelope<T> = { name, ready: true, payload };
  const target = filePath(name);
  const temporary = `${target}.partial`;
  writeFileSync(temporary, `${JSON.stringify(envelope, null, 2)}\n`, 'utf8');
  // Atomic within a filesystem, which is what the reader needs.
  renameSync(temporary, target);
  console.info(`[handshake] published ${name} -> ${target}`);
  return target;
}

/** Reads a payload another process published. */
export function readHandshake<T>(name: string): HandshakeEnvelope<T> {
  return JSON.parse(readFileSync(filePath(name), 'utf8')) as HandshakeEnvelope<T>;
}

/**
 * Waits for the orchestrator to acknowledge that it applied the fault.
 *
 * The suite needs this for one reason: its assertions are about work that was
 * interrupted, and if the runner never managed to interrupt anything then the
 * honest outcome is an invalid case rather than a green one. The acknowledgement
 * carries the evidence the runner collected, so the suite can state it too.
 */
export interface FaultAcknowledgement {
  applied: boolean;
  detail: string;
  /** Process identity before and after, as the runner observed it. */
  processBefore?: string;
  processAfter?: string;
  faultObservedAt?: string;
  recoveryObservedAt?: string;
  /** Running recovered service's address on the database network. */
  recoveredWorkerAddress?: string;
  /** The dependence chain the victim owned when the fault landed. */
  dependenceChainId?: string;
  workerIdBefore?: string;
  workerIdAfter?: string;
}

export async function waitForFaultAcknowledgement(
  name: string,
  timeoutMs = 10 * 60_000,
): Promise<FaultAcknowledgement> {
  const deadline = Date.now() + timeoutMs;
  for (;;) {
    try {
      return readHandshake<FaultAcknowledgement>(name).payload;
    } catch {
      if (Date.now() >= deadline) {
        throw new Error(
          `the orchestrator never acknowledged the ${name} fault within ${Math.round(timeoutMs / 1000)}s; ` +
            'this case asserts the recovery of interrupted work, so an un-applied fault makes it invalid ' +
            'rather than passed',
        );
      }
      await new Promise((resolve) => setTimeout(resolve, 2_000));
    }
  }
}
