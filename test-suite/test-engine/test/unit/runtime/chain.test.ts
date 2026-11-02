import type { TransactionResponse } from 'ethers';
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { describe, it } from 'node:test';

import { deriveAddress } from '../../../src/config/profiles/accounts.js';
import type { ResolvedChainSection } from '../../../src/config/profiles/types.js';
import type { EvidencePayload } from '../../../src/model/evidence.js';
import { loadArtifact, parseArtifact } from '../../../src/runtime/modules/chain/artifacts.js';
import { Chain } from '../../../src/runtime/modules/chain/chain.js';
import { bufferedGasLimit } from '../../../src/runtime/modules/chain/gas.js';
import { ANVIL_MNEMONIC, COUNTER_ARTIFACT, tempDir } from '../../helpers.js';

/** Accounts 0..4 of the Anvil/Hardhat test mnemonic on m/44'/60'/0'/0. */
const ANVIL_ADDRESSES = [
  '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266',
  '0x70997970C51812dc3A010C7d01b50e0d17dc79C8',
  '0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC',
  '0x90F79bf6EB2c4f870365E785982E1f101E93b906',
  '0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65',
];

export function chainSection(overrides: Partial<ResolvedChainSection> = {}): ResolvedChainSection {
  return {
    rpcUrl: { value: 'http://127.0.0.1:8545', secret: false, source: 'literal' },
    chainId: { value: 31337, secret: false, source: 'literal' },
    mnemonic: { value: ANVIL_MNEMONIC, secret: true, source: 'env:TE_ANVIL_MNEMONIC' },
    hdPath: "m/44'/60'/0'/0",
    wallets: { alice: 0, bob: 1, carol: 2, dave: 3, eve: 4 },
    gas: { bufferPercent: 50, limitCap: 10_000_000 },
    ...overrides,
  };
}

/** A port nothing listens on. */
const UNREACHABLE_URL = 'http://127.0.0.1:9';

const recorder = () => {
  const records: Omit<EvidencePayload, 'timestamp'>[] = [];
  return {
    records,
    evidence: {
      record: (kind: string, data: Record<string, unknown>, options: { label?: string } = {}) => {
        records.push({ kind, data, ...options });
      },
    },
  };
};

describe('gas buffer', () => {
  it('adds the buffer to the estimate, like the e2e withGasBuffer', () => {
    assert.equal(bufferedGasLimit(100_000n, { bufferPercent: 50, limitCap: 10_000_000 }), 150_000n);
    assert.equal(bufferedGasLimit(33_333n, { bufferPercent: 50, limitCap: 10_000_000 }), 49_999n);
    assert.equal(bufferedGasLimit(100_000n, { bufferPercent: 0, limitCap: 10_000_000 }), 100_000n);
  });

  it('caps the gas limit', () => {
    assert.equal(bufferedGasLimit(8_000_000n, { bufferPercent: 50, limitCap: 10_000_000 }), 10_000_000n);
  });
});

describe('wallets', () => {
  it('derives alice..eve like the e2e signers (Hardhat accounts 0..4)', () => {
    ANVIL_ADDRESSES.forEach((address, index) => {
      assert.equal(deriveAddress(ANVIL_MNEMONIC, "m/44'/60'/0'/0", index), address);
    });
    const chain = new Chain({ profileName: 'fx', section: chainSection(), evidence: recorder().evidence });
    assert.deepEqual(chain.roles(), ['alice', 'bob', 'carol', 'dave', 'eve']);
    assert.deepEqual(
      chain.roles().map((role) => chain.wallet(role).address),
      ANVIL_ADDRESSES,
    );
    assert.equal(chain.wallet('alice'), chain.wallet('alice'));
    assert.equal(chain.wallet('bob').role, 'bob');
    chain.destroy();
  });

  it('honours custom roles and HD path', () => {
    const chain = new Chain({
      profileName: 'fx',
      section: chainSection({ wallets: { deployer: 4 }, hdPath: "m/44'/60'/1'/0" }),
      evidence: recorder().evidence,
    });
    assert.equal(chain.wallet('deployer').address, deriveAddress(ANVIL_MNEMONIC, "m/44'/60'/1'/0", 4));
    assert.notEqual(chain.wallet('deployer').address, ANVIL_ADDRESSES[4]);
    chain.destroy();
  });

  it('rejects an unknown role, listing the known ones', () => {
    const chain = new Chain({ profileName: 'fx', section: chainSection(), evidence: recorder().evidence });
    assert.throws(
      () => chain.wallet('mallory'),
      /unknown wallet role 'mallory'\. Known roles: alice, bob, carol, dave, eve/,
    );
  });
});

