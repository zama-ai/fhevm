import { emitAssertions } from './assertionEvidence';
import { expect } from 'chai';

import { publishHandshake, waitForFaultAcknowledgement } from './handshake';
import { requireQuorumConfiguration, getCoprocessorDbUrls, assertGatewayTopology, waitForDatabaseReadiness } from './helpers';
import { assertOperatorsAgree, assertQuorumOutcome, deployProbe, operatorSet } from './probe';

/** The POST happens once; only polling its accepted job resumes after the fault. */
describe('Client request survives service replacement', function () {
  this.timeout(20 * 60_000);

  it('decrypts the original accepted request after recovery', async function () {
    if (process.env.RUN_REQUEST_RECOVERY !== '1') this.skip();
    requireQuorumConfiguration('request-recovery', process.env.GATEWAY_RPC_URL ?? '', process.env.GATEWAY_CONFIG_ADDRESS ?? '', process.env.CIPHERTEXT_COMMITS_ADDRESS ?? '');
    const caseId = process.env.FAILURE_CASE_ID!;
    const count = Number(process.env.COPROCESSOR_COUNT ?? '3');
    const databases = getCoprocessorDbUrls(count);
    await waitForDatabaseReadiness(databases);
    const { getSigners, initSigners } = await import('../signers');
    const { createInstances } = await import('../instance');
    await initSigners(2);
    const signers = await getSigners();
    const instances = await createInstances(signers);
    const { contract, address } = await deployProbe(signers.alice);
    await (await contract.combineFromStorage({ gasLimit: 10_000_000 })).wait();
    const handle = (await contract.combined()).toLowerCase();
    await assertOperatorsAgree(databases, operatorSet(count), handle, { timeoutMs: 8 * 60_000 });
    const membership = await assertGatewayTopology(process.env.GATEWAY_RPC_URL!, process.env.GATEWAY_CONFIG_ADDRESS!, count, Number(process.env.CONSENSUS_THRESHOLD));
    await assertQuorumOutcome({
      mode: 'required', gatewayRpcUrl: process.env.GATEWAY_RPC_URL!,
      ciphertextCommitsAddress: process.env.CIPHERTEXT_COMMITS_ADDRESS!, handle,
      authorizedSenders: membership.txSenders, threshold: membership.threshold,
      timeoutMs: 8 * 60_000, label: caseId,
    });

    const realFetch = globalThis.fetch;
    let acceptedJob = '';
    let posts = 0;
    globalThis.fetch = async (input, init) => {
      const url = typeof input === 'string' ? input : input instanceof URL ? input.href : input.url;
      const method = init?.method ?? (input instanceof Request ? input.method : 'GET');
      const response = await realFetch(input, init);
      if (method.toUpperCase() === 'POST' && /\/user-decrypt\/?$/.test(url)) {
        posts += 1;
        expect(response.status, 'the relayer must accept the request before the crash').to.eq(202);
        const body = await response.clone().json() as { result?: { jobId?: string } };
        acceptedJob = body.result?.jobId ?? '';
        expect(acceptedJob).to.match(/^[0-9a-f-]{36}$/i);
        publishHandshake('request-target', { caseId, handle, jobId: acceptedJob });
        // Hold the client between acceptance and polling. The relayer keeps
        // processing the accepted job while the host observes its durable row.
        const fault = await waitForFaultAcknowledgement('request-fault', 5 * 60_000);
        expect(fault.applied, fault.detail).to.eq(true);
      }
      return response;
    };
    try {
      const plaintext = await instances.alice.userDecryptSingleHandle({
        handle, contractAddress: address, signer: signers.alice,
      });
      expect(String(plaintext)).to.eq('12');
      expect(posts, 'recovery must not create a replacement request').to.eq(1);
      expect(acceptedJob).to.not.eq('');
      emitAssertions(caseId, ['precondition', 'liveness', 'correctness'], 'The same accepted pending request completed after acknowledged recovery, with one POST and the expected plaintext.');
      console.info(`[request-recovery] ${caseId} CASE COMPLETE: job ${acceptedJob}, plaintext ${String(plaintext)}, one POST`);
    } finally {
      globalThis.fetch = realFetch;
    }
  });
});
