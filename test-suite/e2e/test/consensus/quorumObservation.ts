import { Contract, FetchRequest, JsonRpcProvider } from 'ethers';

export interface QuorumWindowSnapshot {
  block: number;
  submissions: { sender: string; keyId: string; ciphertextDigest: string; snsCiphertextDigest: string }[];
  consensusCount: number;
}
export interface QuorumWindowOptions {
  read: () => Promise<QuorumWindowSnapshot>;
  authorizedSenders: readonly string[];
  survivorCount: number;
  expected: { keyId: string; ciphertextDigest: string; snsCiphertextDigest: string };
  windowMs?: number;
  submissionTimeoutMs?: number;
  maxStallMs?: number;
  pollMs?: number;
  now?: () => number;
  sleep?: (ms: number) => Promise<void>;
}

/** The negative window begins only after healthy submissions reach Gateway.
 * Head samples bracket every poll inside that same window, not a later grace
 * period in which a stalled chain might have recovered. */
export async function observeForbiddenQuorum(options: QuorumWindowOptions): Promise<{ firstBlock: number; lastBlock: number; samples: number }> {
  const now = options.now ?? Date.now;
  const sleep = options.sleep ?? (ms => new Promise(resolve => setTimeout(resolve, ms)));
  const windowMs = options.windowMs ?? 4 * 60_000;
  const submissionTimeoutMs = options.submissionTimeoutMs ?? 6 * 60_000;
  const maxStallMs = options.maxStallMs ?? 30_000;
  if (!Number.isSafeInteger(options.survivorCount) || options.survivorCount < 1 || windowMs <= 0 || maxStallMs <= 0) {
    throw new Error('invalid forbidden-quorum observation configuration');
  }
  const authorized = new Set(options.authorizedSenders.map(sender => sender.toLowerCase()));
  const started = now();
  let lastProgress = started;
  let windowStarted: number | undefined;
  let firstBlock: number | undefined;
  let lastBlock: number | undefined;
  let samples = 0;
  for (;;) {
    const snapshot = await options.read(); // RPC failure is never evidence of absent quorum.
    const observedAt = now();
    samples += 1;
    if (!Number.isSafeInteger(snapshot.block) || snapshot.block < 0) throw new Error('invalid gateway head');
    if (snapshot.consensusCount !== 0) throw new Error('forbidden Gateway consensus event formed');
    if (lastBlock !== undefined && snapshot.block < lastBlock) throw new Error('gateway head rewound during no-quorum observation');
    if (lastBlock === undefined || snapshot.block > lastBlock) lastProgress = observedAt;
    lastBlock = snapshot.block;
    if (observedAt - lastProgress >= maxStallMs) throw new Error('gateway stopped advancing during no-quorum observation');
    const senders = new Set<string>();
    for (const submission of snapshot.submissions) {
      const sender = submission.sender.toLowerCase();
      if (!authorized.has(sender)) throw new Error('unexpected sender during no-quorum observation');
      if (submission.keyId.toLowerCase() !== options.expected.keyId.toLowerCase() ||
          submission.ciphertextDigest.toLowerCase() !== options.expected.ciphertextDigest.toLowerCase() ||
          submission.snsCiphertextDigest.toLowerCase() !== options.expected.snsCiphertextDigest.toLowerCase()) {
        throw new Error('survivor submission does not bind the agreed key and digests');
      }
      senders.add(sender);
    }
    if (senders.size < options.survivorCount) {
      if (windowStarted !== undefined) throw new Error('healthy submissions disappeared during observation');
      if (observedAt - started >= submissionTimeoutMs) throw new Error('healthy survivors did not submit the identified workload');
    } else if (windowStarted === undefined) {
      windowStarted = observedAt;
      firstBlock = snapshot.block;
    } else if (observedAt - windowStarted >= windowMs) {
      if (snapshot.block <= firstBlock!) throw new Error('gateway never advanced inside the no-quorum window');
      return { firstBlock: firstBlock!, lastBlock: snapshot.block, samples };
    }
    await sleep(options.pollMs ?? 2_000);
  }
}

export async function assertNoQuorumWithSurvivorSubmissions(options: Omit<QuorumWindowOptions, 'read'> & {
  gatewayRpcUrl: string; ciphertextCommitsAddress: string; handle: string;
}): Promise<{ firstBlock: number; lastBlock: number; samples: number }> {
  const request = new FetchRequest(options.gatewayRpcUrl);
  request.timeout = 10_000;
  const provider = new JsonRpcProvider(request);
  const contract = new Contract(options.ciphertextCommitsAddress, [
    'event AddCiphertextMaterial(bytes32 indexed ctHandle, uint256 keyId, bytes32 ciphertextDigest, bytes32 snsCiphertextDigest, address coprocessorTxSender)',
    'event AddCiphertextMaterialConsensus(bytes32 indexed ctHandle, uint256 keyId, bytes32 ciphertextDigest, bytes32 snsCiphertextDigest, address[] coprocessorTxSenders)',
  ], provider);
  try {
    return await observeForbiddenQuorum({ ...options, read: async () => {
      // Pin event reads to the same fresh, uncached head observed for liveness.
      const block = Number(BigInt(await provider.send('eth_blockNumber', [])));
      const [submissions, consensuses] = await Promise.all([
        contract.queryFilter(contract.filters.AddCiphertextMaterial(options.handle), 0, block),
        contract.queryFilter(contract.filters.AddCiphertextMaterialConsensus(options.handle), 0, block),
      ]);
      return { block, consensusCount: consensuses.length, submissions: submissions.map(log => {
        const args = (log as import('ethers').EventLog).args;
        return { sender: String(args[4]), keyId: BigInt(args[1]).toString(), ciphertextDigest: String(args[2]), snsCiphertextDigest: String(args[3]) };
      }) };
    } });
  } finally { provider.destroy(); }
}
