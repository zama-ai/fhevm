import { describe, it, expect } from 'vitest';
import { isUuidV4 } from './uuidv4.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/base/uuidv4.test.ts
////////////////////////////////////////////////////////////////////////////////

describe('uuidv4', () => {
  it('isUuidV4', () => {
    // True
    expect(isUuidV4('018f4d2e-7c3a-4b1e-8c2d-abcdef123456')).toEqual(true);
    expect(isUuidV4('00000000-0000-4000-8000-000000000000')).toEqual(true);
    expect(isUuidV4('ffffffff-ffff-4fff-bfff-ffffffffffff')).toEqual(true);
    expect(isUuidV4('FFFFFFFF-FFFF-4FFF-9FFF-FFFFFFFFFFFF')).toEqual(true); // case-insensitive

    // False: wrong version nibble (not 4)
    expect(isUuidV4('018f4d2e-7c3a-7b1e-8c2d-abcdef123456')).toEqual(false);
    expect(isUuidV4('018f4d2e-7c3a-1b1e-8c2d-abcdef123456')).toEqual(false);

    // False: wrong variant nibble (not 8/9/a/b)
    expect(isUuidV4('018f4d2e-7c3a-4b1e-0c2d-abcdef123456')).toEqual(false);
    expect(isUuidV4('018f4d2e-7c3a-4b1e-7c2d-abcdef123456')).toEqual(false);

    // False: wrong shape
    expect(isUuidV4('not-a-uuid')).toEqual(false);
    expect(isUuidV4('018f4d2e7c3a7b1e8c2dabcdef123456')).toEqual(false); // missing dashes
    expect(isUuidV4('018f4d2e-7c3a-4b1e-8c2d-abcdef12345')).toEqual(false); // too short
    expect(isUuidV4('018f4d2e-7c3a-4b1e-8c2d-abcdef1234567')).toEqual(false); // too long
    expect(isUuidV4('')).toEqual(false);

    // False: wrong type
    expect(isUuidV4(undefined)).toEqual(false);
    expect(isUuidV4(null)).toEqual(false);
    expect(isUuidV4(123)).toEqual(false);
    expect(isUuidV4({})).toEqual(false);
  });
});
