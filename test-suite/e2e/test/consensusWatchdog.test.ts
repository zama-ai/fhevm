import { expect } from 'chai';

import { ConsensusWatchdog } from './consensusWatchdog';

// Helper: create a minimal fake EventLog with positional args.
function fakeEvent(...args: unknown[]) {
  return { args } as any;
}

// Helper: create a mock provider + contracts on a watchdog instance.
function mockWatchdog(): {
  watchdog: ConsensusWatchdog;
  setBlock: (n: number) => void;
  setCiphertextEvents: (submissions: any[], consensuses: any[]) => void;
  setProofEvents: (responses: any[], consensuses: any[]) => void;
} {
  const w = new ConsensusWatchdog('http://fake:1234', '0x1111', '0x2222');

  let blockNumber = 0;
  let ctSubmissions: any[] = [];
  let ctConsensuses: any[] = [];
  let pfResponses: any[] = [];
  let pfConsensuses: any[] = [];

  // Replace provider with stub.
  (w as any).provider = {
    getBlockNumber: async () => blockNumber,
    destroy: () => {},
  };

  // Replace ciphertextCommits contract with stub.
  (w as any).ciphertextCommits = {
    filters: {
      AddCiphertextMaterial: () => 'ct-sub-filter',
      AddCiphertextMaterialConsensus: () => 'ct-con-filter',
    },
    queryFilter: async (filter: string) => {
      return filter === 'ct-sub-filter' ? ctSubmissions : ctConsensuses;
    },
  };

  // Replace inputVerification contract with stub.
  (w as any).inputVerification = {
    filters: {
      VerifyProofResponseCall: () => 'pf-sub-filter',
      VerifyProofResponse: () => 'pf-con-filter',
    },
    queryFilter: async (filter: string) => {
      return filter === 'pf-sub-filter' ? pfResponses : pfConsensuses;
    },
  };

  return {
    watchdog: w,
    setBlock: (n: number) => {
      blockNumber = n;
    },
    setCiphertextEvents: (subs, cons) => {
      ctSubmissions = subs;
      ctConsensuses = cons;
    },
    setProofEvents: (resp, cons) => {
      pfResponses = resp;
      pfConsensuses = cons;
    },
  };
}

