import { expect } from 'chai';
import { assertGatewayWatermarkStopped, requireGatewayWatermark } from './faultEvidence';

describe('fault observation evidence', () => {
  it('rejects missing or malformed gateway observations, including null to null', () => {
    for (const invalid of [null, NaN, Infinity, -1, 1.5]) {
      expect(() => requireGatewayWatermark(invalid, 1)).to.throw('no readable gateway watermark');
      expect(() => assertGatewayWatermarkStopped(invalid, invalid, 1)).to.throw('no readable gateway watermark');
      expect(() => assertGatewayWatermarkStopped(10, invalid, 1)).to.throw('no readable gateway watermark');
    }
  });
  it('requires a real unchanged watermark and rejects advancement or regression', () => {
    expect(() => assertGatewayWatermarkStopped(10, 10, 1)).not.to.throw();
    expect(() => assertGatewayWatermarkStopped(10, 11, 1)).to.throw('changed its gateway watermark');
    expect(() => assertGatewayWatermarkStopped(10, 9, 1)).to.throw('changed its gateway watermark');
  });
});
