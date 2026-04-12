////////////////////////////////////////////////////////////////////////////////
// Auth
////////////////////////////////////////////////////////////////////////////////

export type AuthType = 'BearerToken' | 'ApiKeyHeader' | 'ApiKeyCookie';

/**
 * Bearer Token Authentication
 */
export type AuthBearerToken = {
  type: 'BearerToken';
  /**
   * The Bearer token.
   */
  token: string;
};

/**
 * Custom header authentication
 */
export type AuthApiKeyHeader = {
  type: 'ApiKeyHeader';
  /**
   * The header name. The default value is `x-api-key`.
   */
  header?: string;
  /**
   * The API key.
   */
  value: string;
};

/**
 * Custom cookie authentication
 */
export type AuthApiKeyCookie = {
  type: 'ApiKeyCookie';
  /**
   * The cookie name. The default value is `x-api-key`.
   */
  cookie?: string;
  /**
   * The API key.
   */
  value: string;
};

export type Auth = AuthBearerToken | AuthApiKeyHeader | AuthApiKeyCookie;
