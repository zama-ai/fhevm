import { expect } from 'chai';
import { assertionGroup, emitAssertions } from './assertionEvidence';

describe('Executed assertion receipts', () => {
  it('does not produce PASS evidence when a group is skipped or fails', async () => {
    const previous = process.env.CONSENSUS_RUN_ID;
    process.env.CONSENSUS_RUN_ID = 'isolated-receipts';
    const messages: string[] = [];
    const original = console.info;
    console.info = (message: string) => { messages.push(message); };
    try {
      // A skipped callback is precisely the false-green process-exit scenario.
      const skip = true;
      if (!skip) await assertionGroup('CASE', ['safety'], 'never executed', async () => undefined);
      expect(messages).to.have.length(0);
      let failed = false;
      try { await assertionGroup('CASE', ['safety'], 'failing check', async () => { throw new Error('rejected'); }); } catch { failed = true; }
      expect(failed).to.eq(true);
      expect(messages).to.have.length(0);
      const value = await assertionGroup('CASE', ['bytes'], 'canonical bytes compared', async () => 42);
      expect(value).to.eq(42);
      expect(messages).to.have.length(1);
      expect(JSON.parse(messages[0].slice('[consensus-assertion] '.length))).to.deep.eq({ runId: 'isolated-receipts', caseId: 'CASE', name: 'bytes', outcome: 'pass', detail: 'canonical bytes compared' });
      expect(messages.some(message => JSON.parse(message.slice('[consensus-assertion] '.length)).name === 'safety')).to.eq(false);
    } finally { console.info = original; if (previous === undefined) delete process.env.CONSENSUS_RUN_ID; else process.env.CONSENSUS_RUN_ID = previous; }
  });
  it('refuses an unbound receipt', () => {
    const previous = process.env.CONSENSUS_RUN_ID;
    delete process.env.CONSENSUS_RUN_ID;
    try { expect(() => emitAssertions('CASE', ['safety'], 'checked')).to.throw('require run'); }
    finally { if (previous !== undefined) process.env.CONSENSUS_RUN_ID = previous; }
  });
});
