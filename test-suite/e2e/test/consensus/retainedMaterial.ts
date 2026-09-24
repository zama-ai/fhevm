import fs from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { expect } from 'chai';
import { Contract, hexlify } from 'ethers';
import { ethers } from 'hardhat';
import { Pool } from 'pg';
import { getCoprocessorDbUrls, getRegisteredCoprocessorBuckets, rfc023CiphertextUrl } from './helpers';
import { assertOperatorsAgree, operatorSet } from './probe';
import { assertCiphertext128Format } from './validity';
import { readInputRows, type InputRow } from './inputEvidence';

type State = {
  chain: string; address: string; owner: string; seedTransaction: string;
  input: string; output: string; keys: unknown[]; inputs: InputRow[][];
  objects: { url: string; sha256: string; attestation: string | null }[];
};

const required = (name: string) => {
  const value = process.env[name];
  if (!value) throw new Error(`${name} required for retained-material campaign`);
  return value;
};
async function keys(databases: string[]): Promise<unknown[]> {
  return Promise.all(databases.map(async connectionString => {
    const pool = new Pool({ connectionString, connectionTimeoutMillis: 10_000, statement_timeout: 10_000 });
    try {
      const result = await pool.query("SELECT encode(key_id,'hex') id, encode(key_id_gw,'hex') gateway FROM keys ORDER BY sequence_number");
      expect(result.rows.length, 'installed key identity').to.be.greaterThan(0);
      return result.rows;
    } finally { await pool.end(); }
  }));
}
async function inputRows(databases: string[], handle: string): Promise<InputRow[][]> {
  return Promise.all(databases.map(async connectionString => {
    const pool = new Pool({ connectionString, connectionTimeoutMillis: 10_000, statement_timeout: 10_000 });
    try {
      const result = await pool.query("SELECT '0x'||encode(input_blob_hash,'hex') blob FROM ciphertexts WHERE handle=decode($1,'hex') AND is_input", [handle.slice(2)]);
      expect(result.rows, 'retained verified input has one canonical row').to.have.length(1);
      const rows = await readInputRows(connectionString, result.rows[0].blob);
      expect(rows).to.have.length(1);
      expect(rows[0].handle).to.eq(handle);
      expect(rows[0].type).to.eq(5);
      expect(rows[0].index).to.eq(0);
      return rows;
    } finally { await pool.end(); }
  }));
}
async function readObject(url: string) {
  const response = await fetch(url, { signal: AbortSignal.timeout(30_000) });
  expect(response.status, `retained object ${url}`).to.eq(200);
  const body = Buffer.from(await response.arrayBuffer());
  expect(body.length, 'nonempty retained object').to.be.greaterThan(0);
  return { url, sha256: createHash('sha256').update(body).digest('hex'), attestation: response.headers.get('x-amz-meta-ct-attestation') };
}

