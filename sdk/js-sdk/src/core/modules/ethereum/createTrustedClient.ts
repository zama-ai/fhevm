import type { TrustedClient, trustedClientBrand } from './types.js';
import { createTrustedValue, type TrustedValue } from '../../base/trustedValue.js';

export function createTrustedClient<native>(nativeClient: native, token: symbol): TrustedClient<native> {
  const tc: TrustedValue<native> = createTrustedValue(nativeClient, token);
  return tc as typeof tc & {
    [trustedClientBrand]: never;
  };
}
