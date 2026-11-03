import type { ClearValue } from '#engine';
import { type DataTable, Given, Then, setWorldConstructor } from '@cucumber/cucumber';
import assert from 'node:assert/strict';

import { FheFixtureWorld } from '../shared.js';

setWorldConstructor(FheFixtureWorld);

/** The expected value of a table cell, in the form `userDecrypt` returns it. */
function expectedValue(text: string): ClearValue {
  if (text === 'true' || text === 'false') return text === 'true';
  if (text.startsWith('0x')) return text.toLowerCase();
  return BigInt(text);
}

Given('alice has deployed the UserDecrypt contract', async function (this: FheFixtureWorld) {
  await this.deploy('UserDecrypt', 'alice');
});

Then('alice user-decrypts every value in one request', async function (this: FheFixtureWorld, table: DataTable) {
  const rows = table.hashes() as { getter: string; expected: string }[];
  const handles: string[] = [];
  for (const row of rows) handles.push(await this.handle(row.getter));
  const values = await this.engine.fhe.userDecrypt({ handles, contract: this.deployed, user: 'alice' });
  const actual = values.map((value) => (typeof value === 'string' ? value.toLowerCase() : value));
  assert.deepEqual(
    actual,
    rows.map((row) => expectedValue(row.expected)),
  );
});
