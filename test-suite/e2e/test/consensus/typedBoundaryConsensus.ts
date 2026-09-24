import { assertRunValidity, assertCiphertext128Format } from './validity';
import { expect } from 'chai';
import { ethers } from 'hardhat';
import type { Contract } from 'ethers';
import { assertGatewayTopology, getCoprocessorDbUrls, waitForDatabaseReadiness, waitForConsensusDatabaseReports } from './helpers';
import { assertQuorumOutcome } from './probe';
import { typedBoundaryModel } from './typedBoundaryModel';
import { emitAssertions } from './assertionEvidence';
import type { TypedValue } from '../sdk/types';

const MATRIX = [
  { bits: 2, code: 0, type: 'bool', suffix: 'Bool' },
  { bits: 8, code: 2, type: 'uint8', suffix: '8' },
  { bits: 16, code: 3, type: 'uint16', suffix: '16' },
  { bits: 32, code: 4, type: 'uint32', suffix: '32' },
  { bits: 64, code: 5, type: 'uint64', suffix: '64' },
  { bits: 128, code: 6, type: 'uint128', suffix: '128' },
  { bits: 256, code: 8, type: 'uint256', suffix: '256' },
] as const;

describe('Typed transaction boundary consensus', function () {
  this.timeout(60 * 60_000);
  it('compares every intermediate across local and persisted operand graphs', async function () {
    if (process.env.RUN_TYPED_BOUNDARY_CONSENSUS !== '1') this.skip();
    const count = Number(process.env.COPROCESSOR_COUNT);
    const gateway = process.env.GATEWAY_RPC_URL!;
    const membership = await assertGatewayTopology(gateway, process.env.GATEWAY_CONFIG_ADDRESS!, count, Number(process.env.CONSENSUS_THRESHOLD));
    const databases = getCoprocessorDbUrls(count);
    await waitForDatabaseReadiness(databases);
    await assertRunValidity({ databaseUrls: databases, rpcUrl: process.env.RPC_URL });
    const backend = process.env.CONSENSUS_BACKEND_CLASS;
    if (!backend || (!backend.startsWith('cpu') && !backend.startsWith('gpu'))) throw new Error('typed boundary requires a known execution backend');
    const [{ initSigners, getSigners }, { createInstances }] = await Promise.all([import('../signers'), import('../instance')]);
    await initSigners(2);
    const signers = await getSigners();
    const instance = (await createInstances(signers)).alice;
    const fixture = await (await ethers.getContractFactory('TypedBoundaryFixture', signers.alice)).deploy() as unknown as Contract;
    await fixture.waitForDeployment();
    const address = await fixture.getAddress();
    const chainId = Number((await ethers.provider.getNetwork()).chainId);
    for (const row of MATRIX) {
      const value = row.bits === 2 ? false : (1n << BigInt(row.bits)) - 1n;
      const input = await instance.encryptTypedValues({ values: [{ type: row.type, value } as TypedValue], contractAddress: address, userAddress: signers.alice.address });
      let stageBlock = -1;
      for (const stage of [true, false]) {
        const tx = stage
          ? await fixture[`stage${row.suffix}`](input.handles[0], input.inputProof, { gasLimit: 20_000_000 })
          : await fixture[`consume${row.suffix}`]({ gasLimit: 20_000_000 });
        const receipt = await tx.wait();
        expect(receipt?.status).to.eq(1);
        if (stage) stageBlock = receipt.blockNumber;
        else expect(receipt.blockNumber, 'the consumer must cross a real block boundary').to.be.greaterThan(stageBlock);
        const handles = [...await fixture.values(row.code)].map((handle: string) => handle.toLowerCase());
        const expected = typedBoundaryModel(row.bits, stage);
        expect(handles.length).to.eq(expected.length);
        expect(new Set(handles).size, 'every listed graph output is distinct').to.eq(handles.length);
        const reports = await waitForConsensusDatabaseReports(databases, handles, { timeoutMs: 600_000 });
        for (const report of reports) {
          expect(report.outputs.length).to.eq(handles.length);
          expect(report.transactions).to.have.length(1);
          const completion = report.transactions[0];
          expect(`0x${completion.transactionId.toString('hex')}`).to.eq(receipt.hash.toLowerCase());
          expect(completion.hostChainId).to.eq(chainId);
          expect(completion.blockNumber).to.eq(receipt.blockNumber);
          expect(completion.totalCount).to.eq(handles.length);
          expect(completion.completedCount).to.eq(handles.length);
          expect(completion.errorCount).to.eq(0);
          for (const output of report.outputs) {
            expect(`0x${output.transactionId.toString('hex')}`).to.eq(receipt.hash.toLowerCase());
            expect(output.hostChainId).to.eq(chainId);
            expect(output.blockNumber).to.eq(receipt.blockNumber);
          }
        }
        await assertCiphertext128Format(databases, backend, handles);
        for (let index = 0; index < handles.length; index++) {
          await assertQuorumOutcome({ mode: 'required', gatewayRpcUrl: gateway, ciphertextCommitsAddress: process.env.CIPHERTEXT_COMMITS_ADDRESS!,
            handle: handles[index], authorizedSenders: membership.txSenders, threshold: membership.threshold, label: `typed ${row.type}/${stage}/${index}` });
          expect(await instance.userDecryptSingleHandle({ handle: handles[index], contractAddress: address, signer: signers.alice })).to.eq(expected[index]);
        }
        console.info(`[typed-boundary] ${row.type} stage=${stage} transaction=${receipt.hash} handles=${handles.join(',')} values=${expected.map(String).join(',')}`);
      }
    }
    emitAssertions('MAT-06-TYPED-BOUNDARIES', ['bytes', 'provenance', 'quorum', 'correctness', 'liveness'], 'Every named local/persisted typed graph output matched canonical bytes and receipt provenance, completed atomically, reached authorized quorum and decrypted to the modular model.');
    console.info('[typed-boundary] CASE COMPLETE');
  });
});
