import { expect } from 'chai';
import { hexlify } from 'ethers';
import { ethers } from 'hardhat';
import { Pool } from 'pg';
import { emitAssertions } from './assertionEvidence';
import { publishHandshake, waitForFaultAcknowledgement } from './handshake';
import { assertGatewayTopology, getCoprocessorDbUrls, waitForDatabaseReadiness } from './helpers';
import { assertOperatorsAgree, assertQuorumOutcome, deployProbe, operatorSet } from './probe';
import { assertRunValidity } from './validity';

const CASE = 'SCH-04-GPU-RESERVATION';
describe('Bounded GPU reservation pressure', function () {
  this.timeout(20 * 60_000);
  it('retries the original unstamped work after admission capacity returns', async function () {
    if (process.env.RUN_GPU_PRESSURE !== '1') this.skip();
    const databases = getCoprocessorDbUrls(3);
    await waitForDatabaseReadiness(databases);
    await assertRunValidity({ databaseUrls: databases, rpcUrl: process.env.RPC_URL });
    const membership = await assertGatewayTopology(process.env.GATEWAY_RPC_URL!, process.env.GATEWAY_CONFIG_ADDRESS!, 3, 3);
    const { initSigners, getSigners } = await import('../signers');
    const { createInstances } = await import('../instance');
    await initSigners(2);
    const signers = await getSigners();
    const instance = (await createInstances(signers)).alice;
    const { contract, address } = await deployProbe(signers.alice);
    await (await contract.combineFromStorage({ gasLimit: 10_000_000 })).wait();
    await assertOperatorsAgree(databases, operatorSet(3), await contract.combined(), { timeoutMs: 300_000 });
    const encrypted = await instance.encryptUint64({ value: 5n, contractAddress: address, userAddress: signers.alice.address });
    const heavy = await (await ethers.getContractFactory('SchedulingPressureFixture', signers.alice)).deploy();
    await heavy.waitForDeployment();
    const heavyAddress = await heavy.getAddress();
    const heavyInput = await instance.encryptUint64({ value: 2n, contractAddress: heavyAddress, userAddress: signers.alice.address });
    publishHandshake('pressure-ready', { caseId: CASE });
    expect((await waitForFaultAcknowledgement('pressure-armed', 120_000)).applied).to.eq(true);
    const receipt = await (await heavy.heavy(hexlify(heavyInput.handles[0]), hexlify(heavyInput.inputProof), { gasLimit: 10_000_000 })).wait();
    expect(receipt!.status).to.eq(1);
    const handles = await Promise.all(Array.from({ length: 16 }, async (_, index) => (await heavy.outputs(index)).toLowerCase()));
    const handle = handles[0];
    publishHandshake('pressure-target', { caseId: CASE, handle, transactionHash: receipt!.hash });
    // Healthy operators finish while one reservation pool has no admission budget.
    await assertOperatorsAgree(databases, [0, 2], handle, { timeoutMs: 300_000 });
    expect((await waitForFaultAcknowledgement('pressure-observed', 300_000)).applied).to.eq(true);
    const childReceipt = await (await heavy.child({ gasLimit: 10_000_000 })).wait();
    expect(childReceipt!.status).to.eq(1);
    const dependent = (await heavy.dependent()).toLowerCase();
    // Same process, fresh independent transactions, after the heavy admission
    // timeout was observed. A peer-only liveness check cannot satisfy this.
    const shortHandles: string[] = [];
    for (let index = 0; index < 3; index++) {
      const input = index === 0 ? encrypted : await instance.encryptUint64({ value: BigInt(5 + index), contractAddress: address, userAddress: signers.alice.address });
      await (await contract.consumeExternal(hexlify(input.handles[0]), hexlify(input.inputProof), { gasLimit: 10_000_000 })).wait();
      const short = (await contract.consumed()).toLowerCase();
      await assertOperatorsAgree(databases, operatorSet(3), short, { timeoutMs: 150_000 });
      expect(String(await instance.userDecryptSingleHandle({ handle: short, contractAddress: address, signer: signers.alice }))).to.eq(String(12 + index));
      shortHandles.push(short);
    }
    const pool = new Pool({ connectionString: databases[1], connectionTimeoutMillis: 10_000, statement_timeout: 10_000 });
    try {
      for (const pending of [...handles, dependent]) {
        const target = Buffer.from(pending.slice(2), 'hex');
        const rows = (await pool.query('SELECT is_completed,is_error FROM computations WHERE output_handle=$1', [target])).rows;
        expect(rows, `named pending ${pending}`).to.have.length(1);
        expect(rows[0]).to.deep.eq({ is_completed: false, is_error: false });
        for (const table of ['ciphertexts', 'ciphertext_digest']) {
          expect(Number((await pool.query(`SELECT count(*) AS count FROM ${table} WHERE handle=$1`, [target])).rows[0].count)).to.eq(0);
        }
      }
    } finally { await pool.end(); }
    publishHandshake('pressure-release', { caseId: CASE, handle });
    expect((await waitForFaultAcknowledgement('pressure-restored', 120_000)).applied).to.eq(true);
    const verify = async (target: string, model: string) => {
      await assertOperatorsAgree(databases, operatorSet(3), target, { timeoutMs: 300_000 });
      await assertQuorumOutcome({ mode: 'required', gatewayRpcUrl: process.env.GATEWAY_RPC_URL!, ciphertextCommitsAddress: process.env.CIPHERTEXT_COMMITS_ADDRESS!, handle: target, authorizedSenders: membership.txSenders, threshold: 3, label: CASE });
      expect(String(await instance.userDecryptSingleHandle({ handle: target, contractAddress: address, signer: signers.alice }))).to.eq(model);
    };
    // At the completion checkpoint every operation in the original transaction
    // must be represented; checking just the last result would miss partial work.
    for (const [index, target] of handles.entries()) {
      await assertOperatorsAgree(databases, operatorSet(3), target, { timeoutMs: 300_000 });
      expect(String(await instance.userDecryptSingleHandle({ handle: target, contractAddress: heavyAddress, signer: signers.alice }))).to.eq(String(2n << BigInt(index + 1)));
    }
    await assertOperatorsAgree(databases, operatorSet(3), dependent, { timeoutMs: 300_000 });
    expect(String(await instance.userDecryptSingleHandle({ handle: dependent, contractAddress: heavyAddress, signer: signers.alice }))).to.eq('131073');
    console.info(`[gpu-pressure] same-worker short work=${shortHandles.join(',')} heavy transaction=${receipt!.hash} outputs=${handles.join(',')} pending child=${dependent}`);
    await verify(shortHandles[0], '12');
    await (await contract.consumeCombined({ gasLimit: 10_000_000 })).wait();
    await verify((await contract.consumed()).toLowerCase(), '19');
    emitAssertions(CASE, ['precondition', 'fault', 'safety', 'liveness', 'bytes', 'correctness'], 'A named 16-operation GPU transaction had an actual reservation timeout/retry while three fresh short transactions completed on the same worker; all heavy outputs and its real dependent remained pending without publication, then recovered with fleet bytes and every intermediate plaintext after capacity release. Fresh work also succeeded.');
    console.info('[gpu-pressure] CASE COMPLETE');
  });
});
