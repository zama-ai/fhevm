/** Public API surface: the vault builders' tests, which choose the host policy a builder sees. */
import type { HcuAccounts, HostPolicy } from './internal/hostPolicy.js';

/** A host policy whose HCU accounts are looked up by the application's scope. */
export const testHostPolicy = (
  denyListEnabled: boolean,
  hcuAccountsByScope: Readonly<Record<string, HcuAccounts>> = {},
): HostPolicy => ({
  denyListEnabled,
  hcuAccounts: async ({ scope }) => hcuAccountsByScope[scope] ?? {},
});
