import { expect } from 'chai';
import { observeForbiddenQuorum, type QuorumWindowSnapshot } from './quorumObservation';

const expected = { keyId: '1', ciphertextDigest: '0xaa', snsCiphertextDigest: '0xbb' };
const submissions = ['0x01', '0x02'].map(sender => ({ sender, ...expected }));
function fixture(snapshot: (time: number) => QuorumWindowSnapshot, survivorCount = 2) {
  let time = 0;
  return {
    run: () => observeForbiddenQuorum({ read: async () => snapshot(time), expected,
      authorizedSenders: ['0x01', '0x02', '0x03'], survivorCount,
      windowMs: 100, maxStallMs: 30, pollMs: 10, submissionTimeoutMs: 100,
      now: () => time, sleep: async ms => { time += ms; } }),
    time: () => time,
  };
}
async function rejects(run: () => Promise<unknown>, message: string) {
  let caught: unknown;
  try { await run(); } catch (error) { caught = error; }
  expect(String(caught)).to.include(message);
}
describe('no-quorum observation validity', () => {
  it('requires exactly one visible sender for the below-majority partition', async () => {
    expect((await fixture(time => ({ block: time + 1, consensusCount: 0, submissions: submissions.slice(0, 1) }), 1).run()).samples).to.be.greaterThan(1);
    await rejects(fixture(time => ({ block: time + 1, consensusCount: 0, submissions }), 1).run, 'additional senders');
  });
  it('starts the full window only after healthy survivor submissions and observes progress inside it', async () => {
    const test = fixture(time => ({ block: time + 1, consensusCount: 0, submissions: time < 20 ? [] : submissions }));
    expect(await test.run()).to.deep.eq({ firstBlock: 21, lastBlock: 121, samples: 13 });
  });
  it('rejects a gateway stalled throughout the window even if it resumes immediately afterwards', async () => {
    const test = fixture(time => ({ block: time <= 100 ? 1 : time, consensusCount: 0, submissions }));
    await rejects(test.run, 'stopped advancing');
    expect(test.time()).to.eq(30);
  });
  it('rejects a gateway that advances once then stalls for the rest of the window', async () => {
    await rejects(fixture(time => ({ block: Math.min(time, 20) + 1, consensusCount: 0, submissions })).run, 'stopped advancing');
  });
  it('rejects absent survivor submissions rather than treating a broken sender as threshold enforcement', async () => {
    await rejects(fixture(time => ({ block: time + 1, consensusCount: 0, submissions: submissions.slice(0, 1) })).run, 'did not submit');
  });
  it('does not count repeated submissions by one operator as two survivors', async () => {
    await rejects(fixture(time => ({ block: time + 1, consensusCount: 0, submissions: [submissions[0], submissions[0]] })).run, 'did not submit');
  });
  it('rejects a forbidden event, an RPC error and a submission for different material', async () => {
    await rejects(fixture(time => ({ block: time + 1, consensusCount: time >= 50 ? 1 : 0, submissions })).run, 'forbidden Gateway');
    await rejects(fixture(() => { throw new Error('RPC unavailable'); }).run, 'RPC unavailable');
    await rejects(fixture(time => ({ block: time + 1, consensusCount: 0,
      submissions: [{ ...submissions[0], ciphertextDigest: '0xcc' }, submissions[1]] })).run, 'does not bind');
  });
});
