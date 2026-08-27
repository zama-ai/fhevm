// Pins the deploy sequence's nonce layout, and — separately — the localhost addresses it produces.
//
// The two are different claims with different scopes, and conflating them is the trap:
//
//   1. The LAYOUT is universal. `precomputeAddresses` must place each contract at a fixed OFFSET from
//      whatever start nonce it is given, for any deployer. The API exists to serve any signer, so the
//      addresses themselves are not invariant — the offsets are.
//   2. The ADDRESSES are local-only. Applying that layout to the localhost deployer at nonce 0 must
//      reproduce the set `ZamaConfig.sol` compiles into every dApp. A caller
//      passing an explicit `precomputed` set, or targeting a chain other than 31337, has opted out.
//
// Why this file exists at all: `deploy()` derives its addresses from `precomputeAddresses` and then checks
// them with `assertDeployedAddress` — against those same derived values. That guards the deploy order
// against the precompute, but both come from the same source, so the TS path was only ever checked against
// itself. Everything that does reach the ZamaConfig literals goes through `NONCE_OFFSET` in
// internal/constants.ts (a deliberate duplicate of pkg/ts/addresses.ts) or through the forge path — never
// through pkg/ts. A consistent reordering of the TS sequence therefore passed the whole suite while moving
// the stack off the addresses dApps are compiled against. This is the missing edge.
//
// Runs against the installed tarball fixture, so it checks the built package rather than the sources. No
// node required: `precomputeAddresses` reaches the chain only through the injected `ethUtils`.
import { precomputeAddresses, type AbstractEthereumUtils } from '@fhevm/host-contracts-cleartext/ts';
import { getContractAddress, type Address } from 'viem';
import { expect, test } from 'vitest';

////////////////////////////////////////////////////////////////////////////////

/**
 * Nonce offset, relative to the start nonce, at which each address is created.
 *
 * The fourth copy of this ordering (pkg/ts/addresses.ts, internal/constants.ts NONCE_OFFSET, and
 * pkg/forge/script/ComputeAddresses.s.sol are the others) — but the only one that is an assertion rather
 * than an input, which is what makes it a check instead of a fifth thing to keep in step.
 *
 * Offsets 0 and 2 are absent because they carry no named address: they are the two empty-proxy
 * implementations each proxy is constructed over.
 */
const LAYOUT = {
  aclAddress: 1n,
  fhevmExecutorAddress: 3n,
  kmsVerifierAddress: 4n,
  inputVerifierAddress: 5n,
  hcuLimitAddress: 6n,
  protocolConfigAddress: 7n,
  kmsGenerationAddress: 8n,
  cleartextArithmeticAddress: 9n,
  cleartextDbAddress: 10n,
  pauserSetAddress: 11n,
} as const;

/**
 * What `precomputeAddresses` reports as the next free nonce: one past the last address it places.
 *
 * Derived from LAYOUT so it cannot disagree with it. LAYOUT itself stays hand-written on purpose — it is
 * an independent oracle for the deploy order, and deriving it from the code under test would turn this
 * file from a check into a fifth copy of the same table.
 */
const NEXT_START_NONCE_OFFSET = Object.values(LAYOUT).reduce((hi, o) => (o > hi ? o : hi), 0n) + 1n;

/**
 * Deployers the layout must hold for. The first is the localhost one (test 2 below); the others are
 * arbitrary, and are the point — a layout that only worked for one account would be a latent constraint
 * on every other caller.
 */
const DEPLOYERS: readonly Address[] = [
  '0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4',
  '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266',
  '0x0000000000000000000000000000000000000001',
];

/** Start nonces the layout must hold for — including a fresh account (0) and a well-used one. */
const START_NONCES: readonly bigint[] = [0n, 1n, 7n, 12_345n];

////////////////////////////////////////////////////////////////////////////////

/**
 * An `AbstractEthereumUtils` that answers with the nonce it was asked about, encoded as the address.
 *
 * This is what makes the offsets observable: a real derivation returns an opaque hash, so reading an
 * offset back out of it would mean re-deriving with the same function under test. Here the answer *is*
 * the question, so `BigInt(address)` recovers the nonce exactly.
 */
function nonceProbe(): { readonly ethUtils: AbstractEthereumUtils; readonly callers: readonly string[] } {
  const callers: string[] = [];
  const ethUtils: AbstractEthereumUtils = {
    getContractAddress: ({ from, nonce }) => {
      callers.push(from);
      return `0x${nonce.toString(16).padStart(40, '0')}`;
    },
    encodeCall: () => {
      throw new Error('precomputeAddresses must not encode calls');
    },
    // The CREATE2 primitives are not part of the nonce path. Throwing rather than stubbing a plausible
    // value is the point: if `precomputeAddresses` ever reached for one, this test would say so instead of
    // quietly agreeing with whatever the stub returned.
    keccak256: () => {
      throw new Error('precomputeAddresses must not hash');
    },
    encodeAbiParameters: () => {
      throw new Error('precomputeAddresses must not ABI-encode');
    },
    getCreate2Address: () => {
      throw new Error('precomputeAddresses must not derive a CREATE2 address');
    },
  };
  return { ethUtils, callers };
}