describe('Retained input and object compatibility', function () {
  this.timeout(20 * 60_000);
  it('[retained-material] preserves old inputs and objects and computes with fresh input', async function () {
    const phase = process.env.RETAINED_PHASE;
    if (!phase) this.skip();
    if (!['seed', 'verify', 'retired-sns', 'retired-tfhe', 'retired-check', 'retired-recover'].includes(phase!)) throw new Error('unknown retained-material phase');
    const file = required('RETAINED_STATE_FILE');
    const count = Number(required('COPROCESSOR_COUNT'));
    expect(Number.isInteger(count) && count >= 2).to.eq(true);
    const databases = getCoprocessorDbUrls(count);
    const [{ createInstances }, { initSigners, getSigners }] = await Promise.all([import('../instance'), import('../signers')]);
    await initSigners(2);
    const signers = await getSigners();
    const instance = (await createInstances(signers)).alice;
    const chain = String((await ethers.provider.getNetwork()).chainId);
    const buckets = await getRegisteredCoprocessorBuckets(required('GATEWAY_RPC_URL'), required('GATEWAY_CONFIG_ADDRESS'));
    expect(buckets).to.have.length(count);
    let state: State;
    let contract: Contract;
    const decrypt = async (handle: string, expected: bigint, address: string) => {
      expect(await instance.userDecryptSingleHandle({ handle, contractAddress: address, signer: signers.alice })).to.eq(expected);
      const publicValue = await instance.publicDecrypt([handle]);
      expect(publicValue.clearValues[handle as `0x${string}`]).to.eq(expected);
    };
    if (phase === 'seed') {
      try { await fs.access(file); throw new Error('retained fixture already exists; refuse reseeding'); }
      catch (error) { if ((error as NodeJS.ErrnoException).code !== 'ENOENT') throw error; }
      contract = await (await ethers.getContractFactory('RetainedMaterialFixture', signers.alice)).deploy() as unknown as Contract;
      await contract.waitForDeployment();
      const address = await contract.getAddress();
      const encrypted = await instance.encryptUint64({ value: 7n, contractAddress: address, userAddress: signers.alice.address });
      const receipt = await (await contract.seed(hexlify(encrypted.handles[0]), encrypted.inputProof)).wait();
      expect(receipt.status).to.eq(1);
      const input = (await contract.retainedInput()).toLowerCase();
      const output = (await contract.retainedOutput()).toLowerCase();
      await assertOperatorsAgree(databases, operatorSet(count), output, { timeoutMs: 300_000 });
      await decrypt(input, 7n, address); await decrypt(output, 49n, address);
      state = { chain, address, owner: signers.alice.address, seedTransaction: receipt.hash, input, output,
        keys: await keys(databases), inputs: await inputRows(databases, input),
        objects: await Promise.all(buckets.map(bucket => readObject(rfc023CiphertextUrl(bucket.bucketUrl, output)))) };
      await fs.writeFile(file, JSON.stringify(state), { flag: 'wx', mode: 0o600 });
    } else {
      state = JSON.parse(await fs.readFile(file, 'utf8'));
      expect(state.chain).to.eq(chain); expect(state.owner).to.eq(signers.alice.address);
      if (phase!.startsWith('retired-')) {
        contract = await ethers.getContractAt('RetainedMaterialFixture', state.address, signers.alice) as unknown as Contract;
        if (phase === 'retired-sns' || phase === 'retired-tfhe') {
          const increment = phase === 'retired-sns' ? 1 : 2;
          const receipt = await (await contract.probeRetired(increment)).wait();
          expect(receipt.status).to.eq(1);
          const handle = (await contract.retiredProbe()).toLowerCase();
          await fs.writeFile(`${file}.${phase}.json`, JSON.stringify({ handle, transactionHash: receipt.hash, model: String(49 + increment) }), { flag: 'wx', mode: 0o600 });
          const deadline = Date.now() + 180_000;
          for (const connectionString of databases) {
            const pool = new Pool({ connectionString, connectionTimeoutMillis: 10_000, statement_timeout: 10_000 });
            try {
              for (;;) {
                const rows = (await pool.query('SELECT is_completed,is_error FROM computations WHERE output_handle=$1 AND transaction_id=$2', [Buffer.from(handle.slice(2), 'hex'), Buffer.from(receipt.hash.slice(2), 'hex')])).rows;
                if (rows.length === 1 && !rows[0].is_error && rows[0].is_completed === (phase === 'retired-sns')) break;
                if (Date.now() >= deadline) throw new Error(`retired probe did not reach intended ${phase} queue`);
                await new Promise(resolve => setTimeout(resolve, 1_000));
              }
            } finally { await pool.end(); }
          }
        } else {
          for (const lane of ['sns', 'tfhe']) {
            const target = JSON.parse(await fs.readFile(`${file}.retired-${lane}.json`, 'utf8'));
            if (phase === 'retired-recover') {
              await assertOperatorsAgree(databases, operatorSet(count), target.handle, { timeoutMs: 300_000 });
              await decrypt(target.handle, BigInt(target.model), state.address);
              continue;
            }
            for (const connectionString of databases) {
              const pool = new Pool({ connectionString, connectionTimeoutMillis: 10_000, statement_timeout: 10_000 });
              try {
                const bytes = Buffer.from(target.handle.slice(2), 'hex');
                const rows = (await pool.query('SELECT is_completed,is_error FROM computations WHERE output_handle=$1 AND transaction_id=$2', [bytes, Buffer.from(target.transactionHash.slice(2), 'hex')])).rows;
                expect(rows).to.deep.eq([{ is_completed: lane === 'sns', is_error: false }]);
                if (lane === 'tfhe') expect(Number((await pool.query('SELECT count(*) n FROM ciphertexts WHERE handle=$1', [bytes])).rows[0].n)).to.eq(0);
                expect(Number((await pool.query('SELECT count(*) n FROM ciphertext_digest WHERE handle=$1 AND (ciphertext128 IS NOT NULL OR txn_is_sent)', [bytes])).rows[0].n)).to.eq(0);
              } finally { await pool.end(); }
            }
            for (const bucket of buckets) {
              const response = await fetch(rfc023CiphertextUrl(bucket.bucketUrl, target.handle), { signal: AbortSignal.timeout(30_000) });
              expect(response.status, 'retired SNS must not publish the prepared object').to.eq(404);
              await response.arrayBuffer();
            }
          }
        }
        console.info(`[retained-material] ${phase} complete chain=${chain} original=${state.seedTransaction}`);
        return;
      }
      expect(await keys(databases), 'original key identity retained').to.deep.eq(state.keys);
      expect(await inputRows(databases, state.input), 'original verified input bytes and identity retained').to.deep.eq(state.inputs);
      expect(await Promise.all(buckets.map(bucket => readObject(rfc023CiphertextUrl(bucket.bucketUrl, state.output)))),
        'original release-produced serving objects and metadata retained').to.deep.eq(state.objects);
      await decrypt(state.input, 7n, state.address); await decrypt(state.output, 49n, state.address);
      contract = await ethers.getContractAt('RetainedMaterialFixture', state.address, signers.alice) as unknown as Contract;
      const encrypted = await instance.encryptUint64({ value: 9n, contractAddress: state.address, userAddress: signers.alice.address });
      const receipt = await (await contract.consume(hexlify(encrypted.handles[0]), encrypted.inputProof)).wait();
      expect(receipt.status).to.eq(1);
      const fresh = (await contract.freshInput()).toLowerCase();
      expect(fresh).not.to.eq(state.input);
      await decrypt(fresh, 9n, state.address);
      const freshRows = await inputRows(databases, fresh);
      for (const rows of freshRows) expect(rows, 'fresh verified input agrees across operators').to.deep.eq(freshRows[0]);
      for (const [handle, model] of [[await contract.fromInput(), 16n], [await contract.fromOutput(), 58n]] as const) {
        await assertOperatorsAgree(databases, operatorSet(count), handle, { timeoutMs: 300_000 });
        await decrypt(handle, model, state.address);
        if (process.env.RETAINED_REQUIRE_GPU === '1') await assertCiphertext128Format(databases, 'gpu', [handle]);
      }
      await fs.writeFile(`${file}.verified-${Date.now()}.json`, JSON.stringify({ chain, seedTransaction: state.seedTransaction,
        consumptionTransaction: receipt.hash, oldInput: state.input, oldOutput: state.output, freshInput: fresh,
        fromInput: await contract.fromInput(), fromOutput: await contract.fromOutput() }), { mode: 0o600 });
    }
    console.info(`[retained-material] ${phase} complete chain=${chain} original=${state.seedTransaction}`);
  });
});
