import { assert } from '../../../base/errors/InternalError.js';
import { InvalidUrlError } from '../../../base/errors/InvalidUrlError.js';
import { removePrefix, removeSuffix } from '../../../base/string.js';

/**
 * Validates and normalizes a relayer URL before it's used to build the request.
 *
 * Validation:
 *   - non-empty string
 *   - parses as a URL
 *   - protocol is `http:` or `https:`
 *   - contains no userinfo (`user:pass@host`)
 *   - contains no query string or fragment (they would corrupt path concatenation)
 *   - if `hasAuth` is true, requires `https:` unless the host is localhost
 *
 * Normalization (via WHATWG `URL`):
 *   - scheme and host lower-cased
 *   - default ports collapsed (`:443` for https, `:80` for http)
 *   - path percent-encoded canonically
 *   - bare-origin URLs get a single trailing `/` (`https://x` → `https://x/`)
 *
 * Throws {@link InvalidUrlError} on any failure so the caller surfaces a
 * uniform error type.
 *
 * @returns The normalized URL object.
 */
export function validateRelayerBaseUrl(relayerUrl: unknown, hasAuth: boolean): URL {
  if (typeof relayerUrl !== 'string' || relayerUrl.length === 0) {
    throw new InvalidUrlError({ message: 'Invalid relayerUrl: must be a non-empty string.' }, {});
  }

  let parsed: URL;
  try {
    parsed = new URL(relayerUrl);
  } catch (cause) {
    throw new InvalidUrlError({ message: `Invalid relayerUrl: cannot parse as URL}.`, cause }, {});
  }

  if (parsed.protocol !== 'https:' && parsed.protocol !== 'http:') {
    throw new InvalidUrlError(
      { message: `Invalid relayerUrl protocol: ${parsed.protocol} (expected http: or https:).` },
      {},
    );
  }

  if (parsed.username !== '' || parsed.password !== '') {
    throw new InvalidUrlError(
      { message: 'Invalid relayerUrl: URL must not contain credentials (user:pass@host).' },
      {},
    );
  }

  if (hasAuth && parsed.protocol === 'http:') {
    const isLocalhost =
      parsed.hostname === 'localhost' || parsed.hostname === '127.0.0.1' || parsed.hostname === '[::1]';
    if (!isLocalhost) {
      throw new InvalidUrlError(
        { message: 'Invalid relayerUrl: HTTPS is required when auth credentials are provided.' },
        {},
      );
    }
  }

  // At the end of validateRelayerUrl, just before `return parsed;`
  if (!parsed.pathname.endsWith('/')) {
    parsed.pathname += '/';
  }

  return parsed;
}

export function buildRelayerUrlString(relayerBaseUrl: URL, path: string): string {
  assert(relayerBaseUrl.pathname.endsWith('/'));
  const sanitizedPath = removeSuffix(removePrefix(path, '/'), '/');
  const url = new URL(sanitizedPath, relayerBaseUrl).toString();
  return removeSuffix(url, '/');
}
