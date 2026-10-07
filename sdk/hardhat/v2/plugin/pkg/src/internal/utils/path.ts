import * as path from 'path';

import { assertHHFhevm } from '../error';

export function toUnixRelPath(p: string): string {
  assertHHFhevm(!path.isAbsolute(p), 'assert(!path.isAbsolute(p)) failed.');
  const pUnix = p.replace(/\\/g, '/');
  assertHHFhevm(!pUnix.includes('\\'), 'assert(p.indexOf("\\") < 0) failed.');
  return pUnix;
}
