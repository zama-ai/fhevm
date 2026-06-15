import type { Auth } from '../types/auth.js';
import { normalizeHeaderName, validateHeaderValue } from './fetch.js';
import { isNonEmptyString } from './string.js';

/**
 * Set the authentication method for the request. The default is no authentication.
 * It supports:
 * - Custom header (default header name `x-api-key`)
 */
export function setAuth(init: RequestInit, auth?: Auth): RequestInit {
  if (auth) {
    const h = isNonEmptyString(auth.header) ? normalizeHeaderName(auth.header) : 'x-api-key';
    (init.headers as Record<string, string>)[h] = validateHeaderValue(auth.value);
  }
  return init;
}
