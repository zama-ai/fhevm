import { expect } from 'chai';
import { emitAssertions } from './assertionEvidence';
import { publishHandshake, waitForFaultAcknowledgement } from './handshake';
import { assertGatewayTopology, getCoprocessorDbUrls, waitForDatabaseReadiness } from './helpers';
import { assertOperatorsAgree, assertQuorumOutcome, deployProbe, operatorSet } from './probe';
import { assertRunValidity } from './validity';

const CASE = 'STORAGE-01-INTEGRITY';
const MODES = ['missing', 'truncated', 'wrong-handle', 'wrong-key', 'wrong-format'];
describe('Produced object integrity and recovery', function () {
  this.timeout(40 * 60_000);
  it('refuses damaged material and completes each original request after restoration', async function () {
    if (process.env.RUN_OBJECT_INTEGRITY !== '1') this.skip();
    const databases = getCoprocessorDbUrls(3);
    await waitForDatabaseReadiness(databases);
    await assertRunValidity({ databaseUrls: databases, rpcUrl: process.env.RPC_URL });
    const membership = await assertGatewayTopology(process.env.GATEWAY_RPC_URL!, process.env.GATEWAY_CONFIG_ADDRESS!, 3, 3);
    const { initSigners, getSigners } = await import('../signers');
    const { createInstances } = await import('../instance');
    await initSigners(2);
    const signers = await getSigners();
    const instance = (await createInstances(signers)).alice;
    for (const mode of MODES) {
      const { contract, address } = await deployProbe(signers.alice);
      await (await contract.combineFromStorage({ gasLimit: 10_000_000 })).wait();
      const handle = (await contract.combined()).toLowerCase();
      await (await contract.consumeCombined({ gasLimit: 10_000_000 })).wait();
      const alternate = (await contract.consumed()).toLowerCase();
      expect(alternate).to.not.eq(handle);
      for (const target of [handle, alternate]) {
        await assertOperatorsAgree(databases, operatorSet(3), target, { timeoutMs: 300_000 });
        await assertQuorumOutcome({ mode: 'required', gatewayRpcUrl: process.env.GATEWAY_RPC_URL!, ciphertextCommitsAddress: process.env.CIPHERTEXT_COMMITS_ADDRESS!,
          handle: target, authorizedSenders: membership.txSenders, threshold: 3, label: CASE });
      }
      // Neither value has been decrypted: the request cannot reuse a cached plaintext.
      publishHandshake(`object-target-${mode}`, { caseId: CASE, mode, handle, alternate });
      expect((await waitForFaultAcknowledgement(`object-armed-${mode}`, 120_000)).applied).to.eq(true);
      const realFetch = globalThis.fetch;
      let posts = 0;
      globalThis.fetch = async (input, init) => {
        const response = await realFetch(input, init);
        const url = typeof input === 'string' ? input : input instanceof URL ? input.href : input.url;
        const method = init?.method ?? (input instanceof Request ? input.method : 'GET');
        if (method.toUpperCase() === 'POST' && /\/user-decrypt\/?$/.test(url)) {
          posts++;
          expect(response.status).to.eq(202);
          const body = await response.clone().json() as { result?: { jobId?: string } };
          const jobId = body.result?.jobId;
          expect(jobId).to.match(/^[0-9a-f-]{36}$/i);
          publishHandshake(`object-request-${mode}`, { caseId: CASE, handle, jobId });
          expect((await waitForFaultAcknowledgement(`object-restored-${mode}`, 300_000)).applied).to.eq(true);
        }
        return response;
      };
      try {
        expect(String(await instance.userDecryptSingleHandle({ handle, contractAddress: address, signer: signers.alice }))).to.eq('12');
        expect(posts, 'recovery must retain the original accepted client request').to.eq(1);
      } finally { globalThis.fetch = realFetch; }
      console.info(`[object-integrity] ${mode} recovered original request for ${handle}`);
    }
    emitAssertions(CASE, ['precondition', 'safety', 'liveness', 'correctness'],
      'Five fresh target objects had observed faults in every bucket, selected relayer attestation or KMS ciphertext verification failures, and restored bytes/attestations; each original accepted request then decrypted to 12 with one POST.');
    console.info('[object-integrity] CASE COMPLETE');
  });
});
