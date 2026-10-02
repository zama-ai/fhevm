import { assertRunValidity } from './validity';
import { expect } from 'chai';
import { ethers } from 'hardhat';
import { Contract, EventLog, JsonRpcProvider, Wallet, concat, hexlify, keccak256, toUtf8Bytes } from 'ethers';
import { Pool } from 'pg';
import { assertGatewayTopology, getCoprocessorDbUrls, waitForDatabaseReadiness } from './helpers';
import { compareInputRows, readInputRows, waitForInputAgreement, type InputIdentity } from './inputEvidence';
import { emitAssertions } from './assertionEvidence';
import type { TypedValue } from '../sdk/types';

const ABI = [
  'event VerifyProofRequest(uint256 indexed zkProofId,uint256 indexed contractChainId,address contractAddress,address userAddress,bytes ciphertextWithZKProof,bytes extraData)',
  'event VerifyProofResponse(uint256 indexed zkProofId,bytes32[] ctHandles,bytes[] signatures)',
  'function verifyProofRequest(uint256,address,address,bytes,bytes)',
  'function isProofVerified(uint256) view returns(bool)',
  'function isProofRejected(uint256) view returns(bool)',
  'function getVerifyProofConsensusTxSenders(uint256) view returns(address[])',
  'function getRejectProofConsensusTxSenders(uint256) view returns(address[])',
];
const MATRIX = [
  { type: 'bool', code: 0, bits: 2 }, { type: 'uint8', code: 2, bits: 8 },
  { type: 'uint16', code: 3, bits: 16 }, { type: 'uint32', code: 4, bits: 32 },
  { type: 'uint64', code: 5, bits: 64 }, { type: 'uint128', code: 6, bits: 128 },
  { type: 'uint256', code: 8, bits: 256 },
] as const;

async function keyIdentities(databases: string[]): Promise<string[]> {
  return Promise.all(databases.map(async connectionString => {
    const pool = new Pool({ connectionString, connectionTimeoutMillis: 10_000, statement_timeout: 10_000 });
    try {
      const rows = await pool.query("SELECT encode(key_id_gw,'hex') AS gateway, encode(key_id,'hex') AS material FROM keys ORDER BY key_id_gw");
      expect(rows.rows.length, 'input campaign needs installed key identities').to.be.greaterThan(0);
      return JSON.stringify(rows.rows);
    } finally { await pool.end(); }
  }));
}

