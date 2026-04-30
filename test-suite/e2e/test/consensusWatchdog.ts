import { ethers } from 'ethers';

// Minimal ABIs — only the events we need to monitor.
const CIPHERTEXT_COMMITS_ABI = [
  'event AddCiphertextMaterial(bytes32 indexed ctHandle, uint256 keyId, bytes32 ciphertextDigest, bytes32 snsCiphertextDigest, address coprocessorTxSender)',
  'event AddCiphertextMaterialConsensus(bytes32 indexed ctHandle, uint256 keyId, bytes32 ciphertextDigest, bytes32 snsCiphertextDigest, address[] coprocessorTxSenders)',
];

const INPUT_VERIFICATION_ABI = [
  'event VerifyProofResponseCall(uint256 indexed zkProofId, bytes32[] ctHandles, bytes signature, address coprocessorTxSender, bytes extraData)',
  'event VerifyProofResponse(uint256 indexed zkProofId, bytes32[] ctHandles, bytes[] signatures)',
];

const CONSENSUS_TIMEOUT_MS = 3 * 60 * 1000; // 3 minutes
const POLL_INTERVAL_MS = 2_000;

interface CiphertextSubmission {
  coprocessor: string;
  ciphertextDigest: string;
  snsCiphertextDigest: string;
  keyId: bigint;
}

interface ProofSubmission {
  coprocessor: string;
  ctHandles: string[];
}

interface PendingHandle {
  firstSeenAt: number;
  submissions: CiphertextSubmission[];
}

interface PendingProof {
  firstSeenAt: number;
  submissions: ProofSubmission[];
}

interface CiphertextPollResult {
  pendingHandles: Map<string, PendingHandle>;
  resolvedHandles: Set<string>;
  resolvedHandleDelta: number;
  divergences: string[];
  divergenceKeys: Set<string>;
}

interface ProofPollResult {
  pendingProofs: Map<string, PendingProof>;
  resolvedProofs: Set<string>;
  resolvedProofDelta: number;
  divergences: string[];
  divergenceKeys: Set<string>;
}

export class ConsensusWatchdog {
  private provider: ethers.JsonRpcProvider;
  private ciphertextCommits: ethers.Contract;
  private inputVerification: ethers.Contract | null;
  private pendingHandles = new Map<string, PendingHandle>();
  private pendingProofs = new Map<string, PendingProof>();
  // Handles/proofs that already reached consensus. Late submissions for these
  // must be ignored, since the contract only emits one consensus event per handle.
  private resolvedHandles = new Set<string>();
  private resolvedProofs = new Set<string>();
  private resolvedHandleCount = 0;
  private resolvedProofCount = 0;
  private divergences: string[] = [];
  private divergenceKeys = new Set<string>();
  private pollTimer: ReturnType<typeof setInterval> | null = null;
  private pollInFlight: Promise<void> | null = null;
  private lastBlock = 0;

  constructor(gatewayRpcUrl: string, ciphertextCommitsAddress: string, inputVerificationAddress?: string) {
    this.provider = new ethers.JsonRpcProvider(gatewayRpcUrl);
    this.ciphertextCommits = new ethers.Contract(ciphertextCommitsAddress, CIPHERTEXT_COMMITS_ABI, this.provider);
    this.inputVerification = inputVerificationAddress
      ? new ethers.Contract(inputVerificationAddress, INPUT_VERIFICATION_ABI, this.provider)
      : null;
  }

  async start(): Promise<void> {
    this.lastBlock = await this.provider.getBlockNumber();
    this.pollTimer = setInterval(() => this.poll(), POLL_INTERVAL_MS);
  }

  async stop(): Promise<void> {
    if (this.pollTimer) {
      clearInterval(this.pollTimer);
      this.pollTimer = null;
    }
    this.provider.destroy();
  }

  /** Force a poll cycle — used by Mocha hooks to catch events before checking health. */
  async flush(): Promise<void> {
    if (this.pollInFlight) {
      await this.pollInFlight;
    }
    return this.poll();
  }

  private async poll(): Promise<void> {
    if (this.pollInFlight) return this.pollInFlight;
    this.pollInFlight = this.runPoll().finally(() => {
      this.pollInFlight = null;
    });
    return this.pollInFlight;
  }

