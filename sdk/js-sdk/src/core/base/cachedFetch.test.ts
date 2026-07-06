import { describe, expect, it } from 'vitest';

import { CACHE_TTL_15MIN } from './cachedFetch.js';

describe('cachedFetch', () => {
  it('defines CACHE_TTL_15MIN as fifteen minutes', () => {
    expect(CACHE_TTL_15MIN).to.equal(15 * 60 * 1000);
  });
});
