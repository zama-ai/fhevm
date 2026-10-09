import type { Address, GetAccountInfoApi, Rpc } from '@solana/kit';
import {
  fetchHostConfig,
  fetchMaybeHcuTrustedAppRecord,
  findHcuBlockMeterPda,
  findHcuTrustedAppRecordPda,
  findHostConfigPda,
  type HcuBlockMeterSeeds,
} from '@fhevm/solana-zama-host';

// zama-host's unrestricted block cap: no execution touches a meter or a trust record (block_cap.rs).
const UNRESTRICTED_HCU_BLOCK_CAP = 2n ** 64n - 1n;

/** The optional HCU accounts of one execution's application. */
export type HcuAccounts = {
  readonly hcuBlockMeter?: Address | undefined;
  readonly hcuTrustedAppRecord?: Address | undefined;
};

/** What the host's `HostConfig` asks of the instructions a flow builds. */
export type HostPolicy = {
  /** `grant_deny_list_enabled`: each execution carries the deny record of every application it touches. */
  readonly denyListEnabled: boolean;
  /**
   * The HCU accounts an execution of `app` carries: none while the block cap is unrestricted, the
   * trust record of a trusted application, otherwise the application's block meter.
   */
  readonly hcuAccounts: (app: HcuBlockMeterSeeds) => Promise<HcuAccounts>;
};

export type HostPolicyParameters = {
  /** The host policy read at the start of the flow; without it the instruction carries neither. */
  readonly host?: HostPolicy | undefined;
};

/**
 * Reads the host's deny-list flag and block cap once per flow. A change after this read makes the
 * program reject the transaction (a deny-record or missing-meter error); it never lands with wrong
 * accounts.
 */
export async function readHostPolicy(rpc: Rpc<GetAccountInfoApi>): Promise<HostPolicy> {
  const { data } = await fetchHostConfig(rpc, (await findHostConfigPda())[0]);
  return {
    denyListEnabled: data.grantDenyListEnabled,
    async hcuAccounts(app) {
      if (data.hcuBlockCapPerApp === UNRESTRICTED_HCU_BLOCK_CAP) return {};
      const [trustRecord] = await findHcuTrustedAppRecordPda(app);
      const trust = await fetchMaybeHcuTrustedAppRecord(rpc, trustRecord);
      if (trust.exists && trust.data.trusted) return { hcuTrustedAppRecord: trustRecord };
      return { hcuBlockMeter: (await findHcuBlockMeterPda(app))[0] };
    },
  };
}