  private async runPoll(): Promise<void> {
    try {
      const currentBlock = await this.provider.getBlockNumber();
      if (currentBlock <= this.lastBlock) return;

      const fromBlock = this.lastBlock + 1;
      const toBlock = currentBlock;

      const [ciphertextResult, proofResult] = await Promise.all([
        this.pollCiphertextEvents(fromBlock, toBlock),
        this.inputVerification
          ? this.pollInputVerificationEvents(fromBlock, toBlock)
          : Promise.resolve<ProofPollResult>({
              pendingProofs: this.clonePendingProofs(),
              resolvedProofs: new Set(this.resolvedProofs),
              resolvedProofDelta: 0,
              divergences: [],
              divergenceKeys: new Set(this.divergenceKeys),
            }),
      ]);

      this.pendingHandles = ciphertextResult.pendingHandles;
      this.pendingProofs = proofResult.pendingProofs;
      this.resolvedHandles = ciphertextResult.resolvedHandles;
      this.resolvedProofs = proofResult.resolvedProofs;
      this.resolvedHandleCount += ciphertextResult.resolvedHandleDelta;
      this.resolvedProofCount += proofResult.resolvedProofDelta;
      this.divergences.push(...ciphertextResult.divergences, ...proofResult.divergences);
      this.divergenceKeys = new Set([...ciphertextResult.divergenceKeys, ...proofResult.divergenceKeys]);
      this.lastBlock = toBlock;
    } catch (err) {
      // Transient RPC errors shouldn't crash the watchdog — log and retry next poll.
      console.warn('[consensus-watchdog] poll error:', (err as Error).message);
    }
  }

  private async pollCiphertextEvents(fromBlock: number, toBlock: number): Promise<CiphertextPollResult> {
    const [submissions, consensuses] = await Promise.all([
      this.ciphertextCommits.queryFilter(
        this.ciphertextCommits.filters.AddCiphertextMaterial(),
        fromBlock,
        toBlock,
      ),
      this.ciphertextCommits.queryFilter(
        this.ciphertextCommits.filters.AddCiphertextMaterialConsensus(),
        fromBlock,
        toBlock,
      ),
    ]);

    const pendingHandles = this.clonePendingHandles();
    const resolvedHandles = new Set(this.resolvedHandles);
    const divergences: string[] = [];
    const divergenceKeys = new Set(this.divergenceKeys);
    let resolvedHandleDelta = 0;

    for (const event of submissions) {
      const log = event as ethers.EventLog;
      const ctHandle = log.args[0] as string;
      const keyId = log.args[1] as bigint;
      const ciphertextDigest = log.args[2] as string;
      const snsCiphertextDigest = log.args[3] as string;
      const coprocessor = log.args[4] as string;

      if (resolvedHandles.has(ctHandle)) continue;

      if (!pendingHandles.has(ctHandle)) {
        pendingHandles.set(ctHandle, {
          firstSeenAt: Date.now(),
          submissions: [],
        });
      }

      const pending = pendingHandles.get(ctHandle)!;
      pending.submissions.push({ coprocessor, ciphertextDigest, snsCiphertextDigest, keyId });

      // Check for divergence: compare all submissions for this handle.
      this.checkCiphertextDivergence(ctHandle, pending, divergences, divergenceKeys);
    }

    for (const event of consensuses) {
      const log = event as ethers.EventLog;
      const ctHandle = log.args[0] as string;
      resolvedHandles.add(ctHandle);
      if (pendingHandles.delete(ctHandle)) {
        resolvedHandleDelta++;
      }
    }

    return { pendingHandles, resolvedHandles, resolvedHandleDelta, divergences, divergenceKeys };
  }

  private async pollInputVerificationEvents(fromBlock: number, toBlock: number): Promise<ProofPollResult> {
    const [responses, consensuses] = await Promise.all([
      this.inputVerification!.queryFilter(
        this.inputVerification!.filters.VerifyProofResponseCall(),
        fromBlock,
        toBlock,
      ),
      this.inputVerification!.queryFilter(
        this.inputVerification!.filters.VerifyProofResponse(),
        fromBlock,
        toBlock,
      ),
    ]);

    const pendingProofs = this.clonePendingProofs();
    const resolvedProofs = new Set(this.resolvedProofs);
    const divergences: string[] = [];
    const divergenceKeys = new Set(this.divergenceKeys);
    let resolvedProofDelta = 0;

    for (const event of responses) {
      const log = event as ethers.EventLog;
      const zkProofId = String(log.args[0]);
      const ctHandles = log.args[1] as string[];
      const coprocessor = log.args[3] as string;

      if (resolvedProofs.has(zkProofId)) continue;

      if (!pendingProofs.has(zkProofId)) {
        pendingProofs.set(zkProofId, {
          firstSeenAt: Date.now(),
          submissions: [],
        });
      }

      const pending = pendingProofs.get(zkProofId)!;
      pending.submissions.push({ coprocessor, ctHandles: [...ctHandles] });

      this.checkProofDivergence(zkProofId, pending, divergences, divergenceKeys);
    }

    for (const event of consensuses) {
      const log = event as ethers.EventLog;
      const zkProofId = String(log.args[0]);
      resolvedProofs.add(zkProofId);
      if (pendingProofs.delete(zkProofId)) {
        resolvedProofDelta++;
      }
    }

    return { pendingProofs, resolvedProofs, resolvedProofDelta, divergences, divergenceKeys };
  }

