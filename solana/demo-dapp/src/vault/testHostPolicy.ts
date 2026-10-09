/** Public API surface: the vault builders' tests, which choose the host policy a builder sees. */
import type { Address } from '@solana/kit';
import { findHcuBlockMeterPda, findHcuTrustedAppRecordPda, type HcuBlockMeterSeeds } from '@fhevm/solana-zama-host';
import type { HcuAccounts, HostPolicy } from './internal/hostPolicy.js';

/** Both HCU accounts of `app`, which differ for every `(appProgram, scope)`. */
const hcuAccountsOf = async (app: HcuBlockMeterSeeds): Promise<Required<HcuAccounts>> => ({
  hcuBlockMeter: (await findHcuBlockMeterPda(app))[0],
  hcuTrustedAppRecord: (await findHcuTrustedAppRecordPda(app))[0],
});

/**
 * A host policy for builder tests. With `hcu`, every application gets both its block meter and its
 * trust record (the real policy passes one), so a test can tell which application's account sits in
 * each slot and catch a meter/trust swap.
 */
export const testHostPolicy = (denyListEnabled: boolean, hcu = false): HostPolicy => ({
  denyListEnabled,
  hcuAccounts: async (app) => (hcu ? hcuAccountsOf(app) : {}),
});

type ParsedAccounts = Readonly<Record<string, { readonly address: Address } | undefined>>;

/**
 * The addresses in an instruction's HCU slots, next to the ones `testHostPolicy(_, true)` gives each
 * slot's application. `slots` maps a slot prefix (`joinMint` for `joinMintHcuBlockMeter`, `''` for
 * `hcuBlockMeter`) to its application.
 */
export async function hcuSlots(accounts: ParsedAccounts, slots: Readonly<Record<string, HcuBlockMeterSeeds>>) {
  const actual: Record<string, Address | undefined> = {};
  const expected: Record<string, Address> = {};
  for (const [prefix, app] of Object.entries(slots)) {
    const hcu = await hcuAccountsOf(app);
    for (const [slot, address] of [
      [prefix ? `${prefix}HcuBlockMeter` : 'hcuBlockMeter', hcu.hcuBlockMeter],
      [prefix ? `${prefix}HcuTrustedAppRecord` : 'hcuTrustedAppRecord', hcu.hcuTrustedAppRecord],
    ] as const) {
      actual[slot] = accounts[slot]?.address;
      expected[slot] = address;
    }
  }
  return { actual, expected };
}
