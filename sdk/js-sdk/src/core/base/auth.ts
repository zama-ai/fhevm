import type { Auth } from '../types/auth.js';
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
        (init.headers as Record<string, string>).Authorization = `Bearer ${auth.token}`;
        break;
      }

      case 'ApiKeyHeader': {
        const h = isNonEmptyString(auth.header) ? auth.header : 'x-api-key';
        (init.headers as Record<string, string>)[h] = auth.value;
        break;
      }

      case 'ApiKeyCookie': {
        const h = isNonEmptyString(auth.cookie) ? auth.cookie : 'x-api-key';
        if (typeof window !== 'undefined') {
          document.cookie = `${h}=${auth.value}; path=/; SameSite=Lax; Secure; HttpOnly;`;
          init.credentials = 'include';
        } else {
          (init.headers as Record<string, string>).Cookie = `${h}=${auth.value};`;
        }
        break;
      }
    }
  }
  return init;
}