  private checkCiphertextDivergence(
    ctHandle: string,
    pending: PendingHandle,
    divergences: string[],
    divergenceKeys: Set<string>,
  ): void {
    if (pending.submissions.length < 2) return;

    const first = pending.submissions[0];
    const sub = pending.submissions[pending.submissions.length - 1];
    if (sub.ciphertextDigest !== first.ciphertextDigest || sub.snsCiphertextDigest !== first.snsCiphertextDigest) {
      const msg =
        `[consensus-watchdog] CIPHERTEXT DIVERGENCE for handle ${ctHandle}\n` +
        `  Coprocessor ${first.coprocessor}: ctDigest=${first.ciphertextDigest} snsDigest=${first.snsCiphertextDigest}\n` +
        `  Coprocessor ${sub.coprocessor}: ctDigest=${sub.ciphertextDigest} snsDigest=${sub.snsCiphertextDigest}`;
      const key = `ct:${ctHandle}:${first.ciphertextDigest}:${first.snsCiphertextDigest}:${sub.ciphertextDigest}:${sub.snsCiphertextDigest}`;
      this.recordDivergence(key, msg, divergences, divergenceKeys);
    }
  }

  private checkProofDivergence(
    zkProofId: string,
    pending: PendingProof,
    divergences: string[],
    divergenceKeys: Set<string>,
  ): void {
    if (pending.submissions.length < 2) return;

    const first = pending.submissions[0];
    const firstHandles = first.ctHandles.join(',');
    const sub = pending.submissions[pending.submissions.length - 1];
    const subHandles = sub.ctHandles.join(',');
    if (firstHandles !== subHandles) {
      const msg =
        `[consensus-watchdog] INPUT VERIFICATION DIVERGENCE for zkProofId ${zkProofId}\n` +
        `  Coprocessor ${first.coprocessor}: handles=[${firstHandles}]\n` +
        `  Coprocessor ${sub.coprocessor}: handles=[${subHandles}]`;
      const key = `pf:${zkProofId}:${firstHandles}:${subHandles}`;
      this.recordDivergence(key, msg, divergences, divergenceKeys);
    }
  }

  private recordDivergence(key: string, msg: string, divergences: string[], divergenceKeys: Set<string>): void {
    if (divergenceKeys.has(key)) return;
    divergenceKeys.add(key);
    console.error(msg);
    divergences.push(msg);
  }

  private clonePendingHandles(): Map<string, PendingHandle> {
    return new Map(
      [...this.pendingHandles.entries()].map(([handle, pending]) => [
        handle,
        { firstSeenAt: pending.firstSeenAt, submissions: [...pending.submissions] },
      ]),
    );
  }

  private clonePendingProofs(): Map<string, PendingProof> {
    return new Map(
      [...this.pendingProofs.entries()].map(([proofId, pending]) => [
        proofId,
        {
          firstSeenAt: pending.firstSeenAt,
          submissions: pending.submissions.map((submission) => ({ ...submission, ctHandles: [...submission.ctHandles] })),
        },
      ]),
    );
  }

  private stalledPendingCount(now = Date.now()): number {
    let count = 0;
    for (const pending of this.pendingHandles.values()) {
      if (now - pending.firstSeenAt > CONSENSUS_TIMEOUT_MS) count++;
    }
    for (const pending of this.pendingProofs.values()) {
      if (now - pending.firstSeenAt > CONSENSUS_TIMEOUT_MS) count++;
    }
    return count;
  }