describe('ConsensusWatchdog', function () {
  describe('checkHealth — divergence detection', function () {
    it('should throw on ciphertext digest divergence', async function () {
      const { watchdog, setBlock, setCiphertextEvents } = mockWatchdog();

      // Two coprocessors submit different digests for the same handle.
      setCiphertextEvents(
        [
          fakeEvent('0xhandle1', 1n, '0xdigestA', '0xsnsDigestA', '0xCoprocessor1'),
          fakeEvent('0xhandle1', 1n, '0xdigestB', '0xsnsDigestA', '0xCoprocessor2'),
        ],
        [],
      );

      setBlock(1);
      await watchdog.flush();

      expect(() => watchdog.checkHealth()).to.throw('Consensus divergence detected');
      expect(() => watchdog.checkHealth()).to.not.throw(); // divergences cleared after first throw
    });

    it('should throw on SNS digest divergence', async function () {
      const { watchdog, setBlock, setCiphertextEvents } = mockWatchdog();

      setCiphertextEvents(
        [
          fakeEvent('0xhandle1', 1n, '0xdigestA', '0xsnsA', '0xCopro1'),
          fakeEvent('0xhandle1', 1n, '0xdigestA', '0xsnsB', '0xCopro2'),
        ],
        [],
      );

      setBlock(1);
      await watchdog.flush();

      expect(() => watchdog.checkHealth()).to.throw('CIPHERTEXT DIVERGENCE');
    });

    it('should throw on input verification divergence', async function () {
      const { watchdog, setBlock, setProofEvents } = mockWatchdog();

      setProofEvents(
        [
          fakeEvent(42n, ['0xhandleA', '0xhandleB'], '0xsig1', '0xCopro1', '0x'),
          fakeEvent(42n, ['0xhandleA', '0xhandleC'], '0xsig2', '0xCopro2', '0x'),
        ],
        [],
      );

      setBlock(1);
      await watchdog.flush();

      expect(() => watchdog.checkHealth()).to.throw('INPUT VERIFICATION DIVERGENCE');
    });
  });

  describe('checkHealth — stall detection', function () {
    it('should throw when consensus is not reached within timeout', async function () {
      const { watchdog, setBlock, setCiphertextEvents } = mockWatchdog();

      // Single submission, no consensus.
      setCiphertextEvents([fakeEvent('0xhandle1', 1n, '0xdigest', '0xsns', '0xCopro1')], []);

      setBlock(1);
      await watchdog.flush();

      // Backdate the firstSeenAt to exceed timeout.
      const pending = (watchdog as any).pendingHandles.get('0xhandle1');
      pending.firstSeenAt = Date.now() - 4 * 60 * 1000; // 4 minutes ago

      expect(() => watchdog.checkHealth()).to.throw('Consensus stall');
      expect(() => watchdog.checkHealth()).to.throw('only 1 coprocessor(s)');
    });

    it('should not throw when within timeout', async function () {
      const { watchdog, setBlock, setCiphertextEvents } = mockWatchdog();

      setCiphertextEvents([fakeEvent('0xhandle1', 1n, '0xdigest', '0xsns', '0xCopro1')], []);

      setBlock(1);
      await watchdog.flush();

      expect(() => watchdog.checkHealth()).to.not.throw();
    });
  });

  describe('consensus resolution — map pruning', function () {
    it('should remove handle from pendingHandles on consensus', async function () {
      const { watchdog, setBlock, setCiphertextEvents } = mockWatchdog();

      // First poll: submissions arrive.
      setCiphertextEvents(
        [
          fakeEvent('0xhandle1', 1n, '0xdigest', '0xsns', '0xCopro1'),
          fakeEvent('0xhandle1', 1n, '0xdigest', '0xsns', '0xCopro2'),
        ],
        [],
      );
      setBlock(1);
      await watchdog.flush();
      expect((watchdog as any).pendingHandles.size).to.equal(1);

      // Second poll: consensus event arrives.
      setCiphertextEvents([], [fakeEvent('0xhandle1', 1n, '0xdigest', '0xsns', ['0xCopro1', '0xCopro2'])]);
      setBlock(2);
      await watchdog.flush();

      expect((watchdog as any).pendingHandles.size).to.equal(0);
      expect((watchdog as any).resolvedHandleCount).to.equal(1);
    });

    it('should remove proof from pendingProofs on consensus', async function () {
      const { watchdog, setBlock, setProofEvents } = mockWatchdog();

      setProofEvents(
        [fakeEvent(99n, ['0xh1'], '0xsig', '0xCopro1', '0x'), fakeEvent(99n, ['0xh1'], '0xsig', '0xCopro2', '0x')],
        [],
      );
      setBlock(1);
      await watchdog.flush();
      expect((watchdog as any).pendingProofs.size).to.equal(1);

      setProofEvents([], [fakeEvent(99n, ['0xh1'], ['0xsig1', '0xsig2'])]);
      setBlock(2);
      await watchdog.flush();

      expect((watchdog as any).pendingProofs.size).to.equal(0);
      expect((watchdog as any).resolvedProofCount).to.equal(1);
    });
  });

  describe('happy path — matching submissions', function () {
    it('should not throw when all coprocessors agree', async function () {
      const { watchdog, setBlock, setCiphertextEvents } = mockWatchdog();

      setCiphertextEvents(
        [
          fakeEvent('0xhandle1', 1n, '0xdigest', '0xsns', '0xCopro1'),
          fakeEvent('0xhandle1', 1n, '0xdigest', '0xsns', '0xCopro2'),
        ],
        [fakeEvent('0xhandle1', 1n, '0xdigest', '0xsns', ['0xCopro1', '0xCopro2'])],
      );

      setBlock(1);
      await watchdog.flush();

      expect(() => watchdog.checkHealth()).to.not.throw();
    });
  });

  describe('polling guard', function () {
    it('should prevent overlapping polls', async function () {
      const { watchdog, setBlock, setCiphertextEvents } = mockWatchdog();

      let pollCount = 0;
      const origGetBlock = (watchdog as any).provider.getBlockNumber;
      (watchdog as any).provider.getBlockNumber = async () => {
        pollCount++;
        // Simulate slow RPC.
        await new Promise((r) => setTimeout(r, 50));
        return origGetBlock();
      };

      setCiphertextEvents([], []);
      setBlock(1);

      // Launch two concurrent flushes.
      await Promise.all([watchdog.flush(), watchdog.flush()]);

      // Only one should have actually polled (the other was guarded).
      expect(pollCount).to.equal(1);
    });
  });

  describe('summary', function () {
    it('should report resolved and pending counts', async function () {
      const { watchdog, setBlock, setCiphertextEvents } = mockWatchdog();

      // Resolve one handle.
      setCiphertextEvents(
        [fakeEvent('0xresolved', 1n, '0xd', '0xs', '0xC1'), fakeEvent('0xresolved', 1n, '0xd', '0xs', '0xC2')],
        [fakeEvent('0xresolved')],
      );
      setBlock(1);
      await watchdog.flush();

      // Add one pending handle (no consensus).
      setCiphertextEvents([fakeEvent('0xpending', 1n, '0xd', '0xs', '0xC1')], []);
      setBlock(2);
      await watchdog.flush();

      const summary = watchdog.summary()!;
      expect(summary).to.include('1 ciphertext(s) and 0 proof(s) reached consensus');
      expect(summary).to.include('1 ciphertext handle(s) never reached consensus');
      expect(summary).to.include('0xpending');
    });

    it('should report clean summary when all resolved', async function () {
      const { watchdog } = mockWatchdog();
      (watchdog as any).resolvedHandleCount = 5;
      (watchdog as any).resolvedProofCount = 3;

      const summary = watchdog.summary()!;
      expect(summary).to.include('5 ciphertext(s) and 3 proof(s) reached consensus');
      expect(summary).to.not.include('WARNING');
    });
  });

  describe('mochaHooks — disabled when env vars not set', function () {
    it('should not start watchdog without GATEWAY_RPC_URL', async function () {
      const { mochaHooks } = require('./consensusWatchdog');
      const origGw = process.env.GATEWAY_RPC_URL;
      const origCt = process.env.CIPHERTEXT_COMMITS_ADDRESS;
      delete process.env.GATEWAY_RPC_URL;
      delete process.env.CIPHERTEXT_COMMITS_ADDRESS;

      // beforeAll should be a no-op.
      await mochaHooks.beforeAll.call({});
      // afterEach and afterAll should also be no-ops (watchdog is null).
      await mochaHooks.afterEach.call({});
      await mochaHooks.afterAll.call({});

      // Restore.
      if (origGw) process.env.GATEWAY_RPC_URL = origGw;
      if (origCt) process.env.CIPHERTEXT_COMMITS_ADDRESS = origCt;
    });
  });
});
