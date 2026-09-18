import { assert, expect } from 'chai';

const traceInputFlowStage = async <TAction extends () => Promise<unknown>>(
  label: string,
  action: TAction,
): Promise<Awaited<ReturnType<TAction>>> => {
  const startedAt = Date.now();
  console.log(`[input-flow] ${label}: started at ${new Date(startedAt).toISOString()}`);
  try {
    const result = await action();
    console.log(`[input-flow] ${label}: completed in ${Date.now() - startedAt}ms`);
    return result as Awaited<ReturnType<TAction>>;
  } catch (error) {
    console.error(`[input-flow] ${label}: failed after ${Date.now() - startedAt}ms`, error);
    throw error;
  }
};

// Encrypts 7, runs TestInput.add42ToInput64, then asserts the result decrypts to 49
// through both user decryption and public decryption.
export const runAdd42InputAndDecrypt = async function (this: Mocha.Context) {
  const encryptedInput = await traceInputFlowStage('encrypt input', () =>
    this.instances.alice.encryptUint64({
      value: 7n,
      contractAddress: this.contractAddress,
      userAddress: this.signers.alice.address,
    }),
  );

  const tx = await traceInputFlowStage('submit add42 transaction', async () => {
    try {
      return await this.contract.add42ToInput64(encryptedInput.handles[0], encryptedInput.inputProof);
    } catch (error) {
      // Estimation failures have no transaction hash. Trace the same call on
      // Anvil so a missing selector can be distinguished from proof rejection.
      try {
        const call = await this.contract.add42ToInput64.populateTransaction(
          encryptedInput.handles[0],
          encryptedInput.inputProof,
        );
        const trace = await this.signers.alice.provider.send('debug_traceCall', [
          { from: this.signers.alice.address, to: call.to, data: call.data },
          'latest',
          { tracer: 'callTracer' },
        ]);
        console.error('[input-flow] add42 call trace:', JSON.stringify(trace));
      } catch (traceError) {
        console.error('[input-flow] call trace unavailable:', traceError);
      }
      throw error;
    }
  });
  console.log(`[input-flow] add42 transaction hash: ${tx.hash}`);
  const receipt = await traceInputFlowStage('wait for add42 transaction receipt', () => tx.wait());
  expect(receipt.status).to.equal(1);

  const handle = await traceInputFlowStage('read result handle', () => this.contract.resUint64());
  console.log(`[input-flow] result handle: ${handle}`);

  // User decrypt the result - should be 7 + 42 = 49.
  const decryptedValue = await traceInputFlowStage('user decrypt result', () =>
    this.instances.alice.userDecryptSingleHandle({
      handle,
      contractAddress: this.contractAddress,
      signer: this.signers.alice,
    }),
  );
  expect(decryptedValue).to.equal(49n);

  // Public decrypt the result - should be 49.
  const res = await traceInputFlowStage('public decrypt result', () => this.instances.alice.publicDecrypt([handle]));
  assert.deepEqual(res.clearValues, { [handle]: 49n });
};
