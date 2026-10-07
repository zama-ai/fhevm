import { strict as assert } from 'node:assert';
import { test } from 'vitest';
import { constantNodeFromAnchorV01 } from '@codama/nodes-from-anchor';
import { renderProgramConstants } from './render-solana-constants.mjs';

const render = (constants) =>
  renderProgramConstants(
    constants.map((c) => constantNodeFromAnchorV01(c)),
    constants,
    [],
  );

test('renders byte labels and integer widths without losing wide integer precision', () => {
  const result = render([
    { name: 'LABEL', type: { array: ['u8', 3] }, value: '[1, 2, 255]' },
    { name: 'SMALL', type: 'u32', value: '4294967295' },
    { name: 'SIGNED', type: 'i16', value: '-2' },
    { name: 'WIDE', type: 'u64', value: '18446744073709551615' },
    { name: 'WIDER', type: 'u128', value: '340282366920938463463374607431768211455' },
  ]);
  assert.match(result, /LABEL: Uint8Array = new Uint8Array\(\[1,2,255\]\)/);
  assert.match(result, /SMALL: number = 4294967295/);
  assert.match(result, /SIGNED: number = -2/);
  assert.match(result, /WIDE: bigint = 18446744073709551615n/);
  assert.match(result, /WIDER: bigint = 340282366920938463463374607431768211455n/);
});

test('renders one discriminator per event and refuses a malformed one', () => {
  const events = [{ name: 'FheExecutedEvent', discriminator: [234, 26, 200, 201, 187, 114, 93, 208] }];
  assert.match(
    renderProgramConstants([], [], events),
    /FHE_EXECUTED_EVENT_DISCRIMINATOR: Uint8Array = new Uint8Array\(\[234,26,200,201,187,114,93,208\]\)/,
  );
  assert.throws(
    () => renderProgramConstants([], [], [{ name: 'Short', discriminator: [1, 2] }]),
    /Invalid event discriminator/,
  );
});

test('renders all constants and refuses malformed byte values', () => {
  assert.match(render([{ name: 'FUTURE', type: 'bool', value: 'true' }]), /FUTURE: boolean = true/);
  assert.throws(() => render([{ name: 'BAD', type: { array: ['u8', 1] }, value: '[256]' }]), /Invalid byte constant/);
  assert.throws(() => renderProgramConstants([], [{ name: 'MISSING' }], []), /constant nodes are missing/);
});
