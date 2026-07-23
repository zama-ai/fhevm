import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import test from 'node:test';
import {
  ADDRESS_NAMES,
  PACKAGE_ROOT,
  TARGET_CONTRACTS,
  artifactPathFor,
  normalizeHex,
  readJson,
  type Artifact,
  type HexString,
} from '../internal/generateTemplates.ts';

type AddressName = (typeof ADDRESS_NAMES)[number];

type Template = {
  contractName: string;
  sourcePath: string;
  artifactPath: string;
  bytecode: HexString;
  deployedBytecode: HexString;
  addressReferences: Partial<
    Record<
      AddressName,
      {
        placeholder: HexString;
        bytecodeOffsets: number[];
        deployedBytecodeOffsets: number[];
      }
    >
  >;
};

type TemplateAddressReference = NonNullable<Template['addressReferences'][AddressName]>;

const CONFIG_PATH = `${PACKAGE_ROOT}/config/addresses.sol`;

const ALTERNATE_ADDRESSES = {
  ACL_ADDRESS: '0x7011121314151617181920212223242526272829',
  FHEVM_EXECUTOR_ADDRESS: '0x8021222324252627282930313233343536373839',
  KMS_VERIFIER_ADDRESS: '0x9031323334353637383940414243444546474849',
  INPUT_VERIFIER_ADDRESS: '0x7141424344454647484950515253545556575859',
  HCU_LIMIT_ADDRESS: '0x8151525354555657585960616263646566676869',
  PROTOCOL_CONFIG_ADDRESS: '0x7211121314151617181920212223242526272829',
  KMS_GENERATION_ADDRESS: '0x8221222324252627282930313233343536373839',
  CLEARTEXT_ARITHMETIC_ADDRESS: '0x7311223344556677889900112233445566778899',
  CLEARTEXT_DB_ADDRESS: '0x8311223344556677889900112233445566778899',
  PAUSER_SET_ADDRESS: '0x9161626364656667686970717273747576777879',
  CONFIDENTIAL_BRIDGE_ADDRESS: '0x9261626364656667686970717273747576777879',
} satisfies Record<AddressName, HexString>;

function lowerHex(value: HexString): HexString {
  return `0x${normalizeHex(value, 'hex value')}`;
}

function readTemplate(contractName: string): Template {
  return readJson<Template>(`${PACKAGE_ROOT}/templates/${contractName}.json`);
}

function readArtifact(target: (typeof TARGET_CONTRACTS)[number]): Artifact {
  return readJson<Artifact>(artifactPathFor(target));
}

function addressesFromTemplate(template: Template): Record<AddressName, HexString> {
  const addresses = {} as Record<AddressName, HexString>;

  for (const name of ADDRESS_NAMES) {
    addresses[name] = getAddressReference(template, name).placeholder;
  }

  return addresses;
}

function getAddressReference(template: Template, name: AddressName): TemplateAddressReference {
  const reference = template.addressReferences[name];
  assert.ok(reference, `${template.contractName} template is missing ${name}`);
  return reference;
}

function patchBytecode(
  template: Template,
  field: 'bytecode' | 'deployedBytecode',
  addresses: Record<AddressName, HexString>,
): HexString {
  const offsetField = field === 'bytecode' ? 'bytecodeOffsets' : 'deployedBytecodeOffsets';
  let hex = normalizeHex(template[field], `${template.contractName}.${field}`);

  for (const name of ADDRESS_NAMES) {
    const reference = getAddressReference(template, name);
    const placeholder = normalizeHex(reference.placeholder, `${template.contractName}.${name}.placeholder`);
    const replacement = normalizeHex(addresses[name], `${name} replacement`);

    assert.equal(replacement.length, placeholder.length, `${name} replacement must be address-sized`);

    for (const byteOffset of reference[offsetField]) {
      const hexOffset = byteOffset * 2;
      assert.equal(
        hex.slice(hexOffset, hexOffset + placeholder.length),
        placeholder,
        `${template.contractName}.${field} ${name} offset ${byteOffset} does not point to the placeholder`,
      );

      hex = `${hex.slice(0, hexOffset)}${replacement}${hex.slice(hexOffset + placeholder.length)}`;
    }
  }

  return `0x${hex}`;
}

function addressConfigSource(addresses: Record<AddressName, HexString>): string {
  return `// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

address constant ACL_ADDRESS = address(${addresses.ACL_ADDRESS});
address constant FHEVM_EXECUTOR_ADDRESS = address(${addresses.FHEVM_EXECUTOR_ADDRESS});
address constant KMS_VERIFIER_ADDRESS = address(${addresses.KMS_VERIFIER_ADDRESS});
address constant INPUT_VERIFIER_ADDRESS = address(${addresses.INPUT_VERIFIER_ADDRESS});
address constant HCU_LIMIT_ADDRESS = address(${addresses.HCU_LIMIT_ADDRESS});
address constant PROTOCOL_CONFIG_ADDRESS = address(${addresses.PROTOCOL_CONFIG_ADDRESS});
address constant KMS_GENERATION_ADDRESS = address(${addresses.KMS_GENERATION_ADDRESS});
address constant CONFIDENTIAL_BRIDGE_ADDRESS = address(${addresses.CONFIDENTIAL_BRIDGE_ADDRESS});
address constant CLEARTEXT_ARITHMETIC_ADDRESS = address(${addresses.CLEARTEXT_ARITHMETIC_ADDRESS});
address constant CLEARTEXT_DB_ADDRESS = address(${addresses.CLEARTEXT_DB_ADDRESS});
address constant PAUSER_SET_ADDRESS = address(${addresses.PAUSER_SET_ADDRESS});
`;
}

