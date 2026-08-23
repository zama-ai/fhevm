import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { basename, join } from 'node:path';
import test from 'node:test';
import {
  CODE_KIND,
  ADDRESSES_OUTPUT_PATH as LOCAL_HOST_ADDRESSES_PATH,
  OUTPUT_PATH as LOCAL_HOST_BYTECODE_PATH,
  localHostAddresses,
} from '../internal/generateLocalHostBytecode.ts';
import {
  ADDRESS_NAMES,
  CONSTANT_NAMES,
  PACKAGE_ROOT_ABS_PATH,
  PKG_DIR_ABS_PATH,
  ZAMA_LOCAL_CONFIG,
  type AddressName,
} from '../internal/constants.ts';
import {
  PATCH_SITES_PATH,
  patchSiteCounts,
  TARGET_CONTRACTS,
  artifactPathFor,
  type Artifact,
  type HexString,
} from '../internal/generateTemplates.ts';
import { normalizeHex, readJson } from '../internal/utils.ts';
import { derivePlaceholder } from '../internal/generatePlaceholders.ts';

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

const CONFIG_PATH = `${PACKAGE_ROOT_ABS_PATH}/internal/placeholders/addresses.sol`;

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
} satisfies Record<AddressName, HexString>;

function lowerHex(value: HexString): HexString {
  return `0x${normalizeHex(value, 'hex value')}`;
}

