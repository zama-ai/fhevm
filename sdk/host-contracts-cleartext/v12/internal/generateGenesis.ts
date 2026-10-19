// Generates pkg/state/genesis.json — a ready-to-load snapshot of the canonical local cleartext stack.
//
// The output lets a consumer skip the deploy entirely:
//
//   forge test    vm.loadAllocs("<pkg>/state/genesis.json")   — no anvil at all
//   live node     anvil --init <pkg>/state/genesis.json
//
// One file serves both. `vm.loadAllocs` parses a bare allocs map first and falls back to a full genesis,
// taking `.alloc` (foundry: crates/cheatcodes/src/evm.rs), and `anvil --init` reads a genesis directly.
//
// How it is built: start a throwaway anvil, run the real deploy through scripts/deploy.sh — which verifies
// The result — then snapshot only the accounts that matter. So the artifact is only ever produced from a
// deploy that passed verification.
//
// ## The account set is chosen by inclusion, not by filtering
//
// The deploy creates 22 contracts, and a raw `anvil_dumpState` also hands back accounts that must not
// ship. Rather than strip those out by heuristic, this script names what it wants:
//
//   10  the addresses in the generated fhevm-config addresses.sol (9 proxies + PauserSet)
//    1  the ACLOwner, found via ACL.owner()
//    9  the real implementations, read out of each proxy's ERC-1967 implementation slot
//    1  the deployer EOA — for its nonce; see below
//   ---
//   21  entries
//
// Deliberately absent:
//
//   - `EmptyUUPSProxyACL` and `EmptyUUPSProxy` (deployer nonces 0 and 2). Once `ACLOwner.upgrade` has run
//     every proxy points at a real implementation, so these two are unreachable. Verified rather than
//     assumed: the script requires all nine implementation slots to resolve to accounts that hold code,
//     and those nine are what it ships.
//   - anvil's predeployed CREATE2 factory at 0x4e59b4...4956c, which already has code on a *fresh* anvil.
//     anvil provides it itself.
//   - the other funded dev EOAs. `anvil --init` overlays the alloc on top of the accounts it derives from
//     The mnemonic, so omitting them leaves their normal balances rather than pinning them to whatever
//     this particular run happened to spend.
//
// ## Two details that will bite if changed
//
// `nonce` is optional in a genesis account but applied *unconditionally* — both loaders do
// `nonce.unwrap_or_default()` — so an entry without one has its nonce reset to 0. The deployer must carry
// its post-deploy nonce or the next CREATE from it collides with the stack.
//
// `balance` is required (alloy-genesis gives it no serde default), so every entry states one. The
// deployer's is normalized to anvil's default rather than copied: the real post-deploy balance depends on
// gas paid, which depends on the EIP-1559 basefee, which moves with wall-clock block timing. Excluding
// that — and all block metadata — is what makes this file byte-identical between runs, and therefore what
// makes its SHA a check on the contents rather than merely on the download.

import { execFileSync, spawn } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { gunzipSync } from 'node:zlib';
import { getAddress } from 'viem';
import { mnemonicToAccount } from 'viem/accounts';
import {
  ADDRESS_NAMES,
  DEPLOYER_ADDRESS_INDEX,
  MNEMONIC,
  NONCE_LABEL,
  PACKAGE_ROOT_ABS_PATH,
  PKG_DIR_ABS_PATH,
  ERC_1967_IMPL_SLOT,
} from './constants.ts';
import { writeJson } from './utils.ts';
import { rpc, waitForNode } from './utils.ts';

////////////////////////////////////////////////////////////////////////////////

/** Port for the throwaway node. Deliberately not 8545, so a developer's own anvil is never touched. */
const ANVIL_PORT = 8945;

/** `owner()` — read to find the standing ACLOwner, whose address is not derivable from the config. */
const OWNER_SELECTOR = '0x8da5cb5b';

/** anvil's default per-account genesis balance (10000 ETH). See the note on normalization above. */
const DEPLOYER_BALANCE = '0x21e19e0c9bab2400000';

const CONFIG_DIR_ABS_PATH = join(PACKAGE_ROOT_ABS_PATH, 'internal', '.deploy-config');
const STATE_DIR_ABS_PATH = join(PKG_DIR_ABS_PATH, 'state');
const GENESIS_OUTPUT_PATH = join(STATE_DIR_ABS_PATH, 'genesis.json');
const SHA_OUTPUT_PATH = `${GENESIS_OUTPUT_PATH}.sha256`;