/** Pure input storage agreement, separate from computation/SNS output agreement. */
describe('Verified compact input consensus', function () {
  this.timeout(40 * 60_000);
  it('checks compact-list bytes, exact replay and explicit proof rejection', async function () {
    if (process.env.RUN_INPUT_CONSENSUS !== '1') this.skip();
    const count = Number(process.env.COPROCESSOR_COUNT);
    const rpc = process.env.GATEWAY_RPC_URL!;
    const membership = await assertGatewayTopology(rpc, process.env.GATEWAY_CONFIG_ADDRESS!, count, Number(process.env.CONSENSUS_THRESHOLD));
    const databases = getCoprocessorDbUrls(count);
    await waitForDatabaseReadiness(databases);
    await assertRunValidity({ databaseUrls: databases, rpcUrl: process.env.RPC_URL });
    const keysBefore = await keyIdentities(databases);
    expect(new Set(keysBefore).size, 'operators must have the same installed key identities').to.eq(1);
    const [{ createInstances }, { initSigners, getSigners }] = await Promise.all([import('../instance'), import('../signers')]);
    await initSigners(2);
    const signers = await getSigners();
    const instance = (await createInstances(signers)).alice;
    const fixture = await (await ethers.getContractFactory('InputConsensusFixture', signers.alice)).deploy() as unknown as Contract;
    await fixture.waitForDeployment();
    const address = await fixture.getAddress();
    const chainId = (await ethers.provider.getNetwork()).chainId;
    const gateway = new JsonRpcProvider(rpc);
    try {
      const verifier = new Contract(process.env.INPUT_VERIFICATION_ADDRESS!, ABI, gateway);
      const sender = new Wallet(process.env.GATEWAY_DEPLOYER_PRIVATE_KEY!, gateway);
      const payment = new Contract(process.env.PROTOCOL_PAYMENT_ADDRESS!, ['function getInputVerificationPrice() view returns(uint256)'], gateway);
      // The direct replay and two rejection requests bypass the funded relayer.
      // Fund their own sender using the stack's mocked token, keeping the deployed
      // fee policy intact. This setup is not a payment-protocol assertion.
      const budget = 3n * await payment.getInputVerificationPrice();
      if (budget > 0n) {
        const token = new Contract(process.env.ZAMA_OFT_ADDRESS!, [
          'function mint(address,uint256)', 'function approve(address,uint256) returns(bool)',
          'function balanceOf(address) view returns(uint256)',
          'function allowance(address,address) view returns(uint256)',
        ], sender);
        const balance = await token.balanceOf(sender.address) as bigint;
        if (balance < budget) expect((await (await token.mint(sender.address, budget - balance)).wait())?.status).to.eq(1);
        expect((await (await token.approve(await payment.getAddress(), budget)).wait())?.status).to.eq(1);
        expect(await token.balanceOf(sender.address)).to.be.at.least(budget);
        expect(await token.allowance(sender.address, await payment.getAddress())).to.be.at.least(budget);
      }
      const writer = verifier.connect(sender) as Contract;
      const accepted = async (id: bigint, handles: string[]) => {
        const deadline = Date.now() + 300_000;
        while (!await verifier.isProofVerified(id)) {
          if (await verifier.isProofRejected(id)) throw new Error(`valid input ${id} rejected`);
          if (Date.now() >= deadline) throw new Error(`input ${id} acceptance timed out`);
          await new Promise(resolve => setTimeout(resolve, 1_000));
        }
        const senders = (await verifier.getVerifyProofConsensusTxSenders(id) as string[]).map(sender => sender.toLowerCase());
        expect(new Set(senders).size).to.be.at.least(membership.threshold);
        expect(senders.every(sender => membership.txSenders.includes(sender))).to.eq(true);
        const events = await verifier.queryFilter(verifier.filters.VerifyProofResponse(id));
        expect(events).to.have.length(1);
        expect([...(events[0] as EventLog).args.ctHandles].map((value: string) => value.toLowerCase())).to.deep.eq(handles);
      };
      let replayTarget: { request: EventLog; identity: InputIdentity } | undefined;
      // Three values per blob: zero/max/repeated max. Largest list is 768 bits,
      // below the SDK's 2048-bit compact-list limit. Equal values have different indices.
      for (const row of MATRIX) {
        const max = row.type === 'bool' ? true : (1n << BigInt(row.bits)) - 1n;
        const values = [row.type === 'bool' ? false : 0n, max, max];
        const start = await gateway.getBlockNumber();
        const encrypted = await instance.encryptTypedValues({
          values: values.map(value => ({ type: row.type, value })) as TypedValue[], contractAddress: address, userAddress: signers.alice.address,
        });
        const handles = encrypted.handles.map(handle => hexlify(handle).toLowerCase());
        const requests = (await verifier.queryFilter(verifier.filters.VerifyProofRequest(null, chainId), start)).filter((event): event is EventLog =>
          event instanceof EventLog && event.args.contractAddress.toLowerCase() === address.toLowerCase() && event.args.userAddress.toLowerCase() === signers.alice.address.toLowerCase());
        const matches: EventLog[] = [];
        for (const request of requests) {
          const responses = await verifier.queryFilter(verifier.filters.VerifyProofResponse(request.args.zkProofId), start);
          if (responses.some(event => event instanceof EventLog && JSON.stringify([...event.args.ctHandles].map((h: string) => h.toLowerCase())) === JSON.stringify(handles))) matches.push(request);
        }
        expect(matches, 'one receipt-identified input request must own the exact list').to.have.length(1);
        const request = matches[0];
        const identity: InputIdentity = { handles, types: handles.map(() => row.code), chainId, acl: process.env.ACL_CONTRACT_ADDRESS!,
          blobHash: keccak256(concat([toUtf8Bytes('ZK-w_rct'), request.args.ciphertextWithZKProof])) };
        await accepted(request.args.zkProofId, handles);
        await waitForInputAgreement(databases, identity);
        const receipt = await (await fixture.accept(handles, row.code, encrypted.inputProof, { gasLimit: 15_000_000 })).wait();
        expect(receipt?.status).to.eq(1);
        for (let index = 0; index < handles.length; index++) {
          expect(await instance.userDecryptSingleHandle({ handle: handles[index], contractAddress: address, signer: signers.alice })).to.eq(values[index]);
        }
        console.info(`[input-consensus] ${row.type} request=${request.args.zkProofId} tx=${request.transactionHash} handles=${handles.join(',')} values=${values.map(String).join(',')}`);
        replayTarget = { request, identity };
      }
      if (!replayTarget) throw new Error('empty compact-input matrix');
      const { request, identity } = replayTarget;
      const original = await waitForInputAgreement(databases, identity);
      const submit = async (blob: string, user: string): Promise<bigint> => {
        const tx = await writer.verifyProofRequest(chainId, address, user, blob, request.args.extraData, { gasLimit: 15_000_000 });
        const receipt = await tx.wait();
        expect(receipt?.status).to.eq(1);
        const events = receipt.logs.map((log: { topics: string[]; data: string }) => { try { return verifier.interface.parseLog(log); } catch { return null; } })
          .filter((event: { name: string } | null) => event?.name === 'VerifyProofRequest');
        expect(events).to.have.length(1);
        return events[0].args.zkProofId;
      };
      const replay = await submit(request.args.ciphertextWithZKProof, signers.alice.address);
      await accepted(replay, identity.handles);
      const replayRows = await waitForInputAgreement(databases, identity);
      expect(replayRows).to.deep.eq(original);
      // Independent encryption of the same values must be correct in its own
      // identity domain; neither handle nor ciphertext equality is assumed.
      const independent = await instance.encryptTypedValues({ values: [0n, (1n << 256n) - 1n, (1n << 256n) - 1n].map(value => ({ type: 'uint256' as const, value })), contractAddress: address, userAddress: signers.alice.address });
      const independentHandles = independent.handles.map(handle => hexlify(handle));
      expect(independentHandles).to.not.deep.eq(identity.handles);
      const independentStart = request.blockNumber;
      const independentResponses = (await verifier.queryFilter(verifier.filters.VerifyProofResponse(), independentStart))
        .filter((event): event is EventLog => event instanceof EventLog && JSON.stringify([...event.args.ctHandles].map((h: string) => h.toLowerCase())) === JSON.stringify(independentHandles));
      expect(independentResponses).to.have.length(1);
      const independentId = independentResponses[0].args.zkProofId;
      const independentRequests = await verifier.queryFilter(verifier.filters.VerifyProofRequest(independentId), independentStart);
      expect(independentRequests).to.have.length(1);
      const independentBlob = (independentRequests[0] as EventLog).args.ciphertextWithZKProof;
      await accepted(independentId, independentHandles);
      await waitForInputAgreement(databases, { ...identity, handles: independentHandles,
        blobHash: keccak256(concat([toUtf8Bytes('ZK-w_rct'), independentBlob])) });
      const independentReceipt = await (await fixture.accept(independentHandles, 8, independent.inputProof, { gasLimit: 15_000_000 })).wait();
      expect(independentReceipt?.status).to.eq(1);
      for (let i = 0; i < independentHandles.length; i++) expect(await instance.userDecryptSingleHandle({ handle: independentHandles[i], contractAddress: address, signer: signers.alice })).to.eq(i === 0 ? 0n : (1n << 256n) - 1n);
      for (const [blob, user] of [[request.args.ciphertextWithZKProof.slice(0, -2), signers.alice.address], [request.args.ciphertextWithZKProof, signers.bob.address]]) {
        const hash = keccak256(concat([toUtf8Bytes('ZK-w_rct'), blob]));
        const before = await Promise.all(databases.map(db => readInputRows(db, hash)));
        const rejected = await submit(blob, user);
        const deadline = Date.now() + 300_000;
        while (!await verifier.isProofRejected(rejected)) {
          if (await verifier.isProofVerified(rejected)) throw new Error(`invalid proof ${rejected} was accepted`);
          if (Date.now() >= deadline) throw new Error(`no explicit rejection for invalid proof ${rejected}`);
          await new Promise(resolve => setTimeout(resolve, 1_000));
        }
        expect(await verifier.isProofVerified(rejected)).to.eq(false);
        const senders = (await verifier.getRejectProofConsensusTxSenders(rejected) as string[]).map(sender => sender.toLowerCase());
        expect(new Set(senders).size).to.be.at.least(membership.threshold);
        expect(senders.every(sender => membership.txSenders.includes(sender))).to.eq(true);
        expect(await Promise.all(databases.map(db => readInputRows(db, hash)))).to.deep.eq(before);
        console.info(`[input-consensus] explicitly rejected request=${rejected}; original material unchanged`);
      }
      compareInputRows(identity, await waitForInputAgreement(databases, identity));
      expect(await keyIdentities(databases)).to.deep.eq(keysBefore);
      emitAssertions('INPUT-01-COMPACT-LIST', ['bytes', 'quorum', 'correctness', 'safety'], 'All typed input lists matched their exact blob/index/chain identities and fleet bytes, reached authorized acceptance, and decrypted to their model. Installed key identities stayed unchanged.');
      emitAssertions('INPUT-02-REPLAY', ['bytes', 'quorum', 'safety'], 'Replayed the exact verified blob through Gateway; same handles and canonical rows remained unique and unchanged. Independently encrypted equal values decrypted correctly.');
      emitAssertions('INPUT-03-INVALID-PROOF', ['safety', 'quorum'], 'Truncated proof and incompatible user binding reached explicit authorized rejection without acceptance or changing existing input material.');
      console.info('[input-consensus] CASE COMPLETE');
    } finally { gateway.destroy(); }
  });
});
