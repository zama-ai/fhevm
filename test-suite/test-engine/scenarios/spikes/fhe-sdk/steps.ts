import { EngineWorld, loadArtifact } from '#engine';
import { Given, Then, When, setWorldConstructor } from '@cucumber/cucumber';
import { type Contract, hexlify } from 'ethers';
import assert from 'node:assert/strict';
import path from 'node:path';

import {
  type FheClient,
  type FheSettings,
  activeResources,
  createFheClient,
  encryptUint64,
  isYes,
  publicDecrypt,
  resourcesAdded,
  userDecrypt,
} from './sdk-client.js';

/** Scenario state. A fresh instance is created for every Gherkin scenario. */
class FheSpikeWorld extends EngineWorld {
  /**
   * Resources alive before the scenario opened anything. Every measurement is taken inside a step
   * after one turn of the event loop, so that each counts that step's own timeout timer (Cucumber
   * arms it once the step function has returned its promise).
   */
  baseline: Record<string, number> = {};
  client: FheClient | undefined;
  contract: Contract | undefined;
  encrypted: { handle: Uint8Array; inputProof: Uint8Array } | undefined;

  settings(): FheSettings {
    const profile = this.engine.profile;
    const text = (key: string) => String(profile.setting(key));
    const address = (key: string) => text(key) as `0x${string}`;
    return {
      relayerUrl: text('relayer.url'),
      chainId: this.engine.chain.chainId,
      gatewayChainId: Number(profile.setting('gateway.chain-id')),
      decryption: address('gateway.decryption'),
      inputVerification: address('gateway.input-verification'),
      acl: address('contracts.acl'),
      inputVerifier: address('contracts.input-verifier'),
      kmsVerifier: address('contracts.kms-verifier'),
      protocolConfig: address('contracts.protocol-config'),
    };
  }

  /** The SDK client, created on first use and timed. */
  async fhe(): Promise<FheClient> {
    if (!this.client) {
      const singleThread = isYes(this.engine.profile.setting('spike.single-thread'));
      const started = Date.now();
      this.client = await createFheClient(this.settings(), this.engine.chain.provider, singleThread);
      this.engine.evidence.record(
        'spike.sdk-ready',
        { singleThread, readyMs: Date.now() - started },
        { label: 'SDK client ready' },
      );
    }
    return this.client;
  }

  get contractAddress(): string {
    return this.contract!.target as string;
  }
}

setWorldConstructor(FheSpikeWorld);

/** Resources alive now, taken on the next turn of the event loop (see `baseline`). */
const settledResources = async (): Promise<Record<string, number>> => {
  await new Promise((resolve) => setImmediate(resolve));
  return activeResources();
};

Given('alice has deployed the TestInput contract', async function (this: FheSpikeWorld) {
  this.baseline = await settledResources();
  const artifact = loadArtifact(path.join(this.engine.scenario.directory, 'artifacts', 'TestInput.json'));
  this.contract = await this.engine.chain.deploy(artifact, [], { from: 'alice', label: 'Deploy TestInput' });
});

When('alice encrypts the uint64 value {int} for the contract', async function (this: FheSpikeWorld, value: number) {
  const client = await this.fhe();
  const alice = this.engine.chain.wallet('alice');
  const started = Date.now();
  this.encrypted = await encryptUint64(client, BigInt(value), this.contractAddress, alice.address);
  this.engine.evidence.record(
    'spike.encrypt',
    {
      type: 'uint64',
      handle: hexlify(this.encrypted.handle),
      inputProofBytes: this.encrypted.inputProof.length,
      encryptMs: Date.now() - started,
    },
    { label: 'alice encrypts' },
  );
});

When('alice calls add42ToInput64 with the encrypted value', async function (this: FheSpikeWorld) {
  const { handle, inputProof } = this.encrypted!;
  await this.engine.chain.send(this.contract!.getFunction('add42ToInput64')(handle, inputProof), {
    label: 'alice calls add42ToInput64',
  });
});

Then('alice user-decrypts the result as {int}', async function (this: FheSpikeWorld, expected: number) {
  const client = await this.fhe();
  const handle = String(await this.contract!.getFunction('resUint64')());
  const started = Date.now();
  const value = await userDecrypt(client, handle, this.contractAddress, this.engine.chain.wallet('alice'));
  this.engine.evidence.record(
    'spike.user-decrypt',
    { handle, value: String(value), decryptMs: Date.now() - started },
    { label: 'alice user-decrypts' },
  );
  assert.equal(BigInt(value as bigint), BigInt(expected));
});

Then('the result is publicly decryptable as {int}', async function (this: FheSpikeWorld, expected: number) {
  const client = await this.fhe();
  const handle = String(await this.contract!.getFunction('resUint64')());
  const started = Date.now();
  const value = await publicDecrypt(client, handle);
  this.engine.evidence.record(
    'spike.public-decrypt',
    { handle, value: String(value), decryptMs: Date.now() - started },
    { label: 'public decryption' },
  );
  assert.equal(BigInt(value as bigint), BigInt(expected));
});

Then('the SDK leaves no open resources once released', async function (this: FheSpikeWorld) {
  const beforeRelease = await settledResources();
  this.client = undefined;
  this.contract = undefined;
  this.engine.chain.destroy();
  await new Promise((resolve) => setTimeout(resolve, 2_000));
  const afterRelease = await settledResources();
  // Anything left above the baseline was opened by the scenario and outlived its release.
  const leftOver = resourcesAdded(this.baseline, afterRelease);
  this.engine.evidence.record(
    'spike.resources',
    { baseline: this.baseline, beforeRelease, afterRelease, leftOver },
    { label: 'resources after release' },
  );
  assert.deepEqual(leftOver, {}, `resources still open after release: ${JSON.stringify(leftOver)}`);
});
