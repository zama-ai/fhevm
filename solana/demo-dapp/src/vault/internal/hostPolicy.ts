import { AccountRole, type Address, type GetAccountInfoApi, type Instruction, type Rpc } from '@solana/kit';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import {
  fetchHostConfig,
  fetchMaybeHcuTrustedAppRecord,
  findHcuBlockMeterPda,
  findHcuTrustedAppRecordPda,
  findDenyScopeRecordPda,
  findHostConfigPda,
  HCU_UNLIMITED,
  type DenyScopeRecordSeeds,
  type HcuBlockMeterSeeds,
} from '@fhevm/solana-zama-host';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './generated/confidentialBatcher/programAddress.js';

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
  /** The host policy read at the start of the flow. */
  readonly host: HostPolicy;
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
      // An unrestricted cap touches neither a meter nor a trust record (block_cap.rs).
      if (data.hcuBlockCapPerApp === HCU_UNLIMITED) return {};
      const [trustRecord] = await findHcuTrustedAppRecordPda(app);
      const trust = await fetchMaybeHcuTrustedAppRecord(rpc, trustRecord);
      if (trust.exists && trust.data.trusted) return { hcuTrustedAppRecord: trustRecord };
      return { hcuBlockMeter: (await findHcuBlockMeterPda(app))[0] };
    },
  };
}

/** The application a mint's token executions run as. */
export const tokenApp = (mint: Address): DenyScopeRecordSeeds => ({
  appProgram: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  scope: mint,
});

/** The application the batcher's own executions for `batch` run as. */
export const batchApp = (batch: Address): DenyScopeRecordSeeds => ({
  appProgram: CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
  scope: batch,
});

/**
 * Appends, while the host's deny list is on, the deny records an instruction takes as its remaining
 * accounts: one per application each execution touches, in the order the instruction documents
 * (`confidential-batcher/src/lib.rs`, `confidential-token/src/fhe/mod.rs`).
 */
export async function withDenyRecords(
  instruction: Instruction,
  denyListEnabled: boolean,
  apps: readonly DenyScopeRecordSeeds[],
): Promise<Instruction> {
  if (!denyListEnabled) return instruction;
  const records = await Promise.all(apps.map((app) => findDenyScopeRecordPda(app)));
  return {
    ...instruction,
    accounts: [
      ...(instruction.accounts ?? []),
      ...records.map(([address]) => ({ address, role: AccountRole.READONLY })),
    ],
  };
}
