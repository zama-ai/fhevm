import { describe, expect, it } from 'vitest';

import {
  TRANSIENT_STORE_NOT_OPENED,
  ZamaHostProgramError,
  getZamaHostErrorMessage,
  isZamaHostError,
  translateZamaHostProgramError,
} from './programError.js';

describe('translateZamaHostProgramError', () => {
  it('maps TransientCloseMissing from an Anchor log line', () => {
    const error = new Error(
      [
        'Transaction simulation failed',
        'Program log: AnchorError caused by account: instructions. Error Code: TransientCloseMissing. Error Number: 6076. Error Message: matching final top-level transient_store close is required.',
      ].join('\n'),
    );
    const translated = translateZamaHostProgramError(error);
    expect(translated).toBeInstanceOf(ZamaHostProgramError);
    expect(translated?.errorName).toBe('TransientCloseMissing');
    expect(translated?.code).toBe(6076);
    expect(translated?.message).toContain('matching final top-level transient_store close is required');
    expect(translated?.docsUrl).toBe('https://docs.zama.org/protocol/solana/errors#TransientCloseMissing');
  });

  it('maps forgotten append as TransientStoreNotOpened from the host IDL code', () => {
    const error = new Error(
      'Program log: AnchorError caused by account: transient_store. Error Code: TransientStoreNotOpened. Error Number: 6080. Error Message: transient store must be opened for this transaction and closed last.',
    );
    const translated = translateZamaHostProgramError(error);
    expect(isZamaHostError(translated)).toBe(true);
    expect(translated?.errorName).toBe(TRANSIENT_STORE_NOT_OPENED);
    expect(translated?.code).toBe(6080);
    expect(translated?.docsUrl).toBe(`https://docs.zama.org/protocol/solana/errors#${TRANSIENT_STORE_NOT_OPENED}`);
    expect(getZamaHostErrorMessage(6080)).toBe('transient store must be opened for this transaction and closed last');
    expect(translateZamaHostProgramError({ context: { code: 6080, logs: [] } })?.errorName).toBe(
      TRANSIENT_STORE_NOT_OPENED,
    );
  });

  it('maps an unopened transient store from Anchor 3007 on that account, not from 0xbbf alone', () => {
    const withAccount = new Error(
      'Program log: AnchorError caused by account: transient_store. Error Code: AccountOwnedByWrongProgram. Error Number: 3007. Error Message: The given account is owned by a different program than expected.',
    );
    const translated = translateZamaHostProgramError(withAccount);
    expect(translated?.errorName).toBe(TRANSIENT_STORE_NOT_OPENED);
    expect(translated?.code).toBe(3007);
    expect(translated?.message).toContain('transient store must be opened');
    expect(translated?.docsUrl).toBe(`https://docs.zama.org/protocol/solana/errors#${TRANSIENT_STORE_NOT_OPENED}`);

    expect(translateZamaHostProgramError(new Error('custom program error: 0xbbf'))).toBeUndefined();
  });

  it('maps a raw host custom code from Kit context when the IDL knows it', () => {
    const error = Object.assign(new Error('Instruction #1 failed'), {
      context: { code: 6076, logs: [] },
    });
    const translated = translateZamaHostProgramError(error);
    expect(translated?.errorName).toBe('TransientCloseMissing');
    expect(getZamaHostErrorMessage(6076)).toBe('matching final top-level transient_store close is required');
  });

  it('leaves wallet and unrelated failures alone', () => {
    expect(translateZamaHostProgramError({ code: 4001, message: 'User rejected the request' })).toBeUndefined();
    expect(translateZamaHostProgramError(new Error('fetch failed'))).toBeUndefined();
  });
});