function readTemplate(contractName: string): Template {
  return readJson<Template>(`${PKG_DIR_ABS_PATH}/templates/${contractName}.json`);
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
address constant CLEARTEXT_ARITHMETIC_ADDRESS = address(${addresses.CLEARTEXT_ARITHMETIC_ADDRESS});
address constant CLEARTEXT_DB_ADDRESS = address(${addresses.CLEARTEXT_DB_ADDRESS});
address constant PAUSER_SET_ADDRESS = address(${addresses.PAUSER_SET_ADDRESS});
`;
}

function run(command: string, args: string[]): void {
  try {
    execFileSync(command, args, { cwd: PACKAGE_ROOT_ABS_PATH, encoding: 'utf8', stdio: 'pipe' });
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

void test('patched templates match a Forge build compiled with different config addresses', () => {
  const originalConfig = readFileSync(CONFIG_PATH, 'utf8');

  try {
    writeFileSync(CONFIG_PATH, addressConfigSource(ALTERNATE_ADDRESSES));
    forge(['clean']);
    forge(['build']);

    for (const target of TARGET_CONTRACTS) {
      const template = readTemplate(target.contractName);
      const artifact = readArtifact(target);

      assert.equal(
        patchBytecode(template, 'bytecode', ALTERNATE_ADDRESSES),
        lowerHex(artifact.bytecode.object),
        `${target.contractName} bytecode should match alternate-address Forge output`,
      );
      assert.equal(
        patchBytecode(template, 'deployedBytecode', ALTERNATE_ADDRESSES),
        lowerHex(artifact.deployedBytecode.object),
        `${target.contractName} deployedBytecode should match alternate-address Forge output`,
      );
    }
  } finally {
    restoreConfigAndGeneratedArtifacts(originalConfig);
  }
});

void test('patch-site counts match the committed baseline', () => {
  // A review tripwire (see PATCH_SITES_PATH in internal/generateTemplates.ts). Nothing here knows
  // whether a placeholder *should* be patched — that needs per-contract AST resolution. What this does
  // is refuse to let the numbers move silently: an upstream src/contracts/ sync that changes how an
  // address is used, a solc bump, or a different optimizer setting all surface here.
  const baseline = readJson<Record<string, Record<string, number>>>(PATCH_SITES_PATH);
  const live: Record<string, Record<string, number>> = {};
  for (const target of TARGET_CONTRACTS) {
    live[target.contractName] = patchSiteCounts(readTemplate(target.contractName).addressReferences);
  }

  const drift: string[] = [];
  for (const contractName of new Set([...Object.keys(baseline), ...Object.keys(live)])) {
    const was = baseline[contractName];
    const now = live[contractName];
    if (was === undefined) {
      drift.push(`${contractName}: new contract, not in the baseline`);
      continue;
    }
    if (now === undefined) {
      drift.push(`${contractName}: in the baseline but no longer generated`);
      continue;
    }
    for (const name of ADDRESS_NAMES) {
      if (was[name] !== now[name]) {
        drift.push(`${contractName}.${name}: baseline ${String(was[name])} -> now ${String(now[name])}`);
      }
    }
  }

  assert.deepEqual(
    drift,
    [],
    `patch sites moved. Review each line: a count falling to 0 for an address the contracts still use ` +
      `means the deploy would bake in a placeholder. Once understood, refresh the baseline at ` +
      `internal/placeholders/patch-sites.json.\n  ${drift.join('\n  ')}`,
  );
});

void test('template patching reproduces a Forge build compiled against a fresh high-entropy config', () => {
  // The load-bearing test for the whole placeholder technique.
  //
  // It proves the equivalence the deploy relies on: patching the recorded offsets in a prebuilt
  // template produces *exactly* the bytecode solc would have emitted had the real addresses been
  // compiled in from the start. If that ever stops holding, every deployed contract is subtly wrong
  // while every other test still passes.
  //
  // Two things make it stronger than the ALTERNATE_ADDRESSES test above:
  //   - the addresses are high-entropy (keccak-derived), not digit runs. Structured values can be
  //     found and patched correctly by luck: they share long byte runs, so an offset that is slightly
  //     wrong may still land on plausible-looking bytes. Random addresses have no such forgiveness.
  //   - the config is written to a dedicated tmp folder and reached by temporarily repointing
  //     remappings.txt, so internal/placeholders/addresses.sol is never modified. The comparison build
  //     also goes to its own --out, leaving the committed artifacts untouched.
  const tmpDir = join(PACKAGE_ROOT_ABS_PATH, 'test', '.tmp-altconfig');
  const tmpOut = join(tmpDir, 'out');
  const remappingsPath = join(PACKAGE_ROOT_ABS_PATH, 'remappings.txt');
  const originalRemappings = readFileSync(remappingsPath, 'utf8');

  const freshAddresses = Object.fromEntries(
    ADDRESS_NAMES.map((name) => [name, derivePlaceholder(`altconfig.${name}`)]),
  ) as Record<AddressName, HexString>;

  // Distinct values, or a wrong-offset patch could be masked by a coincidental match.
  assert.equal(new Set(Object.values(freshAddresses)).size, ADDRESS_NAMES.length, 'fresh addresses must be unique');

  try {
    mkdirSync(tmpDir, { recursive: true });
    writeFileSync(join(tmpDir, 'addresses.sol'), addressConfigSource(freshAddresses));
    writeFileSync(
      remappingsPath,
      originalRemappings.replace(/^fhevm-config-[^=]+=.*$/m, (line) => {
        const prefix = line.split('=')[0];
        return `${prefix ?? ''}=test/.tmp-altconfig/`;
      }),
    );

    forge(['build', '--out', tmpOut]);

    for (const target of TARGET_CONTRACTS) {
      const template = readTemplate(target.contractName);
      const artifact = readJson<Artifact>(join(tmpOut, basename(target.sourcePath), `${target.contractName}.json`));

      assert.equal(
        patchBytecode(template, 'bytecode', freshAddresses),
        lowerHex(artifact.bytecode.object),
        `${target.contractName}: patched template bytecode must equal the fresh-config Forge build`,
      );
      assert.equal(
        patchBytecode(template, 'deployedBytecode', freshAddresses),
        lowerHex(artifact.deployedBytecode.object),
        `${target.contractName}: patched template deployedBytecode must equal the fresh-config Forge build`,
      );
    }
  } finally {
    writeFileSync(remappingsPath, originalRemappings);
    rmSync(tmpDir, { recursive: true, force: true });
  }
});

////////////////////////////////////////////////////////////////////////////////
// pkg/src/generated/LocalHostBytecode.sol
////////////////////////////////////////////////////////////////////////////////

/** Reads the address block the generator writes into the header as `///   NAME = 0x…`. */
function declaredAddresses(source: string): Record<AddressName, HexString> {
  // Partial, not a cast to the full record: the values come from a regex over a generated file, so
  // "every name is present" is exactly what this has to check rather than assume.
  const parsed = new Map<string, HexString>(
    [...source.matchAll(/^\/\/\/\s+([A-Z_]+_ADDRESS) = (0x[0-9a-fA-F]{40})$/gm)].map((match) => [
      match[1] ?? '',
      (match[2] ?? '') as HexString,
    ]),
  );

  const declared = {} as Record<AddressName, HexString>;
  for (const name of ADDRESS_NAMES) {
    const value = parsed.get(name);
    assert.ok(value !== undefined, `${name} missing from the LocalHostBytecode.sol header`);
    declared[name] = value;
  }
  return declared;
}

/** Reads every `bytes constant NAME_(CREATION|RUNTIME)_CODE = hex"…";` declaration. */
function declaredBlobs(source: string): Map<string, { readonly suffix: string; readonly hex: HexString }> {
  const blobs = new Map<string, { readonly suffix: string; readonly hex: HexString }>();
  for (const match of source.matchAll(/bytes constant ([A-Z0-9_]+)_(CREATION|RUNTIME)_CODE\s*=\s*hex"([0-9a-f]*)";/g)) {
    blobs.set(match[1] ?? '', { suffix: match[2] ?? '', hex: (match[3] ?? '') as HexString });
  }
  return blobs;
}

void test('LocalHostBytecode.sol declares the ZamaConfig localhost addresses', () => {
  // Rules 15 and 17. The generator asserts this too, but the committed file is what ships: if it were
  // produced with a different mnemonic or start nonce, every ZamaConfig consumer would talk to nothing.
  const declared = declaredAddresses(readFileSync(LOCAL_HOST_BYTECODE_PATH, 'utf8'));

  assert.equal(declared.ACL_ADDRESS.toLowerCase(), ZAMA_LOCAL_CONFIG.aclAddress.toLowerCase());
  assert.equal(declared.FHEVM_EXECUTOR_ADDRESS.toLowerCase(), ZAMA_LOCAL_CONFIG.fhevmExecutorAddress.toLowerCase());
  assert.equal(declared.KMS_VERIFIER_ADDRESS.toLowerCase(), ZAMA_LOCAL_CONFIG.kmsVerifierAddress.toLowerCase());

  // The whole set, not just the three anchors: a wrong deployer index shifts every address at once.
  const { byName } = localHostAddresses();
  for (const name of ADDRESS_NAMES) {
    assert.equal(declared[name].toLowerCase(), byName[name].toLowerCase(), `${name} in the header`);
  }
});

void test('LocalHostBytecode.sol blobs equal the committed templates patched with those addresses', () => {
  // Ties the two pipelines together. The generated file is compiled against real addresses; the
  // templates are compiled against markers and patched. Test 5 above proves those are equivalent, so
  // these must agree exactly — and if the sources moved without regenerating, they will not.
  const source = readFileSync(LOCAL_HOST_BYTECODE_PATH, 'utf8');
  const declared = declaredAddresses(source);
  const blobs = declaredBlobs(source);

  assert.equal(blobs.size, TARGET_CONTRACTS.length, 'one blob per target contract');

  for (const target of TARGET_CONTRACTS) {
    // CONSTANT_NAMES is total over ContractName, so completeness is a compile-time property here.
    const constantName = CONSTANT_NAMES[target.contractName];
    const blob = blobs.get(constantName);
    assert.ok(blob !== undefined, `${constantName} missing from LocalHostBytecode.sol`);

    const kind = CODE_KIND[target.contractName];
    assert.equal(blob.suffix, kind === 'runtime' ? 'RUNTIME' : 'CREATION', `${target.contractName} code kind`);

    const field = kind === 'runtime' ? 'deployedBytecode' : 'bytecode';
    // `hex"…"` literals carry no 0x prefix; patchBytecode returns one.
    assert.equal(
      `0x${blob.hex}`,
      patchBytecode(readTemplate(target.contractName), field, declared),
      `${constantName}_${blob.suffix}_CODE must equal its template patched with the localhost addresses`,
    );
  }

  // A surviving marker would mean the config injection silently failed.
  for (const name of ADDRESS_NAMES) {
    const marker = derivePlaceholder(`fhevm.placeholder.${name}`).slice(2).toLowerCase();
    for (const [constantName, blob] of blobs) {
      assert.ok(!blob.hex.includes(marker), `${constantName} still contains the ${name} placeholder`);
    }
  }
});

void test('generated interfaces cover every target and share one FheType', () => {
  const interfaceDir = join(PKG_DIR_ABS_PATH, 'forge', 'src', '_internal', 'interfaces');

  for (const target of TARGET_CONTRACTS) {
    const path = join(interfaceDir, `I${target.contractName}.sol`);
    const source = readFileSync(path, 'utf8');

    assert.match(source, /^\/\/ SPDX-License-Identifier: BSD-3-Clause-Clear\n/, `${path}: SPDX`);
    // ^0.8.24: the payload's own floor, and what the harness pins so test/FhevmDeploy.t.sol can compile
    // these files. It also accepts every consumer a ^0.8.27 pragma would.
    assert.match(source, /^pragma solidity \^0\.8\.24;$/m, `${path}: pragma`);
    assert.match(source, new RegExp(`interface I${target.contractName} \\{`), `${path}: interface name`);

    // cast emits `type FheType is uint8;` inside each interface, which makes a distinct type per file.
    // The generator rewrites it to the shared enum; if that ever stops firing the types stop unifying.
    assert.doesNotMatch(source, /type FheType is uint8;/, `${path}: local FheType must be rewritten`);
    if (/\bFheType\b/.test(source)) {
      assert.match(
        source,
        /import \{FheType\} from "\.\.\/\.\.\/\.\.\/\.\.\/src\/contracts\/shared\/FheType\.sol";/,
        `${path}: import`,
      );
    }
  }

  assert.equal(
    readdirSync(interfaceDir).filter((name) => name.endsWith('.sol')).length,
    TARGET_CONTRACTS.length,
    'no stale interfaces left behind',
  );
});

void test('LocalHostAddresses.sol agrees with the derivation and with LocalHostBytecode.sol', () => {
  // The two files are halves of one artifact: the bytecode is compiled against these addresses. If they
  // ever disagree, a consumer deploys code that points somewhere the addresses say nothing about.
  const source = readFileSync(LOCAL_HOST_ADDRESSES_PATH, 'utf8');
  const { deployer, byName, nonceSequence } = localHostAddresses();

  assert.match(source, new RegExp(`address constant DEPLOYER_ADDRESS = ${deployer};`), 'deployer constant');
  assert.match(source, /uint64 constant DEPLOYER_START_NONCE = 0;/, 'start nonce');
  assert.match(source, /uint32 constant DEPLOYER_ADDRESS_INDEX = \d+;/, 'deployer index');
  assert.match(source, /string constant MNEMONIC = "(?:[a-z]+ ){11,}[a-z]+";/, 'mnemonic');

  for (const name of ADDRESS_NAMES) {
    assert.match(source, new RegExp(`address constant ${name} = ${byName[name]};`), `${name} constant`);
  }

  // Every nonce of the sequence is documented, and each named row carries that address.
  nonceSequence.forEach((address, nonce) => {
    assert.ok(source.includes(address), `nonce ${String(nonce)} address ${address} missing from the order table`);
  });
  for (const name of ADDRESS_NAMES) {
    assert.match(source, new RegExp(`nonce\\s+\\d+\\s+${byName[name]}\\s+${name}`), `${name} in the order table`);
  }

  // Same address set as the bytecode file's header — the halves must not drift apart.
  const fromBytecode = declaredAddresses(readFileSync(LOCAL_HOST_BYTECODE_PATH, 'utf8'));
  for (const name of ADDRESS_NAMES) {
    assert.equal(fromBytecode[name].toLowerCase(), byName[name].toLowerCase(), `${name} across the two files`);
  }
});
