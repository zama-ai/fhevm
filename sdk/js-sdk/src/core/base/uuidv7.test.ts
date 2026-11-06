import { describe, it, expect } from 'vitest';
import { isUuidV7 } from './uuidv7.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/base/uuidv7.test.ts
////////////////////////////////////////////////////////////////////////////////

describe('uuidv7', () => {
  it('isUuidV7', () => {
    // True
    expect(isUuidV7('018f4d2e-7c3a-7b1e-8c2d-abcdef123456')).toEqual(true);
    expect(isUuidV7('00000000-0000-7000-8000-000000000000')).toEqual(true);
    expect(isUuidV7('ffffffff-ffff-7fff-bfff-ffffffffffff')).toEqual(true);
    expect(isUuidV7('FFFFFFFF-FFFF-7FFF-9FFF-FFFFFFFFFFFF')).toEqual(true); // case-insensitive

    // False: wrong version nibble (not 7)
    expect(isUuidV7('018f4d2e-7c3a-4b1e-8c2d-abcdef123456')).toEqual(false);
    expect(isUuidV7('018f4d2e-7c3a-1b1e-8c2d-abcdef123456')).toEqual(false);

    // False: wrong variant nibble (not 8/9/a/b)
    expect(isUuidV7('018f4d2e-7c3a-7b1e-0c2d-abcdef123456')).toEqual(false);
    expect(isUuidV7('018f4d2e-7c3a-7b1e-7c2d-abcdef123456')).toEqual(false);

    // False: wrong shape
    expect(isUuidV7('not-a-uuid')).toEqual(false);
    expect(isUuidV7('018f4d2e7c3a7b1e8c2dabcdef123456')).toEqual(false); // missing dashes
    expect(isUuidV7('018f4d2e-7c3a-7b1e-8c2d-abcdef12345')).toEqual(false); // too short
    expect(isUuidV7('018f4d2e-7c3a-7b1e-8c2d-abcdef1234567')).toEqual(false); // too long
    expect(isUuidV7('')).toEqual(false);

    // False: wrong type
    expect(isUuidV7(undefined)).toEqual(false);
    expect(isUuidV7(null)).toEqual(false);
    expect(isUuidV7(123)).toEqual(false);
    expect(isUuidV7({})).toEqual(false);
  });
});
