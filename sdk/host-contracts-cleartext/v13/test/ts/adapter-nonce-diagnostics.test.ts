// Guards the diagnostics around a mis-implemented AbstractEthereumSigner.
//
// The invariant: every transaction this package sends from a signer must occupy that signer's next
// nonce. An adapter that lets its web3 library choose instead fails with `nonce has already been used`
// on an early transaction — a message that names nothing about the actual cause. `sendStep` in
// pkg/ts/utils.ts adds the failing step and, when the error mentions a nonce, the explanation.
//
// Driven through a stub signer rather than a real one: reproducing the genuine failure needs two sends
// inside ethers' 250ms cache window, which is a race and would make this test timing-dependent. What
// matters here is the message a nonce failure produces, so the failure is injected directly.
//
// Runs against the installed tarball fixture, so it checks the built package rather than the sources.
import { setupACLOwner } from '@fhevm/host-contracts-cleartext/ts';
import { expect, test } from 'vitest';

const ACL_ADDRESS = '0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D';
const PAUSER_SET_ADDRESS = '0x590e3330386Fa042843773541aaBb3a45EC3164D';
const SIGNER_ADDRESS = '0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4';

/** What anvil/ethers report when a nonce is reused. The wording is what `sendStep` keys the hint on. */
const NONCE_ERROR = 'nonce has already been used';

////////////////////////////////////////////////////////////////////////////////

/** A signer whose first send fails the way a nonce-reusing adapter does. */
function createNonceReusingSigner(): {
  readonly getAddress: () => Promise<string>;
  readonly deploy: () => Promise<never>;
  readonly writeContract: () => Promise<never>;
} {
  return {
    getAddress: () => Promise.resolve(SIGNER_ADDRESS),
    deploy: () => Promise.reject(new Error(NONCE_ERROR)),
    writeContract: () => Promise.reject(new Error(NONCE_ERROR)),
  };
}

/** A signer that fails for an unrelated reason, to check the nonce hint is not attached to everything. */
function createRevertingSigner(): {
  readonly getAddress: () => Promise<string>;
  readonly deploy: () => Promise<never>;
  readonly writeContract: () => Promise<never>;
} {
  return {
    getAddress: () => Promise.resolve(SIGNER_ADDRESS),
    deploy: () => Promise.reject(new Error('execution reverted: out of gas')),
    writeContract: () => Promise.reject(new Error('execution reverted: out of gas')),
  };
}

async function captureRejection(run: () => Promise<unknown>): Promise<Error> {
  try {
    await run();
  } catch (error) {
    return error instanceof Error ? error : new Error(String(error));
  }
  throw new Error('expected the call to reject, but it resolved');
}

////////////////////////////////////////////////////////////////////////////////

test('a nonce failure names the failing step and explains the cause', async () => {
  const signer = createNonceReusingSigner();

  const error = await captureRejection(() =>
    setupACLOwner({
      deployer: signer,
      currentAclOwner: signer,
      admin: signer,
      aclAddress: ACL_ADDRESS,
      pauserSetAddress: PAUSER_SET_ADDRESS,
    }),
  );

  // Which step failed. Without this the consumer only learns that *something* reused a nonce.
  expect(error.message).toContain('ACLOwner deploy failed');
  // The underlying message is kept, not swallowed.
  expect(error.message).toContain(NONCE_ERROR);
  // The cause, and where the contract is written down.
  expect(error.message).toContain('letting its web3 library choose nonces');
  expect(error.message).toContain('AbstractEthereumSigner in types/public.ts');
  // The original error stays reachable for anything that inspects it.
  expect(error.cause).toBeInstanceOf(Error);
  expect((error.cause as Error).message).toBe(NONCE_ERROR);
});

////////////////////////////////////////////////////////////////////////////////

test('an unrelated failure is labelled but gets no nonce lecture', async () => {
  const signer = createRevertingSigner();

  const error = await captureRejection(() =>
    setupACLOwner({
      deployer: signer,
      currentAclOwner: signer,
      admin: signer,
      aclAddress: ACL_ADDRESS,
      pauserSetAddress: PAUSER_SET_ADDRESS,
    }),
  );

  expect(error.message).toContain('ACLOwner deploy failed');
  expect(error.message).toContain('out of gas');
  // The whole point of making the hint conditional: a revert should not come with a paragraph about
  // nonce caching attached to it.
  expect(error.message).not.toContain('letting its web3 library choose nonces');
});
