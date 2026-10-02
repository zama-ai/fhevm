import { expect } from 'chai';
import { compareInputRows, inputHandle, type InputIdentity, type InputRow } from './inputEvidence';
import { ComparisonMismatch } from './mismatch';

const identity: InputIdentity = { blobHash: `0x${'11'.repeat(32)}`, chainId: 12345n,
  acl: `0x${'22'.repeat(20)}`, types: [5, 5], handles: [] };
identity.handles = identity.types.map((type, index) => inputHandle(identity, index, type));
const rows = (): InputRow[] => identity.handles.map((handle, index) => ({ handle, index, type: 5, version: 0,
  blobHash: identity.blobHash, isInput: true, bytes: `0x123${index}` }));
describe('Verified input oracle', () => {
  it('accepts the exact input set without computation or SNS evidence', () => {
    expect(() => compareInputRows(identity, [rows(), rows(), rows()])).not.to.throw();
  });
  for (const [name, mutate, kind] of [
    ['bytes', (r: InputRow[]) => { r[1].bytes = '0xffff'; }, 'raw-bytes'],
    ['type', (r: InputRow[]) => { r[1].type = 6; }, 'type-version'],
    ['index', (r: InputRow[]) => { r[1].index = 0; }, 'provenance'],
    ['blob', (r: InputRow[]) => { r[1].blobHash = `0x${'33'.repeat(32)}`; }, 'provenance'],
    ['duplicate', (r: InputRow[]) => { r.push(r[0]); }, 'storage-row-uniqueness'],
    ['missing', (r: InputRow[]) => { r.pop(); }, 'storage-row-uniqueness'],
  ] as const) it(`classifies a mutated ${name}`, () => {
    const changed = rows(); mutate(changed);
    let error: unknown;
    try { compareInputRows(identity, [rows(), changed]); } catch (caught) { error = caught; }
    expect(error).to.be.instanceOf(ComparisonMismatch);
    expect((error as ComparisonMismatch).kind).to.eq(kind);
  });
  it('rejects an identity bound to another chain or ACL even when every operator agrees', () => {
    for (const changed of [{ ...identity, chainId: 67890n }, { ...identity, acl: `0x${'33'.repeat(20)}` }]) {
      expect(() => compareInputRows(changed, [rows(), rows()])).to.throw('provenance');
    }
  });
  it('refuses an empty workload or one participant', () => {
    expect(() => compareInputRows(identity, [rows()])).to.throw('participant');
    expect(() => compareInputRows({ ...identity, handles: [], types: [] }, [[], []])).to.throw('participant');
  });
});
