import { expect } from 'chai';
// Runs without a stack: this test invokes only the pure evidence check.
const { assertDivergentReports } = require('./hostReportObservation.cjs');

describe('Divergent host report observation', () => {
  const candidate = '42'.repeat(32);
  const divergent = '43' + '42'.repeat(31);
  it('accepts precisely one changed publication from the selected operator', () => {
    expect(() => assertDivergentReports(candidate, [candidate, divergent, candidate])).not.to.throw();
  });
  it('refuses healthy, missing, wrong-victim and wrong-commitment observations', () => {
    for (const reports of [[candidate, candidate, candidate], [candidate, divergent],
      [divergent, candidate, candidate], [candidate, '00'.repeat(32), candidate]]) {
      expect(() => assertDivergentReports(candidate, reports)).to.throw();
    }
  });
});