/** A real CREATE derivation, for the localhost anchor. */
const viemEthUtils: AbstractEthereumUtils = {
  getContractAddress: ({ from, nonce }) => getContractAddress({ from: from as Address, nonce }),
  encodeCall: () => {
    throw new Error('unused');
  },
  keccak256: () => {
    throw new Error('unused');
  },
  encodeAbiParameters: () => {
    throw new Error('unused');
  },
  getCreate2Address: () => {
    throw new Error('unused');
  },
};

/** The ten placed addresses as one flat record, keyed the same way as {@link LAYOUT}. */
function flatten(result: ReturnType<typeof precomputeAddresses>): Record<keyof typeof LAYOUT, string> {
  return {
    ...result.fhevmAddresses,
    ...result.cleartextAddresses,
    pauserSetAddress: result.pauserSetAddress,
  };
}

const layoutKeys = Object.keys(LAYOUT) as ReadonlyArray<keyof typeof LAYOUT>;

////////////////////////////////////////////////////////////////////////////////

test('the nonce layout is fixed, for any deployer and any start nonce', () => {
  for (const from of DEPLOYERS) {
    for (const startNonce of START_NONCES) {
      const { ethUtils, callers } = nonceProbe();
      const result = precomputeAddresses({ ethUtils, from, startNonce });
      const placed = flatten(result);
      const where = `${from} @ ${startNonce}`;

      // A new address in the API must gain a LAYOUT entry rather than go unchecked.
      expect(Object.keys(placed).sort(), `address set changed (${where})`).toEqual([...layoutKeys].sort());

      for (const name of layoutKeys) {
        const offset = BigInt(placed[name]) - startNonce;
        expect(offset, `${name} offset (${where})`).toBe(LAYOUT[name]);
      }

      expect(result.nextStartNonce - startNonce, `nextStartNonce (${where})`).toBe(NEXT_START_NONCE_OFFSET);

      // Every address must derive from the deployer that was passed in, not from anything ambient.
      expect(new Set(callers), `deployer used (${where})`).toEqual(new Set([from]));
    }
  }
});

////////////////////////////////////////////////////////////////////////////////

/**
 * The localhost deployer: `MNEMONIC` at account index 5, as declared by DEPLOYER_ADDRESS in
 * pkg/forge/src/_internal/LocalHostAddresses.sol. Written out rather than derived from the mnemonic —
 * the deploy mnemonic and the SIGNER mnemonic (`FHEVM_MNEMONIC`) are different things with different
 * jobs, and a spec that carried one of them invites the swap.
 */
const LOCALHOST_DEPLOYER: Address = '0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4';

/** The set a default local `deploy()` must produce — LocalHostAddresses.sol, and nothing else. */
const LOCALHOST_ADDRESSES: Readonly<Record<keyof typeof LAYOUT, string>> = {
  aclAddress: '0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D',
  fhevmExecutorAddress: '0xe3a9105a3a932253A70F126eb1E3b589C643dD24',
  kmsVerifierAddress: '0x901F8942346f7AB3a01F6D7613119Bca447Bb030',
  inputVerifierAddress: '0x36772142b74871f255CbD7A3e89B401d3e45825f',
  hcuLimitAddress: '0x233ff88A48c172d29F675403e6A8e302b0F032D9',
  protocolConfigAddress: '0x44aA028fd264C76BF4A8f8B4d8A5272f6AE25CAc',
  kmsGenerationAddress: '0x216be43148dB537BeddBC268163deb1a802b5553',
  cleartextArithmeticAddress: '0xded0D2a71268DC12622BdD1b55d68a1CB5662327',
  cleartextDbAddress: '0x6933Afcf0F4bCE1A611baD0A6FaafF0337a7ba1E',
  pauserSetAddress: '0x590e3330386Fa042843773541aaBb3a45EC3164D',
};

/** The three `ZamaConfig._getLocalConfig()` returns — the ones consumers are bound to. */
const ZAMA_CONFIG_ANCHORS: ReadonlyArray<keyof typeof LAYOUT> = [
  'aclAddress',
  'fhevmExecutorAddress',
  'kmsVerifierAddress',
];

test('the default localhost deploy lands on the ZamaConfig addresses', () => {
  // LOCAL MODE ONLY: the localhost deployer, starting from a nonce of 0. Any other deployer or start
  // nonce produces a different set by design — that is the previous test's subject, not this one's.
  const placed = flatten(precomputeAddresses({ ethUtils: viemEthUtils, from: LOCALHOST_DEPLOYER, startNonce: 0n }));

  for (const name of layoutKeys) {
    expect(placed[name].toLowerCase(), name).toBe(LOCALHOST_ADDRESSES[name].toLowerCase());
  }

  // Stated again on their own, because these three are not ours to choose: they are compiled into every
  // dApp inheriting ZamaConfig's localhost config, so a default deploy landing elsewhere leaves all of
  // them calling addresses that hold no code.
  for (const name of ZAMA_CONFIG_ANCHORS) {
    expect(placed[name].toLowerCase(), `ZamaConfig anchor ${name}`).toBe(LOCALHOST_ADDRESSES[name].toLowerCase());
  }
});
