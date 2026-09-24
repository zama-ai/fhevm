import { expect } from 'chai';
import { assertGpuExecutionDiversity, parseGpuExecutionCounters } from './gpuExecutionEvidence';

const snapshot = (acquisitions: number, overlap: number, capacity: number) => ({
  gpuExecution: [{ device: 0, capacity, acquisitions, single: acquisitions - overlap }],
});
const classes = '0=streams:4,window:100;1=streams:1,window:1;2=streams:2,window:200';
const before = new Map([[0, snapshot(10, 4, 4)], [1, snapshot(10, 0, 1)], [2, snapshot(10, 0, 2)]]);
const after = () => new Map([[0, snapshot(20, 8, 4)], [1, snapshot(20, 0, 1)], [2, snapshot(20, 0, 2)]]);

describe('Observed GPU execution permit budgets', () => {
  it('requires overlap during this workload, independent of configured flags', () => {
    expect(assertGpuExecutionDiversity(before, after(), classes)).to.include('4 with overlap');
    const noOverlap = after();
    noOverlap.set(0, snapshot(20, 4, 4));
    expect(() => assertGpuExecutionDiversity(before, noOverlap, classes)).to.throw('repeated multi-permit overlap');
  });
  it('rejects unavailable metrics, mismatched budgets, idle workers and counter reset', () => {
    const missing = after(); missing.delete(1);
    expect(() => assertGpuExecutionDiversity(before, missing, classes)).to.throw('lacks one observed');
    expect(() => assertGpuExecutionDiversity(before, after(), classes.replace('streams:4', 'streams:8')))
      .to.throw('matching its stream budget');
    expect(() => assertGpuExecutionDiversity(before, before, classes)).to.throw('recorded no work');
    const reset = after(); reset.set(0, snapshot(5, 0, 4));
    expect(() => assertGpuExecutionDiversity(before, reset, classes)).to.throw('reset');
  });
  it('rejects overlap from a one-permit worker and more than one visible device', () => {
    const invalid = after(); invalid.set(1, snapshot(20, 2, 1));
    expect(() => assertGpuExecutionDiversity(before, invalid, classes)).to.throw('one-permit');
    const multiple = after(); multiple.get(0)!.gpuExecution.push({ device: 1, capacity: 4, acquisitions: 10, single: 5 });
    expect(() => assertGpuExecutionDiversity(before, multiple, classes)).to.throw('lacks one observed');
  });
  it('allows a lazy histogram only before the first acquisition', () => {
    const empty = new Map([0, 1, 2].map(operator => [operator, {}]));
    expect(() => assertGpuExecutionDiversity(empty, after(), classes)).not.to.throw();
    expect(() => assertGpuExecutionDiversity(empty, empty, classes)).to.throw('lacks one observed');
  });
  it('parses real histogram label ordering and refuses incomplete or duplicate samples', () => {
    const count = 'coprocessor_gpu_execution_permits_count{capacity="4",device="0"} 10';
    const bucket = 'coprocessor_gpu_execution_permits_bucket{capacity="4",device="0",le="1"} 6';
    expect(parseGpuExecutionCounters(`${count}\n${bucket}`)).to.deep.eq([
      { device: 0, capacity: 4, acquisitions: 10, single: 6 },
    ]);
    expect(() => parseGpuExecutionCounters(count)).to.throw('incomplete');
    expect(() => parseGpuExecutionCounters(`${count}\n${count}\n${bucket}`)).to.throw('duplicate');
    expect(() => parseGpuExecutionCounters(`${count}\n${bucket.replace(' 6', ' 11')}`)).to.throw('incomplete');
    expect(parseGpuExecutionCounters('# no GPU limiter')).to.deep.eq([]);
  });
});
