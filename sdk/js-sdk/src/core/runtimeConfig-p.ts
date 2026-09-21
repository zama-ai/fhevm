import type { Auth } from './types/auth.js';

export function authsAreEqual(a: Auth | undefined, b: Auth | undefined): boolean {
  if (a === undefined || b === undefined) {
    return a === b;
  }

  switch (a.type) {
    case 'BearerToken':
      return b.type === 'BearerToken' && a.token === b.token;
    case 'ApiKeyHeader':
      return b.type === 'ApiKeyHeader' && a.value === b.value && a.header === b.header;
    case 'ApiKeyCookie':
      return b.type === 'ApiKeyCookie' && a.value === b.value && a.cookie === b.cookie;
  }
}
