import { afterEach, describe, expect, it, vi } from 'vitest';

const sendAndConfirm = vi.hoisted(() => vi.fn());
vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  sendAndConfirmTransactionFactory: () => sendAndConfirm,
}));
import { address, generateKeyPairSigner } from '@solana/kit';

import { testDemoClient } from './testDemoClient';

describe('demo client', () => {
  afterEach(() => vi.restoreAllMocks());

  it('waits for finalization under the caller abort signal', async () => {
    const payer = await generateKeyPairSigner();
    const { client } = testDemoClient(payer);
    const { signal } = new AbortController();

    await client.sendTransaction([{ programAddress: address('MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr') }], { abortSignal: signal });

    expect(sendAndConfirm).toHaveBeenCalledWith(expect.anything(), expect.objectContaining({ abortSignal: signal }));
  });
});
