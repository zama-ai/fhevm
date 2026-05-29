import type { Auth } from '../types/auth.js';
import { normalizeHeaderName, validateHeaderValue } from './fetch.js';
import { isNonEmptyString } from './string.js';

/**
 * Set the authentication method for the request. The default is no authentication.
 * It supports:
 * - Bearer Token
 * - Custom header
 * - Custom cookie
 */
export function setAuth(init: RequestInit, auth?: Auth): RequestInit {
  if (auth) {
    switch (auth.type) {
      case 'BearerToken': {
        (init.headers as Record<string, string>).authorization = `Bearer ${validateHeaderValue(auth.token)}`;
        break;
      }

      case 'ApiKeyHeader': {
        const h = isNonEmptyString(auth.header) ? normalizeHeaderName(auth.header) : 'x-api-key';
        (init.headers as Record<string, string>)[h] = validateHeaderValue(auth.value);
        break;
      }

      case 'ApiKeyCookie': {
        const h = isNonEmptyString(auth.cookie) ? normalizeHeaderName(auth.cookie) : 'x-api-key';
        const v = validateHeaderValue(auth.value);
        if (typeof window !== 'undefined') {
          document.cookie = `${h}=${v}; path=/; SameSite=Lax; Secure; HttpOnly;`;
          init.credentials = 'include';
        } else {
          (init.headers as Record<string, string>).cookie = `${h}=${v};`;
        }
        break;
      }
    }
  }
  return init;
}
