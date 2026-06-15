export type AuthType = 'ApiKeyHeader';

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

export type Auth = AuthApiKeyHeader;