  /**
   * Check for divergences (instant) and stalls (3-minute timeout).
   * Called in afterEach to fail the current test if consensus is broken.
   */
  checkHealth(): void {
    // Force a sync check of divergences accumulated since last poll.
    if (this.divergences.length > 0) {
      const msg = this.divergences.join('\n\n');
      this.divergences = [];
      this.divergenceKeys.clear();
      throw new Error(`Consensus divergence detected:\n\n${msg}`);
    }

    // Check for stalls: handles that received a first submission but no consensus within timeout.
    const now = Date.now();

    for (const [ctHandle, pending] of this.pendingHandles) {
      const elapsed = now - pending.firstSeenAt;
      if (elapsed > CONSENSUS_TIMEOUT_MS) {
        const coprocessors = pending.submissions.map((s) => s.coprocessor).join(', ');
        this.pendingHandles.delete(ctHandle);
        throw new Error(
          `Consensus stall for ciphertext handle ${ctHandle}: ` +
            `only ${pending.submissions.length} coprocessor(s) submitted after ${Math.round(elapsed / 1000)}s ` +
            `(submitters: ${coprocessors})`,
        );
      }
    }

    for (const [zkProofId, pending] of this.pendingProofs) {
      const elapsed = now - pending.firstSeenAt;
      if (elapsed > CONSENSUS_TIMEOUT_MS) {
        const coprocessors = pending.submissions.map((s) => s.coprocessor).join(', ');
        this.pendingProofs.delete(zkProofId);
        throw new Error(
          `Consensus stall for input verification zkProofId ${zkProofId}: ` +
            `only ${pending.submissions.length} coprocessor(s) submitted after ${Math.round(elapsed / 1000)}s ` +
            `(submitters: ${coprocessors})`,
        );
      }
    }
  }

  /** Summary for afterAll — reports any remaining pending handles. */
  summary(): string {
    const lines: string[] = [];
    lines.push(
      `[consensus-watchdog] Summary: ${this.resolvedHandleCount} ciphertext(s), ${this.resolvedProofCount} proof(s), ${this.divergences.length} divergence(s), ${this.stalledPendingCount()} stalled pending item(s)`,
    );

    if (this.pendingHandles.size > 0) {
      lines.push(`  WARNING: ${this.pendingHandles.size} ciphertext handle(s) never reached consensus:`);
      for (const [handle, p] of this.pendingHandles) {
        lines.push(`    - ${handle} (${p.submissions.length} submission(s))`);
      }
    }

    if (this.pendingProofs.size > 0) {
      lines.push(`  WARNING: ${this.pendingProofs.size} proof(s) never reached consensus:`);
      for (const [id, p] of this.pendingProofs) {
        lines.push(`    - zkProofId ${id} (${p.submissions.length} submission(s))`);
      }
    }

    return lines.join('\n');
  }
}

// Singleton — shared across all tests in a Mocha run.
let watchdog: ConsensusWatchdog | null = null;

function isEnabled(): boolean {
  return !!(process.env.GATEWAY_RPC_URL && process.env.CIPHERTEXT_COMMITS_ADDRESS);
}

export const mochaHooks = {
  async beforeAll(this: Mocha.Context) {
    if (!isEnabled()) return;

    const gatewayRpcUrl = process.env.GATEWAY_RPC_URL!;
    const ciphertextCommitsAddress = process.env.CIPHERTEXT_COMMITS_ADDRESS!;
    const inputVerificationAddress = process.env.INPUT_VERIFICATION_ADDRESS;

    if (!inputVerificationAddress) {
      console.warn('[consensus-watchdog] INPUT_VERIFICATION_ADDRESS not set, skipping proof monitoring');
    }

    console.log(`[consensus-watchdog] Starting — gateway=${gatewayRpcUrl} ciphertextCommits=${ciphertextCommitsAddress}`);
    watchdog = new ConsensusWatchdog(gatewayRpcUrl, ciphertextCommitsAddress, inputVerificationAddress);
    await watchdog.start();
  },

  async afterEach(this: Mocha.Context) {
    if (!watchdog) return;

    // Force one last poll before checking health so we catch recent events.
    await watchdog.flush();
    watchdog.checkHealth();
  },

  async afterAll(this: Mocha.Context) {
    if (!watchdog) return;

    // Final poll + summary.
    await watchdog.flush();
    const summary = watchdog.summary();
    if (summary) console.log(summary);

    await watchdog.stop();
    watchdog = null;
  },
};