/** One account as `anvil_dumpState` reports it. */
type DumpedAccount = {
  readonly balance: string;
  readonly nonce: number;
  readonly code?: string;
  readonly storage?: Record<string, string>;
};

/** One account as a genesis `alloc` entry. `balance` is required by alloy-genesis; `nonce` defaults to 0. */
type AllocEntry = {
  readonly balance: string;
  readonly nonce: string;
  readonly code?: string;
  readonly storage?: Record<string, string>;
};

/** What the generator produced, for the caller to report. */
type GenesisSummary = {
  readonly path: string;
  readonly accountCount: number;
  readonly codeBytes: number;
  readonly deployer: string;
  readonly deployerNonce: number;
  readonly bytes: number;
  readonly sha256: string;
};

////////////////////////////////////////////////////////////////////////////////

/**
 * The address set the build was compiled against, read back from the file step 1 of the deploy wrote.
 *
 * Read rather than recomputed on purpose: this is the file the contracts' own bytecode was compiled
 * against, so it is the only definition of "where the stack is" that cannot disagree with reality.
 */
function _namedAddresses(): ReadonlyMap<string, string> {
  const source = readFileSync(join(CONFIG_DIR_ABS_PATH, 'addresses.sol'), 'utf8');
  const found = new Map<string, string>();
  for (const match of source.matchAll(/^address constant (\w+) = address\((0x[0-9a-fA-F]{40})\);/gm)) {
    const [, name, address] = match;
    if (name !== undefined && address !== undefined) {
      found.set(name, address);
    }
  }
  if (found.size !== ADDRESS_NAMES.length) {
    throw new Error(`expected ${String(ADDRESS_NAMES.length)} named addresses, found ${String(found.size)}`);
  }
  return found;
}

////////////////////////////////////////////////////////////////////////////////

