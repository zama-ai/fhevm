import { parseSchedulingClasses } from './helpers';
import { InvalidRunError } from './validity';

export interface GpuExecutionCounters {
  device: number;
  capacity: number;
  acquisitions: number;
  single: number;
}

/** Read the limiter's capacity, not merely the launcher's requested setting. */
export function parseGpuExecutionCounters(exposition: string): GpuExecutionCounters[] {
  const rows = new Map<string, Partial<GpuExecutionCounters>>();
  for (const line of exposition.split('\n')) {
    const match = /^coprocessor_gpu_execution_permits_(count|bucket)\{([^}]+)\} (\S+)$/.exec(line);
    if (!match) continue;
    const labels = new Map([...match[2].matchAll(/(\w+)="([^"]*)"/g)].map(m => [m[1], m[2]]));
    if (match[1] === 'bucket' && labels.get('le') !== '1') continue;
    const device = Number(labels.get('device'));
    const capacity = Number(labels.get('capacity'));
    const value = Number(match[3]);
    if (!Number.isSafeInteger(device) || device < 0 || !Number.isSafeInteger(capacity) || capacity < 1 ||
        !Number.isSafeInteger(value) || value < 0) throw new InvalidRunError('invalid GPU execution histogram');
    const key = `${device}/${capacity}`;
    const row = rows.get(key) ?? { device, capacity };
    const field = match[1] === 'count' ? 'acquisitions' : 'single';
    if (row[field] !== undefined) throw new InvalidRunError('duplicate GPU execution histogram sample');
    row[field] = value;
    rows.set(key, row);
  }
  return [...rows.values()].map(row => {
    if (row.acquisitions === undefined || row.single === undefined || row.single > row.acquisitions) {
      throw new InvalidRunError('incomplete GPU execution histogram');
    }
    return row as GpuExecutionCounters;
  });
}

export function assertGpuExecutionDiversity(
  before: Map<number, { gpuExecution?: GpuExecutionCounters[] }>,
  after: Map<number, { gpuExecution?: GpuExecutionCounters[] }>,
  schedulingClasses: string,
): string {
  const classes = parseSchedulingClasses(schedulingClasses);
  let serial = false;
  let overlapping = false;
  const summaries: string[] = [];
  for (const [operator, start] of before) {
    const setting = /(?:^|,)streams:(\d+)(?:,|$)/.exec(classes.get(operator) ?? '');
    const capacity = setting ? Number(setting[1]) : NaN;
    const end = after.get(operator)?.gpuExecution;
    if (!Number.isSafeInteger(capacity) || capacity < 1 || end?.length !== 1 || end[0].capacity !== capacity) {
      throw new InvalidRunError(`operator ${operator} lacks one observed GPU limiter matching its stream budget`);
    }
    const previous = start.gpuExecution ?? [];
    if (previous.length > 1 || previous.some(row => row.device !== end[0].device || row.capacity !== capacity)) {
      throw new InvalidRunError(`operator ${operator}'s GPU limiter changed during the workload`);
    }
    const acquired = end[0].acquisitions - (previous[0]?.acquisitions ?? 0);
    const single = end[0].single - (previous[0]?.single ?? 0);
    const overlap = acquired - single;
    if (acquired <= 0 || single < 0 || overlap < 0) {
      throw new InvalidRunError(`operator ${operator}'s GPU execution counters reset or recorded no work`);
    }
    if (capacity === 1) {
      if (overlap !== 0) throw new InvalidRunError('one-permit GPU worker recorded overlapping execution');
      serial = true;
    } else if (overlap >= 2) overlapping = true;
    summaries.push(`${operator}: capacity ${capacity}, ${acquired} acquisitions, ${overlap} with overlap`);
  }
  if (!serial || !overlapping) {
    throw new InvalidRunError('GPU workload must exercise one-permit execution and repeated multi-permit overlap');
  }
  return summaries.join('; ');
}
