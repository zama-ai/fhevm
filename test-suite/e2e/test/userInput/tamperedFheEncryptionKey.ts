import { expect } from 'chai';

import { createInstance } from '../instance';
import { type FheEncryptionKeyBytes, FhevmSdk } from '../sdk/fhevm-sdk/sdk';

// The titles carry "test user input uint64" so the `input-proof` profile (and
// so the light and standard suites) runs them.

/** A copy of `bytes` with its first byte flipped. */
const flipFirstByte = <T extends Uint8Array>(bytes: T): T => {
  const tampered = bytes.slice() as T;
  tampered[0] ^= 1;
  return tampered;
};

describe('Input Flow with a tampered options.fheEncryptionKey', function () {
  let fetched: FheEncryptionKeyBytes;

  before(async function () {
    const instance = await createInstance();
    if (!(instance instanceof FhevmSdk)) {
      // Legacy @zama-fhe/relayer-sdk adapter active: the check pinned here is @fhevm/sdk's.
      this.skip();
    }
    // The active key / CRS, as served by the relayer and verified by the SDK.
    fetched = await instance.rawClient.fetchFheEncryptionKeyBytes();
  });

  const expectVerificationError = async (fheEncryptionKey: FheEncryptionKeyBytes, subject: string) => {
    let error: unknown;
    try {
      await createInstance({ fheEncryptionKey });
    } catch (e) {
      error = e;
    }
    expect(error, 'ready should reject').to.be.instanceOf(Error);
    // TODO: also assert `name === 'FheEncryptionKeyVerificationError'` once
    // FhevmErrorBase keeps subclass names (it reports 'FhevmErrorBase' today).
    expect((error as Error).message).to.match(new RegExp(`${subject} digest mismatch`));
  };

  it('test user input uint64 refused: FHE public key one byte off', async function () {
    await expectVerificationError(
      { ...fetched, publicKeyBytes: { ...fetched.publicKeyBytes, bytes: flipFirstByte(fetched.publicKeyBytes.bytes) } },
      'FHE public key',
    );
  });

  it('test user input uint64 refused: CRS one byte off', async function () {
    await expectVerificationError(
      { ...fetched, crsBytes: { ...fetched.crsBytes, bytes: flipFirstByte(fetched.crsBytes.bytes) } },
      'CRS',
    );
  });
});