function run(command: string, args: string[]): void {
  try {
    execFileSync(command, args, { cwd: PACKAGE_ROOT, encoding: 'utf8', stdio: 'pipe' });
  } catch (error) {
    const failure = error as { stdout?: string; stderr?: string };
    throw new Error(`${command} ${args.join(' ')} failed\n${failure.stdout ?? ''}${failure.stderr ?? ''}`, {
      cause: error,
    });
  }
}

function forge(args: string[]): void {
  run('forge', args);
}

function restoreConfigAndGeneratedArtifacts(originalConfig: string): void {
  writeFileSync(CONFIG_PATH, originalConfig);
  forge(['clean']);
  forge(['build']);
  run(process.execPath, ['internal/generateTemplates.ts']);
}

void test('patching templates with their original addresses is an identity operation', () => {
  for (const target of TARGET_CONTRACTS) {
    const template = readTemplate(target.contractName);
    const originalAddresses = addressesFromTemplate(template);

    assert.equal(patchBytecode(template, 'bytecode', originalAddresses), lowerHex(template.bytecode));
    assert.equal(patchBytecode(template, 'deployedBytecode', originalAddresses), lowerHex(template.deployedBytecode));
  }
});

/**
 * Contracts whose bytecode solc lays out DIFFERENTLY depending on the address values, so that patching a
 * template in place is not byte-identical to recompiling with those addresses.
 *
 * Why this happens: when an address constant is referenced only once, solc's constant optimiser does not
 * inline it as a PUSH20 — it pools the 32-byte word into the code's trailing data section and reaches it
 * with `PUSH2 <offset>; CODECOPY`. The ORDER of that pool depends on the constant's VALUE, so changing an
 * address can permute the pool and rewrite the PUSH2 offsets that reference it. `HCULimit` is the only
 * contract that trips this: it references `FHEVM_EXECUTOR_ADDRESS` exactly once. (Verified: holding the
 * executor address fixed while changing every other address preserves the pool order.)
 *
 * The patched bytecode is still CORRECT — its PUSH2 offsets point at the pool slots we patched, so it
 * reads the right address; the fresh build simply chose the opposite order. That is proven end to end by
 * `test/ts/deploy-v14.test.ts`, which deploys the whole stack from patched bytecode and then runs an FHE
 * op, charging HCU through HCULimit's `onlyFHEVMExecutor` gate. A wrong executor address there would
 * revert every operation.
 *
 * So byte-identity is a stronger claim than solc guarantees. The test below asserts it wherever it holds
 * (13 of 14 contracts) and falls back to the property that IS guaranteed — the patch replaces every
 * occurrence of every address, and the result carries exactly the addresses a real compile carries. The
 * set is pinned: if another contract starts reordering, or HCULimit stops, this test fails.
 */
const SOLC_REORDERS_CONSTANT_POOL: readonly string[] = ['HCULimit'];

function countOccurrences(haystack: string, needle: string): number {
  let count = 0;
  for (let i = haystack.indexOf(needle); i !== -1; i = haystack.indexOf(needle, i + 1)) {
    count++;
  }
  return count;
}

/**
 * The address-level equivalence that survives a constant-pool permutation: same code size, every
 * placeholder gone, and each address present exactly as often as in a real compile. A template that
 * missed an occurrence fails the count check.
 */
function assertAddressesEquivalent(contractName: string, patched: string, fresh: string, field: string): void {
  assert.equal(patched.length, fresh.length, `${contractName}.${field} length differs from the Forge build`);

  for (const name of ADDRESS_NAMES) {
    const placeholder = normalizeHex(getAddressReference(readTemplate(contractName), name).placeholder, name);
    const replacement = normalizeHex(ALTERNATE_ADDRESSES[name], name);

    assert.equal(
      countOccurrences(patched, placeholder),
      0,
      `${contractName}.${field} still contains the ${name} placeholder after patching`,
    );
    assert.equal(
      countOccurrences(patched, replacement),
      countOccurrences(fresh, replacement),
      `${contractName}.${field} contains ${name} a different number of times than the Forge build`,
    );
  }
}

void test('patched templates match a Forge build compiled with different config addresses', () => {
  const originalConfig = readFileSync(CONFIG_PATH, 'utf8');

  try {
    writeFileSync(CONFIG_PATH, addressConfigSource(ALTERNATE_ADDRESSES));
    forge(['clean']);
    forge(['build']);

    const reordered: string[] = [];

    for (const target of TARGET_CONTRACTS) {
      const template = readTemplate(target.contractName);
      const artifact = readArtifact(target);

      const patchedInit = patchBytecode(template, 'bytecode', ALTERNATE_ADDRESSES);
      const patchedRuntime = patchBytecode(template, 'deployedBytecode', ALTERNATE_ADDRESSES);
      const freshInit = lowerHex(artifact.bytecode.object);
      const freshRuntime = lowerHex(artifact.deployedBytecode.object);

      if (patchedInit === freshInit && patchedRuntime === freshRuntime) {
        continue;
      }

      // Not byte-identical: only acceptable for a known constant-pool reordering, and only if the
      // addresses themselves still line up exactly.
      reordered.push(target.contractName);
      assertAddressesEquivalent(
        target.contractName,
        normalizeHex(patchedInit, 'p'),
        normalizeHex(freshInit, 'f'),
        'bytecode',
      );
      assertAddressesEquivalent(
        target.contractName,
        normalizeHex(patchedRuntime, 'p'),
        normalizeHex(freshRuntime, 'f'),
        'deployedBytecode',
      );
    }

    assert.deepEqual(
      reordered.sort(),
      [...SOLC_REORDERS_CONSTANT_POOL].sort(),
      'the set of contracts whose constant pool solc reorders has changed — see the comment above',
    );
  } finally {
    restoreConfigAndGeneratedArtifacts(originalConfig);
  }
});
