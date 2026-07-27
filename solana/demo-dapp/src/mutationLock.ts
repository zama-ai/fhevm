import type { DemoSession } from './demoSession';

const lockName = (session: DemoSession): string =>
  `fhevm-solana-demo:mutation:${session.config.chainId}:${session.signer.address}`;

export const withWalletMutationLock = async <T>(
  session: DemoSession,
  action: () => Promise<T>,
  locks: LockManager | null | undefined = globalThis.navigator?.locks,
): Promise<T> => {
  if (locks == null) {
    throw new Error('This browser cannot safely coordinate wallet actions across tabs');
  }
  return locks.request(lockName(session), { ifAvailable: true, mode: 'exclusive' }, async (lock) => {
    if (lock === null) {
      throw new Error('This wallet already has an action running in another tab');
    }
    session.assertActive();
    const result = await action();
    session.assertActive();
    return result;
  });
};
