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
 * Custom header authentication.
 *
 * This is the only form accepted by Zama's hosted relayer (it requires the API
 * key in the `x-api-key` request header). Use it for Zama-hosted endpoints.
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

/**
 * Supported authentication methods.
 *
 * All three forms are supported so the SDK works out of the box with both Zama-hosted
 * and self-hosted relayers. A self-hosted relayer may accept any of them depending on
 * its deployment. Zama's hosted relayer accepts only `ApiKeyHeader` (`x-api-key`
 * request header) — `BearerToken` and `ApiKeyCookie` are rejected at the edge.
 */
export type Auth = AuthBearerToken | AuthApiKeyHeader | AuthApiKeyCookie;
