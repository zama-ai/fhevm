import { expect } from 'chai';
import { typedBoundaryModel } from './typedBoundaryModel';

describe('Typed boundary independent model', () => {
  it('distinguishes reversed subtraction and overflow from unbounded arithmetic', () => {
    const results = typedBoundaryModel(8, false);
    expect(results.slice(0, 8)).to.deep.eq([0n, 1n, 2n, 254n, 127n, 1n, 254n, 255n]);
    expect(results.slice(8)).to.deep.eq([254n, true, 255n, 255n]);
  });
  it('keeps wide arithmetic exact and truncates at the actual destination width', () => {
    const max = (1n << 128n) - 1n;
    expect(typedBoundaryModel(128, false)).to.deep.eq([0n, 1n, 2n, max - 1n, max / 2n, 1n, max - 1n, max, max - 1n, true, max, 255n]);
    expect(typedBoundaryModel(256, false)).to.deep.eq([(1n << 256n) - 2n, true, (1n << 256n) - 1n, 255n]);
  });
  it('models the boolean producer and both boolean boundary consumers', () => {
    expect(typedBoundaryModel(2, true)).to.deep.eq([true, false, true, false]);
    expect(typedBoundaryModel(2, false)).to.deep.eq([false, true, false]);
  });
});
