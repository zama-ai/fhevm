// Every `startAnvil({ port })` in the vitest suite must claim a distinct port.
//
// Not a style rule. `startAnvil` spawns anvil and returns immediately; when the port is already bound
// anvil exits, and `waitForAnvil` then succeeds against WHATEVER ELSE is listening. The test proceeds
// against a foreign node, and the failure surfaces far from its cause — a collision between two files
// using the same port presented as `Out of gas: gas required exceeds allowance: 0`, because the squatting
// node was funded from a different mnemonic and so the deployer had no balance.
//
// Two files can also collide across GENERATIONS if both suites run at once, which is why v12 and v13 pick
// different ports for the same test rather than sharing one.
//
// Checked by reading the sources rather than by importing them: these tests take minutes and start real
// nodes, so the cheap lane is the right place to catch a numbering mistake.

import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import test from 'node:test';
import { PACKAGE_ROOT_ABS_PATH } from '../internal/constants.ts';

const TEST_TS_DIR = join(PACKAGE_ROOT_ABS_PATH, 'test', 'ts');

/** Every `startAnvil({ port: N })` in `test/ts`, as (port, "file:line"). */
function anvilPorts(): ReadonlyArray<readonly [number, string]> {
  const out: Array<readonly [number, string]> = [];
  for (const name of readdirSync(TEST_TS_DIR)) {
    if (!name.endsWith('.ts')) continue;
    const lines = readFileSync(join(TEST_TS_DIR, name), 'utf8').split('\n');
    lines.forEach((line, i) => {
      const m = /startAnvil\(\{\s*port:\s*(\d+)/.exec(line);
      if (m?.[1] !== undefined) out.push([Number(m[1]), `${name}:${i + 1}`]);
    });
  }
  return out;
}

void test('every startAnvil port in test/ts is unique', () => {
  const ports = anvilPorts();
  // A parser that matched nothing would make the assertion below trivially true, and this suite is known
  // to start well over a dozen nodes.
  assert.ok(ports.length >= 10, `parsed only ${ports.length} startAnvil calls — the parser is broken`);

  const byPort = new Map<number, string[]>();
  for (const [port, where] of ports) {
    byPort.set(port, [...(byPort.get(port) ?? []), where]);
  }
  const collisions = [...byPort]
    .filter(([, wheres]) => wheres.length > 1)
    .map(([port, wheres]) => `${port}: ${wheres.join(', ')}`);

  assert.deepEqual(
    collisions,
    [],
    'these tests would race for the same anvil port. Whichever starts second fails to bind, silently\n' +
      'connects to the first one, and then fails somewhere unrelated.',
  );
});
