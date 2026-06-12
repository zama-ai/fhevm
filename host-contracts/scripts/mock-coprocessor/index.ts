/**
 * CLI entry point for the mock coprocessor.
 *
 * Subcommands:
 *
 *   daemon            Long-running poller (default). Use `Ctrl-C` to stop.
 *   query <handle>    Look up the decoded plaintext for a single handle and exit.
 *                     Exits 0 with the decimal value on stdout, or 1 with an
 *                     error if the handle hasn't been seen yet.
 *   encrypt           Build a single-input encrypted bundle (handle + inputProof)
 *                     and persist its cleartext into the mock DB. Used by
 *                     operators to feed `FHE.fromExternal(...)` contract calls
 *                     on live testnets (e.g. `ConfidentialOFT.mint`). Required
 *                     flags: --contract, --user, --type, --value, --host-chain-id.
 *
 * Invoked via the npm scripts in package.json (`mock:daemon`, `mock:query`,
 * `mock:encrypt`).
 */
import * as dotenv from 'dotenv';
import { ethers } from 'ethers';
import * as path from 'path';

import { RUNTIME } from './config';
import { MockDb } from './db';
import { CliOptions, runMockEncryptCli } from './input';
import { runService } from './service';

dotenv.config({ path: path.resolve(__dirname, '..', '..', '.env') });

async function queryHandle(handleRaw: string): Promise {
  let handle: string;
  try {
    handle = ethers.toBeHex(handleRaw, 32);
  } catch (err) {
    console.error(`Invalid handle: ${handleRaw}`);
    process.exit(1);
  }
  const db = await MockDb.open(RUNTIME.dbPath);
  const value = await db.getClearText(handle);
  await db.close();
  if (value === null) {
    console.error(
      `Handle ${handle} not present in mock DB. The daemon may still be catching up, or the handle was never produced.`
    );
    process.exit(1);
  }
  console.log(value.toString());
}

async function encryptHandle(args: string[]): Promise {
  const flags = parseFlags(args);
  const required = ['contract', 'user', 'type', 'value', 'host-chain-id'];
  const missing = required.filter((k) => flags[k] === undefined);
  if (missing.length > 0) {
    console.error(`Missing required flag(s): ${missing.map((m) => '--' + m).join(', ')}`);
    console.error(
      'Usage: encrypt --contract <addr> --user <addr> --type <euint64|...> --value <num> --host-chain-id <chainId>'
    );
    process.exit(2);
  }
  const hostChainId = Number(flags['host-chain-id']);
  if (!Number.isInteger(hostChainId) || hostChainId <= 0) {
    console.error(`--host-chain-id must be a positive integer (got ${flags['host-chain-id']})`);
    process.exit(2);
  }
  const opts: CliOptions = {
    contract: flags['contract'],
    user: flags['user'],
    type: flags['type'],
    value: flags['value'],
    hostChainId,
  };
  const db = await MockDb.open(RUNTIME.dbPath);
  try {
    await runMockEncryptCli(opts, db);
  } finally {
    await db.close();
  }
}

function parseFlags(argv: string[]): Record {
  const out: Record = {};
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (!arg.startsWith('--')) continue;
    const key = arg.slice(2);
    const next = argv[i + 1];
    if (next === undefined || next.startsWith('--')) {
      out[key] = 'true';
      continue;
    }
    out[key] = next;
    i++;
  }
  return out;
}

async function main(): Promise {
  const [cmd, ...rest] = process.argv.slice(2);
  switch (cmd) {
    case undefined:
    case 'daemon':
      await runService();
      return;
    case 'query':
      if (rest.length !== 1) {
        console.error('Usage: query <handle>');
        process.exit(2);
      }
      await queryHandle(rest[0]);
      return;
    case 'encrypt':
      await encryptHandle(rest);
      return;
    case '--help':
    case '-h':
    case 'help':
      console.log(
        'Usage: ts-node scripts/mock-coprocessor/index.ts <subcommand>\n' +
          '  daemon                                       Run the long-running poller (default).\n' +
          '  query <handle>                               Print cleartext for <handle> and exit.\n' +
          '  encrypt --contract <addr> --user <addr>      Build a handle+inputProof bundle for a single\n' +
          '          --type <euint64|...> --value <num>   FHE.fromExternal-style input and persist its\n' +
          '          --host-chain-id <chainId>            cleartext into the mock DB.'
      );
      return;
    default:
      console.error(`Unknown command: ${cmd}`);
      process.exit(2);
  }
}

main().catch((err) => {
  console.error('[mock-coprocessor] fatal:', err);
  process.exit(1);
});
