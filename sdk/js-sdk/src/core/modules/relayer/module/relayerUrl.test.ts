import { describe, it, expect } from 'vitest';
import { InvalidUrlError } from '../../../base/errors/InvalidUrlError.js';
import { buildRelayerUrlString, validateRelayerBaseUrl } from './relayerUrl.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/relayerUrl.test.ts
////////////////////////////////////////////////////////////////////////////////

describe('validateRelayerBaseUrl', () => {
  //////////////////////////////////////////////////////////////////////////////
  // Happy path + normalization
  //////////////////////////////////////////////////////////////////////////////

  const okCases: ReadonlyArray<{ name: string; input: string; hasAuth: boolean; expectedHref: string }> = [
    {
      name: 'bare origin gets trailing slash',
      input: 'https://relayer.example.com',
      hasAuth: false,
      expectedHref: 'https://relayer.example.com/',
    },
    {
      name: 'origin with trailing slash unchanged',
      input: 'https://relayer.example.com/',
      hasAuth: false,
      expectedHref: 'https://relayer.example.com/',
    },
    {
      name: 'path prefix kept; trailing slash added',
      input: 'https://relayer.example.com/api',
      hasAuth: false,
      expectedHref: 'https://relayer.example.com/api/',
    },
    {
      name: 'path prefix kept; existing trailing slash preserved',
      input: 'https://relayer.example.com/api/',
      hasAuth: false,
      expectedHref: 'https://relayer.example.com/api/',
    },
    {
      name: 'scheme/host lower-cased',
      input: 'HTTPS://Relayer.Example.COM/',
      hasAuth: false,
      expectedHref: 'https://relayer.example.com/',
    },
    {
      name: 'default https port collapsed',
      input: 'https://relayer.example.com:443/',
      hasAuth: false,
      expectedHref: 'https://relayer.example.com/',
    },
    {
      name: 'non-default port preserved',
      input: 'https://relayer.example.com:8443/',
      hasAuth: false,
      expectedHref: 'https://relayer.example.com:8443/',
    },
    {
      name: 'http allowed when no auth',
      input: 'http://relayer.example.com/',
      hasAuth: false,
      expectedHref: 'http://relayer.example.com/',
    },
    {
      name: 'https + hasAuth allowed',
      input: 'https://relayer.example.com/',
      hasAuth: true,
      expectedHref: 'https://relayer.example.com/',
    },
    {
      name: 'http + hasAuth + localhost allowed',
      input: 'http://localhost:8080/',
      hasAuth: true,
      expectedHref: 'http://localhost:8080/',
    },
    {
      name: 'http + hasAuth + 127.0.0.1 allowed',
      input: 'http://127.0.0.1:8080/',
      hasAuth: true,
      expectedHref: 'http://127.0.0.1:8080/',
    },
    {
      name: 'http + hasAuth + ::1 allowed',
      input: 'http://[::1]:8080/',
      hasAuth: true,
      expectedHref: 'http://[::1]:8080/',
    },
  ];

  it.each(okCases)('$name', ({ input, hasAuth, expectedHref }) => {
    const parsed = validateRelayerBaseUrl(input, hasAuth);
    expect(parsed).toBeInstanceOf(URL);
    expect(parsed.href).toBe(expectedHref);
    expect(parsed.pathname.endsWith('/')).toBe(true);
  });

  //////////////////////////////////////////////////////////////////////////////
  // Cases that throw
  //////////////////////////////////////////////////////////////////////////////

  const throwCases: ReadonlyArray<{ name: string; input: unknown; hasAuth: boolean; errorMatch: RegExp }> = [
    { name: 'undefined', input: undefined, hasAuth: false, errorMatch: /non-empty string/ },
    { name: 'null', input: null, hasAuth: false, errorMatch: /non-empty string/ },
    { name: 'empty string', input: '', hasAuth: false, errorMatch: /non-empty string/ },
    { name: 'number', input: 42, hasAuth: false, errorMatch: /non-empty string/ },
    { name: 'unparsable', input: 'not a url', hasAuth: false, errorMatch: /cannot parse/ },
    { name: 'ftp protocol', input: 'ftp://relayer.example.com/', hasAuth: false, errorMatch: /protocol.*ftp:/ },
    { name: 'file protocol', input: 'file:///tmp/x', hasAuth: false, errorMatch: /protocol.*file:/ },
    { name: 'has username', input: 'https://user@relayer.example.com/', hasAuth: false, errorMatch: /credentials/ },
    {
      name: 'has username + password',
      input: 'https://user:pass@relayer.example.com/',
      hasAuth: false,
      errorMatch: /credentials/,
    },
    {
      name: 'http + hasAuth + non-localhost',
      input: 'http://relayer.example.com/',
      hasAuth: true,
      errorMatch: /HTTPS is required/,
    },
    {
      name: 'http + hasAuth + non-loopback IP',
      input: 'http://10.0.0.1/',
      hasAuth: true,
      errorMatch: /HTTPS is required/,
    },
  ];

  it.each(throwCases)('throws: $name', ({ input, hasAuth, errorMatch }) => {
    expect(() => validateRelayerBaseUrl(input, hasAuth)).toThrow(errorMatch);
    expect(() => validateRelayerBaseUrl(input, hasAuth)).toThrow(InvalidUrlError);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('buildRelayerUrlString', () => {
  //////////////////////////////////////////////////////////////////////////////
  // Composition cases — table-driven
  //////////////////////////////////////////////////////////////////////////////

  const cases: ReadonlyArray<{ name: string; base: string; path: string; expected: string }> = [
    {
      name: 'origin + simple path',
      base: 'https://relayer.example.com/',
      path: 'v2/keyurl',
      expected: 'https://relayer.example.com/v2/keyurl',
    },
    {
      name: 'origin + leading-slash path',
      base: 'https://relayer.example.com/',
      path: '/v2/keyurl',
      expected: 'https://relayer.example.com/v2/keyurl',
    },
    {
      name: 'origin + trailing-slash path',
      base: 'https://relayer.example.com/',
      path: 'v2/keyurl/',
      expected: 'https://relayer.example.com/v2/keyurl',
    },
    {
      name: 'origin + leading & trailing slash path',
      base: 'https://relayer.example.com/',
      path: '/v2/keyurl/',
      expected: 'https://relayer.example.com/v2/keyurl',
    },
    {
      name: 'path-prefixed base + simple path',
      base: 'https://relayer.example.com/api/',
      path: 'v2/keyurl',
      expected: 'https://relayer.example.com/api/v2/keyurl',
    },
    {
      name: 'path-prefixed base + leading-slash path',
      base: 'https://relayer.example.com/api/',
      path: '/v2/keyurl',
      expected: 'https://relayer.example.com/api/v2/keyurl',
    },
    {
      name: 'deep path-prefixed base',
      base: 'https://api.example.com/zama/relayer/',
      path: 'v2/keyurl',
      expected: 'https://api.example.com/zama/relayer/v2/keyurl',
    },
    {
      name: 'multi-segment path',
      base: 'https://relayer.example.com/',
      path: 'v2/job/abc-123',
      expected: 'https://relayer.example.com/v2/job/abc-123',
    },
    {
      name: 'single-segment path',
      base: 'https://relayer.example.com/',
      path: 'health',
      expected: 'https://relayer.example.com/health',
    },
  ];

  it.each(cases)('$name', ({ base, path, expected }) => {
    const baseUrl = validateRelayerBaseUrl(base, false);
    expect(buildRelayerUrlString(baseUrl, path)).toBe(expected);
  });

  //////////////////////////////////////////////////////////////////////////////
  // Assertion: base must end with '/'
  //////////////////////////////////////////////////////////////////////////////

  it('throws (assertion) when base pathname does not end with "/"', () => {
    // Construct a URL whose pathname violates the precondition without going
    // through validateRelayerBaseUrl (which always enforces the trailing slash).
    const badBase = new URL('https://relayer.example.com/api');
    expect(badBase.pathname.endsWith('/')).toBe(false);
    expect(() => buildRelayerUrlString(badBase, 'v2/keyurl')).toThrow();
  });

  //////////////////////////////////////////////////////////////////////////////
  // End-to-end: validateRelayerBaseUrl output is safe to feed into builder
  //////////////////////////////////////////////////////////////////////////////

  it('round-trips with validateRelayerBaseUrl regardless of input trailing slash', () => {
    const a = validateRelayerBaseUrl('https://relayer.example.com/api', false);
    const b = validateRelayerBaseUrl('https://relayer.example.com/api/', false);
    expect(buildRelayerUrlString(a, 'v2/keyurl')).toBe('https://relayer.example.com/api/v2/keyurl');
    expect(buildRelayerUrlString(b, 'v2/keyurl')).toBe('https://relayer.example.com/api/v2/keyurl');
  });
});