describe('wallet safety', () => {
  it('refuses connect(), which would drop the gas buffer and the chain ID check', () => {
    const chain = new Chain({ profileName: 'fx', section: chainSection(), evidence: recorder().evidence });
    assert.throws(
      () => chain.wallet('alice').connect(),
      /wallet 'alice' cannot be reconnected: use chain\.wallet\('alice'\)/,
    );
    chain.destroy();
  });
});

describe('artifacts', () => {
  it('loads a Hardhat artifact', () => {
    const artifact = loadArtifact(COUNTER_ARTIFACT);
    assert.equal(artifact.contractName, 'Counter');
    assert.ok(Array.isArray(artifact.abi));
    assert.match(artifact.bytecode, /^0x[0-9a-f]+$/);
  });

  it('reports every problem of an invalid artifact', () => {
    assert.throws(
      () => parseArtifact({ abi: 'nope', bytecode: '0xzz' }, 'bad.json'),
      /bad\.json: invalid contract artifact: missing 'contractName'; 'abi' must be an array; 'bytecode' must be non-empty/,
    );
    // An interface or abstract contract has an empty bytecode.
    assert.throws(() => parseArtifact({ contractName: 'I', abi: [], bytecode: '0x' }), /'bytecode' must be non-empty/);
  });

  it('fails clearly on a missing or malformed file', () => {
    assert.throws(() => loadArtifact('/does/not/exist.json'), /exist\.json: cannot read contract artifact/);
    const file = path.join(tempDir(), 'broken.json');
    writeFileSync(file, readFileSync(COUNTER_ARTIFACT, 'utf8').slice(0, 50));
    assert.throws(() => loadArtifact(file), /broken\.json: cannot read contract artifact/);
  });
});

describe('Chain connection errors', () => {
  it('fails with the URL and the profile name when the RPC is unreachable, before sending anything', async () => {
    const { records, evidence } = recorder();
    const chain = new Chain({
      profileName: 'fx-down',
      section: chainSection({ rpcUrl: { value: UNREACHABLE_URL, secret: false, source: 'literal' } }),
      evidence,
    });
    await assert.rejects(
      chain.verifyConnection(),
      /profile 'fx-down': cannot reach the chain RPC at http:\/\/127\.0\.0\.1:9/,
    );
    const alice = chain.wallet('alice');
    await assert.rejects(
      chain.send(alice.sendTransaction({ to: ANVIL_ADDRESSES[1], value: 1n }), { label: 'transfer' }),
      /transaction 'transfer' was rejected: profile 'fx-down': cannot reach the chain RPC/,
    );
    assert.deepEqual(
      records.map((record) => [record.kind, record.data.status, record.label]),
      [['chain.tx', 'rejected', 'transfer']],
    );
    chain.destroy();
  });

  it('never prints a secret RPC URL', async () => {
    const chain = new Chain({
      profileName: 'fx-down',
      section: chainSection({ rpcUrl: { value: `${UNREACHABLE_URL}/secret-api-key`, secret: true, source: 'env:X' } }),
      evidence: recorder().evidence,
    });
    await assert.rejects(chain.verifyConnection(), (error: Error) => {
      assert.match(error.message, /\[REDACTED:chain\.rpcUrl\]/);
      assert.ok(!error.message.includes('secret-api-key'));
      return true;
    });
    chain.destroy();
  });
});

describe('Chain failure paths (stub provider)', () => {
  /** A Chain whose provider is replaced by a stub. */
  function stubbedChain(provider: object) {
    const { records, evidence } = recorder();
    const chain = new Chain({ profileName: 'fx', section: chainSection(), evidence });
    Object.defineProperty(chain, 'provider', { get: () => provider });
    return { chain, records };
  }

  it('records a broadcast transaction with status unknown when its receipt cannot be obtained', async () => {
    const { chain, records } = stubbedChain({
      waitForTransaction: async () => {
        throw new Error('connection lost');
      },
    });
    const hash = `0x${'ab'.repeat(32)}`;
    const response = { hash, from: ANVIL_ADDRESSES[0] } as unknown as TransactionResponse;
    await assert.rejects(
      chain.send(Promise.resolve(response), { label: 'lost' }),
      new RegExp(`could not get the receipt of transaction ${hash}: connection lost`),
    );
    assert.deepEqual(records, [
      {
        kind: 'chain.tx',
        label: 'lost',
        data: {
          chainId: 31337,
          hash,
          from: { role: 'alice', address: ANVIL_ADDRESSES[0] },
          status: 'unknown',
          error: 'connection lost',
        },
      },
    ]);
  });

  it('checks the chain ID again after a failed check, and keeps a successful one', async () => {
    let calls = 0;
    const { chain } = stubbedChain({
      send: async () => {
        calls += 1;
        if (calls === 1) throw new Error('temporary failure');
        return '0x7a69'; // 31337
      },
    });
    await assert.rejects(chain.verifyConnection(), /temporary failure/);
    await chain.verifyConnection();
    await chain.verifyConnection();
    assert.equal(calls, 2);
  });
});
