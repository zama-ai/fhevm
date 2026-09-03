// D4a1: the error table is well-formed data and the template engine fills it the way v2 did.
//
// Tests import the BUILT payload (pkg/_esm); see connection.test.ts.

import assert from 'node:assert/strict';
import test from 'node:test';

import { HardhatPluginError } from 'hardhat/plugins';

import {
  CUSTOM_ERROR_MESSAGE,
  FHEVM_ERROR_TABLE,
  applyErrorTemplate,
  lookupErrorTemplates,
} from '../pkg/_esm/internal/errors/errorTable.js';

const TAG = /%([a-zA-Z][a-zA-Z0-9]*)?%/g;

void test('every template tag in the table is a well-formed, non-empty variable name', () => {
  const texts = [CUSTOM_ERROR_MESSAGE];
  for (const errors of Object.values(FHEVM_ERROR_TABLE)) {
    for (const templates of Object.values(errors)) texts.push(...Object.values(templates));
  }
  assert.ok(texts.length > 4, 'the table carries entries');
  for (const text of texts) {
    for (const match of text.matchAll(TAG)) assert.notEqual(match[1], undefined, `empty tag in: ${text}`);
  }
});

void test('lookupErrorTemplates finds known entries and nothing else', () => {
  assert.equal(lookupErrorTemplates('FHEVMExecutor', 'ACLNotAllowed')?.title?.includes('ACLNotAllowed'), true);
  assert.equal(lookupErrorTemplates('InputVerifier', 'InvalidSigner')?.longMessage?.includes('%txUserAddress%'), true);
  assert.equal(lookupErrorTemplates('ACL', 'NoSuchError'), undefined);
  assert.equal(lookupErrorTemplates('NoSuchContract', 'SenderNotAllowed'), undefined);
  // Inherited object keys are not table entries.
  assert.equal(lookupErrorTemplates('toString', 'constructor'), undefined);
});

void test('applyErrorTemplate fills every tag, stringifying values as v2 did', () => {
  assert.equal(
    applyErrorTemplate(CUSTOM_ERROR_MESSAGE, { customError: 'ACLNotAllowed()' }),
    "VM Exception while processing transaction: reverted with custom error 'ACLNotAllowed()'",
  );
  assert.equal(applyErrorTemplate('%a% and %a% and %b%', { a: 1n, b: { x: 2n } }), '1 and 1 and {"x":"2"}');
  assert.equal(applyErrorTemplate('%a%|%b%|%c%', { a: undefined, b: null, c: true }), 'undefined|null|true');
  assert.equal(applyErrorTemplate('plain text'), 'plain text');
});

void test('applyErrorTemplate refuses a tagged template without values, bad names and tagged values', () => {
  const isPluginError = (e: unknown): boolean => e instanceof HardhatPluginError;
  assert.throws(() => applyErrorTemplate('needs %x%'), isPluginError);
  assert.throws(() => applyErrorTemplate('%x%', { '1bad': 'v' }), isPluginError);
  assert.throws(() => applyErrorTemplate('%x%', { x: 'has %tag% inside' }), isPluginError);
});
