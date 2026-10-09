import { describe, expect, it } from 'vitest';
import { address, getBase64Decoder, type Address, type ReadonlyUint8Array } from '@solana/kit';
import { base58 } from '@scure/base';
import {
  findHcuBlockMeterPda,
  findHcuTrustedAppRecordPda,
  findHostConfigPda,
  getHcuTrustedAppRecordEncoder,
  getHostConfigEncoder,
  HCU_UNLIMITED,
  MAX_COPROCESSOR_SIGNERS,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from '@fhevm/solana-zama-host';

import { readHostPolicy } from './hostPolicy.js';

const addr = (fill: number): Address => address(base58.encode(new Uint8Array(32).fill(fill)));
const app = { appProgram: addr(1), scope: addr(2) };

const hostConfig = (grantDenyListEnabled: boolean, hcuBlockCapPerApp: bigint) =>
  getHostConfigEncoder().encode({
    admin: addr(3),
    chainId: 1n,
    gatewayChainId: 1n,
    inputVerificationContract: new Uint8Array(20),
    coprocessorSigners: Array.from({ length: MAX_COPROCESSOR_SIGNERS }, () => new Uint8Array(20)),
    coprocessorSignerCount: 1,
    coprocessorThreshold: 1,
    decryptionContract: new Uint8Array(20),
    currentKmsContextId: new Uint8Array(32),
    currentKmsEpochId: new Uint8Array(32),
    paused: { execution: false, verifiedInputs: false, aclWrites: false },
    grantDenyListEnabled,
    maxHcuPerTx: HCU_UNLIMITED,
    maxHcuDepthPerTx: HCU_UNLIMITED,
    hcuBlockCapPerApp,
    bump: 0,
  });

const trustRecord = (trusted: boolean) =>
  getHcuTrustedAppRecordEncoder().encode({ program: app.appProgram, scope: app.scope, trusted, bump: 0 });

/** An RPC holding `accounts`, which records the addresses it is asked for. */
const rpcWith = (accounts: Record<string, ReadonlyUint8Array>) => {
  const reads: string[] = [];
  const rpc = {
    getAccountInfo: (account: Address) => ({
      send: async () => {
        reads.push(account);
        const data = accounts[account];
        return {
          value: data && {
            data: [getBase64Decoder().decode(data), 'base64'],
            owner: ZAMA_HOST_PROGRAM_ADDRESS,
            executable: false,
            lamports: 1n,
            space: BigInt(data.length),
          },
        };
      },
    }),
  };
  return { rpc: rpc as never, reads };
};

describe('readHostPolicy', () => {
  it('carries the deny-list flag and no HCU account while the block cap is unrestricted', async () => {
    const [config] = await findHostConfigPda();
    const { rpc, reads } = rpcWith({ [config]: hostConfig(true, HCU_UNLIMITED) });

    const host = await readHostPolicy(rpc);

    expect(host.denyListEnabled).toBe(true);
    expect(await host.hcuAccounts(app)).toEqual({});
    expect(reads).toEqual([config]);
  });

  it.each([
    ['has no trust record', undefined],
    ['is not trusted', trustRecord(false)],
  ])('passes the block meter of an application that %s under a finite cap', async (_, record) => {
    const [config] = await findHostConfigPda();
    const [trust] = await findHcuTrustedAppRecordPda(app);
    const { rpc } = rpcWith({ [config]: hostConfig(false, 1_000n), ...(record && { [trust]: record }) });

    const host = await readHostPolicy(rpc);

    expect(host.denyListEnabled).toBe(false);
    expect(await host.hcuAccounts(app)).toEqual({ hcuBlockMeter: (await findHcuBlockMeterPda(app))[0] });
  });

  it('passes the trust record of a trusted application under a finite cap', async () => {
    const [config] = await findHostConfigPda();
    const [trust] = await findHcuTrustedAppRecordPda(app);
    const { rpc } = rpcWith({ [config]: hostConfig(false, 1_000n), [trust]: trustRecord(true) });

    expect(await (await readHostPolicy(rpc)).hcuAccounts(app)).toEqual({ hcuTrustedAppRecord: trust });
  });
});