function _lower(address: string): string {
  return address.toLowerCase();
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Storage slots sorted, so the emitted file is byte-stable between runs.
 */
function _sortedStorage(storage: Record<string, string> | undefined): Record<string, string> | undefined {
  if (storage === undefined || Object.keys(storage).length === 0) {
    return undefined;
  }
  return Object.fromEntries(Object.entries(storage).sort(([a], [b]) => a.localeCompare(b)));
}

////////////////////////////////////////////////////////////////////////////////

function _toAllocEntry(account: DumpedAccount, balance: string): AllocEntry {
  const storage = _sortedStorage(account.storage);
  return {
    balance,
    nonce: `0x${BigInt(account.nonce).toString(16)}`,
    ...(account.code !== undefined && account.code !== '0x' ? { code: account.code } : {}),
    ...(storage !== undefined ? { storage } : {}),
  };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Runs a real deploy on a throwaway anvil and snapshots it to pkg/state/genesis.json.
 *
 * Progress is logged as it goes, because the deploy it shells out to is slow and prints its own
 * output; the closing summary is returned instead, for the caller to render.
 */
export async function writeGenesis(): Promise<GenesisSummary> {
  const deployer = mnemonicToAccount(MNEMONIC, { addressIndex: DEPLOYER_ADDRESS_INDEX }).address;
  const url = `http://127.0.0.1:${String(ANVIL_PORT)}`;

  const anvil = spawn(
    'anvil',
    [
      '--host',
      '127.0.0.1',
      '--port',
      String(ANVIL_PORT),
      '--mnemonic',
      MNEMONIC,
      '--derivation-path',
      "m/44'/60'/0'/0/",
    ],
    { stdio: 'ignore' },
  );
  anvil.unref();

  try {
    await waitForNode(url);
    const chainId = Number(await rpc<string>(url, 'eth_chainId'));

    // The real deploy, verification included. Reused rather than reimplemented: the artifact must be a
    // snapshot of what deploy.sh produces, and a second deploy path here could drift from it silently.
    console.log(`  deploying to ${url} (deployer ${deployer})`);
    execFileSync(
      './scripts/deploy.sh',
      ['--rpc-url', url, '--mnemonic', MNEMONIC, '--account-index', String(DEPLOYER_ADDRESS_INDEX)],
      { cwd: PACKAGE_ROOT_ABS_PATH, stdio: 'inherit' },
    );

    const named = _namedAddresses();
    const aclAddress = named.get('ACL_ADDRESS');
    if (aclAddress === undefined) {
      throw new Error('ACL_ADDRESS missing from the generated config');
    }

    const ownerWord = await rpc<string>(url, 'eth_call', [{ to: aclAddress, data: OWNER_SELECTOR }, 'latest']);
    const aclOwner = getAddress(`0x${ownerWord.slice(-40)}`);

    const dumpHex = await rpc<string>(url, 'anvil_dumpState');
    const state = JSON.parse(gunzipSync(Buffer.from(dumpHex.replace(/^0x/, ''), 'hex')).toString('utf8')) as {
      readonly accounts: Record<string, DumpedAccount>;
    };

    const accounts = new Map<string, DumpedAccount>(
      Object.entries(state.accounts).map(([address, account]) => [_lower(address), account]),
    );
    // eslint-disable-next-line @typescript-eslint/naming-convention
    const require_ = (address: string, what: string): DumpedAccount => {
      const account = accounts.get(_lower(address));
      if (account === undefined) {
        throw new Error(`${what} ${address} is absent from the state dump`);
      }
      if (account.code === undefined || account.code === '0x') {
        throw new Error(`${what} ${address} holds no code`);
      }
      return account;
    };

    // The nine implementations, from each proxy's ERC-1967 slot rather than from a nonce offset: under
    // --broadcast the interleaved calls consume nonces too, so the implementations do not sit at a fixed
    // distance from the start nonce (they land at +16..+24, not +13..+21).
    const entries: Array<{ name: string; address: string }> = [];
    for (const [name, address] of named) {
      entries.push({ name, address });
      const account = require_(address, name);
      const slot = Object.entries(account.storage ?? {}).find(([key]) => _lower(key) === ERC_1967_IMPL_SLOT);
      if (slot === undefined) {
        continue; // PauserSet is not a proxy.
      }
      const implementation = getAddress(`0x${slot[1].slice(-40)}`);
      require_(implementation, `${name} implementation`);
      entries.push({ name: `${name} implementation`, address: implementation });
    }

    // One implementation per proxy. Derived from NONCE_LABEL rather than written down: this check used
    // to read `!== 9`, which is the kind of literal a protocol generation silently invalidates.
    const expectedImplementations = ADDRESS_NAMES.filter((name) => NONCE_LABEL[name].startsWith('ERC1967Proxy')).length;
    const implementationCount = entries.filter((e) => e.name.endsWith(' implementation')).length;
    if (implementationCount !== expectedImplementations) {
      throw new Error(
        `expected ${String(expectedImplementations)} implementations behind the proxies, ` +
          `found ${String(implementationCount)}`,
      );
    }

    entries.push({ name: 'ACLOwner', address: aclOwner });
    require_(aclOwner, 'ACLOwner');

    const alloc = new Map<string, AllocEntry>();
    for (const { name, address } of entries) {
      const key = _lower(address);
      if (alloc.has(key)) {
        throw new Error(`${name} ${address} appears twice in the inclusion list`);
      }
      // Contracts hold no ether; a literal keeps the file independent of gas prices.
      alloc.set(key, _toAllocEntry(require_(address, name), '0x0'));
    }

    // The deployer EOA: its nonce is load-bearing, its balance is normalized, and it has no code.
    const deployerAccount = accounts.get(_lower(deployer));
    if (deployerAccount === undefined) {
      throw new Error(`deployer ${deployer} is absent from the state dump`);
    }
    alloc.set(_lower(deployer), {
      balance: DEPLOYER_BALANCE,
      nonce: `0x${BigInt(deployerAccount.nonce).toString(16)}`,
    });

    const genesis = {
      config: { chainId },
      // Values anvil is content to start from; the stack itself lives entirely in `alloc`.
      difficulty: '0x0',
      gasLimit: '0x1c9c380',
      alloc: Object.fromEntries([...alloc].sort(([a], [b]) => a.localeCompare(b))),
    };

    mkdirSync(STATE_DIR_ABS_PATH, { recursive: true });
    // Hashing what writeJson returned, not a re-serialization: the digest has to be over the bytes that
    // actually landed on disk, or the .sha256 checks nothing.
    const json = writeJson(GENESIS_OUTPUT_PATH, genesis);

    const sha = createHash('sha256').update(json).digest('hex');
    writeFileSync(SHA_OUTPUT_PATH, `${sha}  genesis.json\n`, 'utf8');

    const codeBytes = [...alloc.values()].reduce(
      (total, entry) => total + (entry.code === undefined ? 0 : (entry.code.length - 2) / 2),
      0,
    );

    return {
      path: GENESIS_OUTPUT_PATH,
      accountCount: alloc.size,
      codeBytes,
      deployer,
      deployerNonce: deployerAccount.nonce,
      bytes: json.length,
      sha256: sha,
    };
  } finally {
    anvil.kill();
    rmSync(join(CONFIG_DIR_ABS_PATH, 'dumpstate.hex'), { force: true });
  }
}
