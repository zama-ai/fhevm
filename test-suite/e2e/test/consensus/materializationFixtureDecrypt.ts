import type { Signer } from 'ethers';

import type { SdkInstance } from '../sdk/types';
import { FIXTURE_HANDLE_LABELS, type FixtureHandleLabel, type FixtureHandles } from './materializationFixtureModel';

export type FixturePlaintexts = Readonly<Record<FixtureHandleLabel, bigint | boolean | `0x${string}`>>;

/** The only required CPU-to-GPU equality boundary is these decrypted values. */
export async function decryptMaterializationFixture(
  instance: SdkInstance,
  owner: Signer & { readonly address: string },
  contractAddress: string,
  handles: FixtureHandles,
): Promise<FixturePlaintexts> {
  // The external attestation preflight completes before this plaintext oracle,
  // which deliberately uses one terminal-first user-decryption request at a
  // time.  The local Anvil baseline can pack a burst of KMS responses into one
  // gateway block while its WebSocket-only relayer listener misses the first
  // response logs in that block.  KMS and the Gateway have still completed
  // those requests, but the relayer leaves their jobs in ReceiptReceived and
  // an SDK poll waits forever.  That is a separate relayer/listener
  // limitation, not a materialization consensus result; a serial oracle keeps
  // this gate focused on the byte-consensus invariant and makes a failed
  // label directly diagnosable.  A dedicated relay fanout test must exercise
  // that path with a polling-listener backstop instead.
  const firstLabel: FixtureHandleLabel = 'terminal';
  const firstValue = await instance.userDecryptSingleHandle({
    handle: handles[firstLabel],
    contractAddress,
    signer: owner,
  });
  const plaintexts: Partial<Record<FixtureHandleLabel, FixturePlaintexts[FixtureHandleLabel]>> = {
    [firstLabel]: firstValue,
  };
  for (const label of FIXTURE_HANDLE_LABELS) {
    if (label === firstLabel) continue;
    plaintexts[label] = await instance.userDecryptSingleHandle({
      handle: handles[label],
      contractAddress,
      signer: owner,
    });
  }
  return plaintexts as FixturePlaintexts;
}
