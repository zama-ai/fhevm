import { expect } from 'chai';
import { assertOriginalProofSucceeded, explicitProofRejection, invalidAuxiliaryProof, recoveredVerifierOutcomes, waitForOriginalProofSucceeded } from './proofRecovery';

describe('original proof recovery evidence', () => {
  const handle = `0x${'ab'.repeat(32)}`;
  it('does not accept deletion of the interrupted proof', () => {
    expect(() => assertOriginalProofSucceeded([], [handle])).to.throw('no committed success');
  });
  it('does not accept rejection followed by successful fresh encryption', () => {
    expect(() => assertOriginalProofSucceeded([{ verified: false, handles: '' }], [handle])).to.throw('rejected');
  });
  it('requires the original input handles, not unrelated verified material', () => {
    expect(() => assertOriginalProofSucceeded([{ verified: true, handles: 'cd'.repeat(32) }], [handle])).to.throw('exact input handles');
  });
  it('accepts a committed success bound to the original input', () => {
    expect(() => assertOriginalProofSucceeded([{ verified: true, handles: handle.slice(2) }], [handle])).not.to.throw();
  });
  it('requires the recovered verifier client and a post-fault outcome', () => {
    const fault = '2026-09-13T12:00:00Z';
    const success = { verified: true, handles: handle.slice(2), clientAddress: '172.20.0.4', observedAt: '2026-09-13T12:00:01Z', operation: 'replay-delete' };
    const unrelated = [
      { ...success, clientAddress: '172.20.0.5' },
      { ...success, observedAt: '2026-09-13T11:59:59Z' },
      { ...success, observedAt: 'invalid' },
    ];
    expect(() => assertOriginalProofSucceeded(recoveredVerifierOutcomes(unrelated, success.clientAddress, fault), [handle])).to.throw('no committed success');
    expect(() => assertOriginalProofSucceeded(recoveredVerifierOutcomes([...unrelated, success], success.clientAddress, fault), [handle])).not.to.throw();
    expect(() => recoveredVerifierOutcomes([success], undefined, fault)).to.throw('missing recovered verifier');
    expect(() => recoveredVerifierOutcomes([success], success.clientAddress, undefined)).to.throw('missing recovered verifier');
  });
  it('waits for the original local replay after SDK quorum and never treats an absent outcome as success', async () => {
    let reads = 0;
    await waitForOriginalProofSucceeded(async () => ++reads === 1 ? [] : [{ verified: true, handles: handle.slice(2) }], [handle], 100, 1);
    expect(reads).to.eq(2);
    let failure: unknown;
    try { await waitForOriginalProofSucceeded(async () => [], [handle], 1, 1); } catch (error) { failure = error; }
    expect(String(failure)).to.include('no committed success');
  });
  it('does not infer invalid-proof rejection from a missing queue row', () => {
    expect(explicitProofRejection([])).to.eq(false);
  });
  it('rejects an acceptance even if a later outcome is rejection', () => {
    expect(() => explicitProofRejection([{ verified: true, handles: handle }, { verified: false, handles: '' }])).to.throw('accepted');
  });
  it('accepts only an explicit committed rejection', () => {
    expect(explicitProofRejection([{ verified: false, handles: '' }])).to.eq(true);
  });
  it('reuses the exact original proof encoding while changing only its bound user', () => {
    const original = {
      zkProofId: '71', inputHex: 'abcd0001', chainId: '67890',
      contractAddress: `0x${'ab'.repeat(20)}`, userAddress: `0x${'00'.repeat(20)}`,
    };
    const before = JSON.stringify(original);
    const control = invalidAuxiliaryProof(original);
    expect(control.userAddress).to.eq(`0x${'00'.repeat(19)}01`);
    expect({ ...control, userAddress: original.userAddress }).to.deep.eq(original);
    expect(JSON.stringify(original), 'arming evidence must stay immutable').to.eq(before);
  });
  it('refuses missing proof bytes or malformed auxiliary data instead of testing decoder failure', () => {
    const original = {
      zkProofId: '71', inputHex: 'abcd0001', chainId: '67890',
      contractAddress: `0x${'ab'.repeat(20)}`, userAddress: `0x${'cd'.repeat(20)}`,
    };
    for (const invalid of [undefined, { ...original, inputHex: '' }, { ...original, inputHex: 'abc' },
      { ...original, chainId: 'undefined' }, { ...original, contractAddress: '0x1' }, { ...original, userAddress: '0x1' }]) {
      expect(() => invalidAuxiliaryProof(invalid as typeof original)).to.throw('original queued proof');
    }
  });
});
